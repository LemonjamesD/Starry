use starry_ecs::World;
use starry_ecs::resources::Resource;

pub struct TestResource {
    x: i32
}

impl Resource for TestResource {}

pub fn test_resource(world: &World) {
    let mut resource = world.get_resource_write::<TestResource>().unwrap();
    println!("{}", resource.x);
    resource.x += 10;
}

#[test]
pub fn create_resource() {
    let world = World::new().add_system(test_resource).add_resource(TestResource { x: 100 }).start().single_step().single_step();
}
