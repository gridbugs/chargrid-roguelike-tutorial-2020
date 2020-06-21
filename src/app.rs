use crate::game::{GameState, Tile};
use chargrid::{
    app::{App as ChargridApp, ControlFlow},
    input::{keys, Input, KeyboardInput},
    render::{ColModify, Frame, View, ViewCell, ViewContext},
};
use coord_2d::Size;
use direction::CardinalDirection;
use rgb24::Rgb24;
use std::time::Duration;

struct AppData {
    game_state: GameState,
}

impl AppData {
    fn new(screen_size: Size) -> Self {
        Self {
            game_state: GameState::new(screen_size),
        }
    }
    fn handle_input(&mut self, input: Input) {
        match input {
            Input::Keyboard(key) => match key {
                KeyboardInput::Left => self.game_state.maybe_move_player(CardinalDirection::West),
                KeyboardInput::Right => self.game_state.maybe_move_player(CardinalDirection::East),
                KeyboardInput::Up => self.game_state.maybe_move_player(CardinalDirection::North),
                KeyboardInput::Down => self.game_state.maybe_move_player(CardinalDirection::South),
                _ => (),
            },
            _ => (),
        }
    }
}

struct AppView {}

impl AppView {
    fn new() -> Self {
        Self {}
    }
}

impl<'a> View<&'a AppData> for AppView {
    fn view<F: Frame, C: ColModify>(
        &mut self,
        data: &'a AppData,
        context: ViewContext<C>,
        frame: &mut F,
    ) {
        for entity_to_render in data.game_state.entities_to_render() {
            let view_cell = match entity_to_render.tile {
                Tile::Player => ViewCell::new()
                    .with_character('@')
                    .with_foreground(Rgb24::new_grey(255)),
            };
            frame.set_cell_relative(entity_to_render.coord, 0, view_cell, context);
        }
    }
}

pub struct App {
    data: AppData,
    view: AppView,
}

impl App {
    pub fn new(screen_size: Size) -> Self {
        Self {
            data: AppData::new(screen_size),
            view: AppView::new(),
        }
    }
}

impl ChargridApp for App {
    fn on_input(&mut self, input: Input) -> Option<ControlFlow> {
        match input {
            Input::Keyboard(keys::ETX) | Input::Keyboard(keys::ESCAPE) => Some(ControlFlow::Exit),
            other => {
                self.data.handle_input(other);
                None
            }
        }
    }
    fn on_frame<F, C>(
        &mut self,
        _since_last_frame: Duration,
        view_context: ViewContext<C>,
        frame: &mut F,
    ) -> Option<ControlFlow>
    where
        F: Frame,
        C: ColModify,
    {
        self.view.view(&self.data, view_context, frame);
        None
    }
}
