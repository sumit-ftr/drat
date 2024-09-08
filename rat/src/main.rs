#[tokio::main]
async fn main() {
    // rat::startup::startup();
    rat::run().await;
}
