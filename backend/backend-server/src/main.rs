mod route_handlers;
mod pocket_papers;
mod trait_helpers;
mod fs_helpers;
mod logging;

use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};
use fs_helpers::{unpack_files_to_dir, SERVER_DIR};
use logging::activate_logging;
use pocket_papers::{HTK_ABT_LOGO, HTK_DTB_LOGO};
use trait_helpers::into_anyhow_result;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    setup_infrastructure().await.unwrap();

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

async fn setup_infrastructure() -> anyhow::Result<()> {
    
    // Initialize the server dir
    let server_dir = SERVER_DIR.get_or_try_init::<anyhow::Error, _, _>(async || {
        let data_dir = directories::ProjectDirs::from("de", "philippremy", "backend-server").unwrap().data_dir().to_path_buf();
        
        if std::fs::exists(&data_dir)? {
            return Ok(data_dir)
        }
        
        std::fs::create_dir(&data_dir)?;
        
        Ok(data_dir)
    }).await?;

    // Create Resource dir
    let shared_dir = server_dir.join("Shared");
    if !shared_dir.exists() {
        std::fs::create_dir(&shared_dir)?;
    }

    // Unpack shared Resources
    into_anyhow_result(unpack_files_to_dir(&shared_dir, "dtb.svg", HTK_DTB_LOGO).await)?;
    into_anyhow_result(unpack_files_to_dir(&shared_dir, "abteilung.svg", HTK_ABT_LOGO).await)?;

    // Create Log dir
    let log_dir = server_dir.join("Logs");
    if !log_dir.exists() {
        std::fs::create_dir(&log_dir)?;
    }

    // Reroute StdOut and StdErr
    activate_logging(log_dir)?;

    Ok(())
}