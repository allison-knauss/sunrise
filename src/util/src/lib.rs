pub mod sunrise_config;
pub mod sunrise_db;

pub use crate::sunrise_config as config;

#[cfg(test)]
mod tests {
    use crate::config as CONFIG;
    // use std::collections::HashMap;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn create_config() {
        let conf = CONFIG::init();
        assert_eq!(conf.get("a").ok(), Some(3));
        // println!("{:?}",
        //          _conf.try_into::<HashMap<String, String>>().unwrap());
    }
}
