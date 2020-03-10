pub mod record {

    use crate::axonmessage::axonmessage::AxonMessage;
    use crate::handshake::handshake::{AxonMessageType, Handshake};
    use crate::serial::serial_handler::SerialData;
    use serde::{Deserialize, Serialize};
    use serde_repr::*;
    use serialport::prelude::*;
    use std::borrow::BorrowMut;
    use std::io::Error;

    #[derive(Serialize_repr, Deserialize_repr, Debug)]
    #[repr(i8)]
    enum RecordType {
        Simple = 83,
        Multi = 78,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Record {
        node: String,
        recipient: String,
        data: String,
        record_type: RecordType,
        device_id: String,
        sensor_name: String,
        encrypted: bool
    }

    impl AxonMessage for Record {}
    impl Record {
        pub fn watch(path: &String, settings: SerialPortSettings) -> Result<Record, Error> {
            let mut port = SerialData::open_port(settings, &path)?;
            match Handshake::recieve::<Record>(port.borrow_mut(), AxonMessageType::RecordMessage) {
                Ok(response) => {
                    let record: Record = serde_json::from_str(&response.to_json_string()?)?;
                    println!("Some record {:?}", record);
                    Ok(record)
                }
                Err(e) => Err(e),
            }
        }
    }
}
