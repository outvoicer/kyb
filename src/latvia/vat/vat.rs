use serde::{Deserialize, Serialize};

/// is VAT turned on? "ir" - yes, "nav" - no
#[allow(non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum IsActive {
    ir,
    nav,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
/// schema for VAT from open data
/// https://data.gov.lv/dati/dataset/9a5eae1c-2438-48cf-854b-6a2c170f918f/resource/610910e9-e086-4c5b-a7ea-0a896a697672/download/pdb_pvnmaksataji_odata.csv
pub struct VATSchema {
    pub Numurs: String,         // VAT Number
    Nosaukums: String,          // Organisation name
    pub Aktivs: IsActive,       // Is VAT active?
    Registrets: String,         // VAT Registration start
    Buvniecibas_pazime: String, // Construction
    Izslegts: String,           // VAT Registration end
}
