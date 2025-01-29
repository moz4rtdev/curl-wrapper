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
}

impl CurlWrapper {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            method: None,
            headers: Vec::new(),
            body: None,
            proxy: None,
            redirects: false,
            compressed: false,
        }
    }

    pub fn method(&mut self, method: Method) {
        self.method = Some(method)
    }

    pub fn set_header(&mut self, header: &str) {
        self.headers.push(header.to_string());
    }

    pub fn set_headers(&mut self, headers: Vec<&str>) {
        for h in headers {
            self.headers.push(h.to_string());
        }
    }

    pub fn set_body(&mut self, body: &str) {
        self.body = Some(body.to_string())
    }

    pub fn set_proxy(&mut self, proxy: &str) {
        self.proxy = Some(proxy.to_string())
    }

    pub fn redirects(&mut self, r: bool) {
        self.redirects = r;
    }

    pub fn compressed(&mut self, compress: bool) {
        self.compressed = compress;
    }

    pub async fn execute(&self) -> Result<Output, io::Error> {
        let mut curl = Command::new("curl");

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

    #[tokio::test]
    async fn httbin_get() {
        let mut curl = CurlWrapper::new("https://httpbin.org/get");
        curl.set_header("Content-Type: application/json");
        curl.redirects(true);
        let response = curl.execute().await;
        println!("{:?}", curl);
        println!("{:?}", response);
        //assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn google() {
        let mut curl = CurlWrapper::new("https://google.com");
        curl.set_header("Content-Type: application/json");
        curl.redirects(false);
        let response = curl.execute().await;
        println!("{:?}", curl);
        println!("{:?}", response);
    }
}
