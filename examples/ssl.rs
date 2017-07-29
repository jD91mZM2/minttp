extern crate minttp;
use std::io::Read;

fn main() {
	let url = "https://example.com".parse().unwrap();

	let mut output = String::new();
	{
		let mut response = minttp::get(url).unwrap();
		response.body.read_to_string(&mut output).unwrap();
	}

	println!("-------------- High-level standard request + SSL!");
	println!("{}", output);
	println!("--------------");
}
