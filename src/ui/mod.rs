use eframe::{Frame, NativeOptions};
use eframe::epaint::text::TextWrapMode;
use egui::{vec2, Button, Panel, Ui, CentralPanel, Sense, Rect, CornerRadius, Color32};
use crate::emulator::chip::Chip;
use crate::emulator::display;

const EMULATION_WIDTH: f32 = 640.0;
const EMULATION_HEIGHT: f32 = 320.0;

pub fn run_application() -> eframe::Result {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([EMULATION_WIDTH, EMULATION_HEIGHT])
            .with_min_inner_size([EMULATION_WIDTH, EMULATION_HEIGHT]),
        ..NativeOptions::default()
    };

    eframe::run_native(
        "CHIP-8 Emulator",
        options,
        Box::new(|cc| Ok(Box::new(EmulatorApplication::new(cc)))),
    )
}

struct EmulatorApplication {
    chip: Chip
}

impl EmulatorApplication {

    fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self {
            chip: Chip::new()
        }
    }

}

impl eframe::App for EmulatorApplication {
    fn ui(&mut self, ui: &mut Ui, _: &mut Frame) {
        Panel::top("header")
            .default_size(32.0)
            .show(ui, |ui| {
                Panel::left("load")
                    .resizable(false)
                    .show(ui, |ui| {
                        let available_size = ui.available_size();

                        if ui.add_sized(available_size, Button::new("Load")).clicked() {
                            let Some(path) = rfd::FileDialog::new().add_filter("CHIP-8 ROM", &["ch8"]).pick_file() else {
                                return;
                            };

                            match std::fs::read(&path) {
                                Ok(bytes) => {
                                    // todo: error response
                                    self.chip.load(&bytes);
                                },
                                Err(error) => {
                                    // todo: error response
                                    println!("{}", error.to_string());
                                }
                            }
                        }
                    });

                Panel::right("controls")
                    .resizable(false)
                    .show(ui, |ui| {
                        ui.columns_const(|[play, pause, step]| {
                            if play.add_sized(play.available_size(), Button::new("Play")).clicked() {
                                self.chip.run();
                            }

                            if pause.add_sized(pause.available_size(), Button::new("Pause")).clicked() {
                                self.chip.pause();
                            }

                            if step.add_sized(step.available_size(), Button::new("Step Once").wrap_mode(TextWrapMode::Extend)).clicked() {
                                self.chip.step();
                            }
                        });
                    });
            });

        CentralPanel::default()
            .show(ui, |ui| {
                let available_size = ui.available_size();

                let (response, painter) = ui.allocate_painter(available_size, Sense::hover());
                let area = response.rect;

                let scale = (area.width() / display::WIDTH).min(area.height() / display::HEIGHT);
                let emulation_size = vec2(display::WIDTH * scale, display::HEIGHT * scale);
                let emulation_rect = Rect::from_center_size(area.center(), emulation_size);

                painter.rect_filled(emulation_rect, CornerRadius::ZERO, Color32::BLACK);
            });

    }
}
