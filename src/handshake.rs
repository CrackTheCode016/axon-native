pub mod handshake {
    use serde::{Deserialize, Serialize};
    use serial::serial_handler::SerialData;
    use serialport::prelude::*;
    use std::borrow::BorrowMut;
    use std::io::Result as SingleResult;

    #[derive(Serialize, Deserialize)]
    pub enum HandshakeMessage {
        Connect = 0x01,
        ConnectionAccepted = 0x02,
        StateInit = 0x03,
        Unknown = 0x04,
    }

    impl HandshakeMessage {
        pub fn from_str(x: &str) -> HandshakeMessage {
            match x {
                "H1" => HandshakeMessage::Connect,
                "H2" => HandshakeMessage::ConnectionAccepted,
                "SI" => HandshakeMessage::StateInit,
                _ => HandshakeMessage::Unknown,
            }
        }
    }

    pub struct Handshake;
    impl Handshake {
        pub fn handshake(port: &mut Box<dyn SerialPort>) -> SingleResult<HandshakeMessage> {
            let message = format!("H{}", HandshakeMessage::Connect as i8);
            SerialData::write_port(message, port.borrow_mut())?;
            let result = loop {
                let data: String = SerialData::read_port(port.borrow_mut())?;
                if data.contains("H2") {
                    let handshake_response = HandshakeMessage::ConnectionAccepted;
                    break handshake_response;
                }
            };
            Ok(result)
        }
    }
}
