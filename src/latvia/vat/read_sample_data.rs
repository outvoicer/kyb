use crate::latvia::vat::import::get_vat_data;
use csv::ReaderBuilder;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read};

#[allow(dead_code)]
pub async fn read_sample_vat_data() -> Result<HashSet<String>, Box<dyn Error>> {
    // GET SAMPLE DATA
    let path = "./src/latvia/vat/vat.csv";
    let mut file = File::open(path)?;
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents)?;
    let cursor = Cursor::new(contents);

    let rdr = ReaderBuilder::new()
        .has_headers(true) // The first row is a header
        .trim(csv::Trim::All) // Trim whitespace around fields
        .from_reader(cursor);

    let vat_map = get_vat_data(rdr).await?;

    Ok(vat_map)
}
