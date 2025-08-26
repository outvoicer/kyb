use crate::error::KybError;
use crate::latvia::board::query::Query;

pub fn validate_data(query: &Query) -> Result<(), KybError> {
    if query.reg_code.len() != 11 {
        // REG CODE WRONG LENGTH
        return Err(KybError::StringError(
            "Error: reg_code must be 11 characters".to_string(),
        ));
    }
    if query.personal_code.len() < 7 {
        // PERSONAL CODE TO SHORT
        return Err(KybError::StringError(
            "Error: personal_code too short".to_string(),
        ));
    }
    Ok(())
}
