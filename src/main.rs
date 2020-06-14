fn main() {
    use chargrid_graphical::{Config, Context, Dimensions, FontBytes};
    const CELL_SIZE_PX: f64 = 24.;
    let context = Context::new(Config {
        font_bytes: FontBytes {
            normal: include_bytes!("./fonts/PxPlus_IBM_CGAthin.ttf").to_vec(),
            bold: include_bytes!("./fonts/PxPlus_IBM_CGA.ttf").to_vec(),
        },
        title: "Chargrid Tutorial".to_string(),
        window_dimensions_px: Dimensions {
            width: 960.,
            height: 720.,
        },
        cell_dimensions_px: Dimensions {
            width: CELL_SIZE_PX,
            height: CELL_SIZE_PX,
        },
        font_scale: Dimensions {
            width: CELL_SIZE_PX,
            height: CELL_SIZE_PX,
        },
        underline_width_cell_ratio: 0.1,
        underline_top_offset_cell_ratio: 0.8,
        resizable: false,
    });
    let app = App::new();
    context.run_app(app);
}

struct App {}

impl App {
    fn new() -> Self {
        Self {}
    }
}

impl chargrid::app::App for App {
    fn on_input(&mut self, input: chargrid::app::Input) -> Option<chargrid::app::ControlFlow> {
        use chargrid::input::{keys, Input};
        match input {
            Input::Keyboard(keys::ETX) | Input::Keyboard(keys::ESCAPE) => {
                Some(chargrid::app::ControlFlow::Exit)
            }
            _ => None,
        }
    }
    fn on_frame<F, C>(
        &mut self,
        _since_last_frame: chargrid::app::Duration,
        _view_context: chargrid::app::ViewContext<C>,
        _frame: &mut F,
    ) -> Option<chargrid::app::ControlFlow>
    where
        F: chargrid::app::Frame,
        C: chargrid::app::ColModify,
    {
        None
    }
}
