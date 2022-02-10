# Changelog

## [0.1.0](https://github.com/andrey-yantsen/plex-api.rs/compare/v0.0.3...v0.1.0) (2022-02-10)


### ⚠ BREAKING CHANGES

* rename Client to HttpClient

### Features

* add a method for linking new players ([33d811f](https://github.com/andrey-yantsen/plex-api.rs/commit/33d811fc38b728bd4355b385a8b8d3541a6307a4))
* add a stub for Players (aka Clients) ([769d963](https://github.com/andrey-yantsen/plex-api.rs/commit/769d9631e729abbd75f67347868295011e2caa87))
* add structs for receiving webhooks ([cc314e0](https://github.com/andrey-yantsen/plex-api.rs/commit/cc314e071252e55f8ce9daa86bcd66baa965f9d9))
* **device:** improve connection logic from Device ([e6d0741](https://github.com/andrey-yantsen/plex-api.rs/commit/e6d074129c87b79e2e6bcec0d1b736b91f4b15dd))
* **srv:** add new uuid to the Feature enum ([3cd2bae](https://github.com/andrey-yantsen/plex-api.rs/commit/3cd2bae3e91b8b70dfc566cd8c33b88f7ef69b8f))
* **srv:** load server info on connection ([22939a0](https://github.com/andrey-yantsen/plex-api.rs/commit/22939a0e866fe0d9e4562da0414025fc84c7cf0c))
* **tests:** rewrite tests with rstest ([1c193ef](https://github.com/andrey-yantsen/plex-api.rs/commit/1c193ef01d1d43b0304f0b1d8f5ec26973210c86))
* **webhooks:** make most of the fields optional ([a23fd30](https://github.com/andrey-yantsen/plex-api.rs/commit/a23fd303f126fc3bd1dec0fb050893fe65de2749))


### Bug Fixes

* **srv:** country_code is missing in older Plex ([69b1318](https://github.com/andrey-yantsen/plex-api.rs/commit/69b13181e06f08486707a4b8f187ff13e3f45316))
* **srv:** make Server::claim() valid ([64d6eee](https://github.com/andrey-yantsen/plex-api.rs/commit/64d6eee0a565022d69c9d8ebe021951f9d8d643c))
* **srv:** music_analysis is missing in older Plex ([0b05a86](https://github.com/andrey-yantsen/plex-api.rs/commit/0b05a8642068edee9c1af0e4f0368978f6ac304b))


### Code Refactoring

* rename Client to HttpClient ([bb4e05e](https://github.com/andrey-yantsen/plex-api.rs/commit/bb4e05ecaa14461c822c9a5b99aeec0b2337750e))


### Miscellaneous

* **deps:** bump serde_with from 1.11.0 to 1.12.0 ([f25394a](https://github.com/andrey-yantsen/plex-api.rs/commit/f25394a0f995cada7d1599ccde033d5c344dcfc6))
