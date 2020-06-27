use crate::world::{Location, Populate, Tile, World};
use coord_2d::Size;
use direction::CardinalDirection;
use entity_table::Entity;
use rand::SeedableRng;
use rand_isaac::Isaac64Rng;

pub struct EntityToRender {
    pub tile: Tile,
    pub location: Location,
}

pub struct GameState {
    world: World,
    player_entity: Entity,
}

impl GameState {
    pub fn new(screen_size: Size) -> Self {
        let mut world = World::new(screen_size);
        let mut rng = Isaac64Rng::from_entropy();
        let Populate { player_entity } = world.populate(&mut rng);
        Self {
            world,
            player_entity,
        }
    }
    pub fn maybe_move_player(&mut self, direction: CardinalDirection) {
        self.world
            .maybe_move_character(self.player_entity, direction);
    }
    pub fn entities_to_render<'a>(&'a self) -> impl 'a + Iterator<Item = EntityToRender> {
        let tile_component = &self.world.components.tile;
        let spatial_table = &self.world.spatial_table;
        tile_component.iter().filter_map(move |(entity, &tile)| {
            let &location = spatial_table.location_of(entity)?;
            Some(EntityToRender { tile, location })
        })
    }
}
