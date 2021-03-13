use eframe::egui;
use slotmap::{SlotMap};

use crate::mapper::{InputNote, OutputNote, OutputNoteKey, OutputNotes};

pub struct NoteViewer {
    input_panels: Vec<InputNotePanel>,
    output_panels: Vec<OutputNotePanel>,
    outputs: OutputNotes
}

struct InputNotePanel {
    alias: String,
    input: InputNote
}

struct OutputNotePanel {
    alias: String,
    key: OutputNoteKey
}

impl Default for NoteViewer {
    fn default() -> Self {
        Self {
            input_panels: Vec::default(),
            output_panels: Vec::default(),
            outputs: SlotMap::default()
        }
    }
}

impl NoteViewer {
    pub fn add_input(&mut self, note: InputNote) {
        self.input_panels.push(InputNotePanel::new(note));
    }
}

impl InputNotePanel {
    fn new(note: InputNote) -> Self {
        Self {
            alias: String::new(),
            input: note
        }
    }
}

impl OutputNotePanel {
    fn new(key: OutputNoteKey) -> Self {
        Self {
            alias: String::new(),
            key
        }
    }
}

pub fn note_viewer(state: &mut NoteViewer, ui: &mut egui::Ui) {
    ui_top_grid(ui, &mut state.outputs, &mut state.output_panels);

    let available_rect = ui.available_rect_before_wrap_finite();

    let separator_stroke = ui.style().visuals.widgets.noninteractive.bg_stroke;
    let window_padding = ui.style().spacing.window_padding;

    ui.painter().add(egui::Shape::line_segment(
        [ available_rect.center_top(), available_rect.center_bottom() ], 
        separator_stroke
    ));

    let mut pos_y = available_rect.center_bottom();
    pos_y.x -= window_padding.x;

    let ui_rect = egui::Rect::from_two_pos(
        available_rect.left_top(), pos_y
    );

    ui.allocate_ui_at_rect(ui_rect, |ui| {
        egui::ScrollArea::from_max_height(available_rect.height()).show(ui, |ui| {
            ui.vertical(|ui| {
                let mut id_counter = 0u8;

                for panel in state.input_panels.iter_mut() {
                    ui_name_and_alias(ui, panel.input.note.to_str(), &mut panel.alias);

                    ui_maps_to_dropdown(ui, &mut panel.input, &mut state.outputs, format!("maps_to_dropdown_{}", id_counter));
                    id_counter += 1;

                    ui.separator();
                }
            })
        });
    });

    let mut pos_x = available_rect.center_top();
    pos_x.x += window_padding.x;

    let ui_rect = egui::Rect::from_two_pos(
        pos_x, available_rect.right_bottom()
    )
    .translate(egui::vec2(separator_stroke.width, 0.0));
    
    ui.allocate_ui_at_rect(ui_rect, |ui| {
        egui::ScrollArea::from_max_height(available_rect.height()).show(ui, |ui| {
            ui.vertical(|ui| {
                for panel in state.output_panels.iter_mut() {
                    let output = state.outputs[panel.key];
                    ui_name_and_alias(ui, output.note.to_str(), &mut panel.alias);

                    ui.separator();
                }
            })
        });
    });
}

fn ui_top_grid(ui: &mut egui::Ui, outputs: &mut OutputNotes, output_panels: &mut Vec<OutputNotePanel>) {
    let available_rect = ui.available_rect_before_wrap_finite();

    egui::Grid::new("note_viewer_grid").min_col_width(available_rect.width() / 2.0).show(ui, |ui| {
        ui.label("Input");

        ui.horizontal(|ui| {
            ui.label("Output");

            if ui.button("Add output note").clicked() {
                let output_note = OutputNote::new(wmidi::Note::B0);
                let key = outputs.insert(output_note);

                output_panels.push(OutputNotePanel::new(key));
            }
        });

        ui.end_row();
    });

    ui.separator();
}

fn ui_maps_to_dropdown(ui: &mut egui::Ui, input: &mut InputNote, outputs: &mut OutputNotes, id: String) {
    ui.horizontal(|ui| {
        ui.label("Map to: ");

        let selected = if let Some(key) = input.maps_to {
            outputs[key].note.to_str()
        } else {
            "Select a note"
        };

        let id = ui.make_persistent_id(id);
        
        egui::combo_box(ui, id, selected, |ui| {
            for (key, val) in outputs.iter() {
                ui.selectable_value(&mut input.maps_to, Some(key), val.note.to_string());
            }
        });
    });
}

fn ui_name_and_alias(ui: &mut egui::Ui, note: &'static str, alias: &mut String) {
    ui.horizontal(|ui|{
        ui.label(format!("Note: {}", note));

        ui.add(
            egui::TextEdit::singleline(alias)
            .hint_text("alias")
            .desired_width(120.0)
        );
    });
}
