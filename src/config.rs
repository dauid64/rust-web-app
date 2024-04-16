use crate::{Error, Result};
use std::{env, iter::Once, sync::OnceLock};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })
}

pub struct Config {
    pub web_folder: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(
            Config {
                web_folder: get_env("SERVICE_WEB_FOLDER")?,
            }
        )
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}