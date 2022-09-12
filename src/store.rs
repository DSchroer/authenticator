use std::error::Error;
use crate::Secret;
use sodiumoxide::crypto::pwhash::Salt;
use sodiumoxide::crypto::secretbox::xsalsa20poly1305::{Key};
use sodiumoxide::crypto::secretbox::Nonce;
use crate::store::Store::{Encrypted, Plaintext};
use base64::{decode};
use sodiumoxide::crypto::{pwhash, secretbox};
use std::string::String;

#[derive(Debug, PartialEq, Clone)]
pub enum Store {
    Plaintext(Vec<Secret>),
    Encrypted(Key, Salt, Vec<Secret>)
}

impl Store {
    pub fn new() -> Self {
        Plaintext(Vec::new())
    }

    pub fn from_str(input: &str, passwd: impl Fn() -> String) -> Result<Self, Box<dyn Error>> {
        if !input.starts_with("$") {
            let mut secrets = Vec::new();
            for line in input.split('\n').filter(|l|!l.is_empty()) {
                secrets.push(Secret::parse(line)?);
            }
            return Ok(Plaintext(secrets));
        }

        let mut parts = input.split("$");
        parts.next();
        let salt = decode(parts.next().unwrap())?;
        let salt = Salt::from_slice(&salt[..]).unwrap();

        let nonce = decode(parts.next().unwrap())?;
        let nonce = Nonce::from_slice(&nonce[..]).unwrap();

        let key = derive_key(&salt, passwd);

        let body = decode(parts.next().unwrap())?;

        let their_plaintext = secretbox::open(&body, &nonce, &key);
        if their_plaintext.is_err() {
            return Err(Box::new(crate::error::Error::from("Incorrect PIN!")));
        }

        let their_plaintext = String::from_utf8(their_plaintext.unwrap())?;

        let mut secrets = Vec::new();
        for line in their_plaintext.split('\n').filter(|l|!l.is_empty()) {
            secrets.push(Secret::parse(line)?);
        }

        Ok(Encrypted(key, salt, secrets))
    }

    pub fn upgrade(self, passwd: impl Fn() -> String, force: bool) -> (Self, bool) {
        match self {
            Encrypted(a, b, c) => {
                if force {
                    let salt = pwhash::gen_salt();
                    let key = derive_key(&salt, passwd);
                    (Encrypted(key,salt,c), true)
                } else {
                    (Encrypted(a,b,c), false)
                }
            },
            Plaintext(a) => {
                let salt = pwhash::gen_salt();
                let key = derive_key(&salt, passwd);
                (Encrypted(key,salt,a), true)
            },
        }
    }

    pub fn to_string(self) -> String {
        match self {
            Encrypted(key, salt, secrets) => {
                let contents = secrets.iter().map(|f|format!("{}\n", f)).collect::<Vec<String>>().join("");
                let nonce = secretbox::gen_nonce();
                let plaintext = contents.as_bytes();
                let ciphertext = secretbox::seal(plaintext, &nonce, &key);

                let salt_str = base64::encode(salt.0);
                let nonce_str = base64::encode(nonce.0);
                let text_str = base64::encode(ciphertext);

                String::from(format!("${salt_str}${nonce_str}${text_str}"))
            },
            Plaintext(secrets) => {
                let contents = secrets.iter().map(|f|format!("{}\n", f)).collect::<Vec<String>>().join("");
                String::from(contents)
            }
        }
    }

    pub fn secrets(&self) -> &Vec<Secret> {
        match self {
            Plaintext(s) => s,
            Encrypted(_, _, s) => s
        }
    }

    pub fn secrets_mut(&mut self) -> &mut Vec<Secret> {
        match self {
            Plaintext(s) => s,
            Encrypted(_, _, s) => s
        }
    }
}

fn derive_key(salt: &Salt, passwd: impl Fn() -> String) -> Key {
    let password = passwd();

    let mut key = Key([0; secretbox::KEYBYTES]);
    {
        let Key(ref mut kb) = key;
        pwhash::derive_key(kb, password.as_bytes(), salt,
                           pwhash::OPSLIMIT_SENSITIVE,
                           pwhash::MEMLIMIT_SENSITIVE).unwrap();
    }

    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_plain_text() {
        let b = Store::from_str("test\tabcd", pw).unwrap();
        assert_eq!(Plaintext(vec![test_secret()]), b);
    }

    #[test]
    fn it_upgrades_plaintext_to_encrypted() {
        let store = Plaintext(Vec::new());
        let (store, _) = store.upgrade(pw, false);
        assert!(matches!(store, Encrypted(_,_,_)));
    }

    fn pw() -> String {
        String::from("password")
    }

    fn test_secret() -> Secret {
        Secret::new("test".to_string(), "abcd".to_string())
    }
}