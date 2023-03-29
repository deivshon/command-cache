use core::convert;
use std::str::Utf8Error;
use std::fmt;

pub enum ParseError {
	TimestampErr,
	UTF8Err(Utf8Error),
	MalformedCache
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::TimestampErr => write!(f, "Error parsing timestamp from binary cache"),
			ParseError::UTF8Err(e) => write!(f, "Error parsing UTF-8 encoded output from cache: {}", e),
			ParseError::MalformedCache => write!(f, "Error parsing cache: Malformed cache file")
        }
    }
}

impl convert::From<Utf8Error> for ParseError {
	fn from(err: Utf8Error) -> ParseError {
		return ParseError::UTF8Err(err)
	}
}

pub struct Cache {
	pub ts: u128,
	pub output: String
}

impl convert::TryFrom<Vec<u8>> for Cache {
	type Error = ParseError;

	fn try_from(bytes: Vec<u8>) -> Result<Cache, ParseError> {
		if bytes.len() < 16 {
			return Err(ParseError::MalformedCache)
		}

		let Ok(ts_bytes) = bytes[0..16].try_into() else {
			return Err(ParseError::TimestampErr)
		};
		let ts = u128::from_le_bytes(ts_bytes);

		let output = std::str::from_utf8(&bytes[16..])?
			.to_string();
		
		return Ok(Cache {
			ts: ts,
			output: output
		})
	}
}

impl Cache {
	pub fn as_bytes(&self) -> Vec<u8> {
		let ts_bytes: &[u8] = &self.ts.to_le_bytes();
		let output_bytes = self.output.as_bytes();
		
		return [ts_bytes, output_bytes].concat();
	}
}
