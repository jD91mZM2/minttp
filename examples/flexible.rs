extern crate minttp;
use minttp::Request;
use std::io::Read;

fn main() {
	let url = "example.com".parse().unwrap();
	let req = Request::new(url).header("Test", "Hello World");

	let mut output = String::new();
	{
		let mut response = minttp::request(&req).unwrap();
		println!("Status: {} ({})", response.status, response.description);
		response.body.read_to_string(&mut output).unwrap();
	}

	println!("-------------- High-level flexible request");
	println!("{}", output);
	println!("--------------");
}
