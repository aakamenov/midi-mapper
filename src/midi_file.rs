use std::path::PathBuf;

use midly::{Smf, TrackEventKind, MidiMessage, num::u7};
use nohash_hasher::{IntSet, IntMap};

use super::Result;

#[derive(Default, Debug)]
pub struct MidiFile {
    pub tracks: Vec<Vec<wmidi::Note>>,
    bytes: Vec<u8>
}

#[derive(Debug)]
pub struct Mapping {
    pub track: usize,
    pub map: IntMap<u8, wmidi::Note>
}

impl MidiFile {
    pub fn new(bytes: Vec<u8>) -> Result<Self> {
        let midi = Smf::parse(&bytes)?;
        let tracks = unique_notes(midi);

        Ok(Self {
            bytes,
            tracks
        })
    }

    pub fn map_and_save_file(&self, mappings: &[Mapping], file: PathBuf) -> Result<()> {
        let mut midi = Smf::parse(&self.bytes)?;

        for mapping in mappings {
            for event in &mut midi.tracks[mapping.track] {
                match &mut event.kind {
                    TrackEventKind::Midi { message, .. } => {
                        match message {
                            MidiMessage::NoteOn { key, .. } => {
                                if let Some(to) = mapping.map.get(&key.as_int()) {
                                    *key = u7::from_int_lossy(*to as u8);
                                }
                            },
                            MidiMessage::NoteOff { key, .. } => {
                                if let Some(to) = mapping.map.get(&key.as_int()) {
                                    *key = u7::from_int_lossy(*to as u8);
                                }
                            },
                            _ => { }
                        }
                    },
                    _ => { }
                }
            }
        }

        midi.save(file.as_path())?;

        Ok(())
    }
}

fn unique_notes<'a>(midi: Smf<'a>) -> Vec<Vec<wmidi::Note>> {
    let mut result = vec![];

    for track in midi.tracks {
        let mut set = IntSet::default();

        for event in track {
            if let TrackEventKind::Midi { message, .. } = event.kind {
                let note = match message {
                    MidiMessage::NoteOn { key, .. } => {
                        wmidi::Note::from_u8_lossy(key.as_int())
                    },
                    MidiMessage::NoteOff { key, .. } => {
                        wmidi::Note::from_u8_lossy(key.as_int())
                    }
                    _ => continue
                };

                set.insert(note as u8);
            }
        }

        result.push(set.into_iter()
            .map(|x| unsafe { wmidi::Note::from_u8_unchecked(x) })
            .collect()
        )
    }
    
    result
}
