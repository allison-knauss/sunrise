use specs::prelude::*;
use crate::entity::Id;

/// Represents some resource
#[derive(Debug, Component, Clone)]
pub struct Resource(pub String);

/// Represents some amount of some resource
#[derive(Debug, Component)]
pub struct ResourceAmount(pub u32);

/// Represents a per-unit value of some resource
#[derive(Debug, Component)]
pub struct ResourceValue(pub u32);

/// Represents some amount of a resource in a given place
#[derive(Debug, Component)]
pub struct ResourceStorage(pub Resource, pub ResourceAmount, pub u32);

/// Represents the production of a given amount of a resource
#[derive(Debug, Component)]
pub struct ResourceProduction(pub Resource, pub ResourceAmount);

/// Represents the consumption of a given amount of a resource
#[derive(Debug, Component)]
pub struct ResourceConsumption(pub Resource, pub ResourceAmount);

pub enum Actions {
    ParticipateInMarket{ market: Id },
    IssueBuyOrder(Id, Resource, ResourceAmount, ResourceValue),
    IssueSellOrder(Id, Resource, ResourceAmount, ResourceValue),
    FulfillSale { buyer: Id, seller: Id, resource: Resource, amount: ResourceAmount, value: ResourceValue }
}

pub trait RegisterComponents {
    fn register_economy_components(&mut self);
}

impl RegisterComponents for World {
    fn register_economy_components(&mut self) {
        self.register::<Resource>();
        self.register::<ResourceAmount>();
        self.register::<ResourceValue>();
        self.register::<ResourceStorage>();
        self.register::<ResourceProduction>();
        self.register::<ResourceConsumption>();
    }
}