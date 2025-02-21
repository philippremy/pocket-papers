use actix_web::{body::BoxBody, http::StatusCode, ResponseError};

#[derive(Debug)]
pub struct AppError(pub anyhow::Error);

#[derive(Debug)]
pub struct AppErrorWithCode(pub AppError, pub StatusCode);

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl std::fmt::Display for AppErrorWithCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self(anyhow::Error::new(value))
    }
}

impl ResponseError for AppErrorWithCode {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.1
    }

    fn error_response(&self) -> actix_web::HttpResponse<BoxBody> {
        let res = actix_web::HttpResponse::new(self.status_code());
        eprintln!("{:?}", self.0);
        res.set_body(BoxBody::new(format!("Ein Fehler im Backend-Server ist aufgetreten: {}", self.0)))
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let res = actix_web::HttpResponse::new(self.status_code());
        eprintln!("{:?}", self.0);
        res.set_body(BoxBody::new(format!("Ein Fehler im Backend-Server ist aufgetreten: {}", self.0)))
    }
}