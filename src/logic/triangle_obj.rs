use quartz::*;
use crate::constants::*;

pub fn register(cv: &mut Canvas) {
    cv.on_update(|c| {
        if c.key("left") {
            c.run(Action::apply_momentum(
                Target::tag("triangle"), -MOVE_SPEED, 0.0,
            ));
        }
        if c.key("right") {
            c.run(Action::apply_momentum(
                Target::tag("triangle"), MOVE_SPEED, 0.0,
            ));
        }
        if c.key("space") {
            c.run(Action::apply_momentum(
                Target::tag("triangle"), 0.0, JUMP_FORCE,
            ));
        }
    });

    cv.add_event(GameEvent::Collision, Target::tag("triangle"));
}