use loony_scylla::start_scylla_app;
// use loony_pg::start_pg_app;

#[actix_web::main]
async fn main() {
    start_scylla_app().await.unwrap();
}