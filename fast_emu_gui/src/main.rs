use eframe::egui;
use std::collections::HashMap;

#[derive(Default)]
struct Register {
    value: u64,
    display_format: DisplayFormat,
    display_precision: u8,
}

impl Register {
    pub fn new(value: u64, display_format: DisplayFormat, display_precision: u8) -> Self {
        Self {
            value,
            display_format,
            display_precision,
        }
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.display_format.format_value(self.value, self.display_precision as usize))
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
            DisplayFormat::Hex => format!("{:0>width$x}", value, width = display_precision),
            DisplayFormat::Binary => format!("{:0>width$b}", value, width = display_precision),
            DisplayFormat::Decimal => format!("{:0>width$}", value, width = display_precision),
            DisplayFormat::Octal => format!("{:0>width$o}", value, width = display_precision),
        }
    }
}

struct Debugger {
    registers: HashMap<String, Register>,
}

impl Debugger {
    fn new() -> Self {
        Self {
            registers: HashMap::new(),
        }
    }
}

impl eframe::App for Debugger {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Debugger");
            for (name, register) in &self.registers {
                ui.label(format!("{name}: {}", register));
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("FastEmuGUI", options, Box::new(|_cc| Box::new(Debugger::new())))
}
