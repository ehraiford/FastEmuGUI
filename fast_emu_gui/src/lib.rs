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

struct RegisterSet {
    display_format: DisplayFormat,
    display_precision: u8,
    registers: HashMap<String, u64>,
}

impl RegisterSet {
    pub fn new(
        registers: HashMap<String, u64>,
        display_format: DisplayFormat,
        display_precision: u8,
    ) -> Self {
        Self {
            display_format,
            display_precision,
            registers,
        }
    }
    pub fn get_register_strings(&self) -> Vec<String> {
        self.registers
            .iter()
            .map(|(name, val)| {
                format!(
                    "{name}: {}",
                    self.display_format
                        .format_value(*val, self.display_precision as usize)
                )
            })
            .collect()
    }
}

#[derive(Default)]
enum DisplayFormat {
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
}

struct EmuData {
    register_sets: HashMap<String, RegisterSet>,
}

impl EmuData {
    fn new() -> Self {
        Self {
            register_sets: HashMap::new(),
        }
    }
}

fn test_data() -> EmuData {
    let mut register_sets = HashMap::new();
    let mut registers = HashMap::new();
    registers.insert("R1".to_string(), 0x1234);
    registers.insert("R2".to_string(), 0x5678);

    register_sets.insert(
        "General Purpose".to_string(),
        RegisterSet::new(registers, DisplayFormat::Hex, 8),
    );

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
                    let Some(register) = emu_data
                        .register_sets
                        .get_mut(&group_name)
                        .and_then(|set| set.registers.get_mut(&register_name))
                    else {
                        return;
                    };
                    *register = value;
                }
            }
        }
    });

    sender
});
