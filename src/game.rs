use coord_2d::{Coord, Size};
use direction::CardinalDirection;
use entity_table::{Entity, EntityAllocator};

#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Player,
    Floor,
    Wall,
}

entity_table::declare_entity_module! {
    components {
        tile: Tile,
    }
}

use components::Components;

spatial_table::declare_layers_module! {
    layers {
        floor: Floor,
        character: Character,
        feature: Feature,
    }
}

pub use layers::Layer;
type SpatialTable = spatial_table::SpatialTable<layers::Layers>;
pub type Location = spatial_table::Location<Layer>;

pub struct GameState {
    screen_size: Size,
    entity_allocator: EntityAllocator,
    components: Components,
    spatial_table: SpatialTable,
    player_entity: Entity,
}

pub struct EntityToRender {
    pub tile: Tile,
    pub location: Location,
}

impl GameState {
    fn spawn_wall(&mut self, coord: Coord) {
        let entity = self.entity_allocator.alloc();
        self.spatial_table
            .update(
                entity,
                Location {
                    coord,
                    layer: Some(Layer::Feature),
                },
            )
            .unwrap();
        self.components.tile.insert(entity, Tile::Wall);
    }
    fn spawn_floor(&mut self, coord: Coord) {
        let entity = self.entity_allocator.alloc();
        self.spatial_table
            .update(
                entity,
                Location {
                    coord,
                    layer: Some(Layer::Floor),
                },
            )
            .unwrap();
        self.components.tile.insert(entity, Tile::Floor);
    }
    fn spawn_player(&mut self, coord: Coord) {
        self.spatial_table
            .update(
                self.player_entity,
                Location {
                    coord,
                    layer: Some(Layer::Character),
                },
            )
            .unwrap();
        self.components
            .tile
            .insert(self.player_entity, Tile::Player);
    }
    fn populate(&mut self, player_coord: Coord) {
        self.spawn_player(player_coord);
        for coord in self.screen_size.coord_iter_row_major() {
            self.spawn_floor(coord);
        }
        self.spawn_wall(player_coord + Coord::new(-1, 2));
        self.spawn_wall(player_coord + Coord::new(0, 2));
        self.spawn_wall(player_coord + Coord::new(1, 2));
    }
    pub fn new(screen_size: Size) -> Self {
        let mut entity_allocator = EntityAllocator::default();
        let components = Components::default();
        let spatial_table = SpatialTable::new(screen_size);
        let player_entity = entity_allocator.alloc();
        let mut game_state = Self {
            screen_size,
            entity_allocator,
            components,
            spatial_table,
            player_entity,
        };
        game_state.populate(screen_size.to_coord().unwrap() / 2);
        game_state
    }
    pub fn maybe_move_player(&mut self, direction: CardinalDirection) {
        let player_coord = self
            .spatial_table
            .coord_of(self.player_entity)
            .expect("player has no coord");
        let new_player_coord = player_coord + direction.coord();
        if new_player_coord.is_valid(self.screen_size) {
            let dest_layers = self.spatial_table.layers_at_checked(new_player_coord);
            if dest_layers.character.is_none() && dest_layers.feature.is_none() {
                self.spatial_table
                    .update_coord(self.player_entity, new_player_coord)
                    .unwrap();
            }
        }
    }
    pub fn entities_to_render<'a>(&'a self) -> impl 'a + Iterator<Item = EntityToRender> {
        let tile_component = &self.components.tile;
        let spatial_table = &self.spatial_table;
        tile_component.iter().filter_map(move |(entity, &tile)| {
            let &location = spatial_table.location_of(entity)?;
            Some(EntityToRender { tile, location })
        })
    }
}
