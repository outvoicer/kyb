use crate::error::KybError;
use std::fs;
use std::path::PathBuf;

pub fn create_home_dir(company_db_path: PathBuf) -> Result<PathBuf, KybError> {
    fs::create_dir_all(&company_db_path)?;
    Ok(company_db_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dirs::data_dir;
    const TEST_COMPANY_DB: &str = "test_finance_folder";

    #[test]
    fn test_create_home_dir() {
        match data_dir() {
            Some(path) => {
                let test_dir = path.join(TEST_COMPANY_DB);
                // Ensure the directory does not exist before the test
                if test_dir.exists() {
                    fs::remove_dir_all(&test_dir).unwrap();
                }
                // Call the function to create the directory
                let result = create_home_dir(test_dir.clone());
                // Assert that the directory now exists
                assert!(test_dir.exists());
                // Assert that the returned PathBuf is correct
                match result {
                    Ok(r) => {
                        assert_eq!(r, test_dir);
                    }
                    Err(e) => {
                        panic!("at the disco: {:?}", e);
                    }
                }
                // Clean up: remove the directory after the test
                fs::remove_dir_all(test_dir).unwrap();
            }
            None => {
                panic!("Unable to test data dir");
            }
        }
    }
    // TODO TEST MISSING PERMISSIONS AND CASES WHEN DIR EXISTS
}
