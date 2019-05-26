use serde::{Deserialize, Serialize};
use specs::{Component, HashMapStorage};
use specs_derive::Component;
use tcod::Color;

#[derive(Clone, Serialize, Deserialize, Debug, Component)]
#[storage(HashMapStorage)]
pub struct Messages {
    pub limit: usize,
    pub inner: Vec<(String, Color)>,
}

impl Messages {
    pub fn new(limit: usize) -> Messages {
        Messages {
            limit,
            inner: Vec::with_capacity(limit),
        }
    }

    pub fn push<T: Into<String>>(&mut self, message: T, color: Color) {
        while self.inner.len() >= self.limit {
            self.inner.remove(0);
        }
        self.inner.push((message.into(), color));
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}
