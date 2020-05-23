#![allow(dead_code)]
#[macro_use]
extern crate serde_derive;
extern crate serde_qs as qs;

pub mod client;
pub mod error;
pub mod model;
pub mod object;
pub mod option;
pub mod sandbox;
pub(crate) mod util;

pub const SORACOM_API_ENDPOINT: &'static str = "api.soracom.io";
pub const SORACOM_GLOBAL_API_ENDPOINT: &'static str = "g.api.soracom.io";
pub const SORACOM_SANDBOX_API_ENDPOINT: &'static str = "api-sandbox.soracom.io";
pub(crate) const SORACOM_API_HEADER_API_KEY: &'static str = "X-Soracom-API-Key";
pub(crate) const SORACOM_API_HEADER_TOKEN: &'static str = "X-Soracom-Token";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
