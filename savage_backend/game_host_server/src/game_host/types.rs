#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClientHandle(pub u64);

impl From<u64> for ClientHandle {
    fn from(value: u64) -> Self {
        ClientHandle(value)
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RoomHandle(pub u64);

impl From<u64> for RoomHandle {
    fn from(value: u64) -> Self {
        RoomHandle(value)
    }
}
