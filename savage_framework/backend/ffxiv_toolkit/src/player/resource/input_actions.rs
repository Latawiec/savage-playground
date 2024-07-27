

#[repr(u64)]
pub enum BaseInputActions {
    Up = 1 << 0,
    Down = 1 << 1,
    Left = 1 << 2,
    Right = 1 << 3,
    Jump = 1 << 4,
    
    Slot1 = 1 << 5,
    Slot2 = 1 << 6,
    Slot3 = 1 << 7,
    Slot4 = 1 << 8,
}