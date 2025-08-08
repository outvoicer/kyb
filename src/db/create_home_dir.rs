use crate::error::KybError;
use std::fs;
use std::path::PathBuf;

pub fn create_home_dir(company_db_path: PathBuf) -> Result<PathBuf, KybError> {
    fs::create_dir_all(&company_db_path)?;
    Ok(company_db_path)
}
