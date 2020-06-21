use app::App;
use chargrid_graphical::{Context, ContextDescriptor, Dimensions, FontBytes};
use coord_2d::Size;

mod app;
mod game;

fn main() {
    const CELL_SIZE_PX: f64 = 24.;
    let context = Context::new(ContextDescriptor {
        font_bytes: FontBytes {
            normal: include_bytes!("./fonts/PxPlus_IBM_CGAthin.ttf").to_vec(),
            bold: include_bytes!("./fonts/PxPlus_IBM_CGA.ttf").to_vec(),
        },
        title: "Chargrid Tutorial".to_string(),
        window_dimensions: Dimensions {
            width: 960.,
            height: 720.,
        },
        cell_dimensions: Dimensions {
            width: CELL_SIZE_PX,
            height: CELL_SIZE_PX,
        },
        font_dimensions: Dimensions {
            width: CELL_SIZE_PX,
            height: CELL_SIZE_PX,
        },
        font_source_dimensions: Dimensions {
            width: CELL_SIZE_PX as f32,
            height: CELL_SIZE_PX as f32,
        },
        underline_width: 0.1,
        underline_top_offset: 0.8,
    })
    .expect("Failed to initialize graphical context");
    let screen_size = Size::new(40, 30);
    let app = App::new(screen_size);
    context.run_app(app);
}
