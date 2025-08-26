use crate::config::KybConfig;
use crate::db::app_folder::app_folder;
use crate::error::KybError;
use std::path::PathBuf;

pub fn db_file() -> Result<PathBuf, KybError> {
    const COMPANY_DB: &str = KybConfig::DB_FILE;
    let folder = app_folder()?;
    Ok(folder.join(COMPANY_DB))
}
