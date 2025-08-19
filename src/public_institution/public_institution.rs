use chrono::NaiveDate;

#[derive(Debug)]

/// schema for company register
pub struct PublicInstitution {
    pub name: String,
    pub registration_number: String,
    taxpayer_unit_number: Option<String>,
    established_on: NaiveDate,
    registered_on: NaiveDate,
    pub status: String,
    status_details: Option<String>,
    removed_on: Option<NaiveDate>,
    independent_taxpayer: bool,
    authority_type: String,
    subordination_type: String,
    website: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    address_register_code: String,
    pub address: String,
    institution_of_higher_education: bool,
    higher_authority_name: Option<String>,
    higher_authority_number: Option<String>,
    higher_authority_email: Option<String>,
    establishing_act_number: String,
    establishing_act_date: NaiveDate,
    establishing_act_title: String,
    establishing_act_type: String,
    establishing_act_legislator_name: String,
    establishing_act_legislator_number: String,
}
