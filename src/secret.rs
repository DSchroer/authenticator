use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::SystemTime;
use totp_rs::{Algorithm, TOTP};

pub struct Secret {
    pub name: String,
    value: String,
}

impl Display for Secret {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t{}", self.name, self.value)
    }
}

impl Secret {
    pub fn new(name: String, value: String) -> Self {
        Secret {
            name,
            value
        }
    }

    pub fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let parts: Vec<&str> = input.split("\t").collect();

        if parts.len() != 2 {
            return Err(Box::new(crate::error::Error::from("secret contains wrong number of parts")));
        }

        Ok(Secret {
            name: parts[0].into(),
            value: parts[1].into(),
        })
    }

    pub fn generate(&self) -> Result<String, Box<dyn Error>>  {
        let decoded = &base32::decode(base32::Alphabet::RFC4648{ padding: false }, &self.value)
            .ok_or(Box::new(crate::error::Error::from("failed to decode secret")))?;

        let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, decoded);
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_secs();

        Ok(totp.generate(time))
    }
}

