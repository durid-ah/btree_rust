pub(crate) enum SearchStatus {
    Found(usize),    // contains the key's index
    NotFound(usize), // contains the potential index location
}

impl SearchStatus {
    pub fn is_found(&self) -> bool {
        match self {
            SearchStatus::Found(_) => true,
            SearchStatus::NotFound(_) => false,
        }
    }

    pub fn unwrap(&self) -> usize {
        match self {
            SearchStatus::Found(val) | SearchStatus::NotFound(val) => *val,
        }
    }
}
