pub mod bindings {

    use crate::axonmessage::axonmessage::AxonMessage;
    use crate::command::command::Command;
    use crate::idenity::device_identity::Identity;
    use crate::init::init::AxonInit;
    use crate::record::record::Record;
    use crate::serial::serial_handler::SerialData;
    use crate::state::device_state::State;
    use neon::prelude::*;
    use serialport::prelude::*;
    use std::borrow::BorrowMut;
    use std::time::Duration;

    const SETTINGS: SerialPortSettings = SerialPortSettings {
        baud_rate: 9600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(20000),
    };

    pub fn serial_read(mut cx: FunctionContext) -> JsResult<JsString> {
        let path = cx.argument::<JsString>(0)?.value();
        let mut port =
            SerialData::open_port(SETTINGS, &path).or_else(|e| cx.throw_error(e.to_string()))?;
        let data =
            SerialData::read_port(port.borrow_mut()).or_else(|e| cx.throw_error(e.to_string()))?;
        Ok(cx.string(data))
    }

    pub fn serial_write(mut cx: FunctionContext) -> JsResult<JsBoolean> {
        let path = cx.argument::<JsString>(0)?.value();
        let data = cx.argument::<JsString>(1)?.value();
        let mut port =
            SerialData::open_port(SETTINGS, &path).or_else(|e| cx.throw_error(e.to_string()))?;
        let result = SerialData::write_port(data, port.borrow_mut())
            .or_else(|e| cx.throw_error(e.to_string()))?;
        Ok(cx.boolean(result))
    }

    pub fn serial_rw(mut cx: FunctionContext) -> JsResult<JsObject> {
        let path = cx.argument::<JsString>(0)?.value();
        let data = cx.argument::<JsString>(1)?.value();
        let status_obj = JsObject::new(&mut cx);
        let mut port = SerialData::open_port(SETTINGS, &path).unwrap();
        let write_result = cx.boolean(SerialData::write_port(data, port.borrow_mut()).unwrap());
        let read_result = cx.string(SerialData::read_port(port.borrow_mut()).unwrap());

        status_obj.set(&mut cx, "writeStatus", write_result)?;
        status_obj.set(&mut cx, "response", read_result)?;
        Ok(status_obj)
    }

    pub fn send_command(mut cx: FunctionContext) -> JsResult<JsObject> {
        let path = cx.argument::<JsString>(0)?.value();
        let command = cx.argument::<JsNumber>(1)?.value() as i8;
        let pin = cx.argument::<JsNumber>(2)?.value() as i8;
        let operation = cx.argument::<JsString>(3)?.value();
        let amount = cx.argument::<JsNumber>(4)?.value() as i8;

        let status_obj = JsObject::new(&mut cx);
        let response = Command::send_command(path, SETTINGS, command, pin, amount, operation)
            .or_else(|e| cx.throw_error(e.to_string()))?;

        let write_status = cx.boolean(response.status);
        let pin = cx.number(response.pin);
        let response = cx.string(response.operation);

        status_obj.set(&mut cx, "status", write_status)?;
        status_obj.set(&mut cx, "pinNo", pin)?;
        status_obj.set(&mut cx, "response", response)?;
        Ok(status_obj)
    }

    pub fn load_identity(mut cx: FunctionContext) -> JsResult<JsString> {
        let path = crate::IDENTITY_PATH.to_string();
        let identity =
            Identity::load_identity_from_path(&path).or_else(|e| cx.throw_error(e.to_string()))?;
        Ok(cx.string(identity))
    }

    pub fn save_state(mut cx: FunctionContext) -> JsResult<JsBoolean> {
        let pk = cx.argument::<JsString>(1)?.value();
        let node_ip = cx.argument::<JsString>(2)?.value();
        let gen_hash = cx.argument::<JsString>(3)?.value();
        State::save_state(pk, node_ip, gen_hash, &String::from(crate::STATE_PATH))
            .or_else(|e| cx.throw_error(e.to_string()))?;
        Ok(cx.boolean(true))
    }

    pub fn load_state(mut cx: FunctionContext) -> JsResult<JsString> {
        let path = cx.argument::<JsString>(0)?.value();
        let state = State::load_state(&path).or_else(|e| cx.throw_error(e.to_string()))?;
        Ok(cx.string(state))
    }

    pub fn watch_state(mut cx: FunctionContext) -> JsResult<JsBoolean> {
        let path = cx.argument::<JsString>(0)?.value();
        let status = State::watch_state(&String::from(crate::STATE_PATH), &path, SETTINGS)
            .or_else(|e| cx.throw_error(e.to_string()))?;
        Ok(cx.boolean(status))
    }

    pub fn watch_record(mut cx: FunctionContext) -> JsResult<JsString> {
        let path = cx.argument::<JsString>(0)?.value();
        let record = Record::watch(&path, SETTINGS).or_else(|e| cx.throw_error(e.to_string()))?;
        let record_serialized = record
            .to_json_string()
            .or_else(|e| cx.throw_error(e.to_string()))?;
        Ok(cx.string(record_serialized))
    }

    pub fn axon_init(mut cx: FunctionContext) -> JsResult<JsBoolean> {
        AxonInit::init_fs().or_else(|e| cx.throw_error(e.to_string()))?;
        Ok(cx.boolean(true))
    }
}
