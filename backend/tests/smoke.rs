use backend::test_client::{Client, Server};
use std::time::Duration;
use tokio::spawn;
use tokio::time::sleep;

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
    let mut client = Client::new();
    client.connect().await;
    client.shutdown_and_wait_server_exit().await;
}

#[tokio::test]
async fn create_new_user_smoke() {
    let mut client = Client::new();
    client.connect().await;
    let user = client.cur_user().await;
    println!("user: {:?}", user);
    client.shutdown_and_wait_server_exit().await;
}

#[tokio::test]
async fn re_login_smoke() {
    let mut client = Client::new();
    client.connect().await;
    let user = client.cur_user().await;
    let uuid = user.uuid;
    let server = client.server();
    client.shutdown_client();
    let mut client2 = Client::new_with_server(server);
    client2.connect_with_uuid(uuid.as_str()).await;
    let user2 = client2.cur_user().await;
    assert_eq!(user.nick_name, user2.nick_name);
}

#[tokio::test]
async fn multiple_user_smoke() {
    let mut client = Client::new();
    client.connect().await;
    let server = client.server();
    for _ in 0..10 {
        let mut client2 = Client::new_with_server(server.clone());
        client2.connect().await;
        let user2 = client2.cur_user().await;
        println!("user: {:?}", user2);
    }
}

#[tokio::test]
async fn change_user_name_smoke() {
    let mut client = Client::new();
    client.connect().await;
    client.change_cur_user_name("test").await.unwrap();
    let user = client.cur_user().await;
    assert_eq!(user.nick_name, "test");
}
