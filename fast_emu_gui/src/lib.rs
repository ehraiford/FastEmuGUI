use crossbeam::channel;
use eframe::egui;
use internal_commands::InternalCommand;
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
pub mod external_commands;
pub mod internal_commands;

struct Register {
    value: u64,
    display_format: DisplayFormat,
    bit_width: u8,
    necessary_precision_for_format: u8,
}

impl Register {
    pub fn new(value: u64, display_format: DisplayFormat, bit_width: u8) -> Self {
        Self {
            value,
            necessary_precision_for_format: display_format.get_required_display_width(bit_width),
            display_format,
            bit_width,
        }
    }
    pub fn update_display_format(&mut self, new_format: DisplayFormat) {
        self.necessary_precision_for_format = new_format.get_required_display_width(self.bit_width);
        self.display_format = new_format;
    }
}

impl Default for Register {
    fn default() -> Self {
        Self {
            value: Default::default(),
            display_format: DisplayFormat::Hex,
            bit_width: 8,
            necessary_precision_for_format: 2,
        }
    }
}

struct RegisterSet {
    registers: HashMap<String, Register>,
}

impl RegisterSet {
    pub fn new(registers: HashMap<String, Register>) -> Self {
        Self { registers }
    }
    pub fn get_register_strings(&self) -> Vec<String> {
        self.registers
            .iter()
            .map(|(name, reg)| {
                format!(
                    "{name}: {}",
                    reg.display_format
                        .format_value(reg.value, reg.necessary_precision_for_format as usize)
                )
            })
            .collect()
    }
}

#[derive(Default)]
#[repr(C)]
pub enum DisplayFormat {
    #[default]
    Hex,
    Binary,
    Decimal,
    Octal,
}

impl DisplayFormat {
    pub fn format_value(&self, value: u64, display_precision: usize) -> String {
        match self {
            DisplayFormat::Hex => format!("0x{:0>width$x}", value, width = display_precision),
            DisplayFormat::Binary => format!("0b{:0>width$b}", value, width = display_precision),
            DisplayFormat::Decimal => format!("{:0>width$}", value, width = display_precision),
            DisplayFormat::Octal => format!("0o{:0>width$o}", value, width = display_precision),
        }
    }
    /// Returns the number of characters needed to display the max value of a register of the given width.
    pub fn get_required_display_width(&self, bit_width: u8) -> u8 {
        match self {
            DisplayFormat::Hex => bit_width.div_ceil(4),
            DisplayFormat::Binary => bit_width,
            DisplayFormat::Decimal => bit_width.div_ceil(3),
            DisplayFormat::Octal => bit_width.div_ceil(3),
        }
    }
}

struct EmuData {
    register_sets: HashMap<String, RegisterSet>,
}

impl EmuData {
    pub fn _new() -> Self {
        Self {
            register_sets: HashMap::new(),
        }
    }
    pub fn get_mut_register(
        &mut self,
        group_name: &str,
        register_name: &str,
    ) -> Option<&mut Register> {
        self.register_sets
            .get_mut(group_name)
            .and_then(|set| set.registers.get_mut(register_name))
    }
}

fn test_data() -> EmuData {
    let mut register_sets = HashMap::new();
    let mut registers = HashMap::new();
    registers.insert(
        "R1".to_string(),
        Register::new(0x1234, DisplayFormat::Hex, 16),
    );
    registers.insert(
        "R2".to_string(),
        Register::new(0x5678, DisplayFormat::Octal, 16),
    );

    register_sets.insert("General Purpose".to_string(), RegisterSet::new(registers));

    EmuData { register_sets }
}

struct App {
    state: Arc<Mutex<EmuData>>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Registers");
            let state = self.state.lock().unwrap();
            for (set_name, set) in &state.register_sets {
                ui.label(set_name);
                for string in &set.get_register_strings() {
                    ui.label(string);
                }
            }
        });

        ctx.request_repaint();
    }
}

static EMU_DATA: Lazy<Arc<Mutex<EmuData>>> = Lazy::new(|| Arc::new(Mutex::new(test_data())));

static SENDER: Lazy<channel::Sender<InternalCommand>> = Lazy::new(|| {
    let (sender, receiver): (
        channel::Sender<InternalCommand>,
        channel::Receiver<InternalCommand>,
    ) = channel::unbounded();

    std::thread::spawn(move || {
        while let Ok(command) = receiver.recv() {
            let mut emu_data = EMU_DATA.lock().unwrap();

            match command {
                InternalCommand::UpdateRegisterValue {
                    group_name,
                    register_name,
                    value,
                } => {
                    if let Some(reg) = emu_data.get_mut_register(&group_name, &register_name) {
                        reg.value = value;
                    }
                }

                InternalCommand::UpdateRegisterFormat {
                    group_name,
                    register_name,
                    new_format,
                } => {
                    if let Some(reg) = emu_data.get_mut_register(&group_name, &register_name) {
                        reg.update_display_format(new_format);
                    }
                }
            }
        }
    });

    sender
});
