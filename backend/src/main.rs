pub mod db;
use rsocket_rust::prelude::*;
use rsocket_rust::utils::EchoRSocket;
use rsocket_rust::Result;
use rsocket_rust_transport_websocket::WebsocketServerTransport;

pub mod model {
    include!(concat!(env!("OUT_DIR"), "/model.rs"));
}

const ADDR: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> Result<()> {
    // let _user_storage = db::InMemoryStorage::<model::User>::default();
    RSocketFactory::receive()
        .acceptor(Box::new(|setup, _socket| {
            println!("accept setup: {:?}", setup);
            Ok(Box::new(EchoRSocket))
        }))
        .transport(WebsocketServerTransport::from(ADDR))
        .serve()
        .await
}

#[cfg(test)]
mod tests {
    use rsocket_rust_transport_websocket::WebsocketClientTransport;

    use super::*;

    #[test]
    fn test() {
        let mut shirt = model::Shirt::default();
        shirt.color = "color".to_string();
        shirt.set_size(model::shirt::Size::Large);
    }

    #[tokio::test]
    async fn test_connect() -> Result<()> {
        let client = RSocketFactory::connect()
            .transport(WebsocketClientTransport::from("127.0.0.1:8080"))
            .setup(Payload::from("READY!"))
            .mime_type("text/plain", "text/plain")
            .start()
            .await?;

        let request_payload = Payload::builder()
            .set_data_utf8("Hello World!")
            .set_metadata_utf8("Rust")
            .build();

        let res = client.request_response(request_payload).await?;
        println!("got response: {:?}", res);
        Ok(())
    }
}
