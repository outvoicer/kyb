pub fn parse_address(address: &String) -> (String, String) {
    let mut address_data: Vec<&str> = address.split(',').map(str::trim).collect();
    let city = address_data.get(0).unwrap_or(&"").to_string();
    let address = address_data.drain(1..).collect::<Vec<&str>>().join(", ");
    (city, address)
}

#[cfg(test)]
mod tests {
    use crate::latvia::company::parse_address::parse_address;

    #[test]
    fn address_parsing() {
        let (city, address) = parse_address(&"Rēzekne, Latgales iela 88".to_string());
        assert_eq!(city, "Rēzekne");
        assert_eq!(address, "Latgales iela 88");
        let (city, address) = parse_address(&"Tukuma nov., Tukums, Eksporta iela 8".to_string());
        assert_eq!(city, "Tukuma nov.");
        assert_eq!(address, "Tukums, Eksporta iela 8");
        let (city, address) =
            parse_address(&"Dundagas nov., Kolkas pag., Kolka, Krastnieki".to_string());
        assert_eq!(city, "Dundagas nov.");
        assert_eq!(address, "Kolkas pag., Kolka, Krastnieki");
    }
}
