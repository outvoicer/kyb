use crate::db::app_folder::app_folder;
use crate::error::KybError;
use std::path::PathBuf;

const COMPANY_DB: &str = "finance.db";

pub fn db_file() -> Result<PathBuf, KybError> {
    let folder = app_folder()?;
    Ok(folder.join(COMPANY_DB))
}
