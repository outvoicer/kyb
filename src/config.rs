pub struct KybConfig;

impl KybConfig {
    pub const SOURCE_CSV: &'static str = "https://dati.ur.gov.lv/officers/officers.csv";
    pub const SERVER_ADDRES: &str = "127.0.0.1:10001";
    pub const UPDATE_HOUR: u32 = 2;
    pub const UPDATE_MINUTE: u32 = 0;
}
