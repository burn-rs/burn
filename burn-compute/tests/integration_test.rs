mod dummy;

use burn_compute::tune::Tuner;
use dummy::{
    client, make_kernel_pool, AdditionOp, ArrayHashable, DummyDevice, DummyElementwiseAddition,
    DummyServer,
};

#[test]
fn created_resource_is_the_same_when_read() {
    let client = client(&DummyDevice);
    let resource = Vec::from([0, 1, 2]);
    let resource_description = client.create(&resource);

    let obtained_resource = client.read(&resource_description);

    assert_eq!(resource, obtained_resource.read())
}

#[test]
fn empty_allocates_memory() {
    let client = client(&DummyDevice);
    let size = 4;
    let resource_description = client.empty(size);
    let empty_resource = client.read(&resource_description);

    assert_eq!(empty_resource.read().len(), 4);
}

#[test]
fn execute_elementwise_addition() {
    let client = client(&DummyDevice);
    let lhs = client.create(&[0, 1, 2]);
    let rhs = client.create(&[4, 4, 4]);
    let out = client.empty(3);

    client.execute(Box::new(DummyElementwiseAddition), &[&lhs, &rhs, &out]);

    let obtained_resource = client.read(&out);

    assert_eq!(obtained_resource.read(), Vec::from([4, 5, 6]))
}

#[test]
fn autotune() {
    let client = client(&DummyDevice);
    let lhs = client.create(&[0, 1, 2]);
    let rhs = client.create(&[4, 4, 4]);
    let out = client.empty(3);
    let binding = [&lhs, &rhs, &out];

    let kernel_pool = make_kernel_pool::<AdditionOp, DummyServer>(&client, &binding);
    let tuner = Tuner::new(kernel_pool);
    tuner.tune(ArrayHashable::new([3, 3, 3]))
}
