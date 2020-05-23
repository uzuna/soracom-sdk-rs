// use crate::error::Error;
use crate::*;
use serde::de;
use std::fmt;

use std::collections::HashMap;
use std::thread;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostmanResponse {
  args: HashMap<String, String>,
  headers: HashMap<String, String>,
  url: String,
}

#[derive(Debug, Default)]
pub struct MyObject {
  client: util::HTTPClient,
  target: Option<String>,
  record: String,
}

impl Drop for MyObject {
  fn drop(&mut self) {
    let addr = match &self.target {
      Some(x) => x,
      _ => return,
    };
    let target = url::Url::parse(&addr).unwrap();

    let handle = thread::spawn(|| {
      util::sync_req(util::HTTPMethod::GET, target, None).unwrap();
    });
    handle.join().unwrap();
    println!("Dropping: `{}`", self.record);
  }
}

impl fmt::Display for MyObject {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.record)
  }
}

impl MyObject {
  pub fn new(record: &str) -> Self {
    Self {
      client: util::HTTPClient::new(),
      target: None,
      record: record.to_owned(),
    }
  }
  pub async fn async_new(target: &str, record: &str) -> Self {
    Self {
      client: util::HTTPClient::new(),
      target: Some(target.to_owned()),
      record: record.to_owned(),
    }
  }
  pub fn set_target(&mut self, target: &str) {
    self.target = Some(target.to_owned());
  }

  pub async fn async_print(&self) {
    tokio_timer::sleep(std::time::Duration::from_millis(1));
    println!("async_print!")
  }

  pub async fn get<T>(&self, url: url::Url, body: Option<String>) -> Result<T, crate::error::Error>
  where
    T: de::DeserializeOwned,
  {
    self
      .client
      .request_json(util::HTTPMethod::GET, url, body)
      .await
  }
}
