pub mod command {

    use serde::{Deserialize, Serialize};
    use serialport::prelude::*;
    use std::borrow::BorrowMut;
    use crate::handshake::handshake::{Handshake, AxonMessageStatus, AxonMessageType};
    use crate::serial::serial_handler::SerialData;
    use crate::axonmessage::axonmessage::{AxonMessage, Sendable};
    use std::io::Error;

    pub struct CommandResponse {
        pub status: bool,
        pub pin: i8,
        pub operation: String
    }

    #[derive(Serialize, Deserialize)]
    pub struct Command {
        operation: String,
        command: i8,
        pin: i8,
    }


    impl AxonMessage for Command {}
    impl Sendable for Command {}

    impl CommandResponse {
        fn success(command: &Command) -> Self {
            CommandResponse {
                status: true,
                pin: command.pin,
                operation: command.operation.clone()
            }
        }

        fn failure(command: &Command) -> Self {
            CommandResponse {
                status: false,
                pin: command.pin,
                operation: command.operation.clone()
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
        ) -> Result<CommandResponse, Error> {
            let mut port = SerialData::open_port(settings, &path)?;

            let command = Command {
                operation: operation,
                command: command,
                pin: pin,
            };

            match Handshake::send::<Command>(port.borrow_mut(), &command, AxonMessageType::CommandMessage) {
                Ok(response) => match response {
                    AxonMessageStatus::Success => Ok(CommandResponse::success(&command)),
                    AxonMessageStatus::Failure => Ok(CommandResponse::failure(&command))
                },
                Err(e) => Err(e)
            }
        }
    }
}
