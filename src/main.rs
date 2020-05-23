extern crate soracom_sdk_rs;
use soracom_sdk_rs::object;





#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let target_addr = "https://postman-echo.com/get?foo1=bar1&foo2=bar2";
  println!("hello world");
  let mut myo = object::MyObject::new("rectod");
  println!("show {}", &myo);
  myo.set_target(&target_addr);
  let target = url::Url::parse(target_addr)?;
  myo.async_print().await;
  let res: object::PostmanResponse = myo.get(target.clone(), None).await?;

  println!("{:?}", res);

  Ok(())
}
