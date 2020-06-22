use grid_2d::{Coord, Grid, Size};
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TerrainTile {
    Player,
    Floor,
    Wall,
}

pub fn generate_dungeon<R: Rng>(size: Size, rng: &mut R) -> Grid<TerrainTile> {
    println!("random int: {}", rng.next_u32());
    let mut grid = Grid::new_copy(size, None);
    for coord in Size::new(5, 5).coord_iter_row_major() {
        *grid.get_checked_mut(coord + Coord::new(1, 1)) = Some(TerrainTile::Floor);
    }
    *grid.get_checked_mut(Coord::new(3, 3)) = Some(TerrainTile::Player);
    grid.map(|t| t.unwrap_or(TerrainTile::Wall))
}
