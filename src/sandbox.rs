use crate::error::Error;
use crate::*;
use serde::{Deserialize, Deserializer};
use serde_derive::*;
use url::{ParseError, Url};

#[derive(Debug, Default)]
pub struct SandboxClient<'a> {
  http_client: reqwest::Client,
  endpoint: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxInitCredential {
  email: String,
  password: String,
  #[serde(rename = "authKeyId")]
  auth_key_id: String,
  #[serde(rename = "authKey")]
  auth_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxToken {
  #[serde(rename = "operatorId")]
  operator_id: String,
  #[serde(rename = "apiKey")]
  api_key: String,
  #[serde(rename = "token")]
  token: String,
}

impl SandboxClient<'_> {
  fn new() -> Self {
    Self {
      http_client: reqwest::Client::builder().build().unwrap(),
      endpoint: SORACOM_SANDBOX_API_ENDPOINT,
      ..Default::default()
    }
  }

  pub async fn init(&self, cred: &SandboxInitCredential) -> Result<SandboxToken, Error> {
    let url = self.build_url("/v1/sandbox/init")?;
    let res = self.raw_post(&url, serde_json::to_string(&cred)?).await?;
    match res.status() {
      reqwest::StatusCode::OK => {
        let resbody = res.text().await?;
        let resbody: SandboxToken = serde_json::from_str(&resbody)?;
        Ok(resbody)
      }
      x => {
        let resbody = res.text().await?;
        Err(error::new_http_error(x, resbody))
      }
    }
  }

  fn build_url<'a>(&self, path: &'a str) -> Result<Url, ParseError> {
    Url::parse(format!("https://{}{}", self.endpoint, path).as_ref())
  }

  async fn raw_post<'a>(&self, url: &Url, reqbody: String) -> reqwest::Result<reqwest::Response> {
    self
      .http_client
      .post(&url.to_string())
      .header(reqwest::header::CONTENT_TYPE, "application/json")
      .body(reqbody)
      .send()
      .await
  }
}

#[cfg(test)]
mod tests {
  use crate::sandbox::*;

  #[tokio::test]
  async fn init() {
    let email = std::env::var("SANDBOX_EMAIL_FOR_TEST").unwrap();
    let password = std::env::var("SANDBOX_PASSWORD_FOR_TEST").unwrap();
    let auth_key_id = std::env::var("SORACOM_AUTHKEY_ID_FOR_TEST").unwrap();
    let auth_key = std::env::var("SORACOM_AUTHKEY_FOR_TEST").unwrap();

    let client = SandboxClient::new();
    let cred = SandboxInitCredential {
      email,
      password,
      auth_key_id,
      auth_key,
    };
    let resp = client.init(&cred).await.unwrap();
    println!("{:?}", resp)
  }
}
