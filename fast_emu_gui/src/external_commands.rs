use crate::internal_commands::InternalCommand;
use crate::{App, EMU_DATA, SENDER};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Arc;

#[unsafe(no_mangle)]
pub extern "C" fn start_fast_emu_gui() {
    let options = eframe::NativeOptions::default();
    let emu_data = Arc::clone(&EMU_DATA);
    let app = App { state: emu_data };
    let _ = eframe::run_native("FastEmuGUI", options, Box::new(|_cc| Box::new(app)));
}

#[unsafe(no_mangle)]
pub extern "C" fn update_register_value(
    group_name: *const c_char,
    register_name: *const c_char,
    value: u64,
) {
    let group_name = unsafe { CStr::from_ptr(group_name) }.to_str().unwrap();
    let register_name = unsafe { CStr::from_ptr(register_name) }.to_str().unwrap();
    let _ = SENDER.send(InternalCommand::UpdateRegisterValue {
        group_name: group_name.to_string(),
        register_name: register_name.to_string(),
        value: value,
    });
}
