#[derive(Debug, Clone)]
pub struct Command {
    pub verb: Verb,
    pub object: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Verb {
    North,
    South,
    East,
    West,
    Up,
    Down,
    Look,
    Inventory,
    Take,
    Drop,
    Examine,
    Open,
    Close,
    Read,
    Use,
    Help,
    Score,
    Save,
    Restore,
    Attack,
    Put,
    Unknown(String),
}
