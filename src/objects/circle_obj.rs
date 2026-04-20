use quartz::*;
use crate::constants::*;

pub fn setup(cv: &mut Canvas) {
    let obj = GameObject::build("circle")
        .position(500.0, 200.0)
        .size(64.0, 64.0)
        .tag("circle")
        .solid_circle(32.0)
        .gravity(GRAVITY)
        .bouncy()
        .layer(LAYER_WORLD)
        .image(load_image("resources/circle.png"))
        .finish();

    cv.add_game_object("circle".into(), obj);
}