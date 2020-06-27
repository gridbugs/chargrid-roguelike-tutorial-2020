use crate::terrain::{self, TerrainTile};
use coord_2d::{Coord, Size};
use direction::CardinalDirection;
use entity_table::{Entity, EntityAllocator};
use rand::Rng;

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

pub struct World {
    pub entity_allocator: EntityAllocator,
    pub components: Components,
    pub spatial_table: SpatialTable,
}

pub struct Populate {
    pub player_entity: Entity,
}

impl World {
    pub fn new(size: Size) -> Self {
        let entity_allocator = EntityAllocator::default();
        let components = Components::default();
        let spatial_table = SpatialTable::new(size);
        Self {
            entity_allocator,
            components,
            spatial_table,
        }
    }
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
    fn spawn_player(&mut self, coord: Coord) -> Entity {
        let entity = self.entity_allocator.alloc();
        self.spatial_table
            .update(
                entity,
                Location {
                    coord,
                    layer: Some(Layer::Character),
                },
            )
            .unwrap();
        self.components.tile.insert(entity, Tile::Player);
        entity
    }
    pub fn populate<R: Rng>(&mut self, rng: &mut R) -> Populate {
        let terrain = terrain::generate_dungeon(self.spatial_table.grid_size(), rng);
        let mut player_entity = None;
        for (coord, &terrain_tile) in terrain.enumerate() {
            match terrain_tile {
                TerrainTile::Player => {
                    self.spawn_floor(coord);
                    player_entity = Some(self.spawn_player(coord));
                }
                TerrainTile::Floor => self.spawn_floor(coord),
                TerrainTile::Wall => {
                    self.spawn_floor(coord);
                    self.spawn_wall(coord);
                }
            }
        }
        Populate {
            player_entity: player_entity.unwrap(),
        }
    }
    pub fn maybe_move_character(&mut self, character_entity: Entity, direction: CardinalDirection) {
        let player_coord = self
            .spatial_table
            .coord_of(character_entity)
            .expect("player has no coord");
        let new_player_coord = player_coord + direction.coord();
        if new_player_coord.is_valid(self.spatial_table.grid_size()) {
            let dest_layers = self.spatial_table.layers_at_checked(new_player_coord);
            if dest_layers.character.is_none() && dest_layers.feature.is_none() {
                self.spatial_table
                    .update_coord(character_entity, new_player_coord)
                    .unwrap();
            }
        }
    }
}
