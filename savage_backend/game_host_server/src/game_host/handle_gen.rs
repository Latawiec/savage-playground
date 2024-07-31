use std::{marker::PhantomData, sync::atomic::AtomicU64};

pub struct HandleGenerator<T>
where
    T: From<u64>,
{
    _type: PhantomData<T>,
    counter: AtomicU64,
}

impl<T> HandleGenerator<T>
where
    T: From<u64>,
{
    pub fn next(&self) -> T {
        T::from(
            self.counter
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        )
    }
}

impl<T: From<u64>> Default for HandleGenerator<T> {
    fn default() -> Self {
        Self { _type: Default::default(), counter: AtomicU64::new(1) }
    }
}
