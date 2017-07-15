use std::str::FromStr;

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

