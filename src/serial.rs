pub mod serial_handler {

    use serialport::prelude::*;
    use std::io::Result as SingleResult;
    use std::io::{BufRead, BufReader, BufWriter, Write};
    use std::path::Path;
    pub struct SerialData;

    impl SerialData {
       pub fn read_port(mut port: &mut Box<dyn SerialPort>) -> SingleResult<String> {
            let mut buffer = String::new();
            let mut reader = BufReader::new(&mut port);
            reader.read_line(&mut buffer)?;
            if buffer.contains("\r\n") {
                let buffer = buffer.replace("\r\n", "");
                Ok(buffer)
            } else {
                Ok(buffer)
            }
        }

       pub fn write_port(data: String, mut port: &mut Box<dyn SerialPort>) -> SingleResult<bool> {
            let mut buffer = data.as_bytes();
            // flush output stream before sending anymore data
            port.flush()?;
            let mut writer = BufWriter::new(&mut port);
            writer.write(&mut buffer)?;
            Ok(true)
        }

       pub fn open_port(
            settings: SerialPortSettings,
            path: &String,
        ) -> SingleResult<Box<dyn SerialPort>> {
            let path = Path::new(&path);
            Ok(serialport::open_with_settings(path, &settings)?)
        }
    }
}
