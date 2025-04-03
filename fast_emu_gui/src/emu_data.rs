use std::collections::HashMap;

use crate::InternalCommand;
use crate::frame_buffer::FrameBuffer;
use crate::registers::{DisplayFormat, Register, RegisterSet};

#[derive(Default)]
pub struct EmuData {
    name: String,
    frequency: Option<Frequency>,
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
            InternalCommand::UpdateFrameBuffer { buffer } => {
                if let Some(ref mut frame_buffer) = self.frame_buffer {
                    if let Err(error) = frame_buffer.update_frame_buffer(buffer) {
                        println!("{}", error);
                    }
                }
            },
        }
    }
}

pub fn test_data() -> EmuData {
    let mut register_sets = HashMap::new();
    let mut registers = HashMap::new();
    registers.insert("R1".to_string(), Register::new(0x1234, DisplayFormat::Hex, 16));
    registers.insert("R2".to_string(), Register::new(0x5678, DisplayFormat::Octal, 16));

    register_sets.insert("General Purpose".to_string(), RegisterSet::new(registers));

    EmuData {
        name: String::from("FastEmuGUI"),
        register_sets,
        frame_buffer: Some(FrameBuffer::new(100, 100)),
        frequency: None,
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
