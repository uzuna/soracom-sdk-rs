use std::default::Default;
extern crate serde_qs as qs;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum TagValueMatchMode {
    Unspecified,
    Exact,
    Prefix,
}

use std::fmt;
impl fmt::Display for TagValueMatchMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagValueMatchMode::Unspecified => write!(f, ""),
            TagValueMatchMode::Exact => write!(f, "exact"),
            TagValueMatchMode::Prefix => write!(f, "prefix"),
        }
    }
}

impl std::default::Default for TagValueMatchMode {
    fn default() -> Self {
        TagValueMatchMode::Unspecified
    }
}
#[derive(Debug, PartialEq, Deserialize, Serialize, Default)]
pub struct ListSubscribersOptions {
    tag_name: Option<String>,
    tag_value: Option<String>,
    tag_value_match_mode: Option<TagValueMatchMode>,
    status_filter: Option<String>,
    type_filter: Option<String>,
    limit: Option<i16>,
    last_evaluated_key: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::option::*;
    #[test]
    fn list_subscriber() {
        let opt = ListSubscribersOptions {
            tag_name: Some("aaa".to_string()),
            ..Default::default()
        };

        println!("{:?}", qs::to_string(&opt))
    }
}
