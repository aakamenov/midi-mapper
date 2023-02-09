use microui_femtovg::microui::{*, const_vec::ConstStr};

const PANEL_NAME: &str = "outputs";

pub struct State {
    outputs: Vec<OutputState>,
    notes_dropdown: dropdown::State,
    notes: Vec<String>
}

struct OutputState {
    note: wmidi::Note,
    alias: ConstStr<16>
}

impl State {
    pub fn draw(&mut self, ctx: &mut Context) {
        ctx.layout_row(&[-1], 0);
        if ctx.w(Dropdown::new(
                &mut self.notes_dropdown,
                &self.notes
            ).visible_items(16)
        ).submit {
            let note = unsafe {
                wmidi::Note::from_u8_unchecked(self.notes_dropdown.index as u8) 
            }; 

            self.outputs.push(OutputState::new(note));
        }

        ctx.layout_row(&[-1], -1);
        Panel::new(PANEL_NAME).show(ctx, |ctx| {
            self.draw_entries(ctx);
        });
    }

    fn draw_entries(&mut self, ctx: &mut Context) {
        let separator_color = ctx.style.colors[WidgetColor::Base];

        for state in &mut self.outputs {
            ctx.layout_row(&[42, 100], 0);
            ctx.label("Note:");
            ctx.label(state.note.to_string());
    
            ctx.layout_row(&[48, 150], 0);
            ctx.label("Alias:");
            ctx.textbox(&mut state.alias);

            ctx.layout_row(&[-1], 1);

            let rect = ctx.layout_next();
            ctx.draw_box(rect, separator_color);
        }
    }
}

impl Default for State {
    fn default() -> Self {
        let mut notes = Vec::with_capacity(127);

        for i in 0u8..=127u8 {
            let note = unsafe {
                wmidi::Note::from_u8_unchecked(i).to_string()
            };

            notes.push(note);
        }

        Self {
            outputs: Vec::new(),
            notes_dropdown: dropdown::State::default(),
            notes
        }
    }
}

impl OutputState {
    fn new(note: wmidi::Note) -> Self {
        Self {
            note,
            alias: ConstStr::new()
        }
    }
}
