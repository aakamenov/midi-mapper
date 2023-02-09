use std::{fs, thread, sync::mpsc::{self, TryRecvError}};

use microui_femtovg::microui::{*, const_vec::ConstStr};
use rfd::FileDialog;

use crate::midi_file::MidiFile;

const PANEL_NAME: &str = "inputs";

#[derive(Default)]
pub struct State {
    current: usize,
    tracks: Vec<Vec<InputState>>,
    tracks_state: TracksState
}

pub enum Event {
    MidiLoaded(MidiFile),
    TrackChanged(usize)
}

enum TracksState {
    Uninitialized,
    Initializing(mpsc::Receiver<MidiFile>),
    Initialized {
        options: Vec<String>,
        state: dropdown::State
    }
}

struct InputState {
    note: wmidi::Note,
    alias: ConstStr<16>
}

impl State {
    pub fn select_track(&mut self, track: &[wmidi::Note], index: usize) {
        self.current = index;

        if !self.tracks[index].is_empty() || track.is_empty() {
            return;
        }

        for note in track {
            let state = InputState::new(*note);
            self.tracks[index].push(state);
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> Option<Event> {
        let mut event: Option<Event> = None;

        match &mut self.tracks_state {
            TracksState::Uninitialized => {
                ctx.layout_row(&[-1], -1);

                Panel::new(PANEL_NAME).show(ctx, |ctx| {
                    ctx.layout_row(&[-1], 0);

                    if ctx.button("Select MIDI file...") {
                        let file_path = FileDialog::new()
                            .add_filter("MIDI", &["midi", "mid"])
                            .pick_file();
    
                        if let Some(path) = file_path {
                            let (tx, rx) = mpsc::channel();
                                    
                            thread::spawn(move || {
                                let file = fs::read(path.as_path()).unwrap();
                                let midi = MidiFile::new(file).unwrap();
        
                                tx.send(midi).unwrap();
                            });
        
                            self.tracks_state = TracksState::Initializing(rx);
                        }
                    }
                });
            },
            TracksState::Initializing(rx) => {
                let mut next: Option<TracksState> = None;

                ctx.layout_row(&[-1], -1);
                Panel::new(PANEL_NAME).show(ctx, |ctx| {
                    ctx.layout_row(&[-1], 0);
                    ctx.button("Loading...");

                    match rx.try_recv() {
                        Ok(midi) => {
                            let options = midi.tracks.iter()
                                .enumerate()
                                .map(|x|format!("Track {}", x.0 + 1))
                                .collect();
                    
                            next = Some(TracksState::Initialized {
                                options,
                                state: dropdown::State::default()
                            });

                            self.tracks = init_tracks(&midi);
                            event = Some(Event::MidiLoaded(midi));
                        },
                        Err(TryRecvError::Disconnected) =>
                            next = Some(TracksState::Uninitialized),
                        _ => { }
                    }
                });

                if let Some(state) = next {
                    self.tracks_state = state;
                }
            },
            TracksState::Initialized { options, state } => {
                ctx.layout_row(&[-1], 0);
                if ctx.dropdown(state, options) {
                    event = Some(Event::TrackChanged(state.index));
                }

                ctx.layout_row(&[-1], -1);
                Panel::new(PANEL_NAME).show(ctx, |ctx| {
                    self.draw_entries(ctx);
                });
            }
        }

        event
    }

    fn draw_entries(&mut self, ctx: &mut Context) {
        let separator_color = ctx.style.colors[WidgetColor::Base];

        for state in &mut self.tracks[self.current] {
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

fn init_tracks(midi: &MidiFile) -> Vec<Vec<InputState>> {
    let len = midi.tracks.len();

    let mut tracks = Vec::with_capacity(len);
    tracks.extend((0..len).into_iter().map(|_| Vec::new()));

    for note in &midi.tracks[0] {
        let state = InputState::new(*note);
        tracks[0].push(state);
    }

    tracks
}

impl InputState {
    fn new(note: wmidi::Note) -> Self {
        Self {
            note,
            alias: ConstStr::new()
        }
    }
}

impl Default for TracksState {
    fn default() -> Self {
        Self::Uninitialized
    }
}
