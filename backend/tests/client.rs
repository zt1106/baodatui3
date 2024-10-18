use std::time::Duration;
use backend::main_inner;
use tokio::spawn;
use tokio::sync::oneshot::Sender;
use tokio::task::JoinHandle;
use tokio::time::sleep;

pub struct Client {
    shutdown_main_send: Option<Sender<()>>,
    main_join_handle: Option<JoinHandle<()>>,
}

impl Client {
    pub fn new() -> Self {
        let (shutdown_main_send, recv) = tokio::sync::oneshot::channel::<()>();
        let main_join_handle = spawn(async {
            let _ = main_inner(Some(recv)).await;
        });
        Self {
            shutdown_main_send: Some(shutdown_main_send),
            main_join_handle: Some(main_join_handle),
        }
    }

    /// can only call once
    pub fn shutdown_send(&mut self) -> Sender<()> {
        self.shutdown_main_send.take().unwrap()
    }

    /// can only call once
    pub fn join_handle(&mut self) -> JoinHandle<()> {
        self.main_join_handle.take().unwrap()
    }
}

#[tokio::test]
async fn shutdown_smoke() {
    let mut client = Client::new();
    let shutdown_send = client.shutdown_send();
    spawn(async move {
        sleep(Duration::from_millis(1000)).await;
        let _ = shutdown_send.send(());
    });
    let join_handle = client.join_handle();
    join_handle.await.unwrap();
}