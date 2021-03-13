use eframe::{egui, epi};
use crate::note_viewer::{NoteViewer, note_viewer};
use crate::mapper::InputNote;

pub struct MidiMapper {
    note_viewer: NoteViewer
}

impl Default for MidiMapper {
    fn default() -> Self {
        Self {
            note_viewer: NoteViewer::default()
        }
    }
}

impl epi::App for MidiMapper {
    fn name(&self) -> &str {
        "Midi Mapper"
    }

    fn setup(&mut self, _ctx: &egui::CtxRef) {
        for _ in 0..10 {
            self.note_viewer.add_input(InputNote::new(wmidi::Note::A8));
        }
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::TopPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Open midi file...").clicked() {
                        
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_min_size(ui.available_size_before_wrap_finite());

            note_viewer(&mut self.note_viewer, ui);
        });
    }
}
