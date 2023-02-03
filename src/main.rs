mod inputs;
mod outputs;
mod midi_file;

use std::collections::HashMap;
use microui_femtovg::{App, Shell, run, microui::*};

use midi_file::MidiFile;

#[derive(Default)]
struct MidiMapper {
    midi: MidiFile,
    inputs: inputs::State,
    outputs: outputs::State,
    mappings: HashMap<wmidi::Note, Vec<wmidi::Note>>
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
                if let Some(event) = self.inputs.draw(ctx) {
                    match event {
                        inputs::Event::MidiLoaded(midi) => self.midi = midi,
                        inputs::Event::TrackChanged(index) => self.inputs.select_track(
                            &self.midi.tracks[index],
                            index
                        )
                    }
                }
                ctx.layout_end_column();

                ctx.layout_begin_column();
                self.outputs.draw(ctx);
                ctx.layout_end_column();
            });
    }
}
