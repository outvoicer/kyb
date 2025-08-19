fn is_zip(zip_code: &str) -> bool {
    // Check if the string starts with "LV-" and has the correct length
    if zip_code.len() != 7 || !zip_code.starts_with("LV-") {
        return false;
    }

    return true;
}

/// get (city, address, zip) from government address
pub fn parse_gov_address(address: &String) -> (String, String, String) {
    let address_data: Vec<&str> = address.split(',').map(str::trim).collect();
    let mut address = address_data.get(0).unwrap_or(&"").to_string();
    let mut city = address_data.get(1).unwrap_or(&"").to_string();
    let mut zip = "".to_string();
    match address_data.len() {
        1 => {}
        2 => {
            city = address_data.get(0).unwrap_or(&"").to_string();
            address = address_data.get(1).unwrap_or(&"").to_string();
        }
        3 => {
            let entry = address_data.get(2).unwrap_or(&"").to_string();
            if is_zip(&entry) {
                zip = entry;
            }
        }
        4 => {
            let address_1 = address_data.get(1).unwrap_or(&"").to_string();
            let address_2 = address_data.get(2).unwrap_or(&"").to_string();
            let possible_index = address_data.get(3).unwrap_or(&"").to_string();
            if is_zip(&possible_index) {
                zip = possible_index;
                city = format!("{}, {}", address_1, address_2);
            } else {
                city = format!("{}, {}", address_1, possible_index);
            }
        }
        5 => {
            let address_1 = address_data.get(1).unwrap_or(&"").to_string();
            let address_2 = address_data.get(2).unwrap_or(&"").to_string();
            let address_3 = address_data.get(3).unwrap_or(&"").to_string();
            let possible_index = address_data.get(4).unwrap_or(&"").to_string();

            if is_zip(&possible_index) {
                zip = possible_index;
                city = format!("{}, {}, {}", address_1, address_2, address_3);
            } else {
                city = format!(
                    "{}, {}, {}, {}",
                    address_1, address_2, address_3, possible_index
                );
            }
        }
        _ => println!("exception: {:?}", address_data),
    }
    //let address = address_data.drain(2..).collect::<Vec<&str>>().join(", ");
    (city, address, zip)
}

#[cfg(test)]
mod tests {
    use crate::public_institution::parse_address::parse_gov_address;

    #[test]
    fn public_institution_address_parsing() {
        // 5 elements
        let (city, address, zip) = parse_gov_address(
            &"Aldaunes iela 13, Brodi, Ābeļu pag., Jēkabpils nov., LV-5212".to_string(),
        );
        assert_eq!(zip, "LV-5212");
        assert_eq!(city, "Brodi, Ābeļu pag., Jēkabpils nov.");
        assert_eq!(address, "Aldaunes iela 13");
        // 4 elements
        let (city, address, zip) =
            parse_gov_address(&"Gaujas iela 33A, Ādaži, Ādažu nov., LV-2164".to_string());
        assert_eq!(zip, "LV-2164");
        assert_eq!(city, "Ādaži, Ādažu nov.");
        assert_eq!(address, "Gaujas iela 33A");
        // 3
        let (city, address, zip) =
            parse_gov_address(&"Adamova, Vērēmu pag., Rēzeknes nov".to_string());
        assert_eq!(city, "Vērēmu pag.");
        assert_eq!(address, "Adamova");
        assert_eq!(zip, "");
    }
}
