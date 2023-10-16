use starry_ecs::{component::Component, World, resources::Resource};

use std::io::{stdout, Write};

#[derive(Clone, Debug)]
struct TestComponent {
    x: i32
}

impl Component for TestComponent {}

#[derive(Clone, Debug)]
struct RunCounter {
    runs: usize,
}
impl Resource for RunCounter {}

fn test_system(world: &World) {
    let test_comp = &world.try_get_components::<TestComponent>().unwrap()[0];

    assert_eq!(test_comp.x, -100);
}

#[test]
fn create_component() {
    let world = World::new().add_component(TestComponent { x: -100 }).add_system(test_system).start().single_step();
}
