use specs::prelude::*;

#[derive(Debug)]
struct Id(String);

impl Component for Id {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Vel(f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Pos(f32);

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

struct SomeUsefulSystem;

impl<'a> System<'a> for SomeUsefulSystem {
    type SystemData = (ReadStorage<'a, Id>, WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

    fn run(&mut self, (id, mut pos, vel): Self::SystemData) {
        for (id, pos, vel) in (&id, &mut pos, &vel).join() {
            println!("{:?}: {:?} += {:?}", id, pos.0, vel.0);
            pos.0 += vel.0
        }
    }
}

fn main() {
    let mut world = World::new();
    world.register::<Id>();
    world.register::<Pos>();
    world.register::<Vel>();

    world.create_entity().with(Id("1".to_string())).with(Vel(2.0)).with(Pos(0.0)).build();
    world.create_entity().with(Id("2".to_string())).with(Vel(4.0)).with(Pos(1.6)).build();
    world.create_entity().with(Id("3".to_string())).with(Vel(1.5)).with(Pos(5.4)).build();

    world.create_entity().with(Pos(2.0)).build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(SomeUsefulSystem, "useful", &[])
        .build();

    dispatcher.setup(&mut world.res);

    let running = true;

    while running {
        dispatcher.dispatch(&mut world.res);
        world.maintain();
    }

}
