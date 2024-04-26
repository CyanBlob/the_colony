use bevy::math::Vec3;
use bevy::prelude::Component;
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Component)]
pub struct Pos(pub i32, pub i32);

#[allow(unused)]
#[derive(Component)]
pub struct PathPositions {
    pub positions: Vec<Pos>,
    pub abs_positions: Vec<(i32, i32)>,
    pub tile_positions: Vec<TilePos>,
}

impl Pos {
    pub fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    pub fn successors(&self) -> Vec<(Pos, u32)> {
        //let mut rand = thread_rng();
        let &Pos(x, y) = self;
        vec![
            Pos(x + 1, y + 0),
            Pos(x - 1, y + 0),
            Pos(x + 0, y + 1),
            Pos(x + 0, y - 1),
            /*Pos(x + 1, y + 2),
            Pos(x + 1, y - 2),
            Pos(x - 1, y + 2),
            Pos(x - 1, y - 2),
            Pos(x + 2, y + 1),
            Pos(x + 2, y - 1),
            Pos(x - 2, y + 1),
            Pos(x - 2, y - 1),*/
        ]
            .into_iter()
            //.map(|p| (p, rand.gen_range(0..255)))
            .map(|p| (p, 1))
            .collect()
    }

    #[allow(unused)]
    pub fn translation(&self) -> Vec3 {
        Vec3 {
            x: self.0 as f32,
            y: self.1 as f32,
            z: 0f32,
        }
    }

    pub fn new(x: i32, y: i32) -> Self {
        Pos(x, y)
    }
}
