use crate::visibility::{CellVisibility, VisibilityGrid};
use crate::world::{Location, Populate, Tile, World};
use coord_2d::Size;
use direction::CardinalDirection;
use entity_table::Entity;
use rand::SeedableRng;
use rand_isaac::Isaac64Rng;

pub struct EntityToRender {
    pub tile: Tile,
    pub location: Location,
    pub visibility: CellVisibility,
}

pub struct GameState {
    world: World,
    player_entity: Entity,
    shadowcast_context: shadowcast::Context<u8>,
    visibility_grid: VisibilityGrid,
}

impl GameState {
    pub fn new(screen_size: Size) -> Self {
        let mut world = World::new(screen_size);
        let mut rng = Isaac64Rng::from_entropy();
        let Populate { player_entity } = world.populate(&mut rng);
        let shadowcast_context = shadowcast::Context::default();
        let visibility_grid = VisibilityGrid::new(screen_size);
        let mut game_state = Self {
            world,
            player_entity,
            shadowcast_context,
            visibility_grid,
        };
        game_state.update_visibility();
        game_state
    }
    pub fn maybe_move_player(&mut self, direction: CardinalDirection) {
        self.world
            .maybe_move_character(self.player_entity, direction);
    }
    pub fn entities_to_render<'a>(&'a self) -> impl 'a + Iterator<Item = EntityToRender> {
        let tile_component = &self.world.components.tile;
        let spatial_table = &self.world.spatial_table;
        let visibility_grid = &self.visibility_grid;
        tile_component.iter().filter_map(move |(entity, &tile)| {
            let &location = spatial_table.location_of(entity)?;
            let visibility = visibility_grid.cell_visibility(location.coord);
            Some(EntityToRender {
                tile,
                location,
                visibility,
            })
        })
    }
    pub fn update_visibility(&mut self) {
        let player_coord = self
            .world
            .spatial_table
            .coord_of(self.player_entity)
            .unwrap();
        self.visibility_grid
            .update(player_coord, &self.world, &mut self.shadowcast_context);
    }
}
