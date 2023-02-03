use std::collections::HashSet;

use midly::{Smf, TrackEventKind, MidiMessage};

pub struct MidiFile {
    pub tracks: Vec<Vec<wmidi::Note>>,
    bytes: Vec<u8>
}

impl MidiFile {
    pub fn new(bytes: Vec<u8>) -> Result<Self, midly::Error> {
        let midi = Smf::parse(&bytes)?;
        let tracks = Self::get_unique_notes(midi);

        Ok(Self {
            bytes,
            tracks
        })
    }

    fn get_unique_notes<'a>(midi: Smf<'a>) -> Vec<Vec<wmidi::Note>> {
        let mut result = vec![];

        for track in midi.tracks {
            let mut set = HashSet::new();

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

                    set.insert(note);
                }
            }

            result.push(set.into_iter().collect())
        }
        
        result
    }
}

impl Default for MidiFile {
    fn default() -> Self {
        Self {
            bytes: vec![],
            tracks: vec![]
        }
    }
}
