use crate::enviroment::Environment;
use serde_json::{json, Value};

pub fn insert_config_overwrite(
    mut config: Value,
    key: String,
    value: String,
) -> Result<Value, String> {
    let sub_keys = key.split("__").collect::<Vec<&str>>();

    let mut current_sub_object: &mut Value = &mut config;

    for key in sub_keys {
        match current_sub_object.get_mut(key) {
            Some(value) => {
                current_sub_object = value;
            }
            None => {
                return Err("Key not found.".to_owned());
            }
        }
    }

    *current_sub_object = json!(value);

    Ok(config)
}

pub fn read_config_file(Environment(env): Environment) -> Result<Value, String> {
    let config_file = std::fs::read_to_string(format!("./config/{env}.json"))
        .map_err(|error| format!("Unable to load config file. {}", error))?;

    let mut config: Value = serde_json::from_str(&config_file)
        .map_err(|error| format!("Unable to parse config file. {}", error))?;

    if config.is_null() {
        return Err("Config should not be null.".to_owned());
    }

    if config.is_null() {
        return Err("Config should not be null.".to_owned());
    }

    std::env::vars().for_each(|(key, value)| {
        if key.starts_with("CONFIG__") {
            let key = key.replace("CONFIG__", "");

            match insert_config_overwrite(config.clone(), key, value) {
                Ok(next_config) => {
                    config = next_config;
                }
                Err(error) => {
                    println!("{error}");
                }
            }
        }
    });

    Ok(config)
}
