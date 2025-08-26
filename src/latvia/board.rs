pub mod create_table;
pub mod get_new_data;
pub mod log;
pub mod lv_board_handle;
pub mod query;
pub mod verify;

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
pub struct Officer {
    id: i32,
    uri: String,
    pub at_legal_entity_registration_number: String,
    entity_type: String,
    pub position: Option<String>,
    governing_body: String,
    pub name: String,
    pub latvian_identity_number_masked: String,
    birth_date: Option<String>,
    legal_entity_registration_number: Option<String>,
    rights_of_representation_type: String,
    representation_with_at_least: i32,
    registered_on: String,
    last_modified_at: String,
}
