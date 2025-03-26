use std::io::{BufRead, BufReader};

use crate::asset::Mesh;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid header")]
    InvalidHeader,

    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid data format")]
    InvalidDataFormat,

    #[error("unknown statement")]
    UnknownStatement,
}

#[derive(Default)]
enum Format {
    #[default]
    BinaryLittleEndian,
    BinaryBigEndian,
}

#[derive(Default)]
struct ParserState {
    x_offset: usize,
    y_offset: usize,
    z_offset: usize,

    nx_offset: usize,
    ny_offset: usize,
    nz_offset: usize,

    u_offset: usize,
    v_offset: usize,

    parsed_offset: usize,
    format: Format,
}

impl ParserState {
    pub fn set_format(&mut self, format_line: &[u8]) -> Result<(), Error> {
        let mut parts = format_line.split(|x| x.is_ascii_whitespace());

        let _prefix = parts.next().ok_or_else(|| Error::InvalidDataFormat)?;
        let ty = parts.next().ok_or_else(|| Error::InvalidDataFormat)?;

        match ty {
            b"binary_little_endian" => self.format = Format::BinaryLittleEndian,
            b"binary_big_endian" => self.format = Format::BinaryBigEndian,
            _ => return Err(Error::InvalidDataFormat),
        }

        Ok(())
    }
}

pub fn parse_ply(data: &[u8]) -> Result<Mesh, Error> {
    let mut reader = BufReader::new(data);

    let mut line_buf = Vec::new();

    let mut state = ParserState::default();

    reader.read_until(b'\n', &mut line_buf)?;
    if line_buf.trim_ascii() != b"ply" {
        return Err(Error::InvalidHeader);
    }

    line_buf.clear();

    reader.read_until(b'\n', &mut line_buf)?;

    loop {
        line_buf.clear();
        reader.read_until(b'\n', &mut line_buf)?;

        let line = line_buf.trim_ascii();

        let mut parts = line.split(|x| x.is_ascii_whitespace());

        let Some(prefix) = parts.next() else {
            continue;
        };

        match prefix {
            b"comment" => {}
            _ => return Err(Error::UnknownStatement),
        }
    }

    unimplemented!()
}
