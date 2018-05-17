use specs::prelude::*;
use std::borrow::Cow;
use termion::event::Key;
use super::*;

#[derive(Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[allow(dead_code)]
#[derive(Component, Debug, Clone)]
#[component(VecStorage)]
pub struct Velocity(pub isize);
impl Velocity {
    pub fn idle() -> Self {
        Velocity(0)
    }
}

#[allow(dead_code)]
#[derive(Component, Debug, Clone)]
#[component(VecStorage)]
pub enum Appearance {
    Player,
    Enemy,
    Shot,
    Other(String),
}
impl Appearance {
    pub fn to_string(&self) -> Cow<str> {
        match self {
            Appearance::Player   => Cow::Borrowed("/O\\"),
            Appearance::Enemy    => Cow::Borrowed("U"),
            Appearance::Shot     => Cow::Borrowed("|"),
            Appearance::Other(s) => Cow::Borrowed(s),
        }
    }
    pub fn get_width(&self) -> usize {
        match self {
            Appearance::Player   => 3,
            Appearance::Enemy    => 1,
            Appearance::Shot     => 1,
            Appearance::Other(s) => s.len(),
        }
    }
}

#[allow(dead_code)]
struct Projectile {
    pub velocity: f32,
    pub direction: VDirection,
}
impl Component for Projectile {
    type Storage = VecStorage<Self>;
}

#[derive(Component, Debug, Clone)]
#[component(VecStorage)]
pub struct PlayerControls {
    pub key_move_right: Key,
    pub key_move_left: Key,
    pub key_shoot: Key,
}
impl PlayerControls {
    pub fn new() -> Self {
        PlayerControls {
            key_move_right: Key::Right,
            key_move_left: Key::Left,
            key_shoot: Key::Char(' '),
        }
    }
}