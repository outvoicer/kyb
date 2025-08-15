#[derive(Debug, serde::Deserialize)]
/// schema for company register
struct Company {
    reg_code: String,
    name: String,
    address: String,
    zip: String,
    legal_form: String,
    closed: String, // bool
}

impl Company {
    pub fn new(
        name: String,
        reg_code: String,
        address: String,
        zip: String,
        legal_form: String,
        closed: String,
    ) -> Self {
        Self {
            name: name,
            reg_code: reg_code,
            address: address,
            zip: zip,
            legal_form: legal_form,
            closed: closed,
        }
    }
}
