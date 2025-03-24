use eframe::egui;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use std::ffi::CStr;
use std::os::raw::c_char;
use crossbeam::channel;
use once_cell::sync::Lazy;

struct RegisterSet {
    set_name: String,
    display_format: DisplayFormat,
    display_precision: u8,
    registers: HashMap<String, u64>,
}

impl RegisterSet {
    pub fn new(set_name: String, registers: HashMap<String, u64>, display_format: DisplayFormat, display_precision: u8) -> Self {
        Self {
            set_name,
            display_format,
            display_precision,
            registers,
        }
    }
    pub fn get_register_string(self, reg_name: String) -> Option<String> {
        self.registers.get(&reg_name)
        .and_then(|val| Some(self.display_format.format_value(*val, self.display_precision as usize)))
    }
    pub fn get_register_strings(&self) -> Vec<String> {
        self.registers
        .iter()
        .map(|(name, val)| format!("{name}: {}", self.display_format.format_value(*val, self.display_precision as usize))).collect()
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
        RegisterSet::new(
            "General Purpose".to_string(),
            registers,
            DisplayFormat::Hex,
            8,
        ),
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

static SENDER: Lazy<channel::Sender<(String, u64)>> = Lazy::new(|| {
    let (sender, receiver): (channel::Sender<(String, u64)>, channel::Receiver<(String, u64)>) = channel::unbounded();
    
    std::thread::spawn(move || {
        while let Ok((name, value)) = receiver.recv() {
            let mut emu_data = EMU_DATA.lock().unwrap();

            for (_, set) in &mut emu_data.register_sets {
                if let Some(reg) = set.registers.get_mut(&name) {
                    *reg = value;
                }
            }
        }
    });

    sender
});

#[unsafe(no_mangle)]
pub extern "C" fn start_fast_emu_gui() {
    let options = eframe::NativeOptions::default();
    let emu_data = Arc::clone(&EMU_DATA);
    let app = App { state: emu_data };
    let _ = eframe::run_native("FastEmuGUI", options, Box::new(|_cc| Box::new(app)));
}

#[unsafe(no_mangle)]
pub extern "C" fn update_register_value(name: *const c_char, value: u64) {
    let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap();
    let _ = SENDER.send((name.to_string(), value));
}
