use crate::{schema, utils::get_app_data_dir};
use diesel::prelude::*;

#[derive(Queryable)]
pub struct SqlSetting {
    pub key: String,
    pub value: String,
}

#[allow(dead_code)]
#[derive(Queryable)]
pub struct SqlEngine {
    engine_id: String,
    binary_location: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::settings)]
struct NewSetting<'a> {
    key: &'a str,
    value: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = schema::engines)]
struct NewEngine<'a> {
    engine_id: &'a str,
    binary_location: &'a str,
}

pub fn establish_connection() -> SqliteConnection {
    let path_to_db_file = get_app_data_dir().join("db.sqlite");

    let database_url = path_to_db_file
        .to_str()
        .unwrap_or_else(|| panic!("Error convert path {:?} to url", path_to_db_file));

    SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url))
}

pub fn update_setting(key: &str, value: &str) {
    let mut connection = establish_connection();

    // update or insert
    diesel::insert_into(schema::settings::table)
        .values(&NewSetting { key, value })
        .on_conflict(schema::settings::key)
        .do_update()
        .set(schema::settings::value.eq(&value))
        .execute(&mut connection)
        .expect("Error saving new setting");
}

pub fn delete_setting(key: &str) {
    let mut connection = establish_connection();

    diesel::delete(schema::settings::table.filter(schema::settings::key.eq(key)))
        .execute(&mut connection)
        .expect("Error deleting setting");
}

pub fn get_setting(key: &str) -> Option<String> {
    let mut connection = establish_connection();

    let result = schema::settings::table
        .filter(schema::settings::key.eq(key))
        .first::<SqlSetting>(&mut connection);

    match result {
        Ok(setting) => Some(setting.value),
        Err(_) => None,
    }
}

pub fn get_all_settings() -> Vec<SqlSetting> {
    let mut connection = establish_connection();

    schema::settings::table
        .load::<SqlSetting>(&mut connection)
        .expect("Error loading settings")
}

pub fn add_engine(engine_id: &str, binary_location: &str) {
    let mut connection = establish_connection();

    let new_engine = NewEngine {
        engine_id,
        binary_location,
    };

    diesel::insert_into(schema::engines::table)
        .values(&new_engine)
        .on_conflict_do_nothing()
        .execute(&mut connection)
        .expect("Error saving new engine");
}

pub fn delete_engine(engine_id: &str) {
    let mut connection = establish_connection();

    diesel::delete(schema::engines::table.filter(schema::engines::engine_id.eq(engine_id)))
        .execute(&mut connection)
        .expect("Error deleting engine");
}

pub fn get_engine_binary_path(engine_id: &str) -> Option<String> {
    let mut connection = establish_connection();

    let result = schema::engines::table
        .filter(schema::engines::engine_id.eq(engine_id))
        .first::<SqlEngine>(&mut connection);

    match result {
        Ok(engine) => Some(engine.binary_location),
        Err(_) => None,
    }
}

pub fn get_all_engine_binary_paths() -> Vec<SqlEngine> {
    let mut connection = establish_connection();

    schema::engines::table
        .load::<SqlEngine>(&mut connection)
        .expect("Error loading engines")
}

pub fn get_engine_count() -> i64 {
    let mut connection = establish_connection();

    schema::engines::table
        .count()
        .get_result(&mut connection)
        .expect("Error getting engine count")
}
