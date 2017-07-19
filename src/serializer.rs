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
		if self.buffer.len() > 0 {
			self.buffer.push_str(" ");
		}

		self.buffer.push_str(&value.to_string());
	}

	pub fn get_buffer(&self) -> String {
		self.buffer.clone()
	}
}

pub struct IniEntrySerializer {
	buffer: String
}

impl IniEntrySerializer {
	pub fn new() -> IniEntrySerializer {
		IniEntrySerializer {
			buffer: String::new()
		}
	}

	pub fn add_section<T>(&mut self, value: &T)
		where T:ToString {
		self.buffer.push_str("[");
		self.buffer.push_str(&value.to_string());
		self.buffer.push_str("]");
	}

	pub fn add_entry<T>(&mut self, key: &str, value: &T)
		where T:ToString {
		if self.buffer.len() > 0 {
			self.buffer.push_str("\n");
		}

		self.buffer.push_str(key);
		self.buffer.push_str("=");
		self.buffer.push_str(&value.to_string());
	}

	pub fn get_buffer(&self) -> String {
		self.buffer.clone()
	}
}

pub trait StringSerializableStructure {
	fn serialize(&self) -> String;
}