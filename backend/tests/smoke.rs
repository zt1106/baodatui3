use backend::global::handlers::room_handlers::{
    ALL_ROOM_SIMPLE_INFO_STREAM_TYPE, CREATE_ROOM_REQ_TYPE,
};
use backend::global::handlers::user_handlers::{
    CHANGE_CUR_USER_NAME_REQ_TYPE, GET_CUR_USER_REQ_TYPE,
};
use backend::test_client::{Client, Server};
use backend::transport::request::RequestType;
use futures_util::StreamExt;
use std::time::Duration;
use tokio::spawn;
use tokio::time::{sleep, timeout};

#[tokio::test]
async fn server_shutdown_smoke() {
    let mut server = Server::new();
    let shutdown_send = server.shutdown_send();
    spawn(async move {
        sleep(Duration::from_millis(1000)).await;
        let _ = shutdown_send.send(());
    });
    let join_handle = server.join_handle();
    join_handle.await.unwrap();
}

#[tokio::test]
async fn client_connect_smoke() {
    let client = Client::new_and_connect().await;
    client.shutdown_and_wait_server_exit().await;
}

#[tokio::test]
async fn create_new_user_smoke() {
    let client = Client::new_and_connect().await;
    let user = client.request_no_args(GET_CUR_USER_REQ_TYPE).await.unwrap();
    println!("user: {:?}", user);
    client.shutdown_and_wait_server_exit().await;
}

#[tokio::test]
async fn re_login_smoke() {
    let client = Client::new_and_connect().await;
    let user = client.request_no_args(GET_CUR_USER_REQ_TYPE).await.unwrap();
    let uuid = user.uuid;
    let server = client.server();
    client.shutdown_client();
    let mut client2 = Client::new_with_server(server);
    client2.connect_with_uuid(uuid.as_str()).await;
    let user2 = client2
        .request_no_args(GET_CUR_USER_REQ_TYPE)
        .await
        .unwrap();
    assert_eq!(user.nick_name, user2.nick_name);
    client2.shutdown_and_wait_server_exit().await;
}

#[tokio::test]
async fn multiple_user_smoke() {
    let client = Client::new_and_connect().await;
    let server = client.server();
    for _ in 0..10 {
        let mut client2 = Client::new_with_server(server.clone());
        client2.connect().await;
        let user2 = client2
            .request_no_args(GET_CUR_USER_REQ_TYPE)
            .await
            .unwrap();
        println!("user: {:?}", user2);
    }
    client.shutdown_and_wait_server_exit().await;
}

#[tokio::test]
async fn change_user_name_smoke() {
    let client = Client::new_and_connect().await;
    client
        .request(CHANGE_CUR_USER_NAME_REQ_TYPE, &"test".to_string())
        .await
        .unwrap();
    let user = client.request_no_args(GET_CUR_USER_REQ_TYPE).await.unwrap();
    assert_eq!(user.nick_name, "test");
    client.shutdown_and_wait_server_exit().await;
}

const UNREGISTERED_REQ_TYPE: RequestType<(), ()> = RequestType::new("unregistered");

#[tokio::test]
async fn unregistered_handler_smoke() {
    let client = Client::new_and_connect().await;
    let result = client.request_no_args(UNREGISTERED_REQ_TYPE).await;
    assert!(result.is_err());
    client.shutdown_and_wait_server_exit().await;
}

#[tokio::test]
async fn stream_smoke() {
    let client = Client::new_and_connect().await;
    let mut all_room_stream = client
        .stream_no_args(ALL_ROOM_SIMPLE_INFO_STREAM_TYPE)
        .await
        .unwrap();
    client.request_no_args(CREATE_ROOM_REQ_TYPE).await.unwrap();
    let n = all_room_stream.next().await.unwrap();
    assert_eq!(n.len(), 1);
}

#[tokio::test]
async fn stream_debounce_smoke() {
    let client = Client::new_and_connect().await;
    let mut all_room_stream = client
        .stream_no_args(ALL_ROOM_SIMPLE_INFO_STREAM_TYPE)
        .await
        .unwrap();
    client.request_no_args(CREATE_ROOM_REQ_TYPE).await.unwrap();
    let client2 = Client::new_and_connect_with_server(client.server()).await;
    client2.request_no_args(CREATE_ROOM_REQ_TYPE).await.unwrap();
    all_room_stream.next().await.unwrap();
    let should_timeout_result = timeout(Duration::from_millis(1), all_room_stream.next()).await;
    assert!(should_timeout_result.is_err());
}
