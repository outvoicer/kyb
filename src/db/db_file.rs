use crate::db::app_folder::app_folder;
use crate::error::AppError;
use std::path::PathBuf;

const COMPANY_DB: &str = "finance.db";

pub fn db_file() -> Result<PathBuf, AppError> {
    let folder = app_folder()?;
    Ok(folder.join(COMPANY_DB))
}

pub fn does_db_exist() -> Result<bool, AppError> {
    let file = db_file()?;
    if file.exists() && file.is_file() {
        return Ok(true);
    } else {
        return Err(AppError::StringError("Cant find file".to_string()));
    }
}
