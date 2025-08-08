use crate::db::create_home_dir::create_home_dir;
use crate::error::KybError;
use dirs::data_dir;
use std::path::PathBuf;
const COMPANY_DB: &str = "kyb";

pub fn app_folder() -> Result<PathBuf, KybError> {
    let path =
        data_dir().ok_or_else(|| KybError::StringError("Unable to find main directory".into()))?;

    let company_db_path = path.join(COMPANY_DB);

    if !company_db_path.exists() {
        create_home_dir(company_db_path.clone())?;
    }

    Ok(company_db_path)
}
