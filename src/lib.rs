//! A simple wrapper around the curl command-line interface

use std::{io, process::Output};
use tokio::process::Command;

#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct Curl;

#[derive(Debug)]
pub struct CurlBuilder {
    url: String,
    method: Option<Method>,
    headers: Vec<String>,
    body: Option<String>,
    proxy: Option<String>,
    redirects: bool,
    compressed: bool,
    interface: Option<String>,
}

impl Curl {
    /// Create a new `Curl` instance.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    ///
    /// # Returns
    ///
    /// A new `CurlBuilder` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::Curl;
    /// use curl_wrapper::Method;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let curl = Curl::new("https://example.com")
    ///         .method(Method::GET)
    ///         .set_header("User-Agent: curl/7.81.0")
    ///         .set_body("Hello, world!")
    ///         .set_proxy("http://proxy.example.com:8080")
    ///         .redirects(true)
    ///         .compressed(true)
    ///         .interface("eth0");
    ///
    ///     let output = curl.send().await.unwrap();
    ///     println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    /// }
    /// ```
    pub fn new(url: &str) -> CurlBuilder {
        CurlBuilder {
            url: url.to_string(),
            method: None,
            headers: Vec::new(),
            body: None,
            proxy: None,
            redirects: false,
            compressed: false,
            interface: None,
        }
    }
}

impl CurlBuilder {
    /// Sets the HTTP method for the request.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::Curl;
    /// use curl_wrapper::Method;
    ///
    /// let curl = Curl::new("https://example.com")
    ///     .method(Method::GET);
    /// ```
    pub fn method(mut self, method: Method) -> Self {
        self.method = Some(method);
        self
    }

    /// Sets the HTTP headers for the request.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::Curl;
    ///
    /// let curl = Curl::new("https://example.com")
    ///     .set_header("User-Agent: curl/7.81.0")
    ///     .set_header("Accept: application/json");
    /// ```
    pub fn set_header(mut self, header: &str) -> Self {
        self.headers.push(header.to_string());
        self
    }

    /// Sets multiple HTTP headers for the request.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::Curl;
    ///
    /// let curl = Curl::new("https://example.com")
    ///     .set_headers(vec!["User-Agent: curl/7.81.0", "Accept: application/json"]);
    /// ```
    pub fn set_headers(mut self, headers: Vec<&str>) -> Self {
        for h in headers {
            self.headers.push(h.to_string());
        }
        self
    }

    /// Sets the HTTP body for the request.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::Curl;
    ///
    /// let curl = Curl::new("https://example.com")
    ///     .set_body("Hello, World!");
    /// ```
    pub fn set_body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    /// Sets the HTTP proxy for the request.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::Curl;
    ///
    /// let curl = Curl::new("https://example.com")
    ///     .set_proxy("http://proxy.example.com:8080");
    /// ```
    pub fn set_proxy(mut self, proxy: &str) -> Self {
        self.proxy = Some(proxy.to_string());
        self
    }

    /// Enables or disables redirects for the request.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::Curl;
    ///
    /// let curl = Curl::new("https://example.com")
    ///     .redirects(true);
    /// ```
    pub fn redirects(mut self, r: bool) -> Self {
        self.redirects = r;
        self
    }

    /// Enables or disables compression for the request.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::Curl;
    ///
    /// let curl = Curl::new("https://example.com")
    ///     .compressed(true);
    /// ```
    pub fn compressed(mut self, compress: bool) -> Self {
        self.compressed = compress;
        self
    }

    /// Enables or disables interface for the request.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::Curl;
    ///
    /// let curl = Curl::new("https://example.com")
    ///     .interface("eth0");
    /// ```
    pub fn interface(mut self, interface: &str) -> Self {
        self.interface = Some(interface.to_string());
        self
    }

    /// Executes the request and returns the output.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::Curl;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let curl = Curl::new("https://example.com")
    ///         .interface("eth0");
    ///     let output = curl.send().await.unwrap();
    ///     println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    /// }
    /// ```
    pub async fn send(&self) -> Result<Output, io::Error> {
        let mut curl = Command::new("curl");

        if let Some(interface) = &self.interface {
            curl.arg("--interface").arg(interface);
        }

        if self.redirects {
            curl.arg("-L");
        }

        match &self.method {
            Some(Method::GET) => curl.arg("-X").arg("GET"),
            Some(Method::POST) => curl.arg("-X").arg("POST"),
            Some(Method::PUT) => curl.arg("-X").arg("PUT"),
            Some(Method::DELETE) => curl.arg("-X").arg("DELETE"),
            None => curl.arg("-X").arg("GET"),
        };

        if let Some(proxy) = &self.proxy {
            curl.arg("--proxy").arg(proxy);
        }

        curl.arg(&self.url);

        for i in &self.headers {
            curl.arg("-H").arg(i);
        }

        if let Some(body) = &self.body {
            curl.arg("-d").arg(body);
        }

        if self.compressed {
            curl.arg("--compressed");
        }

        let output = curl.output().await?;
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Use cargo test -- --nocapture for printing output

    #[tokio::test]
    async fn get() {
        let curl = Curl::new("https://httpbin.org/get")
            .method(Method::GET)
            .set_header("Content-Type: application/json")
            .set_header("Cookie: test-cookie");
        let response = curl.send().await.unwrap();
        let result = std::str::from_utf8(&response.stdout[..]).unwrap();
        println!("{:?}", curl);
        println!("{}", result);
    }

    #[tokio::test]
    async fn post() {
        let curl = Curl::new("https://httpbin.org/post")
            .method(Method::POST)
            .set_header("Content-Type: application/json")
            .set_header("Cookie: test-cookie");
        let response = curl.send().await.unwrap();
        let result = std::str::from_utf8(&response.stdout[..]).unwrap();
        println!("{:?}", curl);
        println!("{}", result);
    }

    #[tokio::test]
    async fn put() {
        let curl = Curl::new("https://httpbin.org/put")
            .method(Method::PUT)
            .set_header("Content-Type: application/json")
            .set_header("Cookie: test-cookie");
        let response = curl.send().await.unwrap();
        let result = std::str::from_utf8(&response.stdout[..]).unwrap();
        println!("{:?}", curl);
        println!("{}", result);
    }

    #[tokio::test]
    async fn delete() {
        let curl = Curl::new("https://httpbin.org/delete")
            .method(Method::DELETE)
            .set_header("Content-Type: application/json")
            .set_header("Cookie: test-cookie");
        let response = curl.send().await.unwrap();
        let result = std::str::from_utf8(&response.stdout[..]).unwrap();
        println!("{:?}", curl);
        println!("{}", result);
    }

    #[tokio::test]
    async fn redirect() {
        let curl = Curl::new("https://httpbin.org/redirect-to?url=https://httpbin.org/get")
            .set_header("Content-Type: application/json")
            .redirects(true);
        let response = curl.send().await.unwrap();
        let result = std::str::from_utf8(&response.stdout[..]).unwrap();
        println!("{:?}", curl);
        println!("{}", result);
    }
}
