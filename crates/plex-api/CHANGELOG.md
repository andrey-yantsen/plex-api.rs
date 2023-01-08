# Changelog

## [0.0.6](https://github.com/andrey-yantsen/plex-api.rs/compare/plex-api-v0.0.5...plex-api-v0.0.6) (2023-01-08)


### Features

* testing release-please ([31a28e1](https://github.com/andrey-yantsen/plex-api.rs/commit/31a28e12f80464c8260c0974ead9f02ad27b6112))

## 0.0.5 (2023-01-06)


### ⚠ BREAKING CHANGES

* rename Client to HttpClient

### Features

* add a method for linking new players ([33d811f](https://github.com/andrey-yantsen/plex-api.rs/commit/33d811fc38b728bd4355b385a8b8d3541a6307a4))
* add a stub for Players (aka Clients) ([769d963](https://github.com/andrey-yantsen/plex-api.rs/commit/769d9631e729abbd75f67347868295011e2caa87))
* add joined_at field to MyPlexAccount ([4b8f4a7](https://github.com/andrey-yantsen/plex-api.rs/commit/4b8f4a7ea091486035f6e9c77d617bdb8a7029f4))
* add structs for receiving webhooks ([cc314e0](https://github.com/andrey-yantsen/plex-api.rs/commit/cc314e071252e55f8ce9daa86bcd66baa965f9d9))
* **api:** Add a high-level API for browsing libraries on a server. ([a315fcd](https://github.com/andrey-yantsen/plex-api.rs/commit/a315fcd978adb6c326d338eb440e0203ae52973e))
* improve working with pins ([bcb9fd0](https://github.com/andrey-yantsen/plex-api.rs/commit/bcb9fd06c54d05acc4cd58a528a15cd9d91ae76a))
* migrate the code to serde_with v2 ([7de8395](https://github.com/andrey-yantsen/plex-api.rs/commit/7de83955b53e39f2847ec10d7146ba39f61b8573))
* **srv:** add method for getting all settings ([fafb427](https://github.com/andrey-yantsen/plex-api.rs/commit/fafb42743412ad7f49961624b48baa9d263d868c))
* **srv:** add new uuid to the Feature enum ([3cd2bae](https://github.com/andrey-yantsen/plex-api.rs/commit/3cd2bae3e91b8b70dfc566cd8c33b88f7ef69b8f))
* **srv:** add preferences management ([7454bc5](https://github.com/andrey-yantsen/plex-api.rs/commit/7454bc57d84a577ec324c48f2117296e86d88cbc))
* update Feature enum ([48f679f](https://github.com/andrey-yantsen/plex-api.rs/commit/48f679f3a839a00d741221b158afabe7c9107e62))
* **webhooks:** make most of the fields optional ([a23fd30](https://github.com/andrey-yantsen/plex-api.rs/commit/a23fd303f126fc3bd1dec0fb050893fe65de2749))


### Bug Fixes

* certificateVersion is not returned from the myplex API ([8d2d8e4](https://github.com/andrey-yantsen/plex-api.rs/commit/8d2d8e46d8869d812a45a41bb2aa20cad7d4721a))
* get rid of the deprecated method usage ([452f7e3](https://github.com/andrey-yantsen/plex-api.rs/commit/452f7e3cde0f88ab36d5271d09f622ecd8edeb71))
* **srv:** older PMS can send numbers as strings ([96bb0bf](https://github.com/andrey-yantsen/plex-api.rs/commit/96bb0bfc6e8d7259bf542f4224e1f79f0b1c5ba3))


### Code Refactoring

* rename Client to HttpClient ([bb4e05e](https://github.com/andrey-yantsen/plex-api.rs/commit/bb4e05ecaa14461c822c9a5b99aeec0b2337750e))


### Miscellaneous

* add examples for signin & signout ([dbc7086](https://github.com/andrey-yantsen/plex-api.rs/commit/dbc7086ba7e82441007d11f3fd5842618469aac2))
* change MSRV to 1.56 and edition to 2021 ([7cb77b0](https://github.com/andrey-yantsen/plex-api.rs/commit/7cb77b00befcc5265c81e76e74bc8e157a2f0ff5))
* change MSRV to 1.61.0 ([d757b27](https://github.com/andrey-yantsen/plex-api.rs/commit/d757b272264a94c71ed03b02f08d8e76b9fe07d1))
* **deps:** bump async-std from 1.10.0 to 1.11.0 ([4aa9016](https://github.com/andrey-yantsen/plex-api.rs/commit/4aa9016c6918d148c07eb76e3100d225721e6524))
* **deps:** bump async-std from 1.11.0 to 1.12.0 ([7ca1b43](https://github.com/andrey-yantsen/plex-api.rs/commit/7ca1b43cb9e979133480038abc61037a9f8e9ae4))
* **deps:** bump futures from 0.3.21 to 0.3.23 ([ef4c726](https://github.com/andrey-yantsen/plex-api.rs/commit/ef4c72629cae00c865a2b9ce22599d89549e6cde))
* **deps:** bump futures from 0.3.23 to 0.3.24 ([7d06e1e](https://github.com/andrey-yantsen/plex-api.rs/commit/7d06e1e0337d422299aa96d5364d6e2f4d03be95))
* **deps:** bump futures from 0.3.24 to 0.3.25 ([356b230](https://github.com/andrey-yantsen/plex-api.rs/commit/356b2301f3e425f3f5f3d28d9712494235ec163f))
* **deps:** bump http from 0.2.6 to 0.2.7 ([25eb8cd](https://github.com/andrey-yantsen/plex-api.rs/commit/25eb8cd524ae1631455fecb0a8a8c5c6bcf846c6))
* **deps:** bump http from 0.2.7 to 0.2.8 ([2bf3813](https://github.com/andrey-yantsen/plex-api.rs/commit/2bf38138b70d64d8ac7a51b9df9c3e1f256dd7cc))
* **deps:** bump isahc from 1.6.0 to 1.7.0 ([51d0c84](https://github.com/andrey-yantsen/plex-api.rs/commit/51d0c84e421e62ca4123821f47ee1ed5c2d7e293))
* **deps:** bump isahc from 1.7.0 to 1.7.1 ([473bbc6](https://github.com/andrey-yantsen/plex-api.rs/commit/473bbc626beaee31cd4c1ca81a17e6fa9b8a38d7))
* **deps:** bump isahc from 1.7.1 to 1.7.2 ([6480b7b](https://github.com/andrey-yantsen/plex-api.rs/commit/6480b7b2fed656fce08a57dcedebf838ce4d6b71))
* **deps:** bump monostate from 0.1.1 to 0.1.2 ([d43ded7](https://github.com/andrey-yantsen/plex-api.rs/commit/d43ded7b2d26fa1fe82e0d6ffb7a4b475ef1e069))
* **deps:** bump quick-xml from 0.22.0 to 0.23.0 ([3242b93](https://github.com/andrey-yantsen/plex-api.rs/commit/3242b93adebbc759b0cb62a2c222b5515e43085b))
* **deps:** bump quick-xml from 0.23.0 to 0.24.0 ([235b116](https://github.com/andrey-yantsen/plex-api.rs/commit/235b116f4fa6843dbd287d9908e82a5cddec51da))
* **deps:** bump quick-xml from 0.24.0 to 0.25.0 ([7a848eb](https://github.com/andrey-yantsen/plex-api.rs/commit/7a848eb6143bfba123103682e0cec29cd0d980e6))
* **deps:** bump quick-xml from 0.25.0 to 0.26.0 ([e1e799f](https://github.com/andrey-yantsen/plex-api.rs/commit/e1e799f61d432f6c1f1a8be5da10fe4414a4dd90))
* **deps:** bump quick-xml from 0.26.0 to 0.27.1 ([8be7d4f](https://github.com/andrey-yantsen/plex-api.rs/commit/8be7d4fdc83e4cd22be5563434d19b0be36d3cd6))
* **deps:** bump regex from 1.5.6 to 1.6.0 ([8c843ea](https://github.com/andrey-yantsen/plex-api.rs/commit/8c843ea7600f79b5389140482c8d53a4e7565536))
* **deps:** bump regex from 1.6.0 to 1.7.0 ([b2bf01d](https://github.com/andrey-yantsen/plex-api.rs/commit/b2bf01d0d6d760cc004ef7fcd20173c9dfb103d9))
* **deps:** bump rpassword from 5.0.1 to 6.0.1 ([25c1c3f](https://github.com/andrey-yantsen/plex-api.rs/commit/25c1c3fd9504b2f91ed97aab8caa44d9a5bfc932))
* **deps:** bump rpassword from 6.0.1 to 7.0.0 ([20db373](https://github.com/andrey-yantsen/plex-api.rs/commit/20db37398d0fa84727af1330ff0b421bbe22c225))
* **deps:** bump rpassword from 7.0.0 to 7.1.0 ([be47b33](https://github.com/andrey-yantsen/plex-api.rs/commit/be47b331dfb6aaffd29e77fe9c4a9466bcd5bacb))
* **deps:** bump rpassword from 7.1.0 to 7.2.0 ([cf0fc1b](https://github.com/andrey-yantsen/plex-api.rs/commit/cf0fc1b746737fb6be3eabd31af29cf1b77d9c04))
* **deps:** bump rstest from 0.12.0 to 0.13.0 ([f01f31a](https://github.com/andrey-yantsen/plex-api.rs/commit/f01f31afbf4557eeb777a7ec413f9268d2ac6787))
* **deps:** bump rstest from 0.13.0 to 0.14.0 ([a117993](https://github.com/andrey-yantsen/plex-api.rs/commit/a11799366a01688e0c7f7a1be92b41942665ac80))
* **deps:** bump rstest from 0.14.0 to 0.15.0 ([2381b64](https://github.com/andrey-yantsen/plex-api.rs/commit/2381b642cb8bfcd9f57a5861605b14bd35e8d7f1))
* **deps:** bump rstest from 0.15.0 to 0.16.0 ([a044057](https://github.com/andrey-yantsen/plex-api.rs/commit/a0440574b0f6af003c4b585c6bbc47d955fbedb8))
* **deps:** bump serde_plain from 1.0.0 to 1.0.1 ([a365424](https://github.com/andrey-yantsen/plex-api.rs/commit/a3654240e8929d6e70dc1b9660d7bf77eb58b606))
* **deps:** bump serde_with from 1.11.0 to 1.12.0 ([f25394a](https://github.com/andrey-yantsen/plex-api.rs/commit/f25394a0f995cada7d1599ccde033d5c344dcfc6))
* **deps:** bump serde_with from 1.12.1 to 1.13.0 ([ea86bfc](https://github.com/andrey-yantsen/plex-api.rs/commit/ea86bfcfb677fdcb27fb06aa07a65c39373afbc4))
* **deps:** bump serde_with from 1.13.0 to 1.14.0 ([54b514f](https://github.com/andrey-yantsen/plex-api.rs/commit/54b514fc8e35ee1359178e44b5a8d2d90c0f8ee1))
* **deps:** bump serde_with from 1.14.0 to 2.1.0 ([965149a](https://github.com/andrey-yantsen/plex-api.rs/commit/965149a485b8bb7c385d7d66424316465971b573))
* **deps:** bump serde-aux from 4.0.0 to 4.1.0 ([c77a9da](https://github.com/andrey-yantsen/plex-api.rs/commit/c77a9da275a3010ad2507ffc8ac8908778abc00f))
* **deps:** bump serde-aux from 4.1.0 to 4.1.2 ([a6fe462](https://github.com/andrey-yantsen/plex-api.rs/commit/a6fe462cba17043b39c1111e1d8c00f86260af78))
* **deps:** bump uuid from 0.8.2 to 1.0.0 ([2bece57](https://github.com/andrey-yantsen/plex-api.rs/commit/2bece57759df502563bd57391869b79739d64a5c))
* **deps:** bump uuid from 1.0.0 to 1.1.1 ([612bff4](https://github.com/andrey-yantsen/plex-api.rs/commit/612bff4ccf81c4f95076515f4b609b133b675082))
* **deps:** bump uuid from 1.1.2 to 1.2.1 ([c79804f](https://github.com/andrey-yantsen/plex-api.rs/commit/c79804f22e2ec3f292ad5f56ad40905fa22a9345))
* **examples:** fix rpassword usage after upgrade ([b5724a0](https://github.com/andrey-yantsen/plex-api.rs/commit/b5724a02ff01573d9f3663b5e010132a0f984c38))
* **features:** use better check for is_unknown_value ([4177ae1](https://github.com/andrey-yantsen/plex-api.rs/commit/4177ae1b79d0d73f9f46475eea80401e9adeb34e))
* get rid of unused lifetime in build.rs ([e925883](https://github.com/andrey-yantsen/plex-api.rs/commit/e925883d6f97bc11f99a45fdf423df44184227da))
* **main:** release plex-api 0.0.4 ([59ece83](https://github.com/andrey-yantsen/plex-api.rs/commit/59ece83ef6322bad57318afeadd3cda16374e8c0))
* make clippy happier with expect_err() ([1df2b52](https://github.com/andrey-yantsen/plex-api.rs/commit/1df2b52ec63be5ae3867782a20c02be601ee707a))
* make it compile with quick_xml 0.27 ([0f621a6](https://github.com/andrey-yantsen/plex-api.rs/commit/0f621a6baee29b157500ec64b2043f06fb02a36e))
* remove needless borrow (thanks, clippy!) ([abcfdf0](https://github.com/andrey-yantsen/plex-api.rs/commit/abcfdf06cff2eef4669fda9fa1e69697bb69af03))
* update Feature enum ([b28ac67](https://github.com/andrey-yantsen/plex-api.rs/commit/b28ac675fae3baca3161f58a46173a50ad2118d5))
* update Feature enum ([7d318fd](https://github.com/andrey-yantsen/plex-api.rs/commit/7d318fdded24b92249d06070730b8b043f0e31df))
* update Feature enum ([0e62035](https://github.com/andrey-yantsen/plex-api.rs/commit/0e62035d116f268b5ed514a2df62ba754f9ce2d8))
* update Feature enum ([894608d](https://github.com/andrey-yantsen/plex-api.rs/commit/894608dec7edd77c78d116f34f4fdf01da957f70))
* update Feature enum ([fcfa47d](https://github.com/andrey-yantsen/plex-api.rs/commit/fcfa47d203b4b402cd7ebbc797456d36b7227dc3))
* update Feature enum ([175b3b0](https://github.com/andrey-yantsen/plex-api.rs/commit/175b3b0c3171b54d22b79fc160f03977056e348b))
* update Feature enum ([4b6daa9](https://github.com/andrey-yantsen/plex-api.rs/commit/4b6daa9f8288a784f051b060eb5bbfbf77ea4f38))
* update Feature enum ([a855adb](https://github.com/andrey-yantsen/plex-api.rs/commit/a855adbb09f1073276030d157946251d48215158))
* update Feature enum ([43b6f2b](https://github.com/andrey-yantsen/plex-api.rs/commit/43b6f2b0f67fd53712240ddead1a1e64a6c99805))
* update Feature enum ([9595cca](https://github.com/andrey-yantsen/plex-api.rs/commit/9595cca051b3b708592c57f75d4c46f480851894))
* update Feature enum ([e984f67](https://github.com/andrey-yantsen/plex-api.rs/commit/e984f67b1fc6f71bda4330d1d42618813021d826))
* update Feature enum ([20f2d0e](https://github.com/andrey-yantsen/plex-api.rs/commit/20f2d0e8e7ba9b474a7663ad5ca56b8f0ebbcbdc))

### [0.0.4](https://github.com/andrey-yantsen/plex-api.rs/compare/plex-api-v0.0.3...plex-api-v0.0.4) (2022-02-11)


### Features

* **srv:** add preferences management ([7454bc5](https://github.com/andrey-yantsen/plex-api.rs/commit/7454bc57d84a577ec324c48f2117296e86d88cbc))


### Bug Fixes

* **srv:** older PMS can send numbers as strings ([96bb0bf](https://github.com/andrey-yantsen/plex-api.rs/commit/96bb0bfc6e8d7259bf542f4224e1f79f0b1c5ba3))


### Miscellaneous

* **deps:** bump serde_with from 1.11.0 to 1.12.0 ([f25394a](https://github.com/andrey-yantsen/plex-api.rs/commit/f25394a0f995cada7d1599ccde033d5c344dcfc6))
