use anyhow::{anyhow as error, Result};
use std::{env, io::Read, io::Write};

pub fn get_env<S: Into<String>>(name: S) -> Result<String> {
    let local_n = name.into();
    env::var(&local_n).map_err(|_| error!("{} variable not found", local_n.clone()))
}

pub fn get_env_bool<S: Into<String>>(name: S) -> Result<bool> {
    let value = get_env(name)?;
    if value.to_lowercase() == "true" {
        return Ok(true);
    }
    Ok(false)
}
