use ini::Ini;
use std::str::FromStr;

#[macro_export]
macro_rules! unwrap_or_err {
    ( $x:expr, $y:expr ) => {{
        if let Ok(v) = $x {
            v
        } else {
            return Err($y);
        }
    }};
}

#[derive(Debug)]
pub enum INIError {
    LoadError,
    SaveError,
}

pub struct Config {
    pub location: crate::Location,
    pub evil: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            location: crate::Location::Mobile,
            evil: false,
        }
    }
}

pub fn load() -> Result<Config, INIError> {
    let conf = unwrap_or_err!(Ini::load_from_file("conf.ini"), INIError::LoadError);

    let section = conf.section(Some("Settings")).ok_or(INIError::LoadError)?;
    let location = section.get("location").ok_or(INIError::LoadError)?;
    let evil = section.get("evil").ok_or(INIError::LoadError)?;

    let location = unwrap_or_err!(crate::Location::from_str(location), INIError::LoadError);

    let evil = unwrap_or_err!(FromStr::from_str(evil), INIError::LoadError);

    Ok(Config { location, evil })
}

pub fn save(config: &Config) -> Result<(), INIError> {
    let mut conf = Ini::new();
    conf.with_section(Some("Settings"))
        .set("location", config.location.as_string())
        .set("evil", if config.evil { "true" } else { "false" });

    unwrap_or_err!(conf.write_to_file("conf.ini"), INIError::SaveError);

    Ok(())
}
