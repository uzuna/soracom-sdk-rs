use serde::{Deserialize, Serialize};
use serde_derive::*;

const SORACOM_API_ENDPOINT: &'static str = "api.soracom.io";
const SORACOM_SANDBOX_API_ENDPOINT: &'static str = "api-sandbox.soracom.io";

pub struct Client {
  http_client: reqwest::Client,
  api_key: String,
  token: String,
}

#[derive(Serialize, Deserialize)]
struct AuthRequest<'a> {
  #[serde(rename = "authKeyId")]
  auth_key_id: &'a str,
  #[serde(rename = "authKey")]
  auth_key: &'a str,
  #[serde(rename = "tokenTimeoutSeconds")]
  token_timeout_seconds: u32,
}

impl Client {
  fn new<T>(key: T,token: T) -> Self 
  where 
    T: ToString
    {
    Client {
      http_client: reqwest::Client::builder().build().unwrap(),
      api_key:key.to_string(),
      token: token.to_string(),
    }
  }

  #[cfg(test)]
  async fn get(&self, url: &str) -> reqwest::Result<reqwest::Response> {
    self.http_client.get(url).send().await
  }

  async fn auth(&self) -> Result<(), Box<dyn std::error::Error>> {
    // let mut addr = url::Url::parse("/v1/auth")?;
    // addr.set_scheme("https")?;
    // addr.set_host(SORACOM_SANDBOX_API_ENDPOINT)?;
    let mut addr = url::Url::parse("http://httpbin.org/post")?;
    // addr.set_scheme("http");
    // addr.set_host(Some(""))?;
    let reqbody = AuthRequest{
      auth_key_id: self.api_key.as_ref(),
      auth_key: self.token.as_ref(),
      token_timeout_seconds: 3600,
    };
    let res = self.http_client.post(&addr.to_string()).body(serde_json::to_string(&reqbody)?).send().await?.text().await?;
    println!("{:?}", res);
    Ok(())
  }

}

#[cfg(test)]
mod tests {
  use crate::client::*;
  use std::collections::HashMap;

  #[tokio::test]
  async fn async_http_request_with_client() {
    let client = Client::new("","");
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
    let client = Client::new("test","token");
    let resp = client.auth().await;
    println!("{:?}", resp)
  }
}
