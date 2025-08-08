use crate::db::create_home_dir::create_home_dir;
use crate::error::AppError;
use dirs::data_dir;
use std::path::PathBuf;
const COMPANY_DB: &str = "finance_folder";

pub fn app_folder() -> Result<PathBuf, AppError> {
    let path =
        data_dir().ok_or_else(|| AppError::StringError("Unable to find main directory".into()))?;
    let company_db_path = path.join(COMPANY_DB);
    //    println!("{:?}", &company_db_path);

    if !company_db_path.exists() {
        create_home_dir(company_db_path.clone())?;
    }

    Ok(company_db_path)
}
