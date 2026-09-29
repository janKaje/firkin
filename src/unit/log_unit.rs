use std::hash::{Hash, Hasher};

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct LogUnit {
    pub(crate) name: String,
    pub(crate) abbr: String,
    pub(crate) scale: f64,
    pub(crate) scale_per: f64,
}

impl Hash for LogUnit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.abbr.hash(state);
    }
}

impl Eq for LogUnit {}
