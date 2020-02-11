pub mod device_state {
    use crate::axonmessage::axonmessage::AxonMessage;
    use crate::handshake::handshake::{AxonMessageType, Handshake};
    use serde::{Deserialize, Serialize};
    use crate::serial::serial_handler::SerialData;
    use serialport::prelude::*;
    use std::borrow::BorrowMut;
    use std::fs;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::io::Result as SingleResult;
    use std::path::Path;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct State {
        pub user_private_key: String,
        pub node_ip: String,
        pub gen_hash: String,
    }

    impl AxonMessage for State {}

    impl State {
        pub fn exists() -> bool {
            Path::new(crate::STATE_PATH).exists()
        }

        pub fn init_state(parent_path: &String, state_path: &String) -> SingleResult<()> {
            fs::create_dir_all(parent_path)?;
            let empty_state = State {
                user_private_key: String::new(),
                node_ip: String::new(),
                gen_hash: String::new(),
            };
            let mut state_file = File::create(state_path)?;
            state_file.write_all(serde_json::to_string(&empty_state)?.as_bytes())?;
            Ok(())
        }

        pub fn save_state(
            pk: String,
            node_ip: String,
            gen_hash: String,
            path: &String,
        ) -> SingleResult<String> {
            let mut state_file = OpenOptions::new().write(true).open(path)?;
            let new_state = State {
                user_private_key: pk,
                node_ip: node_ip,
                gen_hash: gen_hash,
            };
            let state_json = serde_json::to_string(&new_state)?;
            state_file.write_all(state_json.as_bytes())?;
            Ok(state_json)
        }

        pub fn load_state(path: &String) -> SingleResult<String> {
            let mut state_file = File::open(path)?;
            let mut state_string = String::new();
            state_file.read_to_string(&mut state_string)?;
            Ok(state_string)
        }

        pub fn watch_state(
            state_path: &String,
            path: &String,
            settings: SerialPortSettings,
        ) -> SingleResult<bool> {
            let mut port = SerialData::open_port(settings, &path)?;
            match Handshake::recieve::<State>(port.borrow_mut(), AxonMessageType::State) {
                Ok(response) => {
                    let state: State = serde_json::from_str(&response.to_json_string()?)?;
                    State::save_state(
                        state.user_private_key,
                        state.node_ip,
                        state.gen_hash,
                        state_path,
                    )?;
                    Ok(true)
                }
                Err(_) => Ok(false),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use serde_json;
    use crate::state::device_state::State;
    use std::path::PathBuf;

    #[test]
    fn load_state() {
        let mut test_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("resources/test_files/state/load_state.json");
        let path_as_str = &String::from(test_path.to_str().unwrap());
        let state_config: State = State {
            user_private_key: String::from(
                "3485D98EFD7EB07ADAFCFD1A157D89DE2796A95E780813C0258AF3F5F84ED8CB",
            ),
            node_ip: String::from("http://198.199.80.167:3000"),
            gen_hash: String::from(
                "B626827FBD912D95931E03E9718BFE8FFD7D316E9FBB5416ED2B3C072EA32406",
            ),
        };
        let state_as_str = serde_json::to_string(&state_config).unwrap();
        let loaded_state = State::load_state(path_as_str);
        assert_eq!(loaded_state.is_ok(), true);
        assert_eq!(state_as_str, loaded_state.unwrap());
    }

    #[test]
    fn save_state() {
        let mut test_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("resources/test_files/state/save_state.json");
        let path_as_str = &String::from(test_path.to_str().unwrap());
        let mut rng = rand::thread_rng();
        let rando = rng.gen::<i32>();
        let ip = format!("http://{}.{}.{}.{}:3000", rando, rando, rando, rando);

        let new_state_config: State = State {
            user_private_key: String::from(
                "B626827FBD912D95931E03E9718BFE8FFD7D316E9FBB5416ED2B3C072EA32406",
            ),
            node_ip: ip,
            gen_hash: String::from(
                "8985D98EFD7EB07ADAFCFD1A157D89DE2796A95E780813C0258AF3F5F84ED8CB",
            ),
        };

        let new_config_as_str = serde_json::to_string(&new_state_config).unwrap();
        let saved_state = State::save_state(
            new_state_config.user_private_key,
            new_state_config.node_ip,
            new_state_config.gen_hash,
            path_as_str,
        );
        assert_eq!(saved_state.is_ok(), true);
        assert_eq!(saved_state.unwrap(), new_config_as_str);
    }

    #[test]
    fn init_state() {
        let empty_state = State {
            user_private_key: String::new(),
            node_ip: String::new(),
            gen_hash: String::new(),
        };

        let empty_state_as_str = serde_json::to_string(&empty_state).unwrap();

        let mut root: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        root.push("resources/test_files/state/parent_state");
        let parent_path_as_str = String::from(root.to_str().unwrap());
        root.push("state.json");
        let state_path_as_str = String::from(root.to_str().unwrap());

        println!("{} {}", parent_path_as_str, state_path_as_str);
        let init = State::init_state(&parent_path_as_str, &state_path_as_str);
        assert_eq!(init.is_ok(), true);
        assert_eq!(
            State::load_state(&state_path_as_str).unwrap(),
            empty_state_as_str
        );
    }
}
