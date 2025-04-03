use std::collections::HashMap;
use std::fmt::Display;

use crate::InternalCommand;
use crate::frame_buffer::{FrameBuffer, MutexWrapper};
use crate::registers::{DisplayFormat, Register, RegisterSet};

#[derive(Default)]
pub struct EmuData {
    name: String,
    pub(crate) target_frequency: Option<Frequency>,
    pub(crate) register_sets: HashMap<String, RegisterSet>,
    pub(crate) frame_buffer: Option<FrameBuffer>,
}

impl EmuData {
    pub fn new() -> Self {
        Self { name: "FastEmuGUI".to_string(), ..Default::default() }
    }
    pub(crate) fn get_mut_register(&mut self, group_name: &str, register_name: &str) -> Option<&mut Register> {
        self.register_sets
            .get_mut(group_name)
            .and_then(|set| set.registers.get_mut(register_name))
    }

    pub(crate) fn run_command(&mut self, command: InternalCommand) {
        match command {
            InternalCommand::UpdateRegisterValue { group_name, register_name, value } => {
                if let Some(reg) = self.get_mut_register(&group_name, &register_name) {
                    reg.value = value;
                }
            },
            InternalCommand::UpdateRegisterFormat { group_name, register_name, new_format } => {
                if let Some(reg) = self.get_mut_register(&group_name, &register_name) {
                    reg.update_display_format(new_format);
                }
            },
            InternalCommand::UpdateFrameBuffer { buffer, mutex } => {
                if let Some(ref mut frame_buffer) = self.frame_buffer {
                    if let Err(error) = frame_buffer.update_frame_buffer(buffer, mutex) {
                        println!("{}", error);
                    }
                }
            },
            InternalCommand::SetFrequency(frequency) => self.target_frequency = Some(frequency),
        }
    }
}

#[derive(Debug)]
pub enum Frequency {
    KHz(f32),
    MHz(f32),
    GHz(f32),
}

impl Frequency {
    pub fn deserialize_from_value(value: &serde_yaml::Value) -> Option<Frequency> {
        value
            .get("ClockFrequency")
            .and_then(|value| value.as_str())
            .and_then(Frequency::deserialize_from_string)
    }

    fn deserialize_from_string(string: &str) -> Option<Frequency> {
        if let Some(value) = string.strip_suffix(" KHz") {
            value.parse::<f32>().map(Frequency::KHz).ok()
        } else if let Some(value) = string.strip_suffix(" MHz") {
            value.parse::<f32>().map(Frequency::MHz).ok()
        } else if let Some(value) = string.strip_suffix(" GHz") {
            value.parse::<f32>().map(Frequency::GHz).ok()
        } else {
            None
        }
    }
}

impl Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Frequency::KHz(val) => write!(f, "{val} KHz"),
            Frequency::MHz(val) => write!(f, "{val} MHz"),
            Frequency::GHz(val) => write!(f, "{val} GHz"),
        }
    }
}
