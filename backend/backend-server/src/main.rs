mod route_handlers;
mod pocket_papers;
mod trait_helpers;
mod fs_helpers;

use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let state = rustls_acme::AcmeConfig::new(["api.dtb-kampfrichter.de"])
        .contact(["mailto:philipp.remy@dtb.de"])
        .cache_option(Some(rustls_acme::caches::DirCache::new("/Users/philippremy/Documents/GitHub/pocket-papers-dev/certs")))
        .directory_lets_encrypt(true)
        .state();
    let mut rustls_config = Arc::unwrap_or_clone(state.challenge_rustls_config());
    rustls_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    let f1  = async {
        HttpServer::new(|| {
                let cors1 = Cors::permissive();
                App::new()
                    .service(route_handlers::handle_pocket_paper_spi_request)
                    .service(route_handlers::handle_pocket_paper_vlt_request)
                    .service(route_handlers::handle_pocket_paper_stl_request)
                    .service(route_handlers::handle_download_request)
                    .wrap(cors1)
                    .wrap(Logger::default())
            }  
        )
        .bind_rustls_0_23(("127.0.0.1", 443), rustls_config)?
        .workers(std::thread::available_parallelism()?.into())
        .run()
        .await?;
        Ok::<(), anyhow::Error>(())
    };

    let f2 = async {
        HttpServer::new(|| {
                let cors2 = Cors::permissive();
                if cfg!(debug_assertions) {
                    App::new()
                        .service(route_handlers::handle_pocket_paper_spi_request)
                        .service(route_handlers::handle_pocket_paper_vlt_request)
                        .service(route_handlers::handle_pocket_paper_stl_request)
                        .service(route_handlers::handle_download_request)
                        .wrap(cors2)
                        .wrap(Logger::default())
                } else {
                    App::new().default_service(web::to(async |req: HttpRequest| {
                        let host = req.connection_info().host().replace(":80", "");
                        let redirect_url = format!("https://{}{}", host, req.uri());
                        HttpResponse::PermanentRedirect()
                            .append_header(("Location", redirect_url))
                            .finish()
                    }))
                    .wrap(cors2)
                    .wrap(Logger::default())
                }
            }
        )
        .bind(("127.0.0.1", 80))?
        .workers(std::thread::available_parallelism()?.into())
        .run()
        .await?;
        Ok::<(), anyhow::Error>(())
    };

    let mut join_set = tokio::task::JoinSet::new();
    join_set.spawn_local(f1);
    join_set.spawn_local(f2);
    let jvs = join_set.join_all().await;
    for jv in jvs {
        jv?
    }

    Ok(())

}