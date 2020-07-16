use std::mem;

use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, System, SystemData, WriteStorage},
    renderer::{palette::Srgba, resources::Tint},
};

use crate::pong::{CyclingColor, CyclingState};

#[derive(SystemDesc)]
pub struct CyclingColorSystem;

impl<'s> System<'s> for CyclingColorSystem {
    type SystemData = (
        WriteStorage<'s, Tint>,
        WriteStorage<'s, CyclingColor>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut tints, mut cycling_colors, time): Self::SystemData) {
        let time_delta = time.delta_seconds();
        for (tint, cycle) in (&mut tints, &mut cycling_colors).join() {
            match cycle.state {
                CyclingState::Stopped => {
                    tint.0 = cycle.from;
                }
                CyclingState::Cycling => {
                    cycle.current_cycle = (cycle.current_cycle - time_delta).max(0.0);
                    if cycle.current_cycle == 0.0 {
                        mem::swap(&mut cycle.from, &mut cycle.to);
                        cycle.current_cycle = cycle.cycle_time;
                    } else {
                        let delta = cycle.current_cycle / cycle.cycle_time;

                        let r = cycle.from.red + ((cycle.to.red - cycle.from.red) * delta);
                        let g = cycle.from.green + ((cycle.to.green - cycle.from.green) * delta);
                        let b = cycle.from.blue + ((cycle.to.blue - cycle.from.blue) * delta);
                        let a = cycle.from.alpha + ((cycle.to.alpha - cycle.from.alpha) * delta);

                        tint.0 = Srgba::new(r, g, b, a);
                    }
                }
            }
        }
    }
}
