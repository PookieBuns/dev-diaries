use backend::app;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = app::app().await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
