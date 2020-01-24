pub mod bindings {

    use command::command::Command;
    use init::init::AxonInit;
    use idenity::device_identity::Identity;
    use neon::prelude::*;
    use serial::serial_handler::SerialData;
    use serialport::prelude::*;
    use state::device_state::State;
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
        let mut port = SerialData::open_port(SETTINGS, &path).unwrap();
        let data = SerialData::read_port(port.borrow_mut()).unwrap();
        Ok(cx.string(data))
    }

    pub fn serial_write(mut cx: FunctionContext) -> JsResult<JsBoolean> {
        let path = cx.argument::<JsString>(0)?.value();
        let data = cx.argument::<JsString>(1)?.value();
        let mut port = SerialData::open_port(SETTINGS, &path).unwrap();
        let result = SerialData::write_port(data, port.borrow_mut()).unwrap();
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

        let status_obj = JsObject::new(&mut cx);
        let response = Command::send_command(path, SETTINGS, command, pin, operation)
            .or_else(|e| cx.throw_error(e.to_string()))?;

        let write_status = cx.boolean(response.status);
        let pin = cx.number(response.pin);
        let response = cx.string(response.response);

        status_obj.set(&mut cx, "status", write_status)?;
        status_obj.set(&mut cx, "pinNo", pin)?;
        status_obj.set(&mut cx, "response", response)?;
        Ok(status_obj)
    }

    pub fn load_identity(mut cx: FunctionContext) -> JsResult<JsString> {
        let path = ::IDENTITY_PATH.to_string();
        let identity = Identity::load_identity_from_path(&path).or_else(|e| cx.throw_error(e.to_string()))?;
        Ok(cx.string(identity))
    }

    pub fn save_state(mut cx: FunctionContext) -> JsResult<JsBoolean> {
        let pk = cx.argument::<JsString>(1)?.value();
        let node_ip = cx.argument::<JsString>(2)?.value();
        let gen_hash = cx.argument::<JsString>(3)?.value();
        State::save_state(pk, node_ip, gen_hash)
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
        let status = State::watch_state(&path, SETTINGS).or_else(|e| cx.throw_error(e.to_string()))?;
        Ok(cx.boolean(status))
    }

    pub fn axon_init(mut cx: FunctionContext) -> JsResult<JsBoolean> {
        AxonInit::init_fs().or_else(|e| cx.throw_error(e.to_string()))?;
        Ok(cx.boolean(true))
    }
}
