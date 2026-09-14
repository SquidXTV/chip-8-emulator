pub mod emulator;
pub mod ui;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    ui::run_application()
}
