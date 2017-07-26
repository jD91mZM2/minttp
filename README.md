# minttp

minttp is an experiment to see if I could create a simple and minimal HTTP library without any dependencies.  
I'd say I succeeded.

## Usage

Don't actually use it, please. This thing doesn't even support SSL (yet)!  
Apart from that, it's simple to use.  
<details>

```Rust
extern crate minttp;
use std::collections::HashMap;
use std::io::{Read, BufReader};
use minttp::{consts, DYIRequest};
use minttp::response::Response;

fn main() {
	let mut headers = HashMap::new();
	headers.insert("Host", "example.com");
	headers.insert("Connection", "close");

	let mut output = String::new();
	{
		let conn = minttp::diy_request(&DYIRequest {
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
```

</details>

Oh wait wrong example! This was the DYI request. It lets you specify everything yourself and is the bare minimum.  
Here's the way to use it simply:  
<details>

```Rust
extern crate minttp;
use std::io::Read;

fn main() {
	let url = "example.com".parse().unwrap();

	let mut output = String::new();
	{
		let mut response = minttp::get(url).unwrap();
		response.body.read_to_string(&mut output).unwrap();
	}

	println!("-------------- High-level standard request");
	println!("{}", output);
	println!("--------------");
}
```

</details>

That's not so hard, right? Great!  
Everything is built like layers. First is the "DYI" layer where literally everything is, well, "do it yourself".
Second is the request layer (which you haven't seen yet),
and third is the "standard" layer with standard functions like `get`, `post`, et.c.

## SSL

SSL is **not** implemented, and *might* never be. This library is supposed to be simple and minimal.  
*And not even supposed to be used.*

## URL parsing

URL parsing in this library is done by simply splitting the string over and over.  
But it works pretty well!  
Current parsing implemented is: `[protocol://]domain.tld[:port][/page][?query=params]`

Fun fact: The URL parsing can only fail if the port isn't a number. Everything else will use defaults.  
This also means a URL is not sanitized. Just because `"test".parse::<Url>()` works doesn't mean it's a URL.  
TODO: Implement validating **maybe**.
