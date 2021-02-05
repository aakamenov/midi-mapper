use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use eframe::egui;
use crate::mapper::{InputNote, OutputNote};

const PANEL_SIZE: (f32, f32) = (120f32, 80f32);
const THUMB_RADIUS: f32 = 10f32;

pub struct NoteViewer {
    pub input_notes: Vec<InputNotePanel>,
    pub output_notes: HashSet<OutputNotePanel>
}

pub struct InputNotePanel {
    name: String,
    input_note: InputNote,
    pos: egui::Pos2
}

pub struct OutputNotePanel {
    name: String,
    output_note: OutputNote
}

impl InputNotePanel {
    pub fn new(input_note: InputNote) -> Self {
        InputNotePanel {
            name: String::new(),
            input_note,
            pos: Default::default()
        }
    }
}

impl Hash for InputNotePanel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.input_note.hash(state);
    }
}

impl Default for NoteViewer {
    fn default() -> Self {
        Self {
            input_notes: Vec::default(),
            output_notes: HashSet::default()
        }
    }
}

pub fn note_viewer(state: &mut NoteViewer, ui: &mut egui::Ui) {
    let available_rect = ui.available_rect_before_wrap_finite();

    ui.painter().add(egui::Shape::line_segment(
        [available_rect.center_top(), available_rect.center_bottom()], 
        ui.style().visuals.widgets.noninteractive.bg_stroke
    ));

    let mut frame_style = egui::Frame::window(ui.style());
    frame_style.margin = egui::Vec2::splat(THUMB_RADIUS + 4f32);

    for panel in state.input_notes.iter_mut() {
        let (rect, response) = ui.allocate_exact_size(egui::Vec2::new(PANEL_SIZE.0, PANEL_SIZE.1 + THUMB_RADIUS), egui::Sense::drag());

        let frame_rect = rect.shrink(THUMB_RADIUS / 2f32);
        let inner_rect = frame_rect.shrink2(frame_style.margin);

        let mut panel_ui = ui.child_ui(inner_rect, *ui.layout());
        let frame_pos = panel_ui.painter().add(egui::Shape::Noop);
        input_note_panel(panel, &mut panel_ui);

        let outer_rect = egui::Rect::from_min_max(
            frame_rect.min,
            panel_ui.min_rect().max + frame_style.margin,
        );

        let frame_shape = egui::Shape::Rect {
            rect: outer_rect,
            corner_radius: frame_style.corner_radius,
            fill: frame_style.fill,
            stroke: frame_style.stroke,
        };

        ui.painter().set(frame_pos, frame_shape);

        let visuals = ui.style().interact(&response);
        let thumb = egui::Shape::circle_filled(outer_rect.right_center(), THUMB_RADIUS, visuals.bg_fill);
        ui.painter().add(thumb);
    }
}

fn input_note_panel(state: &mut InputNotePanel, ui: &mut egui::Ui) {
    ui.add(egui::TextEdit::singleline(&mut state.name));
    ui.label(format!("Note: {}", state.input_note.note));
}
