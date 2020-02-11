pub mod record {

    use crate::axonmessage::axonmessage::AxonMessage;
    use crate::serial::serial_handler::SerialData;
    use serialport::prelude::*;
    use crate::handshake::handshake::{Handshake, AxonMessageType};
    use serde::{Serialize, Deserialize};
    use std::io::Error;
    use std::borrow::BorrowMut;

    #[derive(Serialize, Deserialize)]
    enum RecordType {
        Simple = 83,
        Multi = 78
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Record {
        recipient: String,
        data: String,
        record_type: RecordType,
        signer: String,
        device_id: String,
        sensor_name: String,
    }

    impl AxonMessage for Record {}
    impl Record {
        pub fn watch(path: &String, settings: SerialPortSettings) -> Result<Record, Error> {
            let mut port = SerialData::open_port(settings, &path)?;
            match Handshake::recieve::<Record>(port.borrow_mut(), AxonMessageType::Record) {
               Ok(response) => {
                    let record: Record = serde_json::from_str(&response.to_json_string()?)?;
                    Ok(record)
                },
                Err(e) => Err(e)
            }
        }
    }
}