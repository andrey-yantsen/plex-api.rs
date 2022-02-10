mod deserializer;

use super::MediaContainer;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(
    all(test, feature = "tests_deny_unknown_fields"),
    serde(deny_unknown_fields)
)]
pub struct Preferences {
    #[serde(rename = "Setting", default)]
    pub settings: Vec<Setting>,
    #[serde(flatten)]
    pub media_container: MediaContainer,
}

#[derive(Debug, Clone)]
pub struct Setting {
    pub id: String,
    pub label: String,
    pub summary: String,
    pub hidden: bool,
    pub advanced: bool,
    pub group: String,
    pub value: Value,
    pub default: Value,
    pub suggested_values: Option<Vec<SettingEnumValue>>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Text(String),
    Bool(bool),
    Double(f64),
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Text(s) => s.to_owned(),
            Value::Int(i) => i.to_string(),
            Value::Bool(b) => (if *b { "1" } else { "0" }).to_owned(),
            Value::Double(d) => d.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SettingEnumValue {
    pub value: String,
    pub hint: String,
}
