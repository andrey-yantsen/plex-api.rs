use mockito::mock;
use plex_api::url::{MYPLEX_DEVICES, MYPLEX_RESOURCES};

#[plex_api_test_helper::async_offline_test]
async fn devices(myplex: MyPlex) {
    let devices_mock = mock("GET", MYPLEX_DEVICES)
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body_from_file("tests/files/myplex/devices.xml")
        .create();
    let resources_mock = mock("GET", MYPLEX_RESOURCES)
        .with_status(200)
        .with_header("content-type", "application/xml")
        .with_body_from_file("tests/files/myplex/api/resources.xml")
        .create();
    let device_manager = myplex.device_manager();

    let _devices = dbg!(device_manager.get_devices().await);
    devices_mock.assert();

    let _resources = dbg!(device_manager.get_resources().await);
    resources_mock.assert();
}
