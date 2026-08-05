extern crate lazy_static;

use log::{info, warn};
use hyper::StatusCode;
use lazy_static::lazy_static;
use reqwest::Client;

lazy_static! {
    static ref CLIENT: Client = match Client::builder()
                                    .danger_accept_invalid_certs(true)
                                    .build()
    {
        Ok(c) => c,
        Err(e) => {
            warn!("cannot create ACCEPT_INVALID_CERTS HTTP Client.. start creating normal client. (Error: {})", e.to_string());

            Client::new()
        }
    };
}

pub async fn get_with_headers(url: &str, headers: &[(&str, &str)]) -> Result<String, StatusCode> {
    let mut req = CLIENT.get(url);
    for (key, value) in headers {
        req = req.header(*key, *value);
    }

    if let Ok(response) = req.send().await {
        if response.status().is_success() {
            let body = response.text().await
                .expect(format!("failed to read response body from GET request (url: {})", url).as_str());

            return Ok(body);
        }
        else {
            warn!("GET request failed. (url: {}, HTTP Error: {})", url, response.status());

            return Err(response.status());
        }
    } else {
        warn!("failed GET request to {}", url);
        Err(StatusCode::FORBIDDEN)
    }
}

pub async fn get(url: &str) -> Result<String, StatusCode> {
    if let Ok(response) = CLIENT.get(url).send().await {
        if response.status().is_success() {
            let body = response.text().await
                .expect(format!("failed to read response body from GET request (url: {})", url).as_str());

            return Ok(body);
        }
        else {
            warn!("GET request failed. (url: {}, HTTP Error: {})", url, response.status());

            return Err(response.status());
        }
    } else {
        warn!("failed GET request to {}", url);
        Err(StatusCode::FORBIDDEN)
    }
}

pub async fn post(url: &str, payload: &str) -> Result<String, String> {
    let response = CLIENT
        .post(url)
        .body(payload.to_string())
        .send()
        .await
        .expect(format!("failed POST request to {}", url).as_str());

    if response.status().is_success() {
        let body =
            response.text()
            .await
            .expect(format!("failed get response from POST request (url: {})", url).as_str());

        return Ok(body);
    } else {
        return Err(format!("POST request failed. (url: {}, payload: {}, HTTP Error: {})", url, payload.to_string(), response.status()));
    }
}