pub mod device_identity {

    use crypto::ed25519;
    use hex;
    use rand::{os, Rng};
    use serde::{Deserialize, Serialize};
    use std::collections::hash_map::DefaultHasher;
    use std::fs::File;
    use std::hash::{Hash, Hasher};
    use std::io::prelude::*;
    use std::io::Result as SingleResult;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

    #[derive(Serialize, Deserialize)]
    pub struct Identity {
        key: String,
        identifer: String,
    }
    impl Identity {
        pub fn generate_uuid() -> SingleResult<String> {
            let file = File::open("/proc/cpuinfo")?;
            let mut hasher = DefaultHasher::new();
            let reader = BufReader::new(file);
            let serial: String = reader.lines().last().unwrap()?;
            println!("{}", serial);
            serial.hash(&mut hasher);
            Ok(hasher.finish().to_string())
        }

        pub fn generate_private_key() -> SingleResult<String> {
            let mut os_rng = os::OsRng::new()?;
            let mut seed: [u8; 8] = [0; 8];
            os_rng.fill_bytes(&mut seed);
            let keypair: ([u8; 64], [u8; 32]) = ed25519::keypair(&seed);
            let mut pk_vec = keypair.0.to_vec();
            // a tad smelly..
            pk_vec.truncate(32);
            Ok(hex::encode_upper(pk_vec))
        }

        pub fn generate_identity() -> SingleResult<Identity> {
            let pk = Self::generate_private_key()?;
            let uuid = Self::generate_uuid()?;

            Ok(Identity {
                key: pk,
                identifer: uuid,
            })
        }

        pub fn check_identity(path: &String) -> SingleResult<bool> {
            Ok(Path::new(&path).exists())
        }

        pub fn load_identity_from_path(path: &String) -> SingleResult<String> {
            let mut identity_file = File::open(path)?;
            let mut identity_json_string = String::new();
            identity_file.read_to_string(&mut identity_json_string)?;
            Ok(identity_json_string)
        }

        pub fn create_identity(path: &String) -> SingleResult<()> {
            let mut identity_file = File::create(path)?;
            let mut perms = identity_file.metadata()?.permissions();
            let identity = Self::generate_identity()?;
            let idenity_json = serde_json::to_string(&identity)?;
            perms.set_readonly(true);
            identity_file.write_all(idenity_json.as_bytes())?;
            identity_file.set_permissions(perms)?;
            Ok(())
        }

        pub fn identity(path: String) -> SingleResult<String> {
            match Identity::check_identity(&path) {
                Ok(true) => Identity::load_identity_from_path(&path),
                Ok(false) => {
                    Identity::create_identity(&path)?;
                    Identity::load_identity_from_path(&path)
                }
                Err(err) => Err(err),
            }
        }
    }
}
