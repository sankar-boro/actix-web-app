use lily_scylla::start_scylla_app;
// use lily_pg::start_pg_app;

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::fs::create_dir_all("./tmp").unwrap();
    start_scylla_app().await.unwrap();
}