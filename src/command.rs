pub mod command {

    use handshake::handshake::{Handshake, HandshakeMessage};
    use serde::{Deserialize, Serialize};
    use serial::serial_handler::SerialData;
    use serialport::prelude::*;
    use std::borrow::BorrowMut;
    use std::io::Result as SingleResult;
    use std::io::{Error, ErrorKind};

    pub struct CommandResponse {
        pub status: bool,
        pub pin: i8,
        pub response: String,
    }

    #[derive(Serialize, Deserialize)]
    pub enum SensorCommand {
        ON = 0x01,
        OFF = 0x0,
        UNKNOWN = 0x10,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Command {
        operation: String,
        command: i8,
        pin: i8,
    }

    impl SensorCommand {
        pub fn from_i8(command: i8) -> SensorCommand {
            match command {
                0 => SensorCommand::OFF,
                1 => SensorCommand::ON,
                _ => SensorCommand::UNKNOWN,
            }
        }
    }

    impl Command {
        pub fn send_command(
            path: String,
            settings: SerialPortSettings,
            command: i8,
            pin: i8,
            operation: String,
        ) -> SingleResult<CommandResponse> {
            let mut port = SerialData::open_port(settings, &path)?;

            let command = Command {
                operation: operation,
                command: command,
                pin: pin,
            };

            match Handshake::handshake(port.borrow_mut()) {
                Ok(response) => match response {
                    HandshakeMessage::ConnectionAccepted => {
                        let command_str = format!("C{}", serde_json::to_string(&command)?);
                        println!("{}", command_str);
                        let write_status = SerialData::write_port(command_str, port.borrow_mut())?;
                        let response = SerialData::read_port(port.borrow_mut())?;
                        Ok(CommandResponse {
                            status: write_status,
                            pin: command.pin,
                            response: response,
                        })
                    }
                    HandshakeMessage::Unknown => Err(Error::from(ErrorKind::NotConnected)),
                    _ => Err(Error::from(ErrorKind::NotConnected)),
                },

                Err(_) => Err(Error::from(ErrorKind::NotConnected)),
            }
        }
    }
}
