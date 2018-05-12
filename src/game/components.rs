use specs::{Component, VecStorage};
use std::borrow::Cow;

#[derive(Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Appearance {
    Player,
    Enemy,
    Other(String),
}

impl Appearance {
    pub fn to_string(&self) -> Cow<str> {
        match self {
            Appearance::Player => Cow::Borrowed("/O\\"),
            Appearance::Enemy => Cow::Borrowed("U"),
            Appearance::Other(s) => Cow::Borrowed(s),
        }
    }
    pub fn get_width(&self) -> usize {
        match self {
            Appearance::Player   => 3,
            Appearance::Enemy    => 1,
            Appearance::Other(s) => s.len(),
        }
    }
}
impl Component for Appearance {
    type Storage = VecStorage<Self>;
}
