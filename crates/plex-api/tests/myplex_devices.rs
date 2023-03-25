mod fixtures;

mod offline {
    use super::fixtures::offline::{myplex::*, Mocked};
    use httpmock::Method::GET;
    use plex_api::{
        device::DeviceConnection,
        url::{MYPLEX_DEVICES, MYPLEX_RESOURCES, SERVER_MEDIA_PROVIDERS},
        MyPlex,
    };

    #[plex_api_test_helper::offline_test]
    async fn load_devices(#[future] myplex: Mocked<MyPlex>) {
        let (myplex, mock_server) = myplex.split();

        let devices_mock = mock_server.mock(|when, then| {
            when.method(GET).path(MYPLEX_DEVICES);
            then.status(200)
                .header("content-type", "application/xml")
                .body_from_file("tests/mocks/myplex/devices.xml");
        });

        let device_manager = myplex.device_manager().unwrap();

        let devices = device_manager.devices().await;
        devices_mock.assert();
        devices
            .unwrap()
            .into_iter()
            .find(|d| d.name() == "Box")
            .unwrap();
    }

    #[plex_api_test_helper::offline_test]
    async fn load_resources(#[future] myplex: Mocked<MyPlex>) {
        let (myplex, mock_server) = myplex.split();

        let resources_mock = mock_server.mock(|when, then| {
            when.method(GET).path(MYPLEX_RESOURCES);
            then.status(200)
                .header("content-type", "application/xml")
                .body_from_file("tests/mocks/myplex/api/resources.xml");
        });

        let device_manager = myplex.device_manager().unwrap();

        let resources = device_manager.resources().await;
        resources_mock.assert();
        resources.unwrap();
    }

    #[plex_api_test_helper::offline_test]
    async fn connection_from_device(#[future] myplex: Mocked<MyPlex>) {
        let (myplex, mock_server) = myplex.split();
        let body = include_str!("mocks/myplex/api/resources.xml")
            .replace("http://1.0.0.2:443", &mock_server.base_url());

        let mut resources_mock = mock_server.mock(|when, then| {
            when.method(GET).path(MYPLEX_RESOURCES);
            then.status(200)
                .header("content-type", "application/xml")
                .body(body);
        });

        let mut providers_mock = mock_server.mock(|when, then| {
            when.method(GET).path(SERVER_MEDIA_PROVIDERS);
            then.status(200)
                .header("content-type", "application/json")
                .body_from_file("tests/mocks/server/media/providers_free.json");
        });

        let device_manager = myplex.device_manager().unwrap();

        let resources = device_manager.resources().await.unwrap();
        resources_mock.assert();
        resources_mock.delete();

        if let Some(device) = resources.first() {
            use DeviceConnection::*;
            if let Server(server) = device.connect().await.unwrap() {
                assert!(server.media_container.my_plex);
            } else {
                panic!("Connected to a strange device");
            }
            providers_mock.assert();
            providers_mock.delete();
        } else {
            panic!("No devices found");
        }
    }
}

mod online {
    use super::fixtures::online::myplex;
    use plex_api::MyPlex;

    #[plex_api_test_helper::online_test_myplex]
    async fn load_devices(#[future] myplex: MyPlex) {
        // Test deserialization
        _ = myplex.device_manager().unwrap().devices().await.unwrap();
    }

    #[plex_api_test_helper::online_test_myplex]
    async fn load_resources(#[future] myplex: MyPlex) {
        // Test deserialization
        _ = myplex.device_manager().unwrap().resources().await.unwrap();
    }
}
