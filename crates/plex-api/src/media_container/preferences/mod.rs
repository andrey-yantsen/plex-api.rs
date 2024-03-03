mod deserializer;

use std::fmt::Display;

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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Text(s) => write!(f, "{}", s),
            Value::Int(i) => write!(f, "{}", i),
            Value::Bool(b) => write!(f, "{}", if *b { "1" } else { "0" }),
            Value::Double(d) => write!(f, "{}", d),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SettingEnumValue {
    pub value: String,
    pub hint: String,
}
