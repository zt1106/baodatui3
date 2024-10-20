use backend::global::handlers::room_handlers::{
    CHANGE_GAME_CONFIG_REQ_TYPE, CREATE_ROOM_REQ_TYPE, ENTER_ROOM_REQ_TYPE, LEAVE_ROOM_REQ_TYPE,
    LIST_ROOM_SIMPLE_INFO_REQ_TYPE,
};
use backend::model::configs::GameConfigurations;
use backend::test_client::Client;

#[tokio::test]
async fn create_room_test() {
    let client = Client::new_and_connect().await;
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
    let client = Client::new_and_connect().await;
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
async fn multiple_users_enter_room_test() {
    let client = Client::new_and_connect().await;
    client.request_no_args(CREATE_ROOM_REQ_TYPE).await.unwrap();
    let list = client
        .request_no_args(LIST_ROOM_SIMPLE_INFO_REQ_TYPE)
        .await
        .unwrap();
    assert_eq!(list.len(), 1);
    let room_id = list.get(0).unwrap().id;
    let client2 = Client::new_and_connect_with_server(client.server());
    client2
        .await
        .request(ENTER_ROOM_REQ_TYPE, &room_id)
        .await
        .unwrap();
    let list = client
        .request_no_args(LIST_ROOM_SIMPLE_INFO_REQ_TYPE)
        .await
        .unwrap();
    assert_eq!(list.get(0).unwrap().cur_user_count, 2);
}

#[tokio::test]
async fn change_game_config_test() {
    let client = Client::new_and_connect().await;
    client.request_no_args(CREATE_ROOM_REQ_TYPE).await.unwrap();
    let mut new_config = GameConfigurations::default();
    new_config.basic_configs.max_player_count = 4;
    client
        .request(CHANGE_GAME_CONFIG_REQ_TYPE, &new_config)
        .await
        .unwrap();
    client.shutdown_and_wait_server_exit().await;
}

#[tokio::test]
async fn non_owner_change_config_test() {
    let client = Client::new_and_connect().await;
    client.request_no_args(CREATE_ROOM_REQ_TYPE).await.unwrap();
    let mut new_config = GameConfigurations::default();
    new_config.basic_configs.max_player_count = 4;
    let client2 = Client::new_and_connect_with_server(client.server()).await;
    let change_result = client2
        .request(CHANGE_GAME_CONFIG_REQ_TYPE, &new_config)
        .await;
    assert!(change_result.is_err());
    client.shutdown_and_wait_server_exit().await;
}

#[tokio::test]
#[should_panic]
async fn enter_room_over_capacity_test() {
    let client = Client::new_and_connect().await;
    client.request_no_args(CREATE_ROOM_REQ_TYPE).await.unwrap();
    let list = client
        .request_no_args(LIST_ROOM_SIMPLE_INFO_REQ_TYPE)
        .await
        .unwrap();
    let room_id = list.get(0).unwrap().id;
    for i in 0..10 {
        let client2 = Client::new_and_connect_with_server(client.server()).await;
        client2
            .request(ENTER_ROOM_REQ_TYPE, &room_id)
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn non_active_room_test() {}
