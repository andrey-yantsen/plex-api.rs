use serde::Deserialize;

fn main() {
    generate_features();
}

const FEATURE_ENUM_FILE_PATH: &str = "src/media_container/server/feature.rs";

const FEATURE_MOCK_FILE_PATH: &str = "tests/mocks/myplex/api/v2/features.json";

// Features that are not reported by the API, but returned by the server.
// Probably those are deprecated.
const EXTRA_FEATURES: &[&str] = &[
    "download_certificates",
    "loudness",
    "server-manager",
    "shared-radio",
    "photo_autotags",
];

#[derive(Deserialize, PartialEq, Eq, Debug)]
struct Feature {
    id: String,
    uuid: String,
    #[serde(default)]
    deprecated: bool,
}

impl Feature {
    fn id(&self) -> &str {
        &self.id
    }

    fn uuid(&self) -> &str {
        &self.uuid
    }

    fn enum_name(&self) -> String {
        use inflections::Inflect;
        use regex::Regex;

        Regex::new(r"[-_\s]+|([a-z])([A-Z])")
            .unwrap()
            .replace_all(&self.id, r"$1 $2")
            .to_lowercase()
            .to_title_case()
            .replace(' ', "")
    }
}

impl PartialOrd for Feature {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.enum_name().partial_cmp(&other.enum_name())
    }
}

impl Ord for Feature {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn generate_features() {
    use regex::Regex;
    use serde_json::from_reader;
    use std::fs::File;
    use std::io::Write;

    println!("cargo:rerun-if-changed={FEATURE_MOCK_FILE_PATH}");

    let old_features_enum_file =
        std::fs::read_to_string(FEATURE_ENUM_FILE_PATH).unwrap_or_default();
    let mut old_features: Vec<Feature> = vec![];

    for cap in Regex::new(
        r#"(?xs)
        \#\[serde\(\s*
            rename\s*=\s*"(?P<rename>[^"]+)",?
            (?:\s+alias\s*=\s*"(?P<alias>[^"]+)")?
        \s*\)\]"#,
    )
    .unwrap()
    .captures_iter(&old_features_enum_file)
    {
        let id = cap.name("rename").unwrap().as_str();
        let uuid = cap.name("alias").map_or("", |a| a.as_str());

        old_features.push(Feature {
            id: id.to_string(),
            uuid: uuid.to_string(),
            deprecated: false,
        });
    }

    let mut f = File::create(FEATURE_ENUM_FILE_PATH).unwrap();
    f.write_all(
        br#"#![allow(deprecated)]

use serde::{Deserialize, Serialize};
use serde_plain::derive_display_from_serialize;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
#[rustfmt::skip]
pub enum Feature {
"#,
    )
    .unwrap();

    let mut data: Vec<Feature> = from_reader(File::open(FEATURE_MOCK_FILE_PATH).unwrap()).unwrap();

    for extra in EXTRA_FEATURES {
        data.push(Feature {
            id: extra.to_string(),
            uuid: "".to_string(),
            deprecated: false,
        });
    }

    for old_feature in old_features {
        if !data.contains(&old_feature) {
            let deprecated_feature = Feature {
                id: old_feature.id.clone(),
                uuid: old_feature.uuid.clone(),
                deprecated: true,
            };
            data.push(deprecated_feature);
        }
    }

    data.sort();

    for feature in data {
        if feature.uuid().is_empty() {
            f.write_all(format!("    #[serde(rename = \"{}\")]\n", feature.id()).as_bytes())
                .unwrap();
        } else {
            f.write_all(
                format!(
                    "    #[serde(\n        rename = \"{}\",\n        alias = \"{}\"\n    )]\n",
                    feature.id(),
                    feature.uuid()
                )
                .as_bytes(),
            )
            .unwrap();
        }

        if feature.deprecated {
            f.write_all(b"    #[deprecated]\n").unwrap();
        }

        f.write_all(format!("    {},\n", feature.enum_name()).as_bytes())
            .unwrap();
    }

    f.write_all(
        br#"    UnknownUuid(String),
    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[serde(other)]
    UnknownValue,
}

impl ::std::str::FromStr for Feature {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> ::std::result::Result<Feature, Self::Err> {
        let result = serde_plain::from_str(s);

        #[cfg(not(feature = "tests_deny_unknown_fields"))]
        let is_unknown_value = matches!(result, Ok(Feature::UnknownValue));
        #[cfg(feature = "tests_deny_unknown_fields")]
        let is_unknown_value = result.is_err();

        if is_unknown_value
            && s.len() == 36
            && s.as_bytes()[8] == b'-'
            && s.as_bytes()[13] == b'-'
            && s.as_bytes()[18] == b'-'
            && s.as_bytes()[23] == b'-'
        {
            return Ok(Feature::UnknownUuid(s.to_string()));
        }

        result
    }
}
derive_display_from_serialize!(Feature);

#[cfg(test)]
mod test {
    use crate::media_container::server::Feature;
    use std::str::FromStr;

    #[cfg(not(feature = "tests_deny_unknown_fields"))]
    #[plex_api_test_helper::offline_test]
    fn test_unknown_feature() {
        let f = Feature::from_str("unknown_value").unwrap();
        assert_eq!(f, Feature::UnknownValue);
    }

    #[plex_api_test_helper::offline_test]
    fn test_unknown_feature_uuid() {
        let f = Feature::from_str("cc9bea3b-aaaa-bbbb-cccc-4958bb129caa").unwrap();
        assert_eq!(
            f,
            Feature::UnknownUuid("cc9bea3b-aaaa-bbbb-cccc-4958bb129caa".to_owned())
        );
    }

    #[plex_api_test_helper::offline_test]
    fn test_known_feature() {
        let f = Feature::from_str("webhooks").unwrap();
        assert_eq!(f, Feature::Webhooks);

        let f = Feature::from_str("6f82ca43-6117-4e55-ae0e-5ea3b3e99a96").unwrap();
        assert_eq!(f, Feature::Webhooks);
    }

    #[plex_api_test_helper::offline_test]
    fn test_known_deprecated_feature() {
        let f = Feature::from_str("optimize-server-users-endpoint").unwrap();
        assert_eq!(f, Feature::OptimizeServerUsersEndpoint);

        let f = Feature::from_str("ddd730e1-a0a0-429f-a7d3-7c5001d24497").unwrap();
        assert_eq!(f, Feature::OptimizeServerUsersEndpoint);
    }
}
"#,
    )
    .unwrap();
}
