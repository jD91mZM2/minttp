extern crate minttp;
use std::io::Read;

fn main() {
	let url = "example.com".parse().unwrap();

	let mut output = String::new();
	{
		let mut response = minttp::get(url).unwrap();
		println!(
			"Status: {} {} ({})",
			if response.is_success() {
				"SUCCESS"
			} else {
				"FAILED"
			},
			response.status,
			response.description
		);
		response.body.read_to_string(&mut output).unwrap();
	}

	println!("-------------- High-level standard request");
	println!("{}", output);
	println!("--------------");
}
