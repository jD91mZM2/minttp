extern crate minttp;
use minttp::{DIYRequest, consts};
use minttp::response::Response;
use std::collections::HashMap;
use std::io::{BufReader, Read};

fn main() {
	let mut headers = HashMap::new();
	headers.insert("Host", "example.com".as_bytes());
	headers.insert("Connection", "close".as_bytes());

	let mut output = String::new();
	{
		let conn = minttp::diy_request(&DIYRequest {
			ssl: false,
			host: "example.com",
			port: 80,
			method: consts::GET,
			path: "/",
			http_version: "1.1",
			headers: &headers,
			body: None
		}).unwrap();
		let mut response = Response::new(BufReader::new(conn)).unwrap();
		response.body.read_to_string(&mut output).unwrap();
	}
	println!("-------------- DIY Reqest");
	println!("{}", output);
	println!("--------------");
}
