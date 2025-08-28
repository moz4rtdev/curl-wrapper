## A simple wrapper around the curl command-line interface

### INSTALL
```toml
[dependencies]
curl-wrapper = { git = "https://github.com/moz4rtdev/curl-wrapper" }
```

### USAGE
```rust
use curl_wrapper::Curl;
use curl_wrapper::Method;

#[tokio::main]
async fn main() {
  let mut curl = Curl::new("https://example.com")
    .method(Method::GET)
    .set_header("User-Agent: curl/7.81.0")
    .set_body("Hello, world!")
    .set_proxy("http://proxy.example.com:8080")
    .redirects(true)
    .compressed(true)
    .interface("eth0")

  let output = curl.send().await.unwrap();
  println!("status code: {}", curl.status_code);
  println!("headers: {}", curl.headers);
  println!("body: {}", curl.body);

}
```
