use crate::frame_buffer::MutexWrapper;
use crate::internal_commands::InternalCommand;
use crate::{App, DisplayFormat, EMU_DATA, SENDER};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Arc;

#[unsafe(no_mangle)]
pub extern "C" fn start_fast_emu_gui() {
    let options = eframe::NativeOptions::default();
    let emu_data = Arc::clone(&EMU_DATA);
    let app = App { state: emu_data };
    let _ = eframe::run_native("FastEmuGUI", options, Box::new(|_cc| Ok(Box::new(app))));
}

#[unsafe(no_mangle)]
pub extern "C" fn update_register_value(group_name: *const c_char, register_name: *const c_char, value: u64) {
    let group_name = unsafe { CStr::from_ptr(group_name) }.to_str().unwrap();
    let register_name = unsafe { CStr::from_ptr(register_name) }.to_str().unwrap();
    let _ = SENDER.send(InternalCommand::UpdateRegisterValue {
        group_name: group_name.to_string(),
        register_name: register_name.to_string(),
        value: value,
    });
}
#[unsafe(no_mangle)]
pub extern "C" fn update_register_format(
    group_name: *const c_char,
    register_name: *const c_char,
    new_format: DisplayFormat,
) {
    let group_name = unsafe { CStr::from_ptr(group_name) }.to_str().unwrap();
    let register_name = unsafe { CStr::from_ptr(register_name) }.to_str().unwrap();
    let _ = SENDER.send(InternalCommand::UpdateRegisterFormat {
        group_name: group_name.to_string(),
        register_name: register_name.to_string(),
        new_format: new_format,
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn update_frame_buffer_with_pointer(data: *const u8, len: usize, mutex: MutexWrapper) {
    let buffer = unsafe { std::slice::from_raw_parts(data, len) };
    let _ = SENDER.send(InternalCommand::UpdateFrameBuffer { buffer, mutex });
}
