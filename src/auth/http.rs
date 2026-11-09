use crate::error::{HitError, HitResult};
use serde::de::DeserializeOwned;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

impl HttpResponse {
    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    pub fn json<T: DeserializeOwned>(&self) -> HitResult<T> {
        serde_json::from_str(&self.body).map_err(|e| {
            HitError::with_hint(
                "Failed to parse API response",
                format!("Received invalid JSON from API: {}", e),
                format!("Response body:\n{}", self.body),
            )
        })
    }
}

pub struct HttpClient;

impl HttpClient {
    pub fn get(url: &str, headers: &[(&str, &str)]) -> HitResult<HttpResponse> {
        Self::execute_request("GET", url, None, headers)
    }

    pub fn post_json(
        url: &str,
        json_body: &str,
        headers: &[(&str, &str)],
    ) -> HitResult<HttpResponse> {
        let mut all_headers = headers.to_vec();
        all_headers.push(("Content-Type", "application/json"));
        Self::execute_request("POST", url, Some(json_body), &all_headers)
    }

    pub fn post_form(
        url: &str,
        form_data: &[(&str, &str)],
        headers: &[(&str, &str)],
    ) -> HitResult<HttpResponse> {
        let encoded_body = form_data
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding(k), urlencoding(v)))
            .collect::<Vec<_>>()
            .join("&");

        let mut all_headers = headers.to_vec();
        all_headers.push(("Content-Type", "application/x-www-form-urlencoded"));
        Self::execute_request("POST", url, Some(&encoded_body), &all_headers)
    }

    fn execute_request(
        method: &str,
        url: &str,
        body: Option<&str>,
        headers: &[(&str, &str)],
    ) -> HitResult<HttpResponse> {
        let mut cmd = Command::new("curl");
        cmd.arg("-sSL")
            .arg("-w")
            .arg("\n%{http_code}")
            .arg("-X")
            .arg(method)
            .arg("--max-time")
            .arg("15");

        for (k, v) in headers {
            cmd.arg("-H").arg(format!("{}: {}", k, v));
        }

        // Set User-Agent
        cmd.arg("-H").arg("User-Agent: hit-cli/0.1.0");

        if let Some(b) = body {
            cmd.arg("-d").arg(b);
        }

        cmd.arg(url);

        let output = cmd.output().map_err(|e| {
            HitError::with_hint(
                "Network request failed",
                format!("Failed to execute 'curl' for network call: {}", e),
                "Please verify that curl is installed and your network connection is active.",
            )
        })?;

        let raw = String::from_utf8_lossy(&output.stdout);
        let mut lines = raw.lines().collect::<Vec<&str>>();

        let status_code_str = lines.pop().unwrap_or("0").trim();
        let status = status_code_str.parse::<u16>().unwrap_or(0);
        let response_body = lines.join("\n");

        if status == 0 && !output.status.success() {
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            return Err(HitError::with_hint(
                "Network connection error",
                format!("Could not connect to {}: {}", url, stderr_str.trim()),
                "Check your internet connection or proxy settings.",
            ));
        }

        Ok(HttpResponse {
            status,
            body: response_body,
        })
    }
}

fn urlencoding(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for b in input.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            _ => {
                out.push_str(&format!("%{:02X}", b));
            }
        }
    }
    out
}
