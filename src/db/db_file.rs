use crate::db::app_folder::app_folder;
use crate::error::KybError;
use std::path::PathBuf;

const COMPANY_DB: &str = "finance.db";

pub fn db_file() -> Result<PathBuf, KybError> {
    let folder = app_folder()?;
    Ok(folder.join(COMPANY_DB))
}

pub fn does_db_exist() -> Result<bool, KybError> {
    let file = db_file()?;
    if file.exists() && file.is_file() {
        return Ok(true);
    } else {
        return Err(KybError::StringError("Cant find file".to_string()));
    }
}
