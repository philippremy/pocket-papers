use actix_web::{get, http::StatusCode, post, web::{Form, Query}, HttpResponse, Result};
use serde::Deserialize;
use crate::{fs_helpers::{create_uuid_dir, defer_folder_deletion, get_uuid_dir, unpack_files_to_dir}, pocket_papers::{generate_pdf_for_pocketpaper, PocketPaperKind, SPIRequest, STLRequest, VLTRequest, HTK_SPI_MAIN, HTK_STL_MAIN, HTK_VLT_MAIN}, trait_helpers::{AppError, AppErrorWithCode}};

#[post("/vault-pdf")]
pub async fn handle_pocket_paper_vlt_request(form_data: Form<VLTRequest>) -> Result<HttpResponse> {

    // Generate a uuid for the request
    let request_uuid = uuid::Uuid::new_v4().to_string();

    // Generate a folder and extract files needed
    let request_folder = create_uuid_dir(&request_uuid).await?;
    unpack_files_to_dir(&request_folder, "main.typ", HTK_VLT_MAIN).await?;

    // Defer folder deletion
    defer_folder_deletion(&request_uuid).await;

    // Create the PDF
    generate_pdf_for_pocketpaper(PocketPaperKind::VLT(form_data.into_inner()), &request_uuid).await?;

    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "text/plaintext"))
        .body(request_uuid)
    )
}

#[post("/spiral-pdf")]
pub async fn handle_pocket_paper_spi_request(form_data: Form<SPIRequest>) -> Result<HttpResponse> {

    // Generate a uuid for the request
    let request_uuid = uuid::Uuid::new_v4().to_string();

    // Generate a folder and extract files needed
    let request_folder = create_uuid_dir(&request_uuid).await?;
    unpack_files_to_dir(&request_folder, "main.typ", HTK_SPI_MAIN).await?;

    // Defer folder deletion
    defer_folder_deletion(&request_uuid).await;

    // Create the PDF
    generate_pdf_for_pocketpaper(PocketPaperKind::SPI(form_data.into_inner()), &request_uuid).await?;

    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "text/plaintext"))
        .body(request_uuid)
    )
}

#[post("/straightline-pdf")]
pub async fn handle_pocket_paper_stl_request(form_data: Form<STLRequest>) -> Result<HttpResponse> {

    // Generate a uuid for the request
    let request_uuid = uuid::Uuid::new_v4().to_string();

    // Generate a folder and extract files needed
    let request_folder = create_uuid_dir(&request_uuid).await?;
    unpack_files_to_dir(&request_folder, "main.typ", HTK_STL_MAIN).await?;

    // Defer folder deletion
    defer_folder_deletion(&request_uuid).await;

    // Create the PDF
    generate_pdf_for_pocketpaper(PocketPaperKind::STL(form_data.into_inner()), &request_uuid).await?;

    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "text/plaintext"))
        .body(request_uuid)
    )
}

#[derive(Debug, Deserialize)]
struct UUID {
    uuid: String,
}

#[get("/dyn-pocket-paper-download")]
pub async fn handle_download_request(uuid: Query<UUID>) -> Result<HttpResponse> {

    println!("{uuid:?}");

    if let Some(path) = get_uuid_dir(&uuid.uuid).await? {
        println!("{path:?}");
        let bytes = std::fs::read(&path.join("Hosentaschenkarte.pdf"))?;
        return Ok(
            HttpResponse::Ok()
                .insert_header(("Content-Type", "application/pdf"))
                .insert_header(("Content-Length", bytes.len()))
                .insert_header(("Content-Disposition", "attachment; filename='Hosentaschenkarte.pdf'"))
                .insert_header(("Cache-Control", "no-store"))
                .body(bytes)
        )
    } else {
        Err(AppErrorWithCode(
            AppError(anyhow::Error::msg("Die angeforderte Datei wurde auf dem Server nicht gefunden.\nHosentaschenkarten werden automatisch nach zwei Stunden vom Server gelöscht!")),
            StatusCode::NO_CONTENT   
        ).into())
    }

}