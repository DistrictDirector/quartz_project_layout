use quartz::*;

pub fn register(cv: &mut Canvas) {
    cv.add_event(GameEvent::Collision, Target::tag("circle"));

    cv.on_update(|c| {
        if c.collision_between(&Target::tag("circle"), &Target::tag("triangle")) {
            c.run(Action::play_sound("resources/sounds/bounce.ogg"));
        }
    });
}