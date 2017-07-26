use std::io::{Read, BufRead, BufReader};
use std::collections::HashMap;
use std::{self, fmt};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ResponseParseError {
	InvalidStatusLine,
	InvalidHeader,
}
impl std::error::Error for ResponseParseError {
	fn description(&self) -> &str {
		match *self {
			ResponseParseError::InvalidStatusLine => "Invalid status line: Missing parameters",
			ResponseParseError::InvalidHeader => "Invalid header: No value",
		}
	}
}
impl fmt::Display for ResponseParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use std::error::Error;
		write!(f, "{}", self.description())
	}
}

pub struct Response<Stream: Read> {
	pub http_version: String,
	pub status: u16,
	pub description: String,
	pub headers: HashMap<String, String>,
	pub body: BufReader<Stream>,
}
impl<Stream: Read> Response<Stream> {
	pub fn new(mut stream: BufReader<Stream>) -> Result<Response<Stream>, Box<std::error::Error>> {
		let mut status = String::new();
		stream.read_line(&mut status)?;
		let mut parts = status.split(char::is_whitespace);

		let http_version = parts.next().unwrap();
		let status = match parts.next() {
			Some(field) => field,
			None => return Err(Box::new(ResponseParseError::InvalidStatusLine)),
		};
		let status = status.parse()?;
		let description = match parts.next() {
			Some(field) => field,
			None => return Err(Box::new(ResponseParseError::InvalidStatusLine)),
		};

		let mut headers = HashMap::new();

		loop {
			let mut line = String::new();
			stream.read_line(&mut line)?;
			let line = line.trim();
			if line.is_empty() {
				break;
			}

			let mut parts = line.split(":");
			headers.insert(parts.next().unwrap().trim().to_string(), match parts.next() {
				Some(field) => field.trim().to_string(),
				None => return Err(Box::new(ResponseParseError::InvalidHeader)),
			});
		}

		Ok(Response {
			http_version: http_version.to_string(),
			status: status,
			description: description.to_string(),
			headers: headers,
			body: stream,
		})
	}
}
