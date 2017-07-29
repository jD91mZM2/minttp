extern crate minttp;
use minttp::url;

fn main() {
	println!("URL Encode: \"Hello World 😎\"");
	println!("Response:   \"{}\"", url::encode("Hello World 😎"));
}
