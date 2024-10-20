use backend::main_inner;

#[tokio::main]
pub async fn main() -> rsocket_rust::Result<()> {
    main_inner(None, None).await
}
