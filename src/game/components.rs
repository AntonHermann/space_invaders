use specs::{Component, VecStorage};

#[derive(Clone,Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Debug)]
pub enum Appearance {
    Player,
    Enemy,
    Other(String),
}
use std::borrow::Cow;
impl Appearance {
    pub fn to_string(&self) -> Cow<str> {
        match self {
            Appearance::Player => Cow::Borrowed("/O\\"),
            Appearance::Enemy  => Cow::Borrowed("U"),
            Appearance::Other(s) => Cow::Borrowed(s),
        }
    }
}
impl Component for Appearance {
    type Storage = VecStorage<Self>;
}