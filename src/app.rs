use eframe::{egui, epi};
use crate::note_viewer::{NoteViewer, InputNotePanel, OutputNotePanel, note_viewer};
use crate::mapper::InputNote;

pub struct MidiMapper {
    note_viewer: NoteViewer
}

impl Default for MidiMapper {
    fn default() -> Self {
        Self {
            note_viewer: Default::default()
        }
    }
}

impl epi::App for MidiMapper {
    fn name(&self) -> &str {
        "Midi Mapper"
    }

    fn setup(&mut self, _ctx: &egui::CtxRef) {
        self.note_viewer.input_notes.push(InputNotePanel::new(InputNote::new(wmidi::Note::C4)));
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::TopPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked {
                        frame.quit();
                    }
                });
            });
        });

        let available_space = ctx.available_rect();
        let half = available_space.size() / 2f32;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_min_size(ui.available_size_before_wrap_finite());

            egui::Grid::new("note_viewer_grid").spacing(half).show(ui, |ui| {
                ui.label("Input");
                ui.label("Output");
                ui.end_row();
            });

            ui.separator();

            note_viewer(&mut self.note_viewer, ui);
        });
    }
}
