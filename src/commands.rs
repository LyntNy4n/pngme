use crate::args::*;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;
use std::fs;
use std::str::FromStr;

pub fn encode(args: EncodeArgs) -> Result<()> {
    // check if file exists
    if !args.file_path.exists() {
        return Err("File does not exist".into());
    }
    let bytes = fs::read(args.file_path.clone())?;
    let mut png = Png::try_from(&bytes[..])?;
    let new_chunk = Chunk::new(
        ChunkType::from_str(&args.chunk_type)?,
        args.message.as_bytes().to_vec(),
    );
    png.append_chunk(new_chunk);
    // write to file
    if let Some(output) = args.output {
        fs::write(output, png.as_bytes())?;
    } else {
        fs::write(args.file_path, png.as_bytes())?;
    }
    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    // check if file exists
    if !args.file_path.exists() {
        return Err("File does not exist".into());
    }
    let bytes = fs::read(args.file_path)?;
    let png = Png::try_from(&bytes[..])?;
    // find chunk
    let chunk = png
        .chunks()
        .iter()
        .find(|chunk| chunk.chunk_type().to_string() == args.chunk_type)
        .ok_or("Chunk not found")?;
    // decode message
    let message: String = chunk.data().iter().map(|x| *x as char).collect();
    println!("{}", message);
    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    // check if file exists
    if !args.file_path.exists() {
        return Err("File does not exist".into());
    }
    let bytes = fs::read(args.file_path.clone())?;
    let mut png = Png::try_from(&bytes[..])?;
    // remove the chunk
    png.remove_chunk(&args.chunk_type)?;
    // write to file
    fs::write(args.file_path, png.as_bytes())?;
    Ok(())
}

pub fn print(args: PrintArgs) -> Result<()> {
    // check if file exists
    if !args.file_path.exists() {
        return Err("File does not exist".into());
    }
    let bytes = fs::read(args.file_path)?;
    let png = Png::try_from(&bytes[..])?;
    // print chunks
    for chunk in png.chunks() {
        println!("{}", chunk);
    }
    Ok(())
}
