# minttp

minttp is an experiment to see if I could create a simple and minimal HTTP library without any dependencies.  
I'd say I succeeded.

## SSL

This isn't done without any dependencies, but rather it's an optional dependency of `native-tls`.  
You can opt out of the feature with `default-features = false` in Cargo.

## [http](https://users.rust-lang.org/t/announcing-the-http-crate/12123)

minttp **optionally** implements the http crate, which gives you united syntax for web requests  
across libraries.

To see it in action, check out `examples/http.rs`.

*Note: The support for this is disabled by default.*

## Usage

### Don't

Don't actually use it, please.  
The only *acceptable* usecases are:  

 - Testing
 - You *really* need minimality.

minttp is not hosted on crates.io, and never will be.  
It also won't follow the semver guidelines, and just break whatever it wants to.

### Do?

Apart from that, it's simple to use.  
<details>

```Rust
extern crate minttp;
use minttp::{DIYRequest, consts};
use minttp::response::Response;
use std::collections::HashMap;
use std::io::{BufReader, Read};

fn main() {
	let mut headers = HashMap::new();
	headers.insert("Host", "example.com");
	headers.insert("Connection", "close");

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
```

</details>

Oh wait wrong example! This was the DIY request. It lets you specify everything yourself and is the bare minimum.  
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
```

</details>

That's not so hard, right? Great!  
Everything is built like layers. First is the "DIY" layer where literally everything is, well, "do it yourself".
Second is the request layer (which you haven't seen yet),
and third is the "standard" layer with standard functions like `get`, `post`, et.c.

## URL parsing

URL parsing in this library is done by simply splitting the string over and over.  
But it works pretty well!  
Current parsing implemented is: `[protocol://]domain.tld[:port][/page][?key=value]`

Fun fact: The URL parsing can only fail if the port isn't a number. Everything else will use defaults.  
This also means a URL is not sanitized. Just because `"test".parse::<Url>()` works doesn't mean it's a URL.  
TODO: Implement validating **maybe**.
