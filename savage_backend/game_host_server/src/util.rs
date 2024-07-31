
pub struct OnceNotify<T: Clone> {
    notify: tokio::sync::Notify,
    value: tokio::sync::OnceCell<T>,
}

impl<T: Clone> OnceNotify<T> {
    pub async fn notified(&self) -> T {
        let notified_fut = self.notify.notified();
        if let Some(value) = self.value.get() {
            return value.clone();
        }

        notified_fut.await;
        self.value
            .get()
            .expect("Notified without result set.")
            .clone()
    }

    pub fn notify(&self, value: T) {
        let _ = self.value.set(value);
        self.notify.notify_waiters();
    }
}

impl<T: Clone> Default for OnceNotify<T> {
    fn default() -> Self {
        Self {
            notify: Default::default(),
            value: Default::default(),
        }
    }
}