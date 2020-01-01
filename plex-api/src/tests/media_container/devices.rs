use crate::media_container::MediaContainer;
use async_std::task::block_on;
use serde_xml_rs::from_str;

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

#[cfg(feature = "test_connect_authenticated")]
#[test]
fn decode_devices_online() {
    use crate::MyPlexAccount;
    use std::env;
    let acc: Result<MyPlexAccount, _> = {
        let auth_token = env::var("PLEX_API_AUTH_TOKEN").expect("Auth token not specified");
        block_on(MyPlexAccount::by_token(&auth_token))
    };
    assert!(acc.is_ok(), "Unable to authenticate");
    let devices = block_on(acc.unwrap().get_devices());
    assert!(
        devices.is_ok(),
        "Unable to get devices: {:?}",
        devices.err()
    );
}
