pub mod unnamed_pipes;

pub trait IOInterface {
    fn write(&mut self, data: &[u8]);
    fn read(&mut self, buffer: &mut PushVec<u8>);
}

#[derive(Default)]
pub struct PushVec<T: Clone> {
    buffer: Vec<T>,
    slices: Vec<usize>,
}

impl<T: Clone> PushVec<T> {
    pub fn push(&mut self, data: &[T]) {
        let len = data.len();
        self.buffer.extend_from_slice(data);
        self.slices.push(len);
    }

    pub fn iter(&self) -> impl Iterator<Item = &[T]> {
        let mut bytes_counter: usize = 0;
        self.slices.iter().map(move |slice_len| {
            let slice = &self.buffer[bytes_counter .. bytes_counter + slice_len];
            bytes_counter += slice_len;
            slice
        })
    }

    pub fn clear(&mut self) {
        self.slices.clear();
        self.buffer.clear();
    }
}

#[cfg(test)]
mod test {
    use super::PushVec;

    #[test]
    fn multi_vec_test() { 
        let mut multi = PushVec::<i32>::default();
        multi.push(&[1, 2, 3, 4, 5, 6, 7]);
        multi.push(&[8, 9]);
        multi.push(&[10, 11]);

        let get_result: Vec<&[i32]> = multi.iter().collect();
        assert_eq!(get_result[0], &[1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(get_result[1], &[8, 9]);
        assert_eq!(get_result[2], &[10, 11]);
    }
}