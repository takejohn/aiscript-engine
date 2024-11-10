use indexmap::IndexMap;

use super::{DataIndex, Register};

pub(super) enum Reference {
    Variable {
        dest: Register,
    },
    Index {
        target: Register,
        index: Register,
    },
    Prop {
        target: Register,
        name: DataIndex,
    },
    Arr {
        items: Vec<Reference>,
    },
    Obj {
        entries: IndexMap<DataIndex, Reference>,
    },
}
