pub mod device_state {
    use handshake::handshake::{Handshake, HandshakeMessage};
    use serde::{Deserialize, Serialize};
    use serial::serial_handler::SerialData;
    use serialport::prelude::*;
    use std::borrow::BorrowMut;
    use std::fs;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::io::Result as SingleResult;
    use std::io::{Error, ErrorKind};
    use std::path::Path;

    #[derive(Serialize, Deserialize)]
    pub struct State {
        pub user_private_key: String,
        pub node_ip: String,
        pub gen_hash: String,
    }

    impl State {
        pub fn exists() -> SingleResult<bool> {
            Ok(Path::new(::STATE_PATH).exists())
        }

        pub fn init_state() -> SingleResult<()> {
            fs::create_dir_all(::PARENT_PATH)?;
            let empty_state = State {
                user_private_key: String::new(),
                node_ip: String::new(),
                gen_hash: String::new(),
            };
            let mut state_file = File::create(::STATE_PATH)?;
            state_file.write_all(serde_json::to_string(&empty_state)?.as_bytes())?;
            Ok(())
        }

        pub fn save_state(pk: String, node_ip: String, gen_hash: String) -> SingleResult<()> {
            let mut state_file = OpenOptions::new().write(true).open(::STATE_PATH)?;
            let new_state = State {
                user_private_key: pk,
                node_ip: node_ip,
                gen_hash: gen_hash,
            };
            let state_json = serde_json::to_string(&new_state)?;
            state_file.write_all(state_json.as_bytes())?;
            Ok(())
        }

        pub fn load_state(path: &String) -> SingleResult<String> {
            let mut state_file = File::open(path)?;
            let mut state_string = String::new();
            state_file.read_to_string(&mut state_string)?;
            Ok(state_string)
        }

        pub fn watch_state(path: &String, settings: SerialPortSettings) -> SingleResult<bool> {
            let mut port = SerialData::open_port(settings, &path)?;
            loop {
                let data = SerialData::read_port(port.borrow_mut())?;
                if data.contains("I1") {
                    println!("Data found! {}", data);
                    match Handshake::handshake(port.borrow_mut()) {
                        Ok(response) => match response {
                            HandshakeMessage::ConnectionAccepted => {
                                let data = SerialData::read_port(port.borrow_mut())?;
                                println!("Data found! {}", data);
                                if data.contains("SI") {
                                    let state: State =
                                        serde_json::from_str(&data.replace("SI", ""))?;
                                    State::save_state(
                                        state.user_private_key,
                                        state.node_ip,
                                        state.gen_hash,
                                    )?;
                                }
                            }

                            HandshakeMessage::Unknown => (),
                            _ => (),
                        },

                        Err(err) => (),
                    }
                    break Ok(true);
                }
                break Ok(false);
            }
        }
    }
}
