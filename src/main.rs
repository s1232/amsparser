use hdlc::*;
use std::error::Error;
use std::{
    io::{Read, Write},
    process,
};

fn read() -> Result<Vec<u8>, Box<dyn Error>> {
    let mut s = std::io::stdin();
    let mut buffer = Vec::new();
    s.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn extract_reading() -> Result<(), Box<dyn Error>> {
    let input = read()?;
    let decoded = decode(&input, SpecialChars::default());
    if decoded.is_err() {
        return Err("Could not read input")?;
    }
    let decoded = decoded.unwrap();
    let control_byte = &decoded[5];
    let packet_type = &decoded[18];
    // Control byte is Ox13
    if *control_byte == 0x13 {
        match packet_type {
            0x1 => {
                let value = u32::from_be_bytes(decoded[30..34].try_into()?);
                std::io::stdout().write_all(value.to_string().as_bytes())?
            }
            0x9 | 0xC | 0xD | 0x0E | 0x11 | 0x12 => {
                let value = u32::from_be_bytes(decoded[97..101].try_into()?);
                std::io::stdout().write_all(value.to_string().as_bytes())?
            }
            _ => std::io::stdout().write_all(b"No match")?,
        }
    }
    Ok(())
}

fn main() {
    if let Err(_) = extract_reading() {
        process::exit(1);
    }
}
