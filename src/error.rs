use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid byte value, only [a-z] and [A-Z] ASCII")]
    InvalidByteValues,

    #[error(
        "Invalid str length, str used for construction must be 4 bytes long (aka. 4 characters)"
    )]
    InvalidStringLength,

    #[error("Failed to construct valid string from Chunk data")]
    DataAsStringError,

    #[error("Failed to construct Chunk from bytes: to little data, need at least 12 bytes")]
    NotEnoughBytes,

    #[error("Failed to validate checksum when constructing Chunk")]
    InvalidChecksum,

    #[error("Data length too big when constructing Chunk from bytes")]
    DataLengthToBig,

    #[error("Got invalid ChunkType when constructing Chunk from bytes")]
    InvalidChunkType,

    #[error("Chunk with specified ChunkType does not exist in Png file")]
    RemoveChunkError,

    #[error("Header in provided byte stream is not valid PNG header")]
    InvalidHeader,

    #[error("Given path did not match any file")]
    FileNotFound
}