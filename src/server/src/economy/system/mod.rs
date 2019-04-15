use specs::prelude::*;
use crate::entity::Id;
use crate::economy::*;

/// Behaviour when this entity produces a resource
pub struct ProduceResource;

impl<'a> System<'a> for ProduceResource {
    type SystemData = (
        ReadStorage<'a, Id>,
        WriteStorage<'a, ResourceStorage>,
        ReadStorage<'a, ResourceProduction>,
    );

    fn run(&mut self, (id, mut storage, production): Self::SystemData) {
        for (id, storage, production) in (&id, &mut storage, &production).join() {
            let mut amt = &mut storage.1;
            let name = &storage.0;
            let max = storage.2;
            let produced_name = &production.0;
            let produced = &production.1;

            if produced_name.0 != name.0 {
                continue;
            }

            if amt.0 + produced.0 > max {
                println!("{:?} is full of {:?}", id, name);
                amt.0 = max;
                continue;
            }

            println!("{:?} produced {:?} {:?}", id, produced, name);
            amt.0 += produced.0;
        }


    }
}

/// Behaviour when this entity consumes a resource
pub struct ConsumeResource;

impl<'a> System<'a> for ConsumeResource {
    type SystemData = (
        ReadStorage<'a, Id>,
        WriteStorage<'a, ResourceStorage>,
        ReadStorage<'a, ResourceConsumption>,
    );

    fn run(&mut self, (id, mut storage, consumption): Self::SystemData) {
        for (id, storage, consumption) in (&id, &mut storage, &consumption).join() {
            let mut amt = &mut storage.1;
            let name = &storage.0;
            let consumed_name = &consumption.0;
            let consumed = &consumption.1;
            if consumed_name.0 != name.0 {
                continue;
            }
            println!("{:?} consumed {:?} {:?}", id, amt, name);
            amt.0 -= consumed.0;
        }
    }
}

// TODO:
// 1. ParticipateInMarket
// 2. LeaveMarket
// 3. IssueBuyOrder
// 4. IssueSellOrder
// 5. PumpMarket // Fullfill all possible sales

pub trait RegisterSystems { 
    fn register_economy_systems(self) -> Self;
}

impl<'a, 'b> RegisterSystems for DispatcherBuilder<'a, 'b> {

    fn register_economy_systems(self) -> Self {
        self
            .with(ProduceResource, "produce_resource", &[])
            .with(ConsumeResource, "consume_resource", &[])
    }

}