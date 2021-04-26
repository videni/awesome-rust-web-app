use libreauth::pass::{HashBuilder, Algorithm, Hasher};

pub trait PasswordEncoder {
    fn encode_password(&self, raw: &str, salt: Option<&str>) -> String;

    fn is_password_valid(&self, encoded: &str, raw: &str, salt: Option<&str>) -> bool;
}

pub const PWD_SCHEME_VERSION: usize = 1;
pub const PWD_ALGORITHM: Algorithm = Algorithm::Argon2;

pub struct Argon2PasswordEncoder {

}

lazy_static! {
    pub static ref HASHER: Hasher = HashBuilder::new()
    .version(PWD_SCHEME_VERSION)
    .algorithm(PWD_ALGORITHM)
    .finalize()
    .unwrap();
}

impl PasswordEncoder for Argon2PasswordEncoder {
    fn encode_password(&self, raw: &str, _salt: Option<&str>) -> String {
        let encoded_password = HASHER.hash(raw).unwrap();

        return encoded_password;
    }

    fn is_password_valid(&self, encoded: &str, raw: &str, _salt: Option<&str>) -> bool {
        let checker = HashBuilder::from_phc(encoded).unwrap();

        return checker.is_valid(raw);
    }
}
