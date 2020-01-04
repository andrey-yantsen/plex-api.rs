use crate::media_container::MediaContainer;
use serde_xml_rs::from_str;

#[test]
fn decode_server() {
    let s = r##"
<?xml version="1.0" encoding="UTF-8"?>
<MediaContainer size="22" allowCameraUpload="1" allowChannelAccess="1" allowMediaDeletion="1" allowSharing="1" allowSync="1" allowTuners="1" backgroundProcessing="1" certificate="1" companionProxy="1" countryCode="rus" diagnostics="logs,databases,streaminglogs" eventStream="1" friendlyName="LED-Kremen-286" hubSearch="1" itemClusters="1" livetv="7" machineIdentifier="376D9976F5166" mediaProviders="1" multiuser="1" myPlex="1" myPlexMappingState="mapped" myPlexSigninState="ok" myPlexSubscription="1" myPlexUsername="username@example.com" ownerFeatures="002c9f1a-2fc0-4812-b85b-0e6140f21a0f,05690239-443e-43fb-bc1a-95b5d916ca63,0a348865-4f87-46dc-8bb2-f37637975724,0eee866d-782b-4dfd-b42b-3bbe8eb0af16,1417df52-986e-4e4b-8dcd-3997fbc5c976,16d69c53-4c40-4821-b9f3-57ca690b2d4d,1844737f-1a87-45c3-ab20-01435959e63c,1dd846ed-7cde-4dc5-8ef6-53d3ce8c4e9d,222020fb-1504-492d-af33-a0b80a49558a,228a6439-ee2f-4a9b-b0fc-1bfcd48b5095,22d52c96-9e2b-45c0-9e2a-1d6c66ad3474,2ea0e464-ea4f-4be2-97c1-ce6ed4b377dd,300231e0-69aa-4dce-97f4-52d8c00e3e8c,32cc8bf5-b425-4582-a52d-71b4f1cf436b,3a2b0cb6-1519-4431-98e2-823c248c70eb,4b522f91-ae89-4f62-af9c-76f44d8ef61c,4ca03b04-54c1-4f9f-aea2-f813ae48f317,55b9f6ed-5d26-4d2d-a436-68882a9901b5,5b6190a9-77a4-477e-9fbc-c8118e35a4c1,5d819d02-5d04-4116-8eec-f49def4e2d6f,5e2a89ec-fb26-4234-b66e-14d37f35dff2,6380e085-02fe-43b5-8bff-380fa4f2423c,65152b75-13a9-408a-bd30-dbd23a259183,65685ff8-4375-4e4c-a806-ec1f0b4a8b7f,6f82ca43-6117-4e55-ae0e-5ea3b3e99a96,7377e4a0-d893-4403-a592-0f84c8f07043,78643fe5-d192-40c7-8e93-5ccf04c0b767,82999dd3-a2be-482e-9f44-357879b4f603,84a754b0-d1ca-4433-af2d-c949bf4b4936,850f3d1e-3f38-44c1-9c0c-e3c9127b8b5a,85ebfb7b-77fb-4afd-bb1a-2fe2fefdddbe,86da2200-58db-4d78-ba46-f146ba25906b,95149521-f64b-46ea-825c-9114e56afd2c,96cac76e-c5bc-4596-87eb-4fdfef9aaa11,9dc1df45-fb45-4be1-9ab2-eb23eb57f082,a19d495a-1cef-4f7c-ab77-5186e63e17f7,a6e0a154-4735-4cbb-a6ec-7a0a146c8216,abd37b14-706c-461f-8255-fa9563882af3,adaptive_bitrate,b2403ac6-4885-4971-8b96-59353fd87c72,b46d16ae-cbd6-4226-8ee9-ab2b27e5dd42,b58d7f28-7b4a-49bb-97a7-152645505f28,b612f571-83c3-431a-88eb-3f05ce08da4a,bb50c92f-b412-44fe-8d8a-b1684f212a44,bbf73498-4912-4d80-9560-47c4fe212cec,bc8d1fca-deb0-4d0a-a6f4-12cfd681002d,c2409baa-d044-45c7-b1f4-e9e7ccd2d128,c55d5900-b546-416d-a8c5-45b24a13e9bc,c5adf9dc-af13-4a85-a24b-98de6fa2f595,c7ae6f8f-05e6-48bb-9024-c05c1dc3c43e,camera_upload,cloudsync,collections,content_filter,d14556be-ae6d-4407-89d0-b83953f4789a,d20f9af2-fdb1-4927-99eb-a2eb8fbff799,d413fb56-de7b-40e4-acd0-f3dbb7c9e104,download_certificates,dvr,e8230c74-0940-4b91-9e20-6571eb068086,ee352392-2934-4061-ba35-5f3189f19ab4,fb34e64d-cd89-47b8-8bae-a6d20c542bae,federated-auth,hardware_transcoding,home,hwtranscode,item_clusters,kevin-bacon,livetv,loudness,lyrics,music_videos,pass,photo_autotags,photos-v5,photosV6-edit,photosV6-tv-albums,premium_music_metadata,radio,server-manager,session_bandwidth_restrictions,session_kick,shared-radio,sync,trailers,tuner-sharing,type-first,unsupportedtuners,webhooks" photoAutoTag="1" platform="MacOSX" platformVersion="10.13.4" pluginHost="1" readOnlyLibraries="0" requestParametersInCookie="1" streamingBrainABRVersion="3" streamingBrainVersion="2" sync="1" transcoderActiveVideoSessions="0" transcoderAudio="1" transcoderLyrics="1" transcoderPhoto="1" transcoderSubtitles="1" transcoderVideo="1" transcoderVideoBitrates="64,96,208,320,720,1500,2000,3000,4000,8000,10000,12000,20000" transcoderVideoQualities="0,1,2,3,4,5,6,7,8,9,10,11,12" transcoderVideoResolutions="128,128,160,240,320,480,768,720,720,1080,1080,1080,1080" updatedAt="1547852125" updater="1" version="1.14.1.5488-cc260c476" voiceSearch="1">
    <Directory count="1" key="activities" title="activities" />
    <Directory count="1" key="butler" title="butler" />
    <Directory count="1" key="channels" title="channels" />
    <Directory count="1" key="clients" title="clients" />
    <Directory count="1" key="diagnostics" title="diagnostics" />
    <Directory count="1" key="hubs" title="hubs" />
    <Directory count="1" key="library" title="library" />
    <Directory count="3" key="livetv" title="livetv" />
    <Directory count="3" key="media" title="media" />
    <Directory count="1" key="neighborhood" title="neighborhood" />
    <Directory count="1" key="playQueues" title="playQueues" />
    <Directory count="1" key="player" title="player" />
    <Directory count="1" key="playlists" title="playlists" />
    <Directory count="1" key="resources" title="resources" />
    <Directory count="1" key="search" title="search" />
    <Directory count="1" key="server" title="server" />
    <Directory count="1" key="servers" title="servers" />
    <Directory count="1" key="statistics" title="statistics" />
    <Directory count="1" key="system" title="system" />
    <Directory count="1" key="transcode" title="transcode" />
    <Directory count="1" key="updater" title="updater" />
    <Directory count="4" key="video" title="video" />
</MediaContainer>
    "##;

    let mc = from_str::<MediaContainer>(s);
    assert!(mc.is_ok(), "Unable to deserialize users: {:?}", mc.err());
}

#[cfg(any(
    feature = "test_connect_authenticated",
    feature = "test_connect_anonymous"
))]
#[tokio::test]
async fn decode_server_online() {
    use crate::Server;
    use std::env;
    let srv: Result<Server, _> = {
        let server_url = env::var("PLEX_API_SERVER_URL").expect("Server url not specified");
        if cfg!(feature = "test_connect_authenticated") {
            let auth_token = env::var("PLEX_API_AUTH_TOKEN").expect("Auth token not specified");
            Server::login(&server_url, &auth_token).await
        } else {
            Server::connect(&server_url).await
        }
    };
    assert!(srv.is_ok(), "Unable to connect to server: {:?}", srv.err());
}
