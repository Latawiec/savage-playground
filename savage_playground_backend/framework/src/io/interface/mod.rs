pub mod unnamed_pipes;

pub trait IOInterface {
    fn write_msg(&mut self, data: &[u8]);
    fn read_msg(&mut self) -> Option<&[u8]>;
}


