pub fn parse_address(address: &String) -> (String, String) {
    let address_data: Vec<String> = address.split(',').map(|s| s.to_string()).collect();
    match address_data.len() {
        2 => {
            let city = format!("{}", address_data[0].trim());
            let address = format!("{}", address_data[1].trim());
            (city, address)
        }
        3 => {
            let city = format!("{}", address_data[0].trim());
            let address = format!("{}, {}", address_data[1].trim(), address_data[2].trim());
            (city, address)
        }
        4 => {
            let city = format!("{}", address_data[0].trim());
            let address = format!(
                "{}, {}, {}",
                address_data[1].trim(),
                address_data[2].trim(),
                address_data[3].trim()
            );
            (city, address)
        }
        _ => panic!("uncovered {}", address_data.len()),
    }
}

#[cfg(test)]
mod tests {
    use crate::company::parse_address::parse_address;

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
