use std::{self, fmt};
use std::collections::HashMap;
#[cfg(feature = "http")]
use std::io;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ResponseParseError {
	InvalidStatusLine,
	InvalidHeader
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

/// Response struct
pub struct Response<Stream: Read> {
	pub http_version: String,
	pub status: u16,
	pub description: String,
	pub headers: HashMap<String, Vec<u8>>,
	pub body: BufReader<Stream>
}
impl<Stream: Read> Response<Stream> {
	/// Parse a stream into a response struct
	pub fn new(mut stream: BufReader<Stream>) -> Result<Response<Stream>, Box<std::error::Error>> {
		let mut status = String::new();
		stream.read_line(&mut status)?;
		let mut parts = status.split_whitespace();

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

			let mut parts = line.splitn(2, ':');
			headers.insert(
				parts.next().unwrap().trim().to_string(),
				match parts.next() {
					Some(field) => field.trim().as_bytes().to_vec(),
					None => return Err(Box::new(ResponseParseError::InvalidHeader)),
				}
			);
		}

		Ok(Response {
			http_version: http_version.to_string(),
			status: status,
			description: description.to_string(),
			headers: headers,
			body: stream
		})
	}

	#[cfg(not(feature = "http"))]
	/// Returns true if self.status is 2XX, false otherwise
	pub fn is_success(&self) -> bool { (self.status as f32 / 100.0).floor() == 2.0 }

	#[cfg(feature = "http")]
	pub fn try_into(mut self) -> io::Result<::http::Response<Vec<u8>>> {
		let mut body = Vec::new();
		self.body.read_to_end(&mut body)?;

		let headers: Vec<(&str, &[u8])> = self.headers.iter().map(|(k, v)| (&**k, &**v)).collect();

		Ok(
			::http::Response::builder()
				.version(match &*self.http_version {
					"HTTP/2.0" => ::http::version::HTTP_2,
					"HTTP/0.9" => ::http::version::HTTP_09,
					"HTTP/1.0" => ::http::version::HTTP_10,
					"HTTP/1.1" => ::http::version::HTTP_11,
					_ => ::http::version::HTTP_10, // Default because bother printing an error
				})
				.status(self.status)
				.extension(self.description)
				.headers(headers)
				.body(body)
				.unwrap()
		)
	}
}
