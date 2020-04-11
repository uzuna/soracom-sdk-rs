use serde_derive::*;

use chrono::{DateTime, Utc};

const SORACOM_API_ENDPOINT: &'static str = "api.soracom.io";
const SORACOM_GLOBAL_API_ENDPOINT: &'static str = "g.api.soracom.io";
const SORACOM_SANDBOX_API_ENDPOINT: &'static str = "api-sandbox.soracom.io";
const SORACOM_API_HEADER_API_KEY: &'static str = "X-Soracom-API-Key";
const SORACOM_API_HEADER_TOKEN: &'static str = "X-Soracom-Token";

#[derive(Debug, Default)]
pub struct Client<'a> {
  http_client: reqwest::Client,
  api_key: String,
  token: String,
  endpoint: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthRequest<'a> {
  #[serde(rename = "authKeyId")]
  auth_key_id: &'a str,
  #[serde(rename = "authKey")]
  auth_key: &'a str,
  #[serde(rename = "tokenTimeoutSeconds")]
  token_timeout_seconds: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthResponse<'a> {
  #[serde(rename = "apiKey")]
  api_key: &'a str,
  #[serde(rename = "operatorId")]
  operator_id: &'a str,
  #[serde(rename = "token")]
  token: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscriber {
  apn: String,
  #[serde(rename = "createdAt")]
  created_at: DateTime<Utc>,
  #[serde(rename = "expiredAt")]
  expired_at: DateTime<Utc>,
  // #[serde(rename = "expiredAt")]
  // ExpiryAction       String         `json:"expiryAction,omitempty"`
  // GroupID            String         `json:"groupId,omitempty"`
  // ICCID              string          `json:"iccid,omitempty"`
  // IMEILock           *IMEILock       `json:"imeiLock,omitempty"`
  // IMSI               string          `json:"imsi"`
  // IPAddress          *string         `json:"ipAddress,omitempty"`
  // LastModifiedAt     *TimestampMilli `json:"lastModifiedAt"`
  // ModuleType         string          `json:"ModuleType"`
  // MSISDN             string          `json:"msisdn"`
  // OperatorID         string          `json:"operatorId"`
  // Plan               int             `json:"plan"`
  // SerialNumber       string          `json:"serialNumber"`
  // SessionStatus      *SessionStatus  `json:"sessionStatus"`
  // SpeedClass         string          `json:"speedClass"`
  // Status             string          `json:"status"`
  // Tags               Tags            `json:"tags"`
  // TerminationEnabled bool            `json:"terminationEnabled"`
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
    let addr = url::Url::parse(&self.get_url("/v1/auth"))?;
    let reqbody = AuthRequest {
      auth_key_id: key,
      auth_key: secret,
      token_timeout_seconds: 60,
    };
    let res = self
      .http_client
      .post(&addr.to_string())
      .header(reqwest::header::CONTENT_TYPE, "application/json")
      .body(serde_json::to_string(&reqbody)?)
      .send()
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

  async fn list_subscriber(&self) -> Result<String, Box<dyn std::error::Error>> {
    let res = self
      .http_client
      .get(&self.get_url("/v1/subscribers"))
      .header(reqwest::header::CONTENT_TYPE, "application/json")
      .header(SORACOM_API_HEADER_API_KEY, &self.api_key)
      .header(SORACOM_API_HEADER_TOKEN, &self.token)
      .send()
      .await?;
    println!("{:?}", res);
    Ok(res.text().await?)
  }

  fn get_url<'a>(&self, path: &'a str) -> String {
    format!("https://{}{}", self.endpoint, path)
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

    let resp = client.list_subscriber().await;
    println!("{:?}", resp);
  }
}
