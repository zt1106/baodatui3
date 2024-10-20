use backend::global::handlers::room_handlers::{
    CREATE_ROOM_REQ_TYPE, LEAVE_ROOM_REQ_TYPE, LIST_ROOM_SIMPLE_INFO_REQ_TYPE,
};
use backend::test_client::Client;

#[tokio::test]
async fn create_room_test() {
    let mut client = Client::new();
    client.connect().await;
    let list = client
        .request_no_args(LIST_ROOM_SIMPLE_INFO_REQ_TYPE)
        .await
        .unwrap();
    assert_eq!(list.len(), 0);
    client.request(CREATE_ROOM_REQ_TYPE, &()).await.unwrap();
    let list = client
        .request_no_args(LIST_ROOM_SIMPLE_INFO_REQ_TYPE)
        .await
        .unwrap();
    assert_eq!(list.len(), 1);
    client.shutdown_and_wait_server_exit().await;
}

#[tokio::test]
async fn last_user_leave_room_test() {
    let mut client = Client::new();
    client.connect().await;
    client.request_no_args(CREATE_ROOM_REQ_TYPE).await.unwrap();
    let list = client
        .request_no_args(LIST_ROOM_SIMPLE_INFO_REQ_TYPE)
        .await
        .unwrap();
    assert_eq!(list.len(), 1);
    client.request_no_args(LEAVE_ROOM_REQ_TYPE).await.unwrap();
    let list = client
        .request_no_args(LIST_ROOM_SIMPLE_INFO_REQ_TYPE)
        .await
        .unwrap();
    assert_eq!(list.len(), 0);
    client.shutdown_and_wait_server_exit().await;
}

#[tokio::test]
async fn multiple_users_enter_room_test() {}

#[tokio::test]
async fn enter_room_over_capacity_test() {}

#[tokio::test]
async fn non_active_room_test() {}
