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

#[cfg(feature = "test_connect_authenticated")]
#[test]
fn decode_resources_online() {
    use crate::MyPlexAccount;
    use std::env;
    let acc: Result<MyPlexAccount, _> = {
        let auth_token = env::var("PLEX_API_AUTH_TOKEN").expect("Auth token not specified");
        MyPlexAccount::by_token(&auth_token)
    };
    assert!(acc.is_ok(), "Unable to authenticate");
    let resources = acc.unwrap().get_resources();
    assert!(
        resources.is_ok(),
        "Unable to get resources: {:?}",
        resources.err()
    );
}
