#[derive(Default)]
pub struct PushVec<T: Copy> {
    buffer: Vec<T>,
    slices: Vec<usize>,
}

impl<T: Copy> PushVec<T> {
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

    pub fn pop(&mut self) -> Option<&[T]> {

        if let Some(last_slice_len) = self.slices.pop() {
            let end = self.buffer.len();
            let start = end - last_slice_len;
            return unsafe { 
                self.buffer.set_len(end - last_slice_len);
                let result = Some(self.buffer.get_unchecked(start .. end));
                result
            };
        }

        None
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

    #[test]
    fn pop_last_test() {
        let mut multi = PushVec::<i32>::default();
        multi.push(&[1, 2, 3, 4, 5, 6, 7]);
        multi.push(&[8, 9]);
        multi.push(&[10, 11]);

        let last = multi.pop();
        assert_eq!(last.unwrap(), &[10, 11]);

        let last = multi.pop();
        assert_eq!(last.unwrap(), &[8, 9]);

        let last = multi.pop();
        assert_eq!(last.unwrap(), &[1, 2, 3, 4, 5, 6, 7]);
    }
}