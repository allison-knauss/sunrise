use std::path::PathBuf;
use config;

pub fn init() -> config::Config {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("config");
    let mut conf = config::Config::default();
    conf
        .merge(config::File::with_name(d.to_str().unwrap())).unwrap()
        .merge(config::Environment::with_prefix("SUNRISE")).unwrap();
    return conf;
}
