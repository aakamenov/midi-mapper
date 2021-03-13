use std::hash::{Hash, Hasher};
use slotmap::{SlotMap, new_key_type};

pub type OutputNotes = SlotMap<OutputNoteKey, OutputNote>;

#[derive(Clone, Copy, PartialEq)]
pub struct InputNote {
    pub note: wmidi::Note,
    ///`None` if note stays the same
    pub maps_to: Option<OutputNoteKey>
}

#[derive(Clone, Copy, Hash, PartialEq)]
pub struct OutputNote {
    pub note: wmidi::Note
}

new_key_type! {
    pub struct OutputNoteKey;
}

impl InputNote {
    pub fn new(note: wmidi::Note) -> Self {
        InputNote {
            note,
            maps_to: None
        }
    }
}

impl Hash for InputNote {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.note.hash(state);
    }
}

impl OutputNote {
    pub fn new(note: wmidi::Note) -> Self {
        Self {
            note
        }
    }
}
