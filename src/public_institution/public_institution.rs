use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
/// schema for company register
pub struct PublicInstitution {
    pub name: String,
    pub registrationNumber: String,
    taxpayerUnitNumber: Option<String>,
    establishedOn: String,
    registeredOn: String,
    pub Status: String,
    statusDetails: Option<String>,
    removedOn: Option<String>,
    independentTaxpayer: String,
    authorityType: String,
    subordinationType: String,
    website: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    addressRegisterCode: String,
    pub address: String,
    institutionOfHigherEducation: String,
    higherAuthorityName: Option<String>,
    higherAuthorityNumber: Option<String>,
    higherAuthorityEmail: Option<String>,
    establishingActNumber: String,
    establishingActDate: String,
    establishingActTitle: String,
    establishingActType: String,
    establishingActLegislatorName: String,
    establishingActLegislatorNumber: String,
}
