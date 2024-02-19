use std::{marker::PhantomData, sync::atomic::AtomicU64};

#[derive(Default)]
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
