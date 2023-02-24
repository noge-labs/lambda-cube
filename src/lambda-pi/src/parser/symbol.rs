#[derive(Clone, Debug, Hash, Eq)]
pub struct Symbol {
    pub id: usize,
    pub name: String,
}

impl Symbol {
    pub fn new(name: String) -> Self {
        Symbol { id: 0, name }
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.id == other.id
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
