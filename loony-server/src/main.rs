use loony_pg;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    loony_pg::start_pg_app().await.unwrap();
    Ok(())
}