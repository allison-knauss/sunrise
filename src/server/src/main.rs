#[macro_use]
extern crate specs_derive;

use specs::prelude::*;
use crate::entity::Id;

mod entity;
mod economy;

use economy::*;

fn main() {
    let mut world = World::new();
    world.register::<Id>();
    world.register_economy_components();

    let apple = Resource("apple".to_string());

    let apple_tree = world.create_entity()
        .with(Id("apple_tree".to_string()))
        .with(ResourceStorage(apple.clone(), ResourceAmount(10), 50)) // Tree can hold 50 apples
        .with(ResourceProduction(apple.clone(), ResourceAmount(1)))
        .build();

    let _apple_eater = world.create_entity()
        .with(Id("apple_eater".to_string()))
        .with(ResourceStorage(apple.clone(), ResourceAmount(0), 3)) // eater can hold 3 apples
        .build();


    let mut dispatcher = DispatcherBuilder::new()
        .register_economy_systems()
        .build();
    
    dispatcher.setup(&mut world.res);

    let running = true;

    while running {
        dispatcher.dispatch(&mut world.res);
        world.maintain();

        {
            let tree_apple_storage = world.read_storage::<ResourceStorage>();
            println!("Tree's apple storage: {:?}", tree_apple_storage.get(apple_tree).unwrap().0);
        }
    }

}
