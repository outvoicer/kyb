use std::collections::HashSet;

pub fn vat_status(regcode: &str, vat_table: &HashSet<String>) -> (bool, Option<String>) {
    if vat_table.contains(regcode) {
        let vat_number = format!("LV{}", regcode);
        return (true, Some(vat_number));
    }
    return (false, None);
}
