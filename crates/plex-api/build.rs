use serde::Deserialize;

fn main() {
    generate_features();
}

const FEATURE_ENUM_FILE_PATH: &str = "src/media_container/server/feature.rs";
const FEATURE_MOCK_FILE_PATH: &str = "tests/mocks/myplex/api/v2/features.json";

// Features that a not reported by the API, but returned by the server.
const EXTRA_FEATURES: &[&str] = &[
    "download_certificates",
    "loudness",
    "server-manager",
    "shared-radio",
    "photo_autotags",
];

#[derive(Deserialize, PartialEq, Eq)]
struct Feature {
    id: String,
    uuid: String,
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

        self.id
            .replace('-', " ")
            .replace('_', " ")
            .replace("  ", " ")
            .replace("  ", " ")
            .replace("  ", " ")
            .to_lowercase()
            .to_title_case()
            .replace(' ', "")
    }
}

impl<'a> PartialOrd for Feature {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.enum_name().partial_cmp(&other.enum_name())
    }
}

impl<'a> Ord for Feature {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn generate_features() {
    use serde_json::from_reader;
    use std::fs::File;
    use std::io::Write;

    println!("cargo:rerun-if-changed={}", FEATURE_MOCK_FILE_PATH);
    let mut f = File::create(FEATURE_ENUM_FILE_PATH).unwrap();
    f.write_all(b"use serde::{Deserialize, Serialize};\n")
        .unwrap();
    f.write_all(b"use serde_plain::derive_display_from_serialize;\n\n")
        .unwrap();
    f.write_all(b"#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]\n")
        .unwrap();
    f.write_all(b"#[allow(clippy::enum_variant_names)]\n")
        .unwrap();
    f.write_all(b"#[rustfmt::skip]\n").unwrap();
    f.write_all(b"pub enum Feature {\n").unwrap();

    let mut data: Vec<Feature> = from_reader(File::open(FEATURE_MOCK_FILE_PATH).unwrap()).unwrap();

    for extra in EXTRA_FEATURES {
        data.push(Feature {
            id: extra.to_string(),
            uuid: "".to_string(),
        });
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
        f.write_all(format!("    {},\n", feature.enum_name()).as_bytes())
            .unwrap();
    }

    f.write_all(b"    UnknownUuid(String),\n").unwrap();
    f.write_all(b"    #[cfg(not(feature = \"tests_deny_unknown_fields\"))]\n")
        .unwrap();
    f.write_all(b"    #[serde(other)]\n").unwrap();
    f.write_all(b"    UnknownValue,\n").unwrap();
    f.write_all(b"}\n").unwrap();
    f.write_all(
        b"
impl ::std::str::FromStr for Feature {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> ::std::result::Result<Feature, Self::Err> {
        let result = serde_plain::from_str(s);

        #[cfg(not(feature = \"tests_deny_unknown_fields\"))]
        let is_unknown_value = result.is_err() || matches!(result, Ok(Feature::UnknownValue));
        #[cfg(feature = \"tests_deny_unknown_fields\")]
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
",
    )
    .unwrap();
    f.write_all(b"derive_display_from_serialize!(Feature);\n")
        .unwrap();
}
