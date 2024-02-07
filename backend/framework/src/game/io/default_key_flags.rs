use super::input::KeyFlag;


#[derive(Clone, Copy)]
pub enum DefaultKeyFlags {
    Up = 1 << 1,
    Down = 1 << 2,
    Left = 1 << 3,
    Right = 1 << 4,
    Spell1 = 1 << 5,
    Spell2 = 1 << 6,
    Spell3 = 1 << 7,
}

impl Into<KeyFlag> for DefaultKeyFlags {
    fn into(self) -> KeyFlag {
        self as KeyFlag
    }
}
