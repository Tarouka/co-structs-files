/*
Formats:
	- Space Separated (Amount header, 0 Test 20 entry format)
	- INI
	- Binary
*/
pub struct SpaceSeparatedSerializer {
	buffer: String
}

impl SpaceSeparatedSerializer {
	pub fn new() -> SpaceSeparatedSerializer {
		SpaceSeparatedSerializer {
			buffer: String::new()
		}
	}

	pub fn push_value<T>(&mut self, value: &T)
		where T:ToString {
		self.buffer.push_str(&value.to_string());
		self.buffer.push_str(" ");
	}

	pub fn push_last_value<T>(&mut self, value: &T)
		where T:ToString {
		self.buffer.push_str(&value.to_string());
	}

	pub fn get_buffer(&self) -> String {
		self.buffer.clone()
	}
}

pub trait StringSerializableStructure {
	fn serialize(&self) -> String;
}