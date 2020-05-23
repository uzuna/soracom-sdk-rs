use crate::error::*;
use serde::de;

#[derive(Debug)]
pub enum HTTPMethod {
  GET,
  POST,
  DELETE,
}

pub fn sync_req(method: HTTPMethod, url: url::Url, body: Option<String>) -> Result<(), Error> {
  let client = reqwest::blocking::Client::new();
  let method = match method {
    HTTPMethod::GET => reqwest::Method::GET,
    HTTPMethod::POST => reqwest::Method::POST,
    HTTPMethod::DELETE => reqwest::Method::DELETE,
  };
  let reqbld = client.request(method, url);
  let reqbld = match body {
    Some(x) => reqbld.body(x),
    _ => reqbld,
  };
  let res = reqbld.send().unwrap();
  match res.status() {
    reqwest::StatusCode::OK => Ok(()),
    x => {
      let resbody = res.text()?;
      Err(crate::error::new_http_error(x, resbody))
    }
  }
}

#[derive(Debug, Default)]
pub struct HTTPClient {
  client: reqwest::Client,
}

impl HTTPClient {
  pub fn new() -> Self {
    Self {
      client: reqwest::Client::builder().build().unwrap(),
    }
  }
  pub async fn request(
    &self,
    method: HTTPMethod,
    url: url::Url,
    body: Option<String>,
  ) -> Result<(), Error> {
    let reqbld = self.client.request(get_method(method), url);
    let reqbld = match body {
      Some(x) => reqbld.body(x),
      _ => reqbld,
    };
    let res = reqbld
      .header(reqwest::header::CONTENT_TYPE, "application/json")
      .send()
      .await?;
    match res.status() {
      reqwest::StatusCode::OK => Ok(()),
      x => {
        let resbody = res.text().await?;
        Err(crate::error::new_http_error(x, resbody))
      }
    }
  }

  pub async fn request_json<T>(
    &self,
    method: HTTPMethod,
    url: url::Url,
    body: Option<String>,
  ) -> Result<T, Error>
  where
    T: de::DeserializeOwned,
  {
    let reqbld = self.client.request(get_method(method), url);
    let reqbld = match body {
      Some(x) => reqbld.body(x),
      _ => reqbld,
    };
    let res = reqbld
      .header(reqwest::header::CONTENT_TYPE, "application/json")
      .send()
      .await?;
    match res.status() {
      reqwest::StatusCode::OK => {
        let parsed = res.json::<T>().await?;
        Ok(parsed)
      }
      x => {
        let resbody = res.text().await?;
        Err(crate::error::new_http_error(x, resbody))
      }
    }
  }
}

fn get_method(method: HTTPMethod) -> reqwest::Method {
  match method {
    HTTPMethod::GET => reqwest::Method::GET,
    HTTPMethod::POST => reqwest::Method::POST,
    HTTPMethod::DELETE => reqwest::Method::DELETE,
  }
}
