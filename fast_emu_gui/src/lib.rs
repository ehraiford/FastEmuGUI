use crossbeam::channel;
use eframe::egui::{self, TextureOptions};
use frame_buffer::FrameBuffer;
use internal_commands::InternalCommand;
use once_cell::sync::Lazy;
use registers::{DisplayFormat, Register, RegisterSet};
use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, Mutex},
};
mod external_commands;
mod frame_buffer;
mod internal_commands;
mod registers;

#[derive(Default)]
struct EmuData {
    register_sets: HashMap<String, RegisterSet>,
    frame_buffer: Option<FrameBuffer>,
}

impl EmuData {
    pub fn _new() -> Self {
        Self { ..Default::default() }
    }
    pub fn get_mut_register(&mut self, group_name: &str, register_name: &str) -> Option<&mut Register> {
        self.register_sets
            .get_mut(group_name)
            .and_then(|set| set.registers.get_mut(register_name))
    }

    fn run_command(&mut self, command: InternalCommand) {
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

fn test_data() -> EmuData {
    let mut register_sets = HashMap::new();
    let mut registers = HashMap::new();
    registers.insert("R1".to_string(), Register::new(0x1234, DisplayFormat::Hex, 16));
    registers.insert("R2".to_string(), Register::new(0x5678, DisplayFormat::Octal, 16));

    register_sets.insert("General Purpose".to_string(), RegisterSet::new(registers));

    EmuData {
        register_sets,
        frame_buffer: Some(FrameBuffer::new(100, 100)),
    }
}

static EMU_DATA: Lazy<Arc<Mutex<EmuData>>> = Lazy::new(|| Arc::new(Mutex::new(test_data())));

static SENDER: Lazy<channel::Sender<InternalCommand>> = Lazy::new(|| {
    let (sender, receiver): (channel::Sender<InternalCommand>, channel::Receiver<InternalCommand>) =
        channel::unbounded();

    std::thread::spawn(move || {
        while let Ok(command) = receiver.recv() {
            let mut emu_data = EMU_DATA.lock().unwrap();
            emu_data.run_command(command);
        }
    });

    sender
});

struct App {
    state: Arc<Mutex<EmuData>>,
}

impl App {
    fn render_ui(&self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let state = self.state.lock().unwrap();

        for (set_name, set) in &state.register_sets {
            ui.group(|ui| {
                ui.label(set_name);
                ui.separator();
                for string in &set.get_register_strings() {
                    ui.monospace(string);
                }
            });
        }

        if let Some(frame_buffer) = &state.frame_buffer {
            ui.group(|ui| {
                ui.label("Frame Buffer:");
                ui.separator();
                ui.image(&ctx.load_texture(
                    "Frame Buffer",
                    frame_buffer.get_image().clone(),
                    TextureOptions::default(),
                ));
            });
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.separator();
            self.render_ui(ui, ctx);
        });

        ctx.request_repaint();
    }
}

#[derive(Debug)]
enum FastEmuGUIError {
    MismatchedBufferSize { expected: usize, received: usize },
}
impl Display for FastEmuGUIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FastEmuGUIError::MismatchedBufferSize { expected, received } => {
                write!(f, "Received buffer of length: {received}. Expected length: {expected}.")
            },
        }
    }
}
impl std::error::Error for FastEmuGUIError {}
