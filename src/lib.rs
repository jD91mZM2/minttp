use std::collections::HashMap;
use std::io::{self, BufReader, Write};
use std::net::TcpStream;

pub mod consts;
pub mod response;
pub mod url;
use response::Response;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DYIRequest<'a> {
	pub host: &'a str,
	pub port: u16,
	pub method: &'a str,
	pub path: &'a str,
	pub http_version: &'a str,
	pub headers: &'a HashMap<&'a str, &'a str>,
	pub body: Option<&'a [u8]>
}
pub fn diy_request(req: &DYIRequest) -> io::Result<TcpStream> {
	let mut stream = TcpStream::connect((req.host, req.port))?;

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
	pub body: Option<Vec<u8>>
}

impl Request {
	pub fn new(url: Url) -> Request {
		Request {
			url: url,
			method: consts::GET.to_string(),
			headers: HashMap::new(),
			body: None
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
}

pub fn request(req: &Request) -> Result<Response<TcpStream>, Box<std::error::Error>> {
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

	let response = diy_request(&DYIRequest {
		host: &req.url.host,
		port: req.url.port,
		method: &req.method,
		path: &req.url.fullpath,
		http_version: "1.1",
		headers: &headers,
		body: req.body.as_ref().map(|vec| &**vec)
	})?;

	Response::new(BufReader::new(response))
}

macro_rules! gen_func {
	(nobody $name:ident, $method:expr) => {
		pub fn $name(url: Url) -> Result<Response<TcpStream>, Box<std::error::Error>> {
			request(&Request::new(url).method($method))
		}
	};
	(body $name:ident, $method:expr) => {
		pub fn $name(url: Url, body: Vec<u8>) -> Result<Response<TcpStream>, Box<std::error::Error>> {
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
