use derive_more::Constructor;

use crate::world::World;

#[derive(Clone, Constructor)]
pub struct Problem {
    pub world: World,
    pub milliseconds: u64,
    pub n_steps: u64,
    pub start_y: i64,
    pub start_x: i64,
}
