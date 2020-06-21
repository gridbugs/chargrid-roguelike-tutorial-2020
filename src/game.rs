use coord_2d::{Coord, Size};
use direction::CardinalDirection;
use entity_table::{Entity, EntityAllocator};

#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Player,
}

entity_table::declare_entity_module! {
    components {
        coord: Coord,
        tile: Tile,
    }
}

use components::Components;

pub struct GameState {
    screen_size: Size,
    components: Components,
    player_entity: Entity,
}

pub struct EntityToRender {
    pub tile: Tile,
    pub coord: Coord,
}

impl GameState {
    fn spawn_player(&mut self, coord: Coord) {
        self.components.coord.insert(self.player_entity, coord);
        self.components
            .tile
            .insert(self.player_entity, Tile::Player);
    }
    fn populate(&mut self, player_coord: Coord) {
        self.spawn_player(player_coord);
    }
    pub fn new(screen_size: Size) -> Self {
        let mut entity_allocator = EntityAllocator::default();
        let components = Components::default();
        let player_entity = entity_allocator.alloc();
        let mut game_state = Self {
            screen_size,
            components,
            player_entity,
        };
        game_state.populate(screen_size.to_coord().unwrap() / 2);
        game_state
    }
    pub fn maybe_move_player(&mut self, direction: CardinalDirection) {
        let player_coord = self
            .components
            .coord
            .get_mut(self.player_entity)
            .expect("player has no coord component");
        let new_player_coord = *player_coord + direction.coord();
        if new_player_coord.is_valid(self.screen_size) {
            *player_coord = new_player_coord;
        }
    }
    pub fn entities_to_render<'a>(&'a self) -> impl 'a + Iterator<Item = EntityToRender> {
        let tile_component = &self.components.tile;
        let coord_component = &self.components.coord;
        tile_component.iter().filter_map(move |(entity, &tile)| {
            let coord = coord_component.get(entity).cloned()?;
            Some(EntityToRender { tile, coord })
        })
    }
}
