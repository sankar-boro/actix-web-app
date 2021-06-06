use lily_scylla::start_scylla_app;
// use lily_pg::start_pg_app;

#[actix_web::main]
async fn main() {
    start_scylla_app().await.unwrap();
}