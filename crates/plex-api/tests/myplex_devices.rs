mod fixtures;

use fixtures::offline::myplex::*;
use fixtures::offline::Mocked;
use httpmock::Method::GET;
use plex_api::{
    url::{MYPLEX_DEVICES, MYPLEX_RESOURCES},
    MyPlex,
};

#[plex_api_test_helper::async_offline_test]
async fn devices(#[future] myplex: Mocked<MyPlex>) {
    let myplex = myplex.await;
    let (myplex, mock_server) = myplex.split();

    let devices_mock = mock_server.mock(|when, then| {
        when.method(GET).path(MYPLEX_DEVICES);
        then.status(200)
            .header("content-type", "application/xml")
            .body_from_file("tests/files/myplex/devices.xml");
    });

    let resources_mock = mock_server.mock(|when, then| {
        when.method(GET).path(MYPLEX_RESOURCES);
        then.status(200)
            .header("content-type", "application/xml")
            .body_from_file("tests/files/myplex/api/resources.xml");
    });

    let device_manager = myplex.device_manager();

    let _devices = dbg!(device_manager.get_devices().await);
    devices_mock.assert();

    let _resources = dbg!(device_manager.get_resources().await);
    resources_mock.assert();
}
