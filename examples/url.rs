extern crate minttp;
use minttp::url::Url;

fn main() {
	let url = "protocol://example.com:123/path?key=val"
		.parse::<Url>()
		.unwrap();
	println!("{}", url);
	println!("{:?}", url);

	let url = "example.com/path".parse::<Url>().unwrap();
	println!("{}", url);
	println!("{:?}", url);
}
