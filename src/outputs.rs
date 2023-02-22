use microui_femtovg::microui::{*, const_vec::ConstStr};

const PANEL_NAME: &str = "outputs";

pub struct State {
    outputs: Vec<OutputState>,
    notes_dropdown: dropdown::State,
    notes: Vec<String>
}

pub enum Event {
    OutputRemoved {
        note: wmidi::Note,
        removed_index: usize
    }
}

struct OutputState {
    note: wmidi::Note,
    note_string: String,
    alias: ConstStr<16>
}

impl State {
    pub fn draw(&mut self, ctx: &mut Context) -> Option<Event> {
        let mut event: Option<Event> = None;

        ctx.layout_row(&[-1], 0);
        if ctx.w(Dropdown::new(
                &mut self.notes_dropdown,
                &self.notes
            )
            .visible_items(16)
            .placeholder_text("Select output note", true)
        ).submit {
            let index = self.notes_dropdown.index.unwrap();
            let note = unsafe {
                wmidi::Note::from_u8_unchecked(index as u8) 
            };

            let exists = self.outputs.iter().find(|x| x.note == note);

            if exists.is_none() {
                self.outputs.push(OutputState::new(note));
            }
        }

        ctx.layout_row(&[-1], -1);
        Panel::new(PANEL_NAME).show(ctx, |ctx| {
            if let Some(e) = self.draw_entries(ctx) {
                event = Some(e);
            }
        });

        event
    }

    #[inline]
    pub fn output(&self, index: usize) -> wmidi::Note {
        self.outputs[index].note
    }

    #[inline]
    pub fn output_strings(&self) -> impl Iterator<Item = &str> {
        self.outputs.iter().map(|x|
            if x.alias.len() > 0 {
                x.alias.as_str()
            } else {
                &x.note_string
            }
        )
    }

    fn draw_entries(&mut self, ctx: &mut Context) -> Option<Event> {
        let mut event: Option<Event> = None;
        let separator_color = ctx.style.colors[WidgetColor::Base];

        for (i, state) in self.outputs.iter_mut().enumerate() {
            ctx.layout_row(&[42, 100], 0);
            ctx.label("Note:");
            ctx.label(state.note.to_string());

            let last = ctx.last_rect();
            let body = ctx.current_container().body;
            let pad = ctx.style.padding as i32;

            ctx.layout_set_next(
                rect(
                    body.x + (body.w - 20 - pad),
                    last.y,
                    20,
                    20
                ),
                LayoutType::Absolute
            );

            ctx.push_id(&(state as *mut OutputState));
            if ctx.w(
                Button::icon(Icon::Close)
                    .no_frame()
                    .with_cursor()
            ).submit {
                event = Some(Event::OutputRemoved {
                    note: state.note,
                    removed_index: i
                });
            }
            ctx.pop_id();
    
            ctx.layout_row(&[48, 150], 0);
            ctx.label("Alias:");
            ctx.textbox(&mut state.alias);

            ctx.layout_row(&[-1], 1);

            let rect = ctx.layout_next();
            ctx.draw_box(rect, separator_color);
        }

        if let Some(Event::OutputRemoved { removed_index, .. }) = &event {
            self.outputs.remove(*removed_index);
        }

        event
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
            note_string: note.to_string(),
            alias: ConstStr::new()
        }
    }
}
