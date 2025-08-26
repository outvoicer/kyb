use crate::latvia::vat::vat::{IsActive, VATSchema};
use csv::Reader;
use std::collections::HashSet;
use std::error::Error;
use std::io::Cursor;

pub async fn get_vat_data(
    mut rdr: Reader<Cursor<String>>,
) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut vat_map = HashSet::new();
    // Begin a transaction
    for result in rdr.deserialize() {
        let input: VATSchema = result?;
        match input.Aktivs {
            IsActive::ir => {
                // REMOVE LV FROM BEGINNING
                let reg_code: String = (&input.Numurs[2..]).to_string();
                vat_map.insert(reg_code);
            }
            IsActive::nav => {}
        }
    }

    println!("total VAT numbers: {}", vat_map.len());

    Ok(vat_map)
}
