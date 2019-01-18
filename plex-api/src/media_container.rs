use crate::{bool_from_int, option_bool_from_int};
use serde_with::CommaSeparator;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct MediaContainer {
    size: Option<u32>,
    #[serde(rename = "totalSize")]
    total_size: Option<u32>,
    #[serde(rename = "publicAddress")]
    public_address: Option<String>,
    #[serde(rename = "friendlyName")]
    friendly_name: Option<String>,
    identifier: Option<String>,
    #[serde(rename = "machineIdentifier")]
    machine_identifier: Option<String>,
    #[serde(rename = "Device")]
    devices: Option<Vec<Device>>,
    #[serde(rename = "User")]
    users: Option<Vec<User>>,
}

impl MediaContainer {
    pub fn get_devices(&self) -> Option<Vec<Device>> {
        self.devices.clone()
    }
    pub fn get_users(&self) -> Option<Vec<User>> {
        self.users.clone()
    }
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct User {
    id: u32,
    title: String,
    thumb: String,
    #[serde(deserialize_with = "bool_from_int")]
    protected: bool,
    #[serde(deserialize_with = "bool_from_int")]
    home: bool,
    #[serde(rename = "allowSync", deserialize_with = "bool_from_int")]
    allow_sync: bool,
    #[serde(rename = "allowCameraUpload", deserialize_with = "bool_from_int")]
    allow_camera_upload: bool,
    #[serde(rename = "allowChannels", deserialize_with = "bool_from_int")]
    allow_channels: bool,
    #[serde(rename = "allowTuners", deserialize_with = "bool_from_int")]
    allow_tuners: bool,
    #[serde(rename = "allowSubtitleAdmin", deserialize_with = "bool_from_int")]
    allow_subtitle_admin: bool,
    #[serde(deserialize_with = "bool_from_int")]
    restricted: bool,
    #[serde(rename = "filterAll")]
    filter_all: String,
    #[serde(rename = "filterMovies")]
    filter_movies: String,
    #[serde(rename = "filterMusic")]
    filter_music: String,
    #[serde(rename = "filterPhotos")]
    filter_photos: String,
    #[serde(rename = "filterTelevision")]
    filter_television: String,
    #[serde(rename = "Server")]
    servers: Option<Vec<Server>>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct Server {
    id: u32,
    #[serde(rename = "serverId")]
    server_id: u32,
    #[serde(rename = "machineIdentifier")]
    machine_identifier: String,
    name: String,
    #[serde(rename = "lastSeenAt")]
    last_seen_at: u64,
    #[serde(rename = "numLibraries")]
    num_libraries: u32,
    #[serde(rename = "allLibraries", deserialize_with = "bool_from_int")]
    all_libraries: bool,
    #[serde(deserialize_with = "bool_from_int")]
    owned: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pending: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Device {
    name: String,
    #[serde(rename = "publicAddress")]
    public_address: String,
    product: String,
    #[serde(rename = "productVersion")]
    product_version: String,
    platform: String,
    #[serde(rename = "platformVersion")]
    platform_version: String,
    device: String,
    model: Option<String>,
    vendor: Option<String>,
    #[serde(
        deserialize_with = "serde_with::rust::StringWithSeparator::<CommaSeparator>::deserialize"
    )]
    provides: Vec<String>,
    #[serde(rename = "clientIdentifier")]
    client_identifier: String,
    version: Option<String>,
    id: Option<u32>,
    token: Option<String>,
    #[serde(rename = "accessToken")]
    access_token: Option<String>,
    #[serde(rename = "createdAt")]
    created_at: u64,
    #[serde(rename = "lastSeenAt")]
    last_seen_at: u64,
    #[serde(
        rename = "screenResolution",
        deserialize_with = "serde_with::rust::StringWithSeparator::<CommaSeparator>::deserialize",
        default
    )]
    screen_resolution: Vec<String>,
    #[serde(
        rename = "screenDensity",
        deserialize_with = "serde_with::rust::string_empty_as_none::deserialize",
        default
    )]
    screen_density: Option<u8>,
    #[serde(rename = "Connection")]
    connections: Option<Vec<Connection>>,
    #[serde(
        rename = "httpsRequired",
        deserialize_with = "option_bool_from_int",
        default
    )]
    https_required: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    synced: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    relay: Option<bool>,
    #[serde(
        rename = "publicAddressMatches",
        deserialize_with = "option_bool_from_int",
        default
    )]
    public_address_matches: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    presence: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    owned: Option<bool>,
    #[serde(rename = "SyncList")]
    sync_list: Option<SyncList>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct SyncList {
    #[serde(rename = "itemsCompleteCount")]
    items_complete_count: u32,
    #[serde(rename = "totalSize")]
    total_size: u64,
    version: u32,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, serde(deny_unknown_fields))]
struct Connection {
    uri: String,
    protocol: Option<String>,
    address: Option<String>,
    port: Option<u32>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    local: Option<bool>,
    #[serde(deserialize_with = "option_bool_from_int", default)]
    relay: Option<bool>,
}

#[cfg(test)]
mod test {
    use crate::media_container::MediaContainer;
    use serde_xml_rs::from_str;

    #[test]
    fn decode_resources() {
        let s = r##"
<?xml version="1.0" encoding="UTF-8"?>
<MediaContainer size="5">
  <Device name="LED-Kremen-286" product="Plex Media Server" productVersion="1.14.1.5488-cc260c476" platform="MacOSX" platformVersion="10.13.4" device="Mac" clientIdentifier="B5D4D175B93B" createdAt="1506579774" lastSeenAt="1547547365" provides="server" owned="1" accessToken="31E6CD0F0961E" publicAddress="127.0.0.1" httpsRequired="0" synced="0" relay="1" publicAddressMatches="0" presence="1">
    <Connection protocol="https" address="127.0.0.1" port="32400" uri="https://127-0-0-1.s7028573f9e34fcb81ed344f1d628e09.plex.direct:32400" local="1"/>
    <Connection protocol="https" address="127.0.0.1" port="24507" uri="https://127-0-0-1.s7028573f9e34fcb81ed344f1d628e09.plex.direct:24507" local="0"/>
    <Connection protocol="https" address="127.0.0.1" port="8443" uri="https://127-0-0-1.s7028573f9e34fcb81ed344f1d628e09.plex.direct:8443" local="0" relay="1"/>
  </Device>
  <Device name="plex-test-docker-e1491b81-445d-47c5-af76-4e0dcaf518dc" product="Plex Media Server" productVersion="1.13.5.5332-21ab172de" platform="Linux" platformVersion="4.9.125-linuxkit (#1 SMP Fri Sep 7 08:20:28 UTC 2018)" device="Docker Container" clientIdentifier="B4394EF65B91" createdAt="1536877655" lastSeenAt="1547120715" provides="server" owned="1" accessToken="2212D44311BCC" publicAddress="127.0.0.1" httpsRequired="0" synced="0" relay="0" publicAddressMatches="1" presence="0">
    <Connection protocol="https" address="127.0.0.1" port="32400" uri="https://127-0-0-1.fa57cb88b6aa4d8bbccbba16d2a218dd.plex.direct:32400" local="1"/>
    <Connection protocol="https" address="127.0.0.1" port="32400" uri="https://127-0-0-1.fa57cb88b6aa4d8bbccbba16d2a218dd.plex.direct:32400" local="1"/>
    <Connection protocol="https" address="127.0.0.1" port="32400" uri="https://127-0-0-1.fa57cb88b6aa4d8bbccbba16d2a218dd.plex.direct:32400" local="1"/>
  </Device>
  <Device name="MyPassport" product="Plex Media Server" productVersion="1.13.8.5395-10d48da0d" platform="Linux" platformVersion="3.10.24-rtk-nas (#58 SMP PREEMPT Fri Dec 15 22:00:09 CST 2017)" device="PC" clientIdentifier="103ED31BD6E84" createdAt="1535158957" lastSeenAt="1547575416" provides="server" owned="1" accessToken="CBED928B2073" publicAddress="127.0.0.1" httpsRequired="0" synced="0" relay="0" publicAddressMatches="0" presence="1">
    <Connection protocol="https" address="127.0.0.1" port="32400" uri="https://127-0-0-1.a2adc0fb529047dbae963e1522f1f68d.plex.direct:32400" local="1"/>
    <Connection protocol="https" address="127.0.0.1" port="32400" uri="https://127-0-0-1.a2adc0fb529047dbae963e1522f1f68d.plex.direct:32400" local="1"/>
  </Device>
  <Device name="iPad Air 2" product="Plex for iOS" productVersion="5.9.1" platform="iOS" platformVersion="12.1.1" device="iPad" clientIdentifier="2C337C977B820" createdAt="1488380229" lastSeenAt="1547510058" provides="client,controller,sync-target,player,pubsub-player" owned="1" publicAddress="127.0.0.1" publicAddressMatches="0" presence="0">
    <Connection protocol="http" address="127.0.0.1" port="32500" uri="http://127.0.0.1:32500" local="1"/>
  </Device>
  <Device name="Chrome" product="Plex Web" productVersion="3.81.1" platform="Chrome" platformVersion="71.0" device="OSX" clientIdentifier="28D5EA6F2DBE0" createdAt="1537086905" lastSeenAt="1547582844" provides="client,player,pubsub-player" owned="1" publicAddress="127.0.0.1" publicAddressMatches="1" presence="1" accessToken="2056369022936">
    <Connection protocol="https" address="127.0.0.1" port="32400" uri="https://127-0-0-1.s7028573f9e34fcb81ed344f1d628e09.plex.direct:32400" local="1"/>
    <Connection protocol="https" address="127.0.0.1" port="24507" uri="https://127-0-0-1.s7028573f9e34fcb81ed344f1d628e09.plex.direct:24507" local="0"/>
  </Device>
</MediaContainer>
    "##;

        let mc = from_str::<MediaContainer>(s);
        assert!(
            mc.is_ok(),
            "Unable to deserialize resources: {:?}",
            mc.err()
        );
    }

    #[test]
    fn decode_devices() {
        let s = r##"
<?xml version="1.0" encoding="UTF-8"?>
<MediaContainer publicAddress="127.0.0.1">
  <Device name="Chrome" publicAddress="127.0.0.1" product="Plex Web" productVersion="3.81.1" platform="Chrome" platformVersion="71.0" device="OSX" model="" vendor="" provides="client,player,pubsub-player" clientIdentifier="2BB94F010FCCF" version="3.81.1" id="3251167" token="17612B91A63C1" createdAt="1537086905" lastSeenAt="1547582844" screenResolution="1680x948,1680x1050" screenDensity="">
  </Device>
  <Device name="MyPassport Wireless Pro" publicAddress="127.0.0.1" product="plexiglas" productVersion="3.1.0" platform="iOS" platformVersion="11.4.1" device="iPhone" model="8,4" vendor="" provides="sync-target" clientIdentifier="161518A8FBC45" version="3.1.0" id="7982597" token="127C0E9705888" createdAt="1534942362" lastSeenAt="1547575449" screenResolution="" screenDensity="">
    <SyncList itemsCompleteCount="98" totalSize="227001669489" version="2"/>
  </Device>
  <Device name="MyPassport" publicAddress="127.0.0.1" product="Plex Media Server" productVersion="1.13.8.5395-10d48da0d" platform="Linux" platformVersion="3.10.24-rtk-nas (#58 SMP PREEMPT Fri Dec 15 22:00:09 CST 2017)" device="PC" model="arm7" vendor="marvell" provides="server" clientIdentifier="3442AC287F844" version="1.13.8.5395-10d48da0d" id="4116407" token="12C804171CD29" createdAt="1535158957" lastSeenAt="1547575416" screenResolution="" screenDensity="">
    <Connection uri="http://127.0.0.1:32400"/>
    <Connection uri="http://127.0.0.1:32400"/>
  </Device>
  <Device name="LED-Kremen-286" publicAddress="127.0.0.1" product="Plex Media Server" productVersion="1.14.1.5488-cc260c476" platform="MacOSX" platformVersion="10.13.4" device="Mac" model="x86_64" vendor="Apple" provides="server" clientIdentifier="1715AB2469EDD" version="1.14.1.5488-cc260c476" id="8990955" token="185FE447027DD" createdAt="1506579774" lastSeenAt="1547547365" screenResolution="" screenDensity="">
    <Connection uri="http://127.0.0.1:32400"/>
    <Connection uri="http://127.0.0.1:24507"/>
  </Device>
  <Device name="iPad Air 2" publicAddress="127.0.0.1" product="Plex for iOS" productVersion="5.9.1" platform="iOS" platformVersion="12.1.1" device="iPad" model="5,3" vendor="Apple" provides="client,controller,sync-target,player,pubsub-player" clientIdentifier="1E974AA66F3C3" version="5.9.1" id="6649306" token="1983131D839FD" createdAt="1488380229" lastSeenAt="1547510058" screenResolution="2048x1536" screenDensity="2">
    <SyncList itemsCompleteCount="0" totalSize="0" version="2"/>
    <Connection uri="http://127.0.0.1:32500"/>
  </Device>
  <Device name="Chrome" publicAddress="127.0.0.1" product="Plex Web" productVersion="3.81.1" platform="Chrome" platformVersion="71.0" device="OSX" model="" vendor="" provides="" clientIdentifier="13CF2985643D5" version="3.81.1" id="8011935" token="CF186F756082" createdAt="1547505261" lastSeenAt="1547505265" screenResolution="1680x948,1680x1050" screenDensity="">
  </Device>
  <Device name="plex-test-docker-e1491b81-445d-47c5-af76-4e0dcaf518dc" publicAddress="127.0.0.1" product="Plex Media Server" productVersion="1.13.5.5332-21ab172de" platform="Linux" platformVersion="4.9.125-linuxkit (#1 SMP Fri Sep 7 08:20:28 UTC 2018)" device="Docker Container" model="x86_64" vendor="Docker" provides="server" clientIdentifier="359D75642EB2A" version="1.13.5.5332-21ab172de" id="4446393" token="316E227B4699E" createdAt="1536877655" lastSeenAt="1547120715" screenResolution="" screenDensity="">
    <Connection uri="http://127.0.0.1:32400"/>
    <Connection uri="http://127.0.0.1:32400"/>
    <Connection uri="http://127.0.0.1:32400"/>
  </Device>
  <Device name="iPhone XS" publicAddress="127.0.0.1" product="Plex for iOS" productVersion="5.9.1" platform="iOS" platformVersion="12.1.2" device="iPhone" model="11,2" vendor="Apple" provides="client,controller,sync-target,player,pubsub-player" clientIdentifier="231960B093FF" version="5.9.1" id="4654118" token="26A91852372E9" createdAt="1539357381" lastSeenAt="1546302767" screenResolution="1125x2436" screenDensity="3">
    <SyncList itemsCompleteCount="0" totalSize="0" version="2"/>
    <Connection uri="http://127.0.0.1:32500"/>
  </Device>
  <Device name="TV UE55KU6400" publicAddress="127.0.0.1" product="Plex for Samsung" productVersion="3.3.4" platform="Tizen" platformVersion="2.4.0" device="" model="UE55KU6400" vendor="" provides="" clientIdentifier="32BFDA8E3ADAC" version="3.3.4" id="595056" token="10F214F75B0B8" createdAt="1532176881" lastSeenAt="1543003832" screenResolution="1920x1080" screenDensity="">
  </Device>
  <Device name="Safari" publicAddress="127.0.0.1" product="Plex Web" productVersion="3.75.3" platform="Safari" platformVersion="12.0" device="OSX" model="" vendor="" provides="client,player,pubsub-player" clientIdentifier="63D16E7983E2" version="3.75.3" id="9247167" token="34BAECA10428D" createdAt="1541764540" lastSeenAt="1542113657" screenResolution="1680x964,1680x1050" screenDensity="">
  </Device>
  <Device name="Safari" publicAddress="127.0.0.1" product="Plex Web" productVersion="3.73.2" platform="Safari" platformVersion="605.1" device="iOS" model="" vendor="" provides="" clientIdentifier="1F046D83ABA7E" version="3.73.2" id="2260911" token="29E12E368D05B" createdAt="1541489451" lastSeenAt="1541489462" screenResolution="980x1659,375x812" screenDensity="">
  </Device>
  <Device name="Safari" publicAddress="127.0.0.1" product="Plex Web" productVersion="3.73.1" platform="Safari" platformVersion="605.1" device="iOS" model="" vendor="" provides="" clientIdentifier="127EDC2C3558D" version="3.73.1" id="3738618" token="37CF23812F7F4" createdAt="1539644562" lastSeenAt="1539931437" screenResolution="980x1659,375x812" screenDensity="">
  </Device>
  <Device name="TV UE55KU6400" publicAddress="127.0.0.1" product="Plex for Samsung" productVersion="3.3.4" platform="Tizen" platformVersion="2.4.0" device="" model="UE55KU6400" vendor="" provides="" clientIdentifier="1E5A3068674E7" version="3.3.4" id="7565483" token="DF0DF898A8BE" createdAt="1538946198" lastSeenAt="1538946206" screenResolution="1920x1080" screenDensity="">
  </Device>
  <Device name="Plex for Sonos" publicAddress="127.0.0.1" product="Plex for Sonos" productVersion="" platform="Sonos" platformVersion="" device="Sonos" model="" vendor="" provides="" clientIdentifier="F45E1294FCD4" version="" id="8924643" token="271C7FAB6F314" createdAt="1536353804" lastSeenAt="1538427740" screenResolution="" screenDensity="">
  </Device>
  <Device name="Гостиная" publicAddress="127.0.0.1" product="Plex for Apple TV" productVersion="1.29" platform="tvOS" platformVersion="11.4.1" device="Apple TV" model="5,3" vendor="Apple" provides="client,player,pubsub-player" clientIdentifier="19842A5253764" version="1.29" id="1254516" token="BCBDC265D714" createdAt="1482660529" lastSeenAt="1533596198" screenResolution="1920x1080" screenDensity="1">
    <Connection uri="http://127.0.0.1:32500"/>
  </Device>
  <Device name="MacBook-Pro.local" publicAddress="" product="plex-api" productVersion="0.0.0" platform="Darwin" platformVersion="18.2.0" device="Darwin" model="" vendor="" provides="controller" clientIdentifier="F20E872056FC" version="0.0.0" id="7434502" token="BAD257166CE" createdAt="1547576929" lastSeenAt="1547576929" screenResolution="" screenDensity="">
  </Device>
</MediaContainer>
    "##;

        let mc = from_str::<MediaContainer>(s);
        assert!(mc.is_ok(), "Unable to deserialize devices: {:?}", mc.err());
    }

    #[test]
    fn decode_users() {
        let s = r##"
<?xml version="1.0" encoding="UTF-8"?>
<MediaContainer friendlyName="myPlex" identifier="com.plexapp.plugins.myplex" machineIdentifier="19A4B805E9021" totalSize="2" size="2">
    <User id="8676063" title="18A0A7BD14ECC" thumb="https://plex.tv/users/0d9d14488439289a/avatar?c=1536695183" protected="0" home="1" allowSync="0" allowCameraUpload="0" allowChannels="0" allowTuners="0" allowSubtitleAdmin="0" filterAll="" filterMovies="contentRating=G" filterMusic="" filterPhotos="" filterTelevision="contentRating=TV-14" restricted="1">
        <Server id="5558593" serverId="8035121" machineIdentifier="271663AA23AC6" name="LED-Kremen-286" lastSeenAt="1547660334" numLibraries="2" allLibraries="1" owned="1" pending="0"/>
    </User>
    <User id="7393808" title="2CD87DC316A42" thumb="https://plex.tv/users/80dab310d314a591/avatar?c=1536616777" protected="0" home="1" allowSync="0" allowCameraUpload="0" allowChannels="0" allowTuners="0" allowSubtitleAdmin="0" filterAll="" filterMovies="" filterMusic="" filterPhotos="" filterTelevision="" restricted="1">
        <Server id="717069" serverId="7142016" machineIdentifier="2A61C4586017F" name="plex-test-docker-e1491b81-445d-47c5-af76-4e0dcaf518dc" lastSeenAt="1547068479" numLibraries="4" allLibraries="1" owned="1" pending="0"/>
    </User>
    <User id="3609362" title="15FDD85D4BED2" thumb="https://plex.tv/users/80bab310d314a595/avatar?c=1536616777" protected="0" home="1" allowSync="0" allowCameraUpload="0" allowChannels="0" allowTuners="0" allowSubtitleAdmin="0" filterAll="" filterMovies="" filterMusic="" filterPhotos="" filterTelevision="" restricted="1">
    </User>
</MediaContainer>
    "##;

        let mc = from_str::<MediaContainer>(s);
        assert!(mc.is_ok(), "Unable to deserialize users: {:?}", mc.err());
    }
}
