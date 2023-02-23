mod inputs;
mod outputs;
mod midi_file;

use std::{io, fmt::Display};

use microui_femtovg::{App, Shell, run, microui::*};

use midi_file::MidiFile;

const ERR_POPUP_NAME: &str = "Error popup";

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Midly(midly::Error),
    Io(io::Error)
}

#[derive(Default)]
struct MidiMapper {
    midi: MidiFile,
    inputs: inputs::State,
    outputs: outputs::State,
    error: Option<Error>
}

fn main() {
    run(Box::new(MidiMapper::default()))
}

impl App for MidiMapper {
    fn frame(&mut self, ctx: &mut Context, shell: &mut Shell) {
        let screen = shell.screen_size();

        Window::new("main", rect(0, 0, screen.x, screen.y))
            .min_size(screen)
            .max_size(screen)
            .no_title_bar()
            .no_resize()
            .show(ctx, |ctx| {
                let body = ctx.current_container().body;
                let space = (ctx.style.padding * 2) + ctx.style.spacing;
                let panel_width = (body.w - space as i32) / 2;

                ctx.layout_row(&[panel_width, panel_width], 15);
                ctx.label("Inputs:");
                ctx.label("Outputs:");

                ctx.layout_row(&[panel_width, panel_width], -1);

                ctx.layout_begin_column();
                let outputs = ["None"].iter().map(|x| *x).chain(self.outputs.output_strings());

                if let Some(event) = self.inputs.draw(ctx, screen, outputs) {
                    match event {
                        inputs::Event::OutputSelected(selection) => {
                            if selection.output > 0 {
                                let note = self.outputs.output(selection.output - 1);
                                self.inputs.set_mapping(selection, Some(note));
                            } else {
                                self.inputs.set_mapping(selection, None);
                            }
                        }
                        inputs::Event::MidiLoaded(midi) => {
                            self.midi = midi;
                        },
                        inputs::Event::Map { mappings, file } => {
                            if let Err(err) = self.midi.map_and_save_file(&mappings, file) {
                                self.error = Some(err)
                            }
                        },
                        inputs::Event::MidiLoadErr(err) => self.error = Some(err)
                    }
                }
                ctx.layout_end_column();

                ctx.layout_begin_column();
                if let Some(event) = self.outputs.draw(ctx) {
                    match event {
                        outputs::Event::OutputRemoved { note, removed_index } => {
                            self.inputs.reset_mapping(note, removed_index);
                        }
                    };
                }
                ctx.layout_end_column();
            });
        
        self.draw_err_popup(ctx);
    }
}

impl MidiMapper {
    fn draw_err_popup(&mut self, ctx: &mut Context) {
        let Some(err) = self.error.as_ref() else {
            return;
        };

        if let Some(index) = ctx.container_index_by_name(
            ERR_POPUP_NAME,
            ContainerOptions::default()
        ) {
            const MSG_BOX_HEIGHT: i32 = 80;

            ctx.bring_to_front(index);
            ctx.container_mut(index).open = true;

            let pos = ctx.mouse_pos();
            let height = MSG_BOX_HEIGHT + ctx.style.size.x;

            Window::new(ERR_POPUP_NAME, rect(pos.x, pos.y, 300, height))
                .no_resize()
                .no_close()
                .show(ctx, |ctx|
            {
                ctx.layout_row(&[-1], MSG_BOX_HEIGHT);
                Panel::new("Error message box")
                    .no_frame()
                    .show(ctx, |ctx|
                {
                    ctx.text(err.to_string());
                });

                ctx.layout_row(&[-1], 0);
                if ctx.button("Ok") {
                    ctx.container_mut(index).open = false;
                }
            });

            if !ctx.container(index).open {
                self.error = None;
            }
        }
    }
}

impl Display for Error {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Midly(err) => err.fmt(f),
            Error::Io(err) => err.fmt(f)
        }
    }
}

impl From<io::Error> for Error {
    #[inline]
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<midly::Error> for Error {
    #[inline]
    fn from(err: midly::Error) -> Self {
        Self::Midly(err)
    }
}
