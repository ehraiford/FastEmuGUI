use crossbeam::channel;
use eframe::egui::{self, TextureOptions};
use emu_data::{EmuData, test_data};
use internal_commands::InternalCommand;
use once_cell::sync::Lazy;
use registers::DisplayFormat;
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};
mod emu_data;
pub mod external_commands;
pub mod frame_buffer;
pub mod internal_commands;
pub mod registers;
pub mod yaml;

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
    IOError { message: String },
    SerdeError { message: String },
}
impl std::error::Error for FastEmuGUIError {}

impl Display for FastEmuGUIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FastEmuGUIError::MismatchedBufferSize { expected, received } => {
                write!(f, "Received buffer of length: {received}. Expected length: {expected}.")
            },
            FastEmuGUIError::IOError { message } | FastEmuGUIError::SerdeError { message } => write!(f, "{}", message),
        }
    }
}

use std::convert::From;
impl From<std::io::Error> for FastEmuGUIError {
    fn from(error: std::io::Error) -> Self {
        FastEmuGUIError::IOError { message: format!("{}", error) }
    }
}
impl From<serde_yaml::Error> for FastEmuGUIError {
    fn from(error: serde_yaml::Error) -> Self {
        FastEmuGUIError::SerdeError { message: format!("{}", error) }
    }
}
