use specs::prelude::*;
use std::borrow::Cow;
use termion::event::Key;
use super::*;

macro_rules! derive_component {
    ($n:ident) => {
        impl Component for $n {
            type Storage = DenseVecStorage<Self>;
        }
    };
    ($n:ident, $t:ident) => {
        impl Component for $n {
            type Storage = $t<Self>;
        }
    }
}

derive_component!(Position);
#[derive(Debug, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

derive_component!(Velocity, VecStorage);
#[derive(Debug, Clone)]
pub struct Velocity(pub isize);
impl Velocity {
    pub fn idle() -> Self {
        Velocity(0)
    }
}

derive_component!(Appearance, VecStorage);
#[derive(Debug, Clone)]
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

derive_component!(Projectile, VecStorage);
#[derive(Debug, Clone)]
pub enum Projectile {
    Allied,
    Enemy,
}

derive_component!(PlayerControls);
#[derive(Debug, Clone)]
pub struct PlayerControls {
    pub key_move_right: Key,
    pub key_move_left: Key,
    pub key_shoot: Key,
}
impl PlayerControls {
    pub fn default_player() -> Self {
        PlayerControls {
            key_move_right: Key::Right,
            key_move_left: Key::Left,
            key_shoot: Key::Char(' '),
        }
    }
}

derive_component!(Weapon);
#[derive(Debug, Clone)]
pub struct Weapon {
    dir: VDirection,
    base_cooldown: usize,
    pub current_cooldown: usize,
    pub shot: Option<Position>,
}
impl Weapon {
    pub fn new(dir: VDirection, base_cooldown: usize) -> Self {
        Weapon {
            dir,
            base_cooldown,
            current_cooldown: 0,
            shot: None,
        }
    }
    pub fn default_player() -> Self {
        Self::new(VDirection::Up, 10)
    }
    pub fn try_shoot(&mut self, source_pos: &Position) {
        trace!("enter");
        if self.current_cooldown == 0 {
            trace!("shoot");
            self.current_cooldown = self.base_cooldown;
            self.shot = Some(source_pos.clone());
        }
    }
    // pub fn get_base_cooldown(&self) -> usize {
    //     self.base_cooldown
    // }
}