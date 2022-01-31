use crate::flags;
use regex::Regex;
use std::cmp::Ordering;

const VALID_TAGS_REGEXP: &str =
    r#""name"\s*:\s*"(?P<tag>(?P<semver>latest|beta|plexpass|\d+\.\d+.\d+)[^"]*)""#;

const DEFAULT_TAGS_COUNT: u8 = 3;
const DEFAULT_VERSION_JUMP: u8 = 1;

pub(crate) const DOCKER_PLEX_IMAGE_NAME: &str = "plexinc/pms-docker";
pub(crate) const DOCKER_PLEX_IMAGE_TAG_LATEST: &str = "latest";

impl flags::GetLastPlexTags {
    pub(crate) fn run(self) -> anyhow::Result<()> {
        let num = self.num.unwrap_or(DEFAULT_TAGS_COUNT);
        let jump = self.jump.unwrap_or(DEFAULT_VERSION_JUMP);
        let skip_tags = {
            if self.skip_tag.is_empty() {
                vec!["beta".to_owned(), "plexpass".to_owned()]
            } else {
                self.skip_tag.clone()
            }
        };

        let mut tags: Vec<String> = vec![];

        let tag_latest = DOCKER_PLEX_IMAGE_TAG_LATEST.to_owned();
        if !self.skip_tag.contains(&tag_latest) {
            tags.push("latest".to_owned());
        }

        let url = format!(
            "https://registry.hub.docker.com/v1/repositories/{image}/tags",
            image = DOCKER_PLEX_IMAGE_NAME
        );

        let available_tags = ureq::get(&url)
            .set("Accept", "application/json")
            .call()?
            .into_string()?;

        let re = Regex::new(VALID_TAGS_REGEXP)?;
        let mut tags: Vec<(SemverOrString, &str)> = re
            .captures_iter(&available_tags)
            .map(|c| {
                (
                    match semver::Version::parse(c.name("semver").unwrap().as_str()) {
                        Ok(v) => SemverOrString::Semver(v),
                        Err(_) => SemverOrString::Str(c.name("semver").unwrap().as_str()),
                    },
                    c.name("tag").unwrap().as_str(),
                )
            })
            .into_iter()
            .filter(|(s, _)| match s {
                SemverOrString::Semver(s) => !skip_tags.contains(&s.to_string()),
                SemverOrString::Str(s) => !skip_tags.contains(&String::from(*s)),
            })
            .collect();

        tags.sort();

        let mut previous_seen_version = SemverOrString::Str("");
        let mut next_min_invalid_version =
            semver::Version::new(u64::max_value(), u64::max_value(), u64::max_value());

        let mut ret: Vec<&str> = vec![];

        for (v, tag) in tags {
            let tmp = v.clone();
            match v {
                SemverOrString::Str(_) => ret.push(tag),
                SemverOrString::Semver(s) if matches!(previous_seen_version, SemverOrString::Str(v) if !v.is_empty()) =>
                {
                    next_min_invalid_version =
                        semver::Version::new(s.major, s.minor - u64::from(jump), 0);
                }
                SemverOrString::Semver(s) if s < next_min_invalid_version => {
                    ret.push(tag);
                    next_min_invalid_version =
                        semver::Version::new(s.major, s.minor - u64::from(jump), 0);
                }
                SemverOrString::Semver(_) => {}
            }
            previous_seen_version = tmp;

            if ret.len() >= num.into() {
                break;
            }
        }

        println!(r#"["{}"]"#, ret.join(r#"",""#));

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SemverOrString<'a> {
    Str(&'a str),
    Semver(semver::Version),
}

// The sorting ensures the following order of the tags:
// * latest
// * any other non-semver strings (e.g. beta, plexpass) sorted alphabetically
// * semver tags sorted in descending order
impl<'a> PartialOrd for SemverOrString<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let SemverOrString::Str(s) = self {
            if s == &"latest" {
                return Some(Ordering::Less);
            }

            if let SemverOrString::Str(s2) = other {
                if s2 != &"latest" {
                    return Some(s.cmp(s2));
                } else {
                    return Some(Ordering::Greater);
                }
            }

            return Some(Ordering::Less);
        } else if let SemverOrString::Str(s) = other {
            if s == &"latest" {
                return Some(Ordering::Greater);
            }

            if let SemverOrString::Str(s2) = self {
                if s2 != &"latest" {
                    return Some(s2.cmp(s));
                } else {
                    return Some(Ordering::Less);
                }
            }

            return Some(Ordering::Greater);
        } else if let SemverOrString::Semver(v) = self {
            if let SemverOrString::Semver(v2) = other {
                return Some(v.cmp(v2).reverse());
            }
        }

        None
    }
}

impl<'a> Ord for SemverOrString<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
