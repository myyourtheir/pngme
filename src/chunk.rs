use std::convert::TryFrom;
use std::fmt;
use std::io::{BufReader, Read};
use crc::{Crc, CRC_32_ISO_HDLC};
use crate::chunk_type::ChunkType;
use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct Chunk {
	length: u32,
	chunk_type: ChunkType,
	data: Vec<u8>,
	crc: u32
}
const MAX_CHUNK_DATA_LENGTH: u32 = 1 << 31;
const ALGORITHM: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

impl Chunk {

			pub fn new(chunk_type: ChunkType, data: Vec<u8>)->Self {
				let length:u32 = data.len().try_into().unwrap() ;
				let crc = ALGORITHM.checksum(&([&chunk_type.bytes(), data.as_slice()].concat()));

				return Chunk {
					data,
					chunk_type,
					length,
					crc
				}
			}
    /// The length of the data portion of this chunk.
    pub fn length(&self) -> u32 {
        return self.length;
    }

    /// The `ChunkType` of this chunk
    pub fn chunk_type(&self) -> &ChunkType {
        return &self.chunk_type;
    }

    /// The raw data contained in this chunk in bytes
    pub fn data(&self) -> &[u8] {
        return &self.data;
    }

    /// The CRC of this chunk
    pub fn crc(&self) -> u32 {
        return self.crc;
    }

    /// Returns the data stored in this chunk as a `String`. This function will return an error
    /// if the stored data is not valid UTF-8.
    pub fn data_as_string(&self) -> Result<String> {
        let string = String::from_utf8(self.data.to_vec());
				return  match string {
						Ok(s)=>Ok(s),
						Err(_)=>Err(Error::DataAsStringError)
				};
    }

    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        let len_bytes = self.length().to_be_bytes();
				let chunk_type_bytes = self.chunk_type().bytes();
				let data_bytes = self.data();
				let crc_bytes = self.crc().to_be_bytes();

				let mut vec:Vec<u8>	 = Vec::new();
				for b in len_bytes {
					vec.push(b);
				}
				for b in chunk_type_bytes {
					vec.push(b);
				}
				for b in data_bytes {
					vec.push(*b);
				}

				for b in crc_bytes{
					vec.push(b);
				}
				return vec;

    }
}

impl TryFrom<&[u8]> for Chunk{
	type Error = Error;

	fn try_from(value: &[u8]) -> Result<Self> {
			 if value.len() < 4 * 3 {
            return Err(Error::NotEnoughBytes);
        }

        let mut reader = BufReader::new(value);
        let mut input_buf: [u8; 4] = [0; 4];

        reader.read_exact(&mut input_buf).unwrap();
        let length = u32::from_be_bytes(input_buf);
        if length > MAX_CHUNK_DATA_LENGTH || length as usize > value.len() - 3 * 4 {
            return Err(Error::DataLengthToBig);
        }

        reader.read_exact(&mut input_buf).unwrap();
        let chunk_type = ChunkType::try_from(input_buf)?;

        let mut data: Vec<u8> = vec![0; length as usize];
        reader.read_exact(&mut data).unwrap();

        reader.read_exact(&mut input_buf).unwrap();
        let crc = u32::from_be_bytes(input_buf);
        let crc_test = ALGORITHM.checksum(&([&chunk_type.bytes(), data.as_slice()].concat()));
        if crc != crc_test {
            return Err(Error::InvalidChecksum);
        }

        Ok(Self {
            length,
            chunk_type,
            data,
            crc,
        })
    }
	}


impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}