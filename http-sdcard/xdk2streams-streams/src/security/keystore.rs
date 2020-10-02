use crypto::digest::Digest;
use crypto::sha3::Sha3;
use serde::{Deserialize, Serialize};

use std::fs::File;

#[derive(Debug, Deserialize, Serialize)]
pub struct Keystore {
    pub api_key_author: String,
}

#[derive(Debug)]
pub struct KeyManager {
    pub keystore: Keystore,
}

impl KeyManager {
    pub fn new(new_key_aut: String) -> KeyManager {
        let keystore = Keystore {
            api_key_author: calculate_hash(new_key_aut),
        };

        store_keystore(&keystore);

        KeyManager { keystore: keystore }
    }

    pub fn restore() -> KeyManager {
        let rec: Keystore =
            serde_json::from_reader(File::open("src/security/keystore.json").unwrap()).unwrap();
        KeyManager { keystore: rec }
    }
}

pub fn store_keystore(keystore: &Keystore) -> () {
    serde_json::to_writer(
        &File::create("src/security/keystore.json").unwrap(),
        keystore,
    )
    .unwrap();
}

pub fn calculate_hash(t: String) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(&t);
    let hex = hasher.result_str();
    hex
}
