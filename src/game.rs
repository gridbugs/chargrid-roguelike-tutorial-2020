use coord_2d::{Coord, Size};
use direction::CardinalDirection;

pub struct GameState {
    screen_size: Size,
    player_coord: Coord,
}

impl GameState {
    pub fn new(screen_size: Size) -> Self {
        Self {
            screen_size,
            player_coord: screen_size.to_coord().unwrap() / 2,
        }
    }
    pub fn maybe_move_player(&mut self, direction: CardinalDirection) {
        let new_player_coord = self.player_coord + direction.coord();
        if new_player_coord.is_valid(self.screen_size) {
            self.player_coord = new_player_coord;
        }
    }
    pub fn player_coord(&self) -> Coord {
        self.player_coord
    }
}
