use crate::{
    base_headers, X_PLEX_CLIENT_IDENTIFIER, X_PLEX_DEVICE, X_PLEX_DEVICE_NAME, X_PLEX_PLATFORM,
    X_PLEX_PLATFORM_VERSION, X_PLEX_PRODUCT, X_PLEX_PROVIDES, X_PLEX_SYNC_VERSION, X_PLEX_VERSION,
};

#[test]
fn base_headers_contains_required_headers() {
    let headers = base_headers();
    assert_eq!(true, headers.contains_key("x-plex-provides"));
    assert_eq!(true, headers.contains_key("x-plex-product"));
    assert_eq!(true, headers.contains_key("x-plex-version"));
    assert_eq!(true, headers.contains_key("x-plex-sync-version"));
    assert_eq!(true, headers.contains_key("x-plex-platform"));
    assert_eq!(true, headers.contains_key("x-plex-platform-version"));
    assert_eq!(true, headers.contains_key("x-plex-client-identifier"));
    assert_eq!(true, headers.contains_key("x-plex-device"));
    assert_eq!(true, headers.contains_key("x-plex-device-name"));
}

#[test]
fn base_headers_use_provided_values() {
    {
        let mut provides = X_PLEX_PROVIDES.write().unwrap();
        *provides = "plex_provides";
        let mut platform = X_PLEX_PLATFORM.write().unwrap();
        *platform = "plex_platform";
        let mut platform_version = X_PLEX_PLATFORM_VERSION.write().unwrap();
        *platform_version = "plex_platform_version";
        let mut product = X_PLEX_PRODUCT.write().unwrap();
        *product = "plex_product";
        let mut version = X_PLEX_VERSION.write().unwrap();
        *version = "plex_version";
        let mut device = X_PLEX_DEVICE.write().unwrap();
        *device = "plex_device";
        let mut device_name = X_PLEX_DEVICE_NAME.write().unwrap();
        *device_name = "plex_device_name";
        let mut client_identifier = X_PLEX_CLIENT_IDENTIFIER.write().unwrap();
        *client_identifier = "plex_client_identifier";
    }

    let headers = base_headers();

    let provides = X_PLEX_PROVIDES.read().unwrap();
    let platform = X_PLEX_PLATFORM.read().unwrap();
    let platform_version = X_PLEX_PLATFORM_VERSION.read().unwrap();
    let product = X_PLEX_PRODUCT.read().unwrap();
    let version = X_PLEX_VERSION.read().unwrap();
    let device = X_PLEX_DEVICE.read().unwrap();
    let device_name = X_PLEX_DEVICE_NAME.read().unwrap();
    let client_identifier = X_PLEX_CLIENT_IDENTIFIER.read().unwrap();

    assert_eq!(*provides, headers.get("x-plex-provides").unwrap());
    assert_eq!(*product, headers.get("x-plex-product").unwrap());
    assert_eq!(*version, headers.get("x-plex-version").unwrap());
    assert_eq!(
        X_PLEX_SYNC_VERSION,
        headers.get("x-plex-sync-version").unwrap()
    );
    assert_eq!(*platform, headers.get("x-plex-platform").unwrap());
    assert_eq!(
        *platform_version,
        headers.get("x-plex-platform-version").unwrap()
    );
    assert_eq!(
        *client_identifier,
        headers.get("x-plex-client-identifier").unwrap()
    );
    assert_eq!(*device, headers.get("x-plex-device").unwrap());
    assert_eq!(*device_name, headers.get("x-plex-device-name").unwrap());
}
