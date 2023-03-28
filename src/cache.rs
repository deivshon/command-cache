use core::convert;

pub struct Cache {
	pub ts: u128,
	pub output: String
}

impl convert::From<Vec<u8>> for Cache {
	fn from(bytes: Vec<u8>) -> Cache {
		let ts = u128::from_le_bytes(
			bytes[0..16]
				.try_into()
				.expect("Error parsing timestamp from binary cache")
		);
		let output = std::str::from_utf8(&bytes[16..])
			.expect("Could not parse UTF-8 encoded output from cache")
			.to_string();
		
		return Cache {
			ts: ts,
			output: output
		}
	}
}

impl Cache {
	pub fn as_bytes(&self) -> Vec<u8> {
		let ts_bytes: &[u8] = &self.ts.to_le_bytes();
		let output_bytes = self.output.as_bytes();
		
		return [ts_bytes, output_bytes].concat();
	}
}
