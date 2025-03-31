## A simple wrapper around the curl command-line interface

### INSTALL
```toml
[dependencies]
curl-wrapper = { git = "https://github.com/mallocdev/curl-wrapper" }
```

### USAGE
```rust
use curl_wrapper::CurlWrapper;
use curl_wrapper::Method;

#[tokio::main]
async fn main() {
  let mut curl = CurlWrapper::new("https://example.com");
  curl.method(Method::GET);
  curl.set_header("User-Agent: curl/7.81.0");
  curl.set_body("Hello, world!");
  curl.set_proxy("http://proxy.example.com:8080");
  curl.redirects(true);
  curl.compressed(true);
  curl.interface("eth0");

  let output = curl.execute().await.unwrap();
  println!("Output: {}", String::from_utf8_lossy(&output.stdout));
}
```
