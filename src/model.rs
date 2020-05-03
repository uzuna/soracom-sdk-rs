use chrono::offset::TimeZone;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer};
use serde_derive::*;
use std::collections::HashMap;

// use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AuthRequest<'a> {
    #[serde(rename = "authKeyId")]
    pub(crate) auth_key_id: &'a str,
    #[serde(rename = "authKey")]
    pub(crate) auth_key: &'a str,
    #[serde(rename = "tokenTimeoutSeconds")]
    pub(crate) token_timeout_seconds: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AuthResponse<'a> {
    #[serde(rename = "apiKey")]
    pub(crate) api_key: &'a str,
    #[serde(rename = "operatorId")]
    pub(crate) operator_id: &'a str,
    #[serde(rename = "token")]
    pub(crate) token: &'a str,
}

/// map of tags name and value
type Tags = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct IMEILock {
    imei: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cell {
    #[serde(rename = "radioType")]
    radio_type: String,
    mcc: u16,
    mnc: u16,
    tac: u16,
    eci: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionStatus {
    #[serde(rename = "dnsServers")]
    dns_servers: Option<Vec<String>>,
    imei: Option<String>,
    #[serde(rename = "lastUpdatedAt", deserialize_with = "optdatefmt")]
    last_updated_at: Option<DateTime<Utc>>,
    location: Option<String>,
    cell: Option<Cell>,
    online: bool,
    ue_ip_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscriber {
    pub apn: String,

    #[serde(rename = "createdAt", deserialize_with = "datefmt")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "expiredAt", deserialize_with = "optdatefmt")]
    pub expired_at: Option<DateTime<Utc>>,

    #[serde(rename = "expiryAction")]
    pub expiry_action: Option<String>,

    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub iccid: Option<String>,

    #[serde(rename = "imeiLock")]
    pub imei_lock: Option<IMEILock>,
    pub imsi: String,

    #[serde(rename = "ipAddress")]
    pub ip_address: Option<String>,

    #[serde(rename = "lastModifiedAt", deserialize_with = "optdatefmt")]
    pub last_modified_at: Option<DateTime<Utc>>,

    #[serde(rename = "moduleType")]
    pub module_type: String,

    pub msisdn: String,

    #[serde(rename = "operatorId")]
    pub operator_id: String,
    pub plan: i8,

    #[serde(rename = "serialNumber")]
    pub serial_number: String,

    #[serde(rename = "sessionStatus")]
    pub session_status: Option<SessionStatus>,

    #[serde(rename = "speedClass")]
    pub speed_class: String,
    pub status: String,
    pub tags: Tags,

    #[serde(rename = "terminationEnabled")]
    pub termination_enabled: bool,
}

/// unixtime_microsecをDatetime<Utc>に変換
fn datefmt<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = i64::deserialize(deserializer)?;
    let ts = NaiveDateTime::from_timestamp(s / 1000, (s as u32 % 1000) * 1000000);
    Ok(Utc.from_utc_datetime(&ts))
}

fn optdatefmt<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    enum Helper {
        Value(i64),
        Null,
    }
    let helper = Helper::deserialize(deserializer)?;
    match helper {
        Helper::Value(s) => {
            let ts = NaiveDateTime::from_timestamp(s / 1000, (s as u32 % 1000) * 1000000);
            Ok(Some(Utc.from_utc_datetime(&ts)))
        }
        Helper::Null => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use crate::model::*;
    #[test]
    fn parse() {
        let resbody = "[{\"imsi\":\"001050910800000\",\"msisdn\":\"991649918745\",\"ipAddress\":null,\"operatorId\":\"OP9920648489\",\"apn\":\"soracom.io\",\"type\":\"s1.standard\",\"groupId\":null,\"createdAt\":1586584405503,\"lastModifiedAt\":1586584405516,\"expiredAt\":null,\"registeredTime\":null,\"expiryAction\":null,\"terminationEnabled\":false,\"status\":\"ready\",\"tags\":{},\"sessionStatus\":null,\"imeiLock\":null,\"speedClass\":\"s1.standard\",\"moduleType\":\"trio\",\"plan\":1,\"iccid\":\"0012310019719419000\",\"serialNumber\":\"0011900089423197194\",\"subscription\":\"plan01s\",\"lastPortMappingCreatedTime\":null,\"createdTime\":1586584405503,\"expiryTime\":null,\"lastModifiedTime\":1586584405516}]";
        let resbody: Vec<Subscriber> = serde_json::from_str(&resbody).unwrap();
        println!("{:?}", resbody);
        let resbody ="[{\"imsi\":\"001050910800000\",\"msisdn\":\"991649918745\",\"ipAddress\":\"10.145.1.2\",\"operatorId\":\"OP9920648489\",\"apn\":\"soracom.io\",\"type\":\"s1.standard\",\"groupId\":\"1135dedb-ec7e-4e8c-ae43-8f2902aaae10\",\"createdAt\":1586584405503,\"lastModifiedAt\":1586584405516,\"expiredAt\":null,\"registeredTime\":null,\"expiryAction\":null,\"terminationEnabled\":false,\"status\":\"active\",\"tags\":{\"tagA\":\"valueA\"},\"sessionStatus\":{\"gtpcTeid\":0,\"lastUpdatedAt\":1586785804388,\"imei\":null,\"location\":null,\"ueIpAddress\":null,\"dnsServers\":null,\"online\":false},\"imeiLock\":null,\"speedClass\":\"s1.standard\",\"moduleType\":\"trio\",\"plan\":1,\"iccid\":\"0012310019719419000\",\"serialNumber\":\"0011900089423197194\",\"subscription\":\"plan01s\",\"lastPortMappingCreatedTime\":null,\"createdTime\":1586584405503,\"expiryTime\":null,\"lastModifiedTime\":1586584405516}]";
        let resbody: Vec<Subscriber> = serde_json::from_str(&resbody).unwrap();
        println!("{:?}", resbody);
    }
}
