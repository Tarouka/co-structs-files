use std::str::FromStr;
use ini::Ini;

macro_rules! get_value_as {
	( $results:expr, $idx:expr, $type:tt, $field_name:expr ) => {
		{
			if let Result::Ok(result) = $results[0].get_value_as::<$type>() {
				result
			} else {
				return Result::Err(SpaceSeparatedParserError::InvalidCast(String::from($field_name).to_owned()));
			}
		}
	}
}

pub enum SpaceSeparatedParserError {
	InvalidCast(String),
	MissingAtIndex(usize)
}

pub fn get_space_separated_values(line: &String) -> Vec<StringParserResult> {
	line
		.split_whitespace()
		.map(|part| StringParserResult { value: String::from(part) })
		.collect()
}

pub trait SpaceSeparatedParseable<T> {
	fn from_line(line: &String) -> Result<T, SpaceSeparatedParserError>;
 }

pub struct StringParserResult {
	pub value: String
}

impl StringParserResult {
	pub fn get_value(&self) -> &String {
		&self.value
	}

	pub fn get_value_as<T>(&self) -> Result<T, T::Err>
		where T: FromStr {
		self.value.parse::<T>()
	}
}

pub enum IniEntryParserError {
	InvalidCast(String),
	MissingKey(String),
	InvalidIni,
	InvalidIniNoSection
}

pub trait IniEntryParseable<T> {
	fn from_section_string(section: &String) -> Result<T, IniEntryParserError>;
}

pub struct IniEntryParser {
	ini: Ini,
	section_name: String
}

impl IniEntryParser {
	pub fn new_from_str(value: String) -> Result<IniEntryParser, IniEntryParserError> {
		if let Result::Ok(ini) = Ini::load_from_str(&value) {
			return Result::Ok(IniEntryParser {
				section_name: if let Some(section_name) = ini.sections().nth(0) {
					section_name.clone().unwrap()
				} else {
					return Result::Err(IniEntryParserError::InvalidIniNoSection);
				},
				ini: ini
			})
		}

		Result::Err(IniEntryParserError::InvalidIni)
	}

	pub fn get_section(&self) -> String {
		self.section_name.clone()
	}

	pub fn get_section_as<T>(&self) -> Result<T, T::Err>
		where T: FromStr {
		self.section_name.parse::<T>()
	}

	pub fn get_entry_as<T>(&self, entry_key: &str) -> Result<T, IniEntryParserError>
		where T: FromStr {

		match self.get_entry(entry_key) {
			Ok(str_val) => {
				if let Result::Ok(parsed_val) = str_val.parse::<T>() {
					return Result::Ok(parsed_val);
				}

				return Result::Err(IniEntryParserError::InvalidCast(String::from(entry_key)));
			}

			Err(err) => {
				return Result::Err(err);
			}
		}

	}

	pub fn get_entry(&self, entry_key: &str) -> Result<String, IniEntryParserError> {
		if let Some(val) = self.ini.get_from(Option::Some(self.section_name.clone()), &entry_key) {
			return Result::Ok(val.to_string());
		}

		Result::Err(IniEntryParserError::MissingKey(entry_key.to_string()))
	}

}