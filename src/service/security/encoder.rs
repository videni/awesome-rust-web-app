use libreauth::pass::{HashBuilder};

pub trait PasswordEncoder {
    fn encode_password(&self, raw: &str, salt: Option<&str>) -> String;

    fn is_password_valid(&self, encoded: &str, raw: &str, salt: Option<&str>) -> bool;
}

pub const PWD_SCHEME_VERSION: usize = 1;

pub struct Argon2PasswordEncoder {

}

impl PasswordEncoder for Argon2PasswordEncoder {
    fn encode_password(&self, raw: &str, _salt: Option<&str>) -> String {
        let hasher = HashBuilder::new().version(PWD_SCHEME_VERSION).finalize().unwrap();
        let encoded_password = hasher.hash(raw).unwrap();

        return encoded_password;
    }

    fn is_password_valid(&self, encoded: &str, raw: &str, _salt: Option<&str>) -> bool {
        let checker = HashBuilder::from_phc(raw).unwrap();

        return checker.is_valid(encoded);
    }
}
