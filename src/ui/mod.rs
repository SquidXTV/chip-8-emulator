use crate::emulator::display;
use crate::emulator::protocol::{EmulationCommand, EmulationFrame};
use eframe::epaint::text::TextWrapMode;
use eframe::{Frame, NativeOptions};
use egui::{Button, CentralPanel, Color32, CornerRadius, Panel, Rect, Sense, Ui, vec2};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const EMULATION_WIDTH: f32 = 640.0;
const EMULATION_HEIGHT: f32 = 320.0;

pub fn run_application(commands: Sender<EmulationCommand>, current_frame: Arc<Mutex<EmulationFrame>>) -> eframe::Result {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([EMULATION_WIDTH, EMULATION_HEIGHT])
            .with_min_inner_size([EMULATION_WIDTH, EMULATION_HEIGHT]),
        ..NativeOptions::default()
    };

    eframe::run_native(
        "CHIP-8 Emulator",
        options,
        Box::new(|cc| Ok(Box::new(EmulatorApplication::new(commands, current_frame)))),
    )
}

struct EmulatorApplication {
    commands: Sender<EmulationCommand>,
    current_frame: Arc<Mutex<EmulationFrame>>
}

impl EmulatorApplication {

    fn new(commands: Sender<EmulationCommand>, current_frame: Arc<Mutex<EmulationFrame>>) -> Self {
        Self {
            commands,
            current_frame
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
                                    let _ = self.commands.send(EmulationCommand::Load { bytes });
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
                                let _ = self.commands.send(EmulationCommand::Play);
                            }

                            if pause.add_sized(pause.available_size(), Button::new("Pause")).clicked() {
                                let _ = self.commands.send(EmulationCommand::Pause);
                            }

                            if step.add_sized(step.available_size(), Button::new("Step Once").wrap_mode(TextWrapMode::Extend)).clicked() {
                                let _ = self.commands.send(EmulationCommand::Step);
                            }
                        });
                    });
            });

        if let Ok(frame) = self.current_frame.lock() {

        }


        CentralPanel::default()
            .show(ui, |ui| {
                let available_size = ui.available_size();

                let (response, painter) = ui.allocate_painter(available_size, Sense::hover());
                let area = response.rect;

                let scale = (area.width() / display::WIDTH).min(area.height() / display::HEIGHT);

                let emulation_size = vec2(
                    display::WIDTH * scale,
                    display::HEIGHT * scale
                );

                let emulation_rect = Rect::from_center_size(area.center(), emulation_size);

                painter.rect_filled(
                    emulation_rect,
                    CornerRadius::ZERO,
                    Color32::BLACK
                );

                let Ok(frame) = self.current_frame.lock() else {
                    // todo: handle error case
                    return;
                };

                for y in 0..display::PIXEL_HEIGHT {
                    for x in 0..display::PIXEL_WIDTH {
                        if !frame.pixels[y][x] {
                            continue;
                        }

                        let pixel_min = emulation_rect.min + vec2(x as f32 * scale, y as f32 * scale);
                        let pixel_rect = Rect::from_min_size(pixel_min, vec2(scale, scale));

                        painter.rect_filled(pixel_rect, CornerRadius::ZERO, Color32::WHITE);
                    }
                }

            });

        ui.request_repaint_after(Duration::from_millis(16));
    }



}
