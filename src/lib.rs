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

#[derive(Debug)]
pub struct CurlWrapper {
    url: String,
    method: Option<Method>,
    headers: Vec<String>,
    body: Option<String>,
    proxy: Option<String>,
    redirects: bool,
    compressed: bool,
    interface: Option<String>,
}

impl CurlWrapper {
    /// Create a new `CurlWrapper` instance.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the request to.
    ///
    /// # Returns
    ///
    /// A new `CurlWrapper` instance.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::CurlWrapper;
    /// use curl_wrapper::Method;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut curl = CurlWrapper::new("https://example.com");
    ///     curl.method(Method::GET);
    ///     curl.set_header("User-Agent: curl/7.81.0");
    ///     curl.set_body("Hello, world!");
    ///     curl.set_proxy("http://proxy.example.com:8080");
    ///     curl.redirects(true);
    ///     curl.compressed(true);
    ///     curl.interface("eth0");
    ///
    ///     let output = curl.execute().await.unwrap();
    ///     println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    /// }
    /// ```
    pub fn new(url: &str) -> Self {
        Self {
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

    /// Sets the HTTP method for the request.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::CurlWrapper;
    /// use curl_wrapper::Method;
    ///
    /// let mut curl = CurlWrapper::new("https://example.com");
    /// curl.method(Method::GET);
    /// ```
    pub fn method(&mut self, method: Method) {
        self.method = Some(method)
    }

    /// Sets the HTTP headers for the request.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::CurlWrapper;
    ///
    /// let mut curl = CurlWrapper::new("https://example.com");
    /// curl.set_header("User-Agent: curl/7.81.0");
    /// curl.set_header("Accept: application/json");
    /// ```
    pub fn set_header(&mut self, header: &str) {
        self.headers.push(header.to_string());
    }

    /// Sets multiple HTTP headers for the request.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::CurlWrapper;
    ///
    /// let mut curl = CurlWrapper::new("https://example.com");
    /// curl.set_headers(vec!["User-Agent: curl/7.81.0", "Accept: application/json"]);
    /// ```
    pub fn set_headers(&mut self, headers: Vec<&str>) {
        for h in headers {
            self.headers.push(h.to_string());
        }
    }

    /// Sets the HTTP body for the request.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::CurlWrapper;
    ///
    /// let mut curl = CurlWrapper::new("https://example.com");
    /// curl.set_body("Hello, World!");
    /// ```
    pub fn set_body(&mut self, body: &str) {
        self.body = Some(body.to_string())
    }

    /// Sets the HTTP proxy for the request.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::CurlWrapper;
    ///
    /// let mut curl = CurlWrapper::new("https://example.com");
    /// curl.set_proxy("http://proxy.example.com:8080");
    /// ```
    pub fn set_proxy(&mut self, proxy: &str) {
        self.proxy = Some(proxy.to_string())
    }

    /// Enables or disables redirects for the request.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::CurlWrapper;
    ///
    /// let mut curl = CurlWrapper::new("https://example.com");
    /// curl.redirects(true);
    /// ```
    pub fn redirects(&mut self, r: bool) {
        self.redirects = r;
    }

    /// Enables or disables compression for the request.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::CurlWrapper;
    ///
    /// let mut curl = CurlWrapper::new("https://example.com");
    /// curl.compressed(true);
    /// ```
    pub fn compressed(&mut self, compress: bool) {
        self.compressed = compress;
    }

    /// Enables or disables interface for the request.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::CurlWrapper;
    ///
    /// let mut curl = CurlWrapper::new("https://example.com");
    /// curl.interface("eth0");
    /// ```
    pub fn interface(&mut self, interface: &str) {
        self.interface = Some(interface.to_string());
    }

    /// Executes the request and returns the output.
    ///
    /// # Example
    ///
    /// ```
    /// use curl_wrapper::CurlWrapper;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut curl = CurlWrapper::new("https://example.com");
    ///     curl.interface("eth0");
    ///     let output = curl.execute().await.unwrap();
    ///     println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    /// }
    /// ```
    pub async fn execute(&self) -> Result<Output, io::Error> {
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
        let mut curl = CurlWrapper::new("https://httpbin.org/get");
        curl.set_header("Content-Type: application/json");
        curl.set_header("Cookie: test-cookie");
        let response = curl.execute().await;
        println!("{:?}", curl);
        println!("{:?}", response);
    }

    #[tokio::test]
    async fn post() {
        let mut curl = CurlWrapper::new("https://httpbin.org/post");
        curl.set_header("Content-Type: application/json");
        curl.set_header("Cookie: test-cookie");
        let response = curl.execute().await;
        println!("{:?}", curl);
        println!("{:?}", response);
    }

    #[tokio::test]
    async fn put() {
        let mut curl = CurlWrapper::new("https://httpbin.org/put");
        curl.set_header("Content-Type: application/json");
        curl.set_header("Cookie: test-cookie");
        let response = curl.execute().await;
        println!("{:?}", curl);
        println!("{:?}", response);
    }

    #[tokio::test]
    async fn delete() {
        let mut curl = CurlWrapper::new("https://httpbin.org/delete");
        curl.set_header("Content-Type: application/json");
        curl.set_header("Cookie: test-cookie");
        let response = curl.execute().await;
        println!("{:?}", curl);
        println!("{:?}", response);
    }

    #[tokio::test]
    async fn redirect() {
        let mut curl = CurlWrapper::new("https://google.com");
        curl.set_header("Content-Type: application/json");
        curl.redirects(true);
        let response = curl.execute().await;
        println!("{:?}", curl);
        println!("{:?}", response);
    }
}
