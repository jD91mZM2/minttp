# minttp

minttp is an experiment to see if I could create a simple and minimal HTTP library without any dependencies.  
I'd say I succeeded.


## SSL

This is added to a later version of the library.  
Dependencies added! openssl and child dependencies.  
It's still only one single top level dependency...

*... and also optional!*  
You can opt out of the feature with `default-features = false` in Cargo, or run your commands with `--no-default-features`.

## Usage

Don't actually use it, please.  
I mean, if you don't REALLY need minimality. Maybe some openssl mismatch or whatever... Then sure!  
But this generally is meant as a test and not to replace anything. It's not even hosted on cargo!  
Apart from that, it's simple to use.  
<details>

```Rust
[diy]
```

</details>

Oh wait wrong example! This was the DIY request. It lets you specify everything yourself and is the bare minimum.  
Here's the way to use it simply:  
<details>

```Rust
[high-level]
```

</details>

That's not so hard, right? Great!  
Everything is built like layers. First is the "DIY" layer where literally everything is, well, "do it yourself".
Second is the request layer (which you haven't seen yet),
and third is the "standard" layer with standard functions like `get`, `post`, et.c.

## URL parsing

URL parsing in this library is done by simply splitting the string over and over.  
But it works pretty well!  
Current parsing implemented is: `[protocol://]domain.tld[:port][/page][?query=params]`

Fun fact: The URL parsing can only fail if the port isn't a number. Everything else will use defaults.  
This also means a URL is not sanitized. Just because `"test".parse::<Url>()` works doesn't mean it's a URL.  
TODO: Implement validating **maybe**.
