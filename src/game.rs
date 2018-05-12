use failure::Error;
use std::io::{stdin, stdout, Write};
use termion::raw::IntoRawMode;
use termion::cursor;
use termion;

#[derive(Clone,Debug)]
struct Position {
    pub x: usize,
    pub y: usize,
}
#[derive(Clone,Debug)]
enum Geometry {
    Player,
    Enemey
}
impl Geometry {
    pub fn draw<W: Write>(&self, writer: &mut W, pos: &Position) -> Result<(),Error> {
        let (_, height) = termion::terminal_size()?;
        write!(writer, "{}{}", cursor::Goto(height - pos.x as u16, pos.y as u16), "O")?;
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Entity(usize);

#[derive(Debug)]
struct EntitySystem{
    entities: Vec<Entity>,
}
impl EntitySystem {
    pub fn new() -> Self {
        EntitySystem {
            entities: vec![],
        }
    }
    pub fn create(&mut self) -> Entity {
        let new_ent = Entity(self.entities.len());
        self.entities.push(new_ent);
        new_ent
    }
}
#[derive(Debug)]
struct ComponentSystem<T> {
    components: Vec<Option<T>>,
}
impl<T> ComponentSystem<T> {
    pub fn new() -> Self {
        ComponentSystem {
            components: vec![],
        }
    }
}
impl<T: Clone> ComponentSystem<T> {
    pub fn add(&mut self, entity: Entity, data: T) {
        eprintln!("component_add({}) {}", entity.0, self.components.len());
        let index = entity.0;
        if self.components.len() >= index {
            self.components.resize(index+1, None);
        }
        debug_assert!(self.components.get(index).is_some());
        self.components[index] = Some(data);
    }
    pub fn get(&self, entity: Entity) -> Option<&Option<T>> {
        self.components.get(entity.0)
    }
}

pub fn run_game() -> Result<(), Error> {
    let mut stdout = stdout().into_raw_mode()?;
    let mut es = EntitySystem::new();
    let mut ps = ComponentSystem::<Position>::new();
    let mut gs = ComponentSystem::<Geometry>::new();
    let player = es.create();
    ps.add(player, Position {x: 0, y: 0});
    gs.add(player, Geometry::Player);
    for ent in es.entities.iter() {
        if let Some(Some(geom)) = gs.get(*ent) {
            if let Some(Some(pos)) = ps.get(*ent) {
                (*geom).draw(&mut stdout, pos)?;
            }
        }
    }
    eprintln!("es: {:?}", es);
    eprintln!("ps: {:?}", ps);
    Ok(())
}