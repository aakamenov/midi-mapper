#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = midi_mapper::MidiMapper::default();
    eframe::run_native(Box::new(app));
}
