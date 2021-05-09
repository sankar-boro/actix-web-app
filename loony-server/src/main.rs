use loony_scylla::start_scylla_app;

#[actix_web::main]
async fn main() {
    start_scylla_app().await.unwrap();
}