/// Check if the string starts with "LV-" and has the correct length
fn is_zip(zip_code: &str) -> bool {
    if zip_code.len() != 7 || !zip_code.starts_with("LV-") {
        return false;
    }
    return true;
}

/// get (city, address, zip) from government address
pub fn parse_gov_address(address: &str) -> (String, String, String) {
    let address_data: Vec<&str> = address.split(',').map(str::trim).collect();

    let mut address = String::new();
    let mut city = String::new();
    let mut zip = String::new();

    match address_data.len() {
        1 => {
            address = address_data[0].to_string();
        }
        2 => {
            city = address_data[0].to_string();
            address = address_data[1].to_string();
        }
        3 => {
            address = address_data[0].to_string();
            city = address_data[1].to_string();
            if is_zip(address_data[2]) {
                zip = address_data[2].to_string();
            }
        }
        4 => {
            address = address_data[0].to_string();
            city = format!("{}, {}", address_data[1], address_data[2]);
            if is_zip(address_data[3]) {
                zip = address_data[3].to_string();
            } else {
                city = format!("{}, {}", city, address_data[3]);
            }
        }
        5 => {
            address = address_data[0].to_string();
            city = format!(
                "{}, {}, {}",
                address_data[1], address_data[2], address_data[3]
            );
            if is_zip(address_data[4]) {
                zip = address_data[4].to_string();
            } else {
                city = format!("{}, {}", city, address_data[4]);
            }
        }
        _ => println!("Unexpected format: {:?}", address_data),
    }

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
