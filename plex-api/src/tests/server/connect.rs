#[cfg(feature = "test_connect_authenticated")]
#[tokio::test]
async fn decode_server_online_authenticated() {
    use crate::tests::get_server_authenticated;
    get_server_authenticated().await;
}

#[cfg(feature = "test_connect_anonymous")]
#[tokio::test]
async fn decode_server_online_anonymous() {
    use crate::tests::get_server_anonymous;
    get_server_anonymous().await;
}
