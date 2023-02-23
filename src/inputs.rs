use std::{
    fs,
    thread,
    sync::mpsc::{self, TryRecvError},
    path::PathBuf
};

use microui_femtovg::microui::{*, const_vec::ConstStr};
use rfd::FileDialog;
use nohash_hasher::IntMap;

use crate::{
    midi_file::{MidiFile, Mapping},
    Result, Error
};

const PANEL_NAME: &str = "inputs";
const MAP_WINDOW_NAME: &str = "Confirm mapping";

#[derive(Default)]
pub struct State {
    current: VisibleTracks,
    tracks: Vec<Vec<InputState>>,
    tracks_state: TracksState,
    map_window: Option<MapWindowState>
}

#[derive(Debug)]
pub enum Event {
    MidiLoaded(MidiFile),
    MidiLoadErr(Error),
    OutputSelected(SelectedOutput),
    Map {
        mappings: Vec<Mapping>,
        file: PathBuf
    }
}

#[derive(Debug)]
pub struct SelectedOutput {
    pub output: usize,
    track: usize,
    index: usize
}

#[derive(Clone, Copy)]
pub enum VisibleTracks {
    All,
    Single(usize)
}

enum TracksState {
    Uninitialized,
    Initializing(mpsc::Receiver<Result<MidiFile>>),
    Initialized {
        options: Vec<String>,
        state: dropdown::State
    }
}

struct InputState {
    note: wmidi::Note,
    alias: ConstStr<16>,
    map_to_dropdown: dropdown::State,
    map_to: Option<wmidi::Note>
}

struct MapWindowState {
    active_tracks: Vec<bool>
}

impl State {
    #[inline]
    pub fn reset_mapping(&mut self, note: wmidi::Note, removed_at: usize) {
        for states in &mut self.tracks {
            for state in states {
                if let Some(mapped) = state.map_to {
                    if mapped == note {
                        state.map_to = None;
                        state.map_to_dropdown.index = Some(0);
                    }
                }

                let index = state.map_to_dropdown.index.as_mut().unwrap();
                if *index >= removed_at && *index > 1 {
                    *index -= 1;
                }
            }
        }
    }

    #[inline]
    pub fn set_mapping(&mut self, selection: SelectedOutput, note: Option<wmidi::Note>) {
        let SelectedOutput { track, index, .. } = selection;
        self.tracks[track][index].map_to = note;
    }

    pub fn draw<'a>(
        &mut self,
        ctx: &mut Context,
        screen: Vec2,
        outputs: impl Iterator<Item = &'a str>
    ) -> Option<Event> {
        let mut event: Option<Event> = None;

        match &mut self.tracks_state {
            TracksState::Uninitialized => {
                ctx.layout_row(&[-1], -1);

                Panel::new(PANEL_NAME).show(ctx, |ctx| {
                    ctx.layout_row(&[-1], 0);

                    if ctx.button("Select MIDI file...") {
                        let file = FileDialog::new()
                            .add_filter("MIDI", &["midi", "mid"])
                            .pick_file();

                        if let Some(path) = file {
                            let (tx, rx) = mpsc::channel();
                                    
                            thread::spawn(move || {
                                match fs::read(path.as_path()) {
                                    Ok(file) =>
                                        tx.send(MidiFile::new(file)).unwrap(),
                                    Err(err) => tx.send(Err(Error::Io(err))).unwrap(),
                                }
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
                        Ok(Ok(midi)) => {
                            let mut options = Vec::with_capacity(1 + midi.tracks.len());
                            options.push("All Tracks".into());

                            if !midi.tracks.is_empty() {
                                for i in 0..midi.tracks.len() {
                                    options.push(format!("Track {}", i + 1));
                                }
                            }
                    
                            next = Some(TracksState::Initialized {
                                options,
                                state: dropdown::State::with_selection(0)
                            });

                            self.tracks = init_tracks(&midi);
                            event = Some(Event::MidiLoaded(midi));
                        },
                        Ok(Err(err)) => {
                            next = Some(TracksState::Uninitialized);
                            event = Some(Event::MidiLoadErr(err));
                        },
                        Err(TryRecvError::Disconnected) =>
                            next = Some(TracksState::Uninitialized),
                        Err(TryRecvError::Empty) => { }
                    }
                });

                if let Some(state) = next {
                    self.tracks_state = state;
                }
            },
            TracksState::Initialized { options, state } => {
                ctx.layout_row(&[ctx.last_rect().w / 2, -1], 0);
                if ctx.dropdown(state, options) {
                    let index = state.index.unwrap();
                    self.current = if index == 0 {
                        VisibleTracks::All
                    } else {
                        VisibleTracks::Single(index - 1)
                    };
                }

                if ctx.button("Map") {
                    self.init_map_window();
                }

                if let Some(e) = self.draw_map_window(ctx, screen) {
                    event = Some(e);
                }

                ctx.layout_row(&[-1], -1);
                Panel::new(PANEL_NAME).show(ctx, |ctx| {
                    let outputs: Vec<&'a str> = outputs.collect();

                    if let Some(e) = self.draw_entries(ctx, &outputs) {
                        event = Some(e);
                    }
                });
            }
        }

        event
    }

    fn draw_entries<'a>(
        &mut self,
        ctx: &mut Context,
        outputs: &'a [&'a str]
    ) -> Option<Event> {
        let mut event: Option<Event> = None;

        match self.current {
            VisibleTracks::All => {
                for i in 0..self.tracks.len() {
                    if let Some(e) = self.draw_inputs(ctx, outputs, i) {
                        event = Some(e);
                    }
                }
            },
            VisibleTracks::Single(index) =>
                event = self.draw_inputs(ctx, outputs, index),
        }

        event
    }

    fn draw_inputs<'a>(
        &mut self,
        ctx: &mut Context,
        outputs: &'a [&'a str],
        track: usize
    ) -> Option<Event> {
        let mut event: Option<Event> = None;

        let separator_color = ctx.style.colors[WidgetColor::Base];
        let label_width = 53;
        let box_width = 150;

        for (index, state) in self.tracks[track].iter_mut().enumerate() {
            ctx.layout_row(&[label_width, 100], 0);
            ctx.label("Note:");
            ctx.label(state.note.to_string());
    
            ctx.layout_row(&[label_width, box_width], 0);
            ctx.label("Alias:");
            ctx.textbox(&mut state.alias);

            ctx.layout_row(&[label_width, box_width], 0);
            ctx.label("Map to:");
            
            ctx.push_id(&(state as *const InputState));
            if ctx.w(Dropdown::new(
                    &mut state.map_to_dropdown,
                    &outputs
                ).visible_items(10)
            ).submit {
                let output = state.map_to_dropdown.index.unwrap();
                event = Some(Event::OutputSelected(
                    SelectedOutput {
                        output,
                        track,
                        index
                    }
                ));
            }
            ctx.pop_id();

            ctx.layout_row(&[-1], 1);

            let rect = ctx.layout_next();
            ctx.draw_box(rect, separator_color);
        }

        event
    }

    fn draw_map_window(&mut self, ctx: &mut Context, screen: Vec2) -> Option<Event> {
        const PANEL_HEIGHT: i32 = 105;

        let Some(window) = self.map_window.as_mut() else {
            return None;
        };

        let Some(index) = ctx.container_index_by_name(
            MAP_WINDOW_NAME,
            ContainerOptions::default()
        ) else {
            return None;
        };

        ctx.bring_to_front(index);
        ctx.container_mut(index).open = true;

        let mut event: Option<Event> = None;

        let height = PANEL_HEIGHT +
            ctx.style.size.x +
            (ctx.style.padding as i32 * 2) +
            ctx.style.spacing as i32;

        let screen = vec2(screen.x / 2, screen.y / 2);
        let window_rect = rect(
            screen.x - 100,
            screen.y - (height / 2),
            200,
            height
        );

        Window::new(MAP_WINDOW_NAME, window_rect)
            .no_resize()
            .show(ctx, |ctx| 
        {
            ctx.layout_row(&[-1], 0);
            ctx.label("Tracks to map:");

            ctx.layout_row(&[-1], PANEL_HEIGHT);
            Panel::new("Map tracks panel").show(ctx, |ctx| {
                ctx.layout_row(&[-1], 0);
                for (i, track) in window.active_tracks.iter_mut().enumerate() {
                    let label = format!("Track {}", i + 1);
                    ctx.checkbox(label, track);
                }
            });

            ctx.layout_row(&[-1], 0);
            if ctx.button("Execute") {
                let file = FileDialog::new()
                    .add_filter("MIDI", &["midi", "mid"])
                    .save_file()
                    .and_then(|mut x| {
                        x.set_extension("mid");

                        Some(x)
                    });

                if let Some(file) = file {
                    let len = window.active_tracks.iter().filter(|x| **x).count();
                    let mut mappings = Vec::with_capacity(len);
    
                    for (i, track) in self.tracks.iter().enumerate() {
                        if !window.active_tracks[i] {
                            continue;
                        }
    
                        let mut map = IntMap::default();
    
                        for state in track {
                            if let Some(to) = state.map_to {
                                map.insert(state.note as u8, to);
                            }
                        }
    
                        mappings.push(Mapping {
                            track: i,
                            map
                        });
                    }
    
                    event = Some(Event::Map {
                        mappings, 
                        file
                    });
                }
            }
        });

        if !ctx.container(index).open {
            self.map_window = None;
        }

        event
    }

    fn init_map_window(&mut self) {
        let active_tracks = match self.current {
            VisibleTracks::All => vec![true; self.tracks.len()],
            VisibleTracks::Single(index) => {
                let mut tracks = vec![false; self.tracks.len()];
                tracks[index] = true;

                tracks
            },
        };

        self.map_window = Some(MapWindowState {
           active_tracks 
        });
    }
}

fn init_tracks(midi: &MidiFile) -> Vec<Vec<InputState>> {
    let len = midi.tracks.len();
    let mut tracks = Vec::with_capacity(len);

    for track in &midi.tracks {
        let mut states = Vec::with_capacity(track.len());

        for note in track {
            let state = InputState::new(*note);
            states.push(state);
        }

        tracks.push(states);
    }

    tracks
}

impl InputState {
    #[inline]
    fn new(note: wmidi::Note) -> Self {
        Self {
            note,
            alias: ConstStr::new(),
            map_to_dropdown: dropdown::State::with_selection(0),
            map_to: None
        }
    }
}

impl Default for VisibleTracks {
    fn default() -> Self {
        Self::All
    }
}

impl Default for TracksState {
    fn default() -> Self {
        Self::Uninitialized
    }
}
