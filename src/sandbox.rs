use crate::error::Error;
use crate::model::SubscriberRegistration;
use crate::util;
use crate::util::HTTPMethod::*;
use crate::SORACOM_SANDBOX_API_ENDPOINT;

use serde_derive::*;
use url::{ParseError, Url};

use std::thread;

#[derive(Debug, Default)]
pub struct SandboxClient {
  http_client: util::HTTPClient,
  endpoint: String,
  token: Option<SandboxToken>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxInitCredential {
  pub email: String,
  pub password: String,
  #[serde(rename = "authKeyId")]
  pub auth_key_id: String,
  #[serde(rename = "authKey")]
  pub auth_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxToken {
  #[serde(rename = "operatorId")]
  pub operator_id: String,
  #[serde(rename = "apiKey")]
  pub api_key: String,
  #[serde(rename = "token")]
  pub token: String,
}

impl Drop for SandboxClient {
  /// Operatorの有効期限をプログラム中に制限する
  fn drop(&mut self) {
    let token = match &self.token {
      Some(x) => x.clone(),
      _ => return,
    };
    let endpoint = self.endpoint.clone();
    let handle = thread::spawn(move || {
      delete_operator(&endpoint, &token).unwrap();
    });
    handle.join().unwrap();
  }
}

/// delete sandbox operetor
fn delete_operator(endpoint_url: &str, token: &SandboxToken) -> Result<(), Error> {
  let url = Url::parse(
    format!(
      "https://{}/v1/sandbox/operators/{}",
      endpoint_url, token.operator_id,
    )
    .as_ref(),
  )?;
  util::sync_req(DELETE, url, None)
}

/// SORACOM SandBox API Client
///
impl SandboxClient {
  pub fn new() -> Self {
    Self {
      http_client: util::HTTPClient::new(),
      endpoint: SORACOM_SANDBOX_API_ENDPOINT.to_owned(),
      token: None,
    }
  }

  pub async fn init(&mut self, cred: &SandboxInitCredential) -> Result<SandboxToken, Error> {
    let url = self.build_url("/v1/sandbox/init")?;
    let token: SandboxToken = self
      .http_client
      .request_json(POST, url, Some(serde_json::to_string(&cred)?))
      .await?;
    self.token = Some(token.to_owned());
    Ok(token)
  }

  pub async fn create_subscriber(self) -> Result<SubscriberRegistration, Error> {
    let url = self.build_url("/v1/sandbox/subscribers/create")?;
    let subs: SubscriberRegistration = self.http_client.request_json(POST, url, None).await?;
    Ok(subs)
  }

  pub async fn delete_operator(&self, token: SandboxToken) -> Result<(), Error> {
    let url = self.build_url(format!("/v1/sandbox/operators/{}", token.operator_id).as_ref())?;
    self.http_client.request(DELETE, url, None).await
  }

  fn build_url<'a>(&self, path: &'a str) -> Result<Url, ParseError> {
    Url::parse(format!("https://{}{}", self.endpoint, path).as_ref())
  }
}

pub fn read_credential_for_test() -> Result<SandboxInitCredential, Error> {
  let email = std::env::var("SANDBOX_EMAIL_FOR_TEST")?;
  let password = std::env::var("SANDBOX_PASSWORD_FOR_TEST")?;
  let auth_key_id = std::env::var("SORACOM_AUTHKEY_ID_FOR_TEST")?;
  let auth_key = std::env::var("SORACOM_AUTHKEY_FOR_TEST")?;
  Ok(SandboxInitCredential {
    email,
    password,
    auth_key_id,
    auth_key,
  })
}

#[cfg(test)]
mod tests {
  use crate::error::*;
  use crate::sandbox::*;

  #[tokio::test]
  async fn sandbox() -> Result<(), Box<Error>> {
    let mut client = SandboxClient::new();
    let cred = read_credential_for_test().unwrap();
    let _token = client.init(&cred).await.unwrap();
    // println!("{:?}", token);
    let subs = client.create_subscriber().await.unwrap();
    println!("{:?}", subs);
    Ok(())
  }
}
