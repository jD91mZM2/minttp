#[cfg(feature = "openssl")]
extern crate openssl;

use std::collections::HashMap;
use std::io::{self, BufReader, Write};
use std::net::TcpStream;
#[cfg(feature = "openssl")]
use openssl::ssl::{SslMethod, SslConnectorBuilder, SslStream};

pub mod consts;
pub mod response;
pub mod url;
use response::Response;
use url::Url;

pub enum HttpStream {
	Plain(TcpStream),
	#[cfg(feature = "openssl")]
	TLS(SslStream<TcpStream>),
}

macro_rules! perform {
	($self:expr, $fn:ident) => {
		match *$self {
			HttpStream::Plain(ref mut stream) => stream.$fn(),
			#[cfg(feature = "openssl")]
			HttpStream::TLS(ref mut stream) => stream.$fn(),
		}
	};
	($self:expr, $fn:ident, $($args:expr),*) => {
		match *$self {
			HttpStream::Plain(ref mut stream) => stream.$fn($($args),*),
			#[cfg(feature = "openssl")]
			HttpStream::TLS(ref mut stream) => stream.$fn($($args),*),
		}
	}
}
impl Write for HttpStream {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> { perform!(self, write, buf) }
	fn flush(&mut self) -> io::Result<()> { perform!(self, flush) }
	fn write_all(&mut self, buf: &[u8]) -> io::Result<()> { perform!(self, write_all, buf) }
	fn write_fmt(&mut self, fmt: std::fmt::Arguments) -> io::Result<()> { perform!(self, write_fmt, fmt) }
}
impl io::Read for HttpStream {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { perform!(self, read, buf) }
	fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> { perform!(self, read_to_end, buf) }
	fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> { perform!(self, read_to_string, buf) }
	fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> { perform!(self, read_exact, buf) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DYIRequest<'a> {
	pub ssl: bool,
	pub host: &'a str,
	pub port: u16,
	pub method: &'a str,
	pub path: &'a str,
	pub http_version: &'a str,
	pub headers: &'a HashMap<&'a str, &'a str>,
	pub body: Option<&'a [u8]>
}
pub fn diy_request(req: &DYIRequest) -> Result<HttpStream, Box<std::error::Error>> {
	let mut stream = if req.ssl {
		#[cfg(feature = "openssl")]
		{
			let builder = SslConnectorBuilder::new(SslMethod::tls())?.build();
			let stream = TcpStream::connect((req.host, req.port))?;
			HttpStream::TLS(builder.connect(req.host, stream)?)
		}
		#[cfg(not(feature = "openssl"))]
		panic!("Can't use SSL without the --feature \"openssl\"");
	} else {
		HttpStream::Plain(TcpStream::connect((req.host, req.port))?)
	};

	write!(
		stream,
		"{} {} HTTP/{}\r\n",
		req.method,
		req.path,
		req.http_version
	)?;

	for (name, value) in req.headers {
		let mut bytes: Vec<_> = name.bytes().collect();
		bytes.push(':' as u32 as u8);
		bytes.append(&mut value.bytes().collect());
		bytes.push('\r' as u32 as u8);
		bytes.push('\n' as u32 as u8);

		stream.write_all(&bytes)?;
	}

	stream.write_all(b"\r\n")?;

	if let Some(body) = req.body {
		stream.write_all(body)?;
		stream.write_all(b"\r\n")?;
	}

	Ok(stream)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
	pub url: Url,
	pub method: String,
	pub headers: HashMap<String, String>,
	pub body: Option<Vec<u8>>,
}

impl Request {
	pub fn new(url: Url) -> Request {
		Request {
			url: url,
			method: consts::GET.to_string(),
			headers: HashMap::new(),
			body: None,
		}
	}

	pub fn url(mut self, url: Url) -> Self {
		self.url = url;
		self
	}
	pub fn method<S: Into<String>>(mut self, method: S) -> Self {
		self.method = method.into();
		self
	}
	pub fn header<S: Into<String>>(mut self, key: S, val: S) -> Self {
		self.headers.insert(key.into(), val.into());
		self
	}
	pub fn body(mut self, body: Vec<u8>) -> Self {
		self.body = Some(body);
		self
	}

	pub fn request(req: &Request) -> Result<Response<HttpStream>, Box<std::error::Error>> {
		request(req)
	}
}

pub fn request(req: &Request) -> Result<Response<HttpStream>, Box<std::error::Error>> {
	let _body;
	let mut headers: HashMap<&str, &str> = HashMap::new();
	for (key, val) in &req.headers {
		headers.insert(&*key, &*val);
	}

	headers.insert("Host", &req.url.host);
	headers.insert("Connection", "close");
	if let Some(ref body) = req.body {
		_body = body.len().to_string();
		headers.insert("Content-Length", &_body);
	}

	let request = DYIRequest {
		ssl: req.url.protocol == "https",
		host: &req.url.host,
		port: req.url.port,
		method: &req.method,
		path: &req.url.fullpath,
		http_version: "1.1",
		headers: &headers,
		body: req.body.as_ref().map(|vec| &**vec)
	};

	#[cfg(feature = "openssl")]
	println!("ssl");

	let response = diy_request(&request)?;
	Response::new(BufReader::new(response))
}

macro_rules! gen_func {
	(nobody $name:ident, $method:expr) => {
		pub fn $name(url: Url) -> Result<Response<HttpStream>, Box<std::error::Error>> {
			request(&Request::new(url).method($method))
		}
	};
	(body $name:ident, $method:expr) => {
		pub fn $name(url: Url, body: Vec<u8>) -> Result<Response<HttpStream>, Box<std::error::Error>> {
			request(&Request::new(url).method($method).body(body))
		}
	}
}
gen_func!(nobody get, consts::GET);
gen_func!(nobody head, consts::HEAD);
gen_func!(body post, consts::POST);
gen_func!(body put, consts::PUT);
gen_func!(nobody delete, consts::DELETE);
gen_func!(nobody connect, consts::CONNECT);
gen_func!(nobody trace, consts::TRACE);
gen_func!(body patch, consts::PATCH);
