use crate::db::query::Query;

pub fn validate_data(query: &Query) -> Result<(), String> {
    if query.reg_code.len() != 11 {
        // REG CODE WRONG LENGTH
        return Err("Error: reg_code must be 11 characters".to_string());
    }
    if query.personal_code.len() < 7 {
        // PERSONAL CODE TO SHORT
        return Err("Error: personal_code too short".to_string());
    }
    Ok(())
}
