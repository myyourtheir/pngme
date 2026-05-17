mod args;
mod chunk;
mod chunk_type;
mod commands;
mod error;
mod png;

use clap::Parser;
use crate::args::App;
use crate::args::PngMeArgs::*;
use crate::error::Error;
use crate::commands::*;

pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = App::parse();
    match args.command {
        Encode(a) => { encode(a) },
        Decode(a) => { decode(a) },
        Remove(a) => { remove(a) },
        Print(a) => { print(a) },
    }?;

    Ok(())
}