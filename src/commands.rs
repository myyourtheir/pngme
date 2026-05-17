use std::fs;
use std::fs::File;
use std::io::{Write};
use std::path::Path;
use std::str::FromStr;
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::error::Error::FileNotFound;
use crate::Result;
use crate::png::Png;

pub fn encode(args: EncodeArgs) -> Result<()> {
    println!("command: encode");
    let bytes = Png::from_file(&args.path)?;
    let mut png = Png::try_from(bytes.as_slice())?;
    let chunk = Chunk::new(ChunkType::from_str(&args.chunk_type)?, args.message.as_bytes().to_owned());
    png.append_chunk(chunk);
    if let Err(_) = fs::write(&args.path, png.as_bytes()) {
        return Err(FileNotFound)
    }

    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    println!("command: decode");
    let bytes = Png::from_file(&args.path)?;
    let png = Png::try_from(bytes.as_slice())?;
    let message_chunk_option = png.chunk_by_type(&args.chunk_type);
    match message_chunk_option {
        Some(c) => {
            println!("{}: {}", &args.chunk_type, String::from_utf8(c.data().to_vec()).unwrap());
        },
        None => {
            println!("Did not find any chunk with type: {}", &args.chunk_type);
        }
    }

Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    println!("command: remove");
    let bytes = Png::from_file(&args.path)?;
    let mut png = Png::try_from(bytes.as_slice())?;
    loop {
        match png.remove_first_chunk(&args.chunk_type) {
            Err(_) => break,
            _ => continue
        }
    }
    print!("{}", png);
    to_file(&args.path, png.as_bytes().as_slice())?;
    Ok(())
}

pub fn print(args: PrintArgs) -> Result<()> {
    println!("command: print");
    let bytes = Png::from_file(&args.path)?;
    let png = Png::try_from(bytes.as_slice())?;
    print!("{}", png);

    Ok(())
}



fn to_file<P: AsRef<Path>>(path: P, bytes: &[u8]) -> Result<()> {
    if let Ok(mut file) = File::create(path) {
        file.write_all(bytes).unwrap();
        return Ok(());
    }
    return Err(FileNotFound);
}