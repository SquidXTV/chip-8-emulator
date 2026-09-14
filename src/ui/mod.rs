pub mod controls;

use crate::ui::controls::CONTROLS_HEIGHT;
use eframe::{Frame, NativeOptions};
use egui::Ui;

const EMULATION_WIDTH: f32 = 640.0;
const EMULATION_HEIGHT: f32 = 320.0;

pub fn run_application() -> eframe::Result {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([EMULATION_WIDTH, EMULATION_HEIGHT + CONTROLS_HEIGHT])
            .with_min_inner_size([EMULATION_WIDTH, EMULATION_HEIGHT + CONTROLS_HEIGHT]),
        ..NativeOptions::default()
    };

    eframe::run_native(
        "CHIP-8 Emulator",
        options,
        Box::new(|cc| Ok(Box::new(EmulatorApplication::new(cc)))),
    )
}

#[derive(Default)]
struct EmulatorApplication {}

impl EmulatorApplication {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for EmulatorApplication {
    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        egui::CentralPanel::default().show(ui, |ui| ui.add(controls::Controls::new()));
    }
}
