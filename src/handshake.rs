pub mod handshake {
    use crate::axonmessage::axonmessage::{AxonMessage, Message, Sendable};
    use crate::serial::serial_handler::SerialData;
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};
    use serde_repr::*;
    use serialport::prelude::*;
    use std::borrow::BorrowMut;
    use std::io::{Error, ErrorKind};

    const HANDSHAKE_PREFIX: char = 'H';

    #[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
    #[repr(i16)]
    pub enum AxonHandshakeType {
        HandshakeConnect = 18499,
        HandshakeAccept = 18497,
    }

    #[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
    #[repr(i8)]
    pub enum AxonMessageType {
        RecordMessage = 0,
        StateMessage = 1,
        CommandMessage = 2,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct HandshakeRequest {
        handshake_type: AxonHandshakeType,
        message_type: AxonMessageType,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct HandshakeResponse {
        handshake_type: AxonHandshakeType,
    }

    pub enum AxonMessageStatus {
        Success = 0,
        Failure = 1,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Handshake;

    impl AxonMessage for HandshakeResponse {}
    impl AxonMessage for HandshakeRequest {}

    impl Handshake {
        fn check_type_from_str<T>(data: &str) -> bool
        where
            T: DeserializeOwned,
        {
            serde_json::from_str::<T>(&data).is_ok()
        }

        pub fn recieve<T: AxonMessage>(
            port: &mut Box<dyn SerialPort>,
            message_type: AxonMessageType,
        ) -> Result<Message<T>, Error>
        where
            T: serde::de::DeserializeOwned,
        {
            let accept = HandshakeResponse {
                handshake_type: AxonHandshakeType::HandshakeAccept,
            };
            println!("{:?}", message_type);
            let data: String = SerialData::read_port(port.borrow_mut())?;

            let mut accept_stringifed = String::new();
            accept_stringifed.push(HANDSHAKE_PREFIX);
            accept_stringifed.push_str(&accept.to_json_string()?);
            if Self::check_type_from_str::<HandshakeRequest>(&data) {
                let parsed: Result<HandshakeRequest, serde_json::Error> =
                    serde_json::from_str(&data);
                    println!("RECIEVED SOME REQUEST {:?}", parsed);
                match parsed {
                    Ok(result) => match result.handshake_type {
                        AxonHandshakeType::HandshakeConnect => {
                            println!("{}, {:?}, {:?}", result.message_type == message_type, result.message_type, message_type);
                            if result.message_type == message_type {
                                SerialData::write_port(accept_stringifed, port.borrow_mut())?;
                                Ok(loop {
                                    let data = SerialData::read_port(port.borrow_mut())?;
                                    println!("Now we wait {}", data);
                                    if Self::check_type_from_str::<T>(&data) {
                                        let message: Message<T> = serde_json::from_str(&data)?;
                                        break message;
                                    }
                                })
                            } else {
                                Ok(Message::Empty)
                            }
                        }
                        _ => Ok(Message::Empty),
                    },
                    Err(_) => Err(Error::from(ErrorKind::InvalidData)),
                }
            } else {
                Ok(Message::Log {
                    status: 0,
                    data: data,
                })
            }
        }

        pub fn send<T: AxonMessage + Sendable>(
            port: &mut Box<dyn SerialPort>,
            sendable: &T,
            prefix_byte: char,
            message_type: AxonMessageType,
        ) -> Result<AxonMessageStatus, Error> {
            let connect = HandshakeRequest {
                handshake_type: AxonHandshakeType::HandshakeConnect,
                message_type: message_type,
            };
            let mut connect_stringifed = String::new();
            connect_stringifed.push(HANDSHAKE_PREFIX);
            connect_stringifed.push_str(&connect.to_json_string()?);
            SerialData::write_port(connect_stringifed, port.borrow_mut())?;
            Ok(loop {
                let data = SerialData::read_port(port.borrow_mut())?;
                let result: Result<HandshakeResponse, ()> = serde_json::from_str(&data)?;
                match result {
                    Ok(response) => match response.handshake_type {
                        AxonHandshakeType::HandshakeAccept => {
                            let mut sendable_stringified = String::new();
                            sendable_stringified.push(prefix_byte);
                            sendable_stringified.push_str(&sendable.to_json_string()?);
                            SerialData::write_port(sendable_stringified, port.borrow_mut())?;
                            break AxonMessageStatus::Success;
                        }
                        _ => (),
                    },
                    Err(_) => (),
                };
            })
        }
    }
}

#[cfg(test)]
mod tests {}
