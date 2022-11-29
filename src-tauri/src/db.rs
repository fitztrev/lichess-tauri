use std::path;

use diesel::prelude::*;

use crate::schema;

#[derive(Queryable)]
pub struct SqlSetting {
    pub key: String,
    pub value: String,
}

#[derive(Queryable)]
pub struct SqlEngine {
    lichess_id: String,
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
    lichess_id: &'a str,
    binary_location: &'a str,
}

pub fn establish_connection() -> SqliteConnection {
    let path_to_db_file = path::Path::new(&tauri::api::path::local_data_dir().unwrap())
        .join("lichess-tauri")
        .join("lichess-tauri.sqlite");

    let app_data_dir = path_to_db_file.parent().unwrap();

    assert!(
        app_data_dir.exists() || std::fs::create_dir(&app_data_dir).is_ok(),
        "unable to create path {:?}",
        app_data_dir.to_str().unwrap()
    );

    let database_url = path_to_db_file
        .to_str()
        .clone()
        .expect(&format!("Error convert path {:?} to url", path_to_db_file));

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", &database_url))
}

pub fn add_default_setting(key: &str, value: &str) {
    let mut connection = establish_connection();

    let new_setting = NewSetting {
        key: key,
        value: value,
    };

    diesel::insert_into(schema::settings::table)
        .values(&new_setting)
        .on_conflict_do_nothing()
        .execute(&mut connection)
        .expect("Error saving new setting");
}

pub fn update_setting(key: String, value: String) {
    let mut connection = establish_connection();

    // update or insert
    diesel::insert_into(schema::settings::table)
        .values(&NewSetting {
            key: &key,
            value: &value,
        })
        .on_conflict(schema::settings::key)
        .do_update()
        .set(schema::settings::value.eq(&value))
        .execute(&mut connection)
        .expect("Error saving new setting");


    // diesel::update(schema::settings::table)
    //     .set(schema::settings::value.eq(value))
    //     .filter(schema::settings::key.eq(key))
    //     .execute(&mut connection)
    //     .expect("Error updating setting");
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

pub fn add_engine(lichess_id: &str, binary_location: &str) {
    let mut connection = establish_connection();

    let new_engine = NewEngine {
        lichess_id: lichess_id,
        binary_location: binary_location,
    };

    diesel::insert_into(schema::engines::table)
        .values(&new_engine)
        .on_conflict_do_nothing()
        .execute(&mut connection)
        .expect("Error saving new engine");
}

pub fn get_engine_binary_path(lichess_id: &str) -> Option<String> {
    let mut connection = establish_connection();

    let result = schema::engines::table
        .filter(schema::engines::lichess_id.eq(lichess_id))
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
