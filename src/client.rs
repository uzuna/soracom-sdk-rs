use crate::consts::*;
use crate::model::*;
use crate::option;
use crate::option::QueryParams;
use url::{ParseError, Url};

#[derive(Debug, Default)]
pub struct Client<'a> {
  http_client: reqwest::Client,
  api_key: String,
  token: String,
  endpoint: &'a str,
}

impl Client<'_> {
  fn new() -> Self {
    Client {
      http_client: reqwest::Client::builder().build().unwrap(),
      endpoint: SORACOM_GLOBAL_API_ENDPOINT,
      ..Default::default()
    }
  }

  #[cfg(test)]
  async fn get(&self, url: &str) -> reqwest::Result<reqwest::Response> {
    self.http_client.get(url).send().await
  }

  async fn auth(&mut self, key: &str, secret: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addr = &self.get_url("/v1/auth")?;
    let reqbody = AuthRequest {
      auth_key_id: key,
      auth_key: secret,
      token_timeout_seconds: 60,
    };
    let res = self
      .post(&addr.to_string(), serde_json::to_string(&reqbody)?)
      .await?;
    match res.status() {
      reqwest::StatusCode::OK => {
        let resbody = res.text().await?;
        let resbody: AuthResponse = serde_json::from_str(&resbody)?;
        self.api_key = resbody.api_key.to_string();
        self.token = resbody.token.to_string();
        Ok(())
      }
      _ => {
        let resbody = res.text().await?;
        println!("Err {:?}", &resbody);
        Ok(())
      }
    }
  }

  async fn list_subscriber(
    &self,
    opt: Option<option::ListSubscribersOptions>,
  ) -> Result<Vec<Subscriber>, Box<dyn std::error::Error>> {
    let url = &mut self.get_url("/v1/subscribers")?;

    match opt {
      Some(opt) => {
        &url.set_query(opt.to_query_params().as_deref());
      }
      _ => {}
    }
    let req = self
      .http_client
      .get(&url.to_string())
      .header(reqwest::header::CONTENT_TYPE, "application/json")
      .header(SORACOM_API_HEADER_API_KEY, &self.api_key)
      .header(SORACOM_API_HEADER_TOKEN, &self.token);
    let res = req.send().await?;
    println!("{:?}", res);
    let resbody = res.text().await?;
    let resbody: Vec<Subscriber> = serde_json::from_str(&resbody)?;
    Ok(resbody)
  }

  fn get_url<'a>(&self, path: &'a str) -> Result<Url, ParseError> {
    Url::parse(format!("https://{}{}", self.endpoint, path).as_ref())
  }

  async fn post<'a>(&self, url: &str, reqbody: String) -> reqwest::Result<reqwest::Response> {
    self
      .http_client
      .post(url)
      .header(reqwest::header::CONTENT_TYPE, "application/json")
      .body(reqbody)
      .send()
      .await
  }
}

#[cfg(test)]
mod tests {
  use crate::client::*;
  use std::collections::HashMap;

  #[tokio::test]
  async fn async_http_request_with_client() {
    let client = Client::new();
    let resp = client
      .get("https://httpbin.org/uuid")
      .await
      .unwrap()
      .json::<HashMap<String, String>>()
      .await;
    println!("{:?}", resp)
  }

  #[tokio::test]
  async fn auth() {
    let key = std::env::var("SORACOM_AUTHKEY_ID_FOR_TEST").unwrap();
    let secret = std::env::var("SORACOM_AUTHKEY_FOR_TEST").unwrap();
    let mut client = Client::new();
    let resp = client.auth(&key, &secret).await;
    println!("{:?}", resp);

    let resp = client.list_subscriber(None).await;
    println!("{:?}", resp);
  }
}
