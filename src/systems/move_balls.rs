use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
};

use crate::pong::{Ball, BallState};

#[derive(SystemDesc)]
pub struct MoveBallsSystem;

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut balls, mut locals, time): Self::SystemData) {
        // Check whether a ball collided, and bounce off accordingly.
        //
        // We also check for the velocity of the ball every time, to prevent multiple collisions
        // from occurring.
        for (ball, local) in (&mut balls, &mut locals).join() {
            let time_delta = time.delta_seconds();
            match ball.state {
                BallState::Waiting => {
                    if ball.waiting_time == 0. {
                        ball.state = BallState::Moving;
                    } else {
                        ball.waiting_time = (ball.waiting_time - time_delta).max(0.);
                    }
                }
                BallState::Moving => {
                    local.prepend_translation_x(ball.velocity[0] * time_delta);
                    local.prepend_translation_y(ball.velocity[1] * time_delta);
                }
            }
        }
    }
}
