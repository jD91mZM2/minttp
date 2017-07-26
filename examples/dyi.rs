extern crate minttp;
use minttp::{DYIRequest, consts};
use minttp::response::Response;
use std::collections::HashMap;
use std::io::{BufReader, Read};

fn main() {
	let mut headers = HashMap::new();
	headers.insert("Host", "example.com");
	headers.insert("Connection", "close");

	let mut output = String::new();
	{
		let conn = minttp::diy_request(&DYIRequest {
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
		println!("Status: {} ({})", response.status, response.description);
		response.body.read_to_string(&mut output).unwrap();
	}
	println!("-------------- DYI Reqest");
	println!("{}", output);
	println!("--------------");
}
