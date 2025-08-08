pub mod app_folder;
pub mod create_db;
pub mod create_home_dir;
pub mod create_table;
pub mod db_file;
pub mod get_db;
pub mod get_new_data;
pub mod log;
pub mod query;
pub mod respond_lv;

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
struct Officer {
    id: i32,
    uri: String,
    at_legal_entity_registration_number: String,
    entity_type: String,
    position: Option<String>,
    governing_body: String,
    name: String,
    latvian_identity_number_masked: String,
    birth_date: Option<String>,
    legal_entity_registration_number: Option<String>,
    rights_of_representation_type: String,
    representation_with_at_least: i32,
    registered_on: String,
    last_modified_at: String,
}
