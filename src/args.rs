use clap::{Args, Parser, Subcommand};
use std::path::{PathBuf};


/// A tool to embed hidden message into PNG file.
#[derive(Debug, Parser)]
#[clap(name = "PNGme", version = "0.1.0", author = "Jakub Lelonkiewicz")]
pub struct App {
    #[clap(subcommand)]
    pub command: PngMeArgs,
}

/// Command
#[derive(Subcommand, Debug)]
#[command(version, about, long_about = None)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

/// Encode a message into specified file with given chunk type
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct EncodeArgs {
    /// Path to PNG file
    pub path: PathBuf,
    /// Chunk type to encode message into
    pub chunk_type: String,
    /// Message to be embedded
    pub  message: String,
}

/// Decode and display a message from given PNG file
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct DecodeArgs {
    /// Path to PNG file
    pub path: PathBuf,
    /// Chunk type to decode message from
    pub chunk_type: String,
}

/// Remove given chunk types from given PNG file
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct RemoveArgs {
    /// Path to PNG file
    pub path: PathBuf,
    /// Chunk type to remove message from
    pub chunk_type: String,
}

/// Print all chunks of given PNG file
#[derive(Args, Debug)]
#[command(version, about, long_about = None)]
pub struct PrintArgs {
    /// Path to PNG file
    pub path: PathBuf,
}