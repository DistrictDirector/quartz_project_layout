use quartz::*;
use crate::constants::*;

pub fn setup(cv: &mut Canvas) {
    let obj = GameObject::build("triangle")
        .position(300.0, 200.0)
        .size(80.0, 80.0)
        .tag("triangle")
        .solid()
        .gravity(GRAVITY)
        .layer(LAYER_WORLD)
        .image(load_image("resources/triangle.png"))
        .finish();

    cv.add_game_object("triangle".into(), obj);
}