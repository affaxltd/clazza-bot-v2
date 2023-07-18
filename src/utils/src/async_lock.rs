use std::sync::Arc;

use async_rwlock::RwLock;

pub type AsyncLock<T> = Arc<RwLock<T>>;

pub trait IntoLock<T> {
    fn into_lock(self) -> AsyncLock<T>;
}

impl<T> IntoLock<T> for T {
    fn into_lock(self) -> AsyncLock<T> {
        Arc::new(RwLock::new(self))
    }
}
