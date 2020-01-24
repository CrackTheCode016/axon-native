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

use bindings::bindings::{
    axon_init, load_identity, load_state, save_state, send_command, serial_read, serial_rw,
    serial_write, watch_state,
};

pub const PARENT_PATH: &'static str = "/axon";
pub const IDENTITY_PATH: &'static str = "/axon/axon-identity.json";
pub const STATE_PATH: &'static str = "/axon/axon-state.json";

// probably should define my own error types that work across all libraries.

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
