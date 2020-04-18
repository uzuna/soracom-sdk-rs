#[macro_use]
extern crate serde_derive;
extern crate serde_qs as qs;

pub mod client;
pub mod consts;
pub mod model;
pub mod option;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
