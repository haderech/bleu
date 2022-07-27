use std::path::PathBuf;
use actix_files::NamedFile;

use actix_web::{HttpRequest, Result};

pub async fn load_swagger(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./swagger-ui/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}