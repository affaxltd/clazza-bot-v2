use std::sync::Arc;

use async_rwlock::RwLock;

pub type AsyncLock<T> = Arc<RwLock<T>>;

pub trait IntoLock<T> {
    fn into_lock(self) -> AsyncLock<T>;
}

impl<T> IntoLock<T> for T {
    fn into_lock(self) -> AsyncLock<T> {
        async_lock(self)
    }
}

pub fn async_lock<T>(value: T) -> AsyncLock<T> {
    Arc::new(RwLock::new(value))
}
