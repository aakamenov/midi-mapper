use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct InputNote {
    pub note: wmidi::Note,
    ///`None` if note stays the same
    maps_to: Option<OutputNote>
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct OutputNote(wmidi::Note);

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
