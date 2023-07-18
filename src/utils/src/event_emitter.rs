#![allow(clippy::future_not_send, clippy::non_send_fields_in_send_ty)]

use async_rwlock::RwLock;
use async_trait::async_trait;
use futures::Future;
use std::sync::Arc;

#[async_trait(?Send)]
pub trait Listener<T> {
    async fn on(&self, event: T) -> bool;
}

#[derive(Clone, Default)]
pub struct EventEmitter<T> {
    listeners: Arc<RwLock<Vec<Box<dyn Listener<T>>>>>,
}

impl<T> EventEmitter<T> {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add_listener(&self, listener: impl Listener<T> + 'static) -> &Self {
        let mut listeners = self.listeners.write().await;

        listeners.push(Box::new(listener));

        self
    }

    pub async fn emit_raw(&self, event: T)
    where
        T: Clone,
    {
        let listeners = self.listeners.read().await;

        for listener in listeners.iter() {
            if listener.on(event.clone()).await {
                break;
            }
        }
    }
}

unsafe impl<T: Send + Sync> Send for EventEmitter<T> {}
unsafe impl<T: Send + Sync> Sync for EventEmitter<T> {}

#[async_trait(?Send)]
impl<F, Fut, T> Listener<T> for F
where
    T: 'static,
    F: Fn(T) -> Fut,
    Fut: Future<Output = bool>,
{
    async fn on(&self, event: T) -> bool {
        self(event).await
    }
}

macro_rules! impl_tuple {
    ($($T:ident),+) => {
        impl<$($T),+> EventEmitter<($($T),+)>
        where
            $($T: Clone),+
        {
            pub async fn emit(&self, $($T: $T),+) {
                #![allow(non_snake_case)]
                let tuple = ($($T),+);
                self.emit_raw(tuple).await;
            }
        }
    };
}

impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
