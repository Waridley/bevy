use crate::{Resources, Schedule, World};

pub trait Executor {
    fn run(&mut self, schedule: &mut Schedule, world: &mut World, resources: &mut Resources);
    fn initialize(&mut self, resources: &mut Resources);
}

pub struct BasicExecutor;

impl Default for BasicExecutor {
    fn default() -> BasicExecutor {
        BasicExecutor
    }
}

impl Executor for BasicExecutor {
    fn run(&mut self, schedule: &mut Schedule, world: &mut World, resources: &mut Resources) {
        schedule.run(world, resources)
    }

    fn initialize(&mut self, _resources: &mut Resources) {
        ()
    }
}
