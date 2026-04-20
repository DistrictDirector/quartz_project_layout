pub mod constants;
pub mod preferences;
pub mod objects;
pub mod logic;

use quartz::*;

pub struct App;

impl App {
    pub fn new(context: &mut Context, assets: Assets) -> Scene {
        let mut scene = Scene::new(context, CanvasMode::Landscape, 1);
        let layer_id  = LayerId(0);

        let cv = scene.get_layer_mut(layer_id).unwrap().canvas_mut();

        // construct objects
        objects::triangle_obj::setup(cv);
        objects::circle_obj::setup(cv);

        // register logic
        logic::triangle_obj::register(cv);
        logic::circle_obj::register(cv);

        scene
    }
}

ramp::run! { |context: &mut Context, assets: Assets| {
    App::new(context, assets)
}}