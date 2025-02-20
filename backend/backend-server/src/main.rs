mod route_handlers;
mod pocket_papers;
mod trait_helpers;
mod fs_helpers;

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        let cors1 = Cors::permissive();
        App::new()
            .service(route_handlers::handle_pocket_paper_spi_request)
            .service(route_handlers::handle_pocket_paper_vlt_request)
            .service(route_handlers::handle_pocket_paper_stl_request)
            .service(route_handlers::handle_download_request)
            .wrap(cors1)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 5555))?
    .workers(std::thread::available_parallelism()?.into())
    .run()
    .await?;

    Ok(())
}