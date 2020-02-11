#[macro_use]
extern crate neon;
extern crate crypto;
extern crate hex;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate serialport;

pub mod bindings;
pub mod command;
pub mod handshake;
pub mod idenity;
pub mod init;
pub mod serial;
pub mod state;
pub mod axonmessage;
pub mod record;

use bindings::bindings::{
    load_identity, serial_read, serial_rw,
    serial_write, save_state, load_state, watch_state,
    axon_init, send_command
};

pub const PARENT_PATH: &'static str = "/axon";
pub const IDENTITY_PATH: &'static str = "/axon/axon-identity.json";
pub const STATE_PATH: &'static str = "/axon/axon-state.json";

register_module!(mut m, {
    m.export_function("sendCommand", send_command)?;
    m.export_function("loadIdentity", load_identity)?;
    m.export_function("readSerial", serial_read)?;
    m.export_function("writeSerial", serial_write)?;
    m.export_function("rwSerial", serial_rw)?;
    m.export_function("saveState", save_state)?;
    m.export_function("loadState", load_state)?;
    m.export_function("watchState", watch_state)?;
    m.export_function("init", axon_init)?; 
    Ok(())
});
