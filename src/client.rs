use crate::model::*;
use crate::option;
use crate::option::QueryParams;
use crate::*;
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

  fn auth_token(&mut self, api_key: &str, token: &str) {
    self.api_key = api_key.to_owned();
    self.token = token.to_owned();
  }

  pub async fn register_subscriber(
    &self,
    regi: &SubscriberRegistration,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let url = &mut self.get_url(format!("/v1/subscribers/{}/register", regi.imsi).as_ref())?;
    let res = self
      ._post(&url.to_string(), regi.fmt_registration_body())
      .await?;
    let resbody = res.text().await?;
    println!("{:?}", &resbody);
    Ok(())
  }

  pub async fn list_subscriber(
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
    let res = self._get(&url).await?;
    let resbody = res.text().await?;
    println!("{:?}", &resbody);
    let resbody: Vec<Subscriber> = serde_json::from_str(&resbody)?;
    Ok(resbody)
  }

  pub async fn get_subscriber(&self, imsi: &str) -> Result<Subscriber, Box<dyn std::error::Error>> {
    let url = &mut self.get_url(format!("/v1/subscribers/{}", imsi).as_ref())?;

    let res = self._get(&url).await?;
    let resbody = res.text().await?;
    let resbody: Subscriber = serde_json::from_str(&resbody)?;
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

  async fn _get<'a>(&self, url: &Url) -> reqwest::Result<reqwest::Response> {
    self
      .http_client
      .get(&url.to_string())
      .header(reqwest::header::CONTENT_TYPE, "application/json")
      .header(SORACOM_API_HEADER_API_KEY, &self.api_key)
      .header(SORACOM_API_HEADER_TOKEN, &self.token)
      .send()
      .await
  }
  async fn _post<'a>(&self, url: &str, reqbody: String) -> reqwest::Result<reqwest::Response> {
    self
      .http_client
      .post(url)
      .header(reqwest::header::CONTENT_TYPE, "application/json")
      .header(SORACOM_API_HEADER_API_KEY, &self.api_key)
      .header(SORACOM_API_HEADER_TOKEN, &self.token)
      .body(reqbody)
      .send()
      .await
  }
}

#[cfg(test)]
mod tests {
  use crate::client::*;
  use crate::sandbox;
  use std::collections::HashMap;
  use tokio::runtime::Runtime;

  #[test]
  fn with_sandbox() -> Result<(), Box<dyn std::error::Error>> {
    let mut rt = Runtime::new()?;
    rt.block_on(async {
      let mut client = sandbox::SandboxClient::new();
      let cred = sandbox::read_credential_for_test().unwrap();
      let token = client.init(&cred).await.unwrap();
      let subs = client.create_subscriber().await.unwrap();

      {
        let mut client = Client::new();
        client.endpoint = SORACOM_SANDBOX_API_ENDPOINT;
        client.auth_token(&token.api_key, &token.token);
        let _resp = client.register_subscriber(&subs).await.unwrap();
        // println!("{:?}", resp);
      }
    });
    println!("after done!");
    Ok(())
  }

  #[tokio::test]
  async fn auth() {
    let key = std::env::var("SORACOM_AUTHKEY_ID_FOR_TEST").unwrap();
    let secret = std::env::var("SORACOM_AUTHKEY_FOR_TEST").unwrap();
    let mut client = Client::new();
    let resp = client.auth(&key, &secret).await;
    println!("{:?}", resp);

    let resp = client.list_subscriber(None).await;
    println!("list {:?}", resp);

    let subs = resp.unwrap();

    let resp = client.get_subscriber(&subs[0].imsi).await;
    println!("get {:?}", resp);
  }
}
