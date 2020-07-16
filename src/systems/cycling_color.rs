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
        for (tint, cycling) in (&mut tints, &mut cycling_colors).join() {
            match cycling.state {
                CyclingState::Stopped => {
                    tint.0 = cycling.from;
                }
                CyclingState::Cycling => {
                    cycling.current_cycle = (cycling.current_cycle - time_delta).max(0.0);


                    if cycling.current_cycle == 0.0 {
                        let aux = cycling.from;
                        cycling.from = cycling.to;
                        cycling.to = aux;
                        cycling.current_cycle = cycling.cycle_time;
                    } else {
                        let delta = cycling.current_cycle / cycling.cycle_time;

                        let r = cycling.from.red + ((cycling.to.red - cycling.from.red) * delta);
                        let g = cycling.from.green + ((cycling.to.green - cycling.from.green) * delta);
                        let b = cycling.from.blue + ((cycling.to.blue - cycling.from.blue) * delta);
                        let a = cycling.from.alpha + ((cycling.to.alpha - cycling.from.alpha) * delta);

                        tint.0 = Srgba::new(r, g, b, a);
                    }
                }
            }
        }
    }
}
