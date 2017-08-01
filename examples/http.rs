extern crate minttp;
#[cfg(feature = "http")]
extern crate http;

#[cfg(feature = "http")]
use http::{Request, Uri};
#[cfg(feature = "http")]
use std::io::Read;

#[cfg(feature = "http")]
fn main() {
	let uri: Uri = "example.com".parse().unwrap();
	let mut req = Request::builder()
		.uri(uri)
		.method(http::method::GET)
		.header("Test", "Hello World")
		.body([])
		.unwrap();

	let mut output = String::new();
	{
		let mut response = minttp::request(&mut req).unwrap();
		println!("Status: {} ({})", response.status, response.description);
		response.body.read_to_string(&mut output).unwrap();
	}

	println!("-------------- High-level flexible request");
	println!("{}", output);
	println!("--------------");
}
#[cfg(not(feature = "http"))]
fn main() { eprintln!("Please run this example with --features \"http\"") }
