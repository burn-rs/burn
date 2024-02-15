use burn_tensor::{Data, Tensor};
use burn_wgpu::AutoGraphicsApi;

use crate::Autodiff;

pub type TestBackend = burn_wgpu::Wgpu<AutoGraphicsApi, f32, i32>;
pub type TestAutodiffBackend = Autodiff<TestBackend>;
pub type TestAutodiffTensor<const D: usize> = burn_tensor::Tensor<TestAutodiffBackend, D>;

// #[test]
// fn test_div_not_broken() {
//     let device = Default::default();
//     let t1 = Tensor::<AutodiffTestBackend, 2>::from_data([[99., 99.], [99., 99.]], &device);
//     let t2 = Tensor::<AutodiffTestBackend, 2>::from_data([[11., 11.], [11., 11.]], &device);
//     let t3 = t1 / t2;
//     let expected = Tensor::<AutodiffTestBackend, 2>::from_data([[9., 9.], [9., 9.]], &device);
//     t3.to_data().assert_approx_eq(&expected.to_data(), 3);
// }

// #[test]
// fn test_div_not_broken_with_require_grad() {
//     let device = Default::default();
//     let t1 = Tensor::<AutodiffTestBackend, 2>::from_data([[99., 99.], [99., 99.]], &device)
//         .require_grad();
//     let t2 = Tensor::<AutodiffTestBackend, 2>::from_data([[11., 11.], [11., 11.]], &device)
//         .require_grad();
//     let t3 = t1 / t2;
//     let expected = Tensor::<AutodiffTestBackend, 2>::from_data([[9., 9.], [9., 9.]], &device);
//     t3.to_data().assert_approx_eq(&expected.to_data(), 3);
// }

// #[test]
// fn test_div_not_broken_with_backward() {
//     let device = Default::default();
//     let t1 = Tensor::<AutodiffTestBackend, 2>::from_data([[99., 99.], [99., 99.]], &device)
//         .require_grad();
//     let t2 = Tensor::<AutodiffTestBackend, 2>::from_data([[11., 11.], [11., 11.]], &device)
//         .require_grad();
//     let t3 = t1.clone() / t2.clone();
//     let gradients = t3.backward();
//     println!("{:?}", t1.grad(&gradients).unwrap().to_data());
//     println!("{:?}", t2.grad(&gradients).unwrap().to_data());
//     println!("{:?}", t3.grad(&gradients).unwrap().to_data());
//     // let expected = Tensor::<AutodiffTestBackend, 2>::from_data([[9., 9.], [9., 9.]], &device);
//     // t3.to_data().assert_approx_eq(&expected.to_data(), 3);
//     assert!(false);
// }

#[test]
fn should_diff_div() {
    let data_1 = Data::from([1.0, 7.0]);
    let data_2 = Data::from([4.0, 7.0]);

    let device = Default::default();
    let tensor_1 = TestAutodiffTensor::from_data(data_1, &device).require_grad();
    let tensor_2 = TestAutodiffTensor::from_data(data_2, &device).require_grad();

    let tensor_3 = tensor_1.clone().div(tensor_2.clone());
    let grads = tensor_3.backward();

    let grad_1 = tensor_1.grad(&grads).unwrap();
    let grad_2 = tensor_2.grad(&grads).unwrap();

    grad_1
        .to_data()
        .assert_approx_eq(&Data::from([0.25, 0.1429]), 3);
    grad_2
        .to_data()
        .assert_approx_eq(&Data::from([-0.0625, -0.1429]), 3);
}

#[test]
fn should_diff_mul() {
    let data_1 = Data::from([1.0, 7.0]);
    let data_2 = Data::from([4.0, 7.0]);

    let device = Default::default();
    let tensor_1 = TestAutodiffTensor::from_data(data_1.clone(), &device).require_grad();
    let tensor_2 = TestAutodiffTensor::from_data(data_2.clone(), &device).require_grad();

    let tensor_3 = tensor_1.clone().mul(tensor_2.clone());
    let grads = tensor_3.backward();

    let grad_1 = tensor_1.grad(&grads).unwrap();
    let grad_2 = tensor_2.grad(&grads).unwrap();

    assert_eq!(grad_1.to_data(), data_2);
    assert_eq!(grad_2.to_data(), data_1);
    assert_eq!(tensor_3.into_data(), Data::from([4.0, 49.0]));
}

#[test]
fn should_diff_mul_tree() {
    // (ab)(cd)
    let data_a = Data::from([1.0, 7.0]);
    let data_b = Data::from([2.0, 7.0]);
    let data_c = Data::from([3.0, 7.0]);
    let data_d = Data::from([4.0, 7.0]);

    let device = Default::default();
    let tensor_a = TestAutodiffTensor::from_data(data_a, &device).require_grad();
    let tensor_b = TestAutodiffTensor::from_data(data_b, &device).require_grad();
    let tensor_c = TestAutodiffTensor::from_data(data_c, &device).require_grad();
    let tensor_d = TestAutodiffTensor::from_data(data_d, &device).require_grad();

    let tensor_e = tensor_a.clone().mul(tensor_b.clone());
    let tensor_f = tensor_c.clone().mul(tensor_d.clone());
    let tensor_g = tensor_e.mul(tensor_f);

    let grads = tensor_g.backward();
    let grad_a = tensor_a.grad(&grads).unwrap().to_data();
    let grad_b = tensor_b.grad(&grads).unwrap().to_data();
    let grad_c = tensor_c.grad(&grads).unwrap().to_data();
    let grad_d = tensor_d.grad(&grads).unwrap().to_data();

    let expected_a = Data::from([24.0, 343.0]);
    let expected_b = Data::from([12.0, 343.0]);
    let expected_c = Data::from([8.0, 343.0]);
    let expected_d = Data::from([6.0, 343.0]);

    assert_eq!(grad_a, expected_a);
    assert_eq!(grad_b, expected_b);
    assert_eq!(grad_c, expected_c);
    assert_eq!(grad_d, expected_d);
    // assert!(false)
}

#[test]
fn should_diff_div_tree() {
    // (a/b)/(c/d)
    let data_a = Data::from([1.0, 7.0]);
    let data_b = Data::from([2.0, 7.0]);
    let data_c = Data::from([3.0, 7.0]);
    let data_d = Data::from([4.0, 7.0]);

    let device = Default::default();
    let tensor_a = TestAutodiffTensor::from_data(data_a, &device).require_grad();
    let tensor_b = TestAutodiffTensor::from_data(data_b, &device).require_grad();
    let tensor_c = TestAutodiffTensor::from_data(data_c, &device).require_grad();
    let tensor_d = TestAutodiffTensor::from_data(data_d, &device).require_grad();

    let tensor_e = tensor_a.clone().div(tensor_b.clone());
    let tensor_f = tensor_c.clone().div(tensor_d.clone());
    let tensor_g = tensor_e.div(tensor_f);

    let grads = tensor_g.backward();
    let grad_a = tensor_a.grad(&grads).unwrap().to_data();
    let grad_b = tensor_b.grad(&grads).unwrap().to_data();
    let grad_c = tensor_c.grad(&grads).unwrap().to_data();
    let grad_d = tensor_d.grad(&grads).unwrap().to_data();

    let expected_a = Data::from([0.6667, 0.1429]);
    let expected_b = Data::from([-0.3333, -0.1429]);
    let expected_c = Data::from([-0.2222, -0.1429]);
    let expected_d = Data::from([0.1667, 0.1429]);

    grad_a.assert_approx_eq(&expected_a, 3);
    grad_b.assert_approx_eq(&expected_b, 3);
    grad_c.assert_approx_eq(&expected_c, 3);
    grad_d.assert_approx_eq(&expected_d, 3);
}

#[test]
fn should_diff_mul_div_tree() {
    // For this test please put div checkpointed/stateful and mul lazy
    // (a/b)(c/d)
    let data_a = Data::from([1.0, 7.0]);
    let data_b = Data::from([2.0, 7.0]);
    let data_c = Data::from([3.0, 7.0]);
    let data_d = Data::from([4.0, 7.0]);

    let device = Default::default();
    let tensor_a = TestAutodiffTensor::from_data(data_a, &device).require_grad();
    let tensor_b = TestAutodiffTensor::from_data(data_b, &device).require_grad();
    let tensor_c = TestAutodiffTensor::from_data(data_c, &device).require_grad();
    let tensor_d = TestAutodiffTensor::from_data(data_d, &device).require_grad();

    let tensor_e = tensor_a.clone().div(tensor_b.clone());
    let tensor_f = tensor_c.clone().div(tensor_d.clone());
    let tensor_g = tensor_e.mul(tensor_f);

    let grads = tensor_g.backward();
    let grad_a = tensor_a.grad(&grads).unwrap().to_data();
    let grad_b = tensor_b.grad(&grads).unwrap().to_data();
    let grad_c = tensor_c.grad(&grads).unwrap().to_data();
    let grad_d = tensor_d.grad(&grads).unwrap().to_data();

    let expected_a = Data::from([0.375, 0.1429]);
    let expected_b = Data::from([-0.1875, -0.1429]);
    let expected_c = Data::from([0.125, 0.1429]);
    let expected_d = Data::from([-0.0938, -0.1429]);

    grad_a.assert_approx_eq(&expected_a, 3);
    grad_b.assert_approx_eq(&expected_b, 3);
    grad_c.assert_approx_eq(&expected_c, 3);
    grad_d.assert_approx_eq(&expected_d, 3);
}

#[test]
fn should_diff_mul_div_tree_with_reuse() {
    // For this test please put div checkpointed/stateful and mul lazy
    // (a/b)(b/c) .  a=0, b=1, c=2 , a/b=3 , b/c=4
    let data_a = Data::from([1.0, 7.0]);
    let data_b = Data::from([2.0, 7.0]);
    let data_c = Data::from([3.0, 7.0]);

    let device = Default::default();
    let tensor_a = TestAutodiffTensor::from_data(data_a, &device).require_grad();
    let tensor_b = TestAutodiffTensor::from_data(data_b, &device).require_grad();
    let tensor_c = TestAutodiffTensor::from_data(data_c, &device).require_grad();

    let tensor_e = tensor_a.clone().div(tensor_b.clone());
    let tensor_f = tensor_b.clone().div(tensor_c.clone());
    let tensor_g = tensor_e.mul(tensor_f);

    let grads = tensor_g.backward();
    let grad_a = tensor_a.grad(&grads).unwrap().to_data();
    let grad_b = tensor_b.grad(&grads).unwrap().to_data();
    let grad_c = tensor_c.grad(&grads).unwrap().to_data();

    let expected_a = Data::from([0.3333, 0.1429]);
    let expected_b = Data::from([0., 0.]);
    let expected_c = Data::from([-0.1111, -0.1429]);

    grad_a.assert_approx_eq(&expected_a, 3);
    grad_b.assert_approx_eq(&expected_b, 3);
    grad_c.assert_approx_eq(&expected_c, 3);
}

// TODO
// - Cleanup
// .    - Maybe create retro forward instance before prepare to save a clone
// - Check if all works fine
//     - Make several test trees, with many variations, including sharing a parent node
//          - Test for untracked parents
//     - Check if the checkpointer extend is well done and there's only one
// - Is node_tree redundant?
//

#[test]
fn complicated_computation() {
    // Add: MemoryBound, Eager [\]
    // Powf: ComputeBound, Eager (\)
    // Mul_scalar: (Unary) Memory bound, Eager [X]
    // Mul: MemoryBound, Lazy [X]
    // Div: ComputeBound, Lazy (X)
    let data_0 = Data::from([0.0, 7.0]);
    let data_1 = Data::from([1.0, 7.0]);
    let data_2 = Data::from([2.0, 7.0]);
    let data_3 = Data::from([3.0, 7.0]);
    let data_4 = Data::from([4.0, 7.0]);

    let device = Default::default();
    let tensor_0 = TestAutodiffTensor::from_data(data_0, &device).require_grad();
    let tensor_1 = TestAutodiffTensor::from_data(data_1, &device).require_grad();
    let tensor_2 = TestAutodiffTensor::from_data(data_2, &device).require_grad();
    let tensor_3 = TestAutodiffTensor::from_data(data_3, &device).require_grad();
    let tensor_4 = TestAutodiffTensor::from_data(data_4, &device).require_grad();

    let tensor_5 = tensor_0.powf(tensor_1);
    let tensor_6 = tensor_2.div(tensor_3.clone());
    let tensor_7 = tensor_3.add(tensor_4);
    let tensor_8 = tensor_6.div(tensor_7.clone());
    let tensor_9 = tensor_7.mul_scalar(11);
    let tensor_10 = tensor_5.mul(tensor_8.clone());
    let tensor_11 = tensor_8.mul(tensor_9);
    let tensor_12 = tensor_10.div(tensor_11.clone());

    let grads = tensor_12.backward();
    assert!(false)
}

#[test]
fn test_with_edge_cases() {
    let data_0 = Data::from([0.0, 7.0]);
    let data_1 = Data::from([1.0, 7.0]);

    let device = Default::default();
    let tensor_0 = TestAutodiffTensor::from_data(data_0, &device).require_grad();
    let tensor_1 = TestAutodiffTensor::from_data(data_1, &device).require_grad();

    let tensor_2 = tensor_0.add(tensor_1);
    let tensor_3 = tensor_2.clone().add_scalar(11);
    let tensor_4 = tensor_2.clone().add_scalar(11);
    let tensor_5 = tensor_3.add(tensor_4);
    let tensor_6 = tensor_5.clone().powf_scalar(11);
    let tensor_7 = tensor_5.add(tensor_2);
    let tensor_8 = tensor_6.div(tensor_7);

    tensor_8.backward();
    assert!(false);
}

#[test]
fn test_with_many_duplicates() {
    let data_0 = Data::from([4.0, 7.0]);

    let device = Default::default();
    let tensor_0 = TestAutodiffTensor::from_data(data_0, &device).require_grad();

    let tensor_1 = tensor_0.clone().add(tensor_0.clone());
    let tensor_2 = tensor_0.clone().powf(tensor_0.clone());
    let tensor_3 = tensor_0.clone().mul(tensor_0.clone());
    let tensor_4 = tensor_0.clone().div(tensor_0.clone());

    let tensor_5 = tensor_1.clone().add(tensor_0.clone());
    let tensor_6 = tensor_0.clone().add(tensor_5.clone());
    let tensor_7 = tensor_3.clone().div(tensor_5.clone());
    let tensor_8 = tensor_4.clone().powf(tensor_2.clone());
    let tensor_9 = tensor_6.mul(tensor_7);
    let tensor_10 = tensor_0.add(tensor_9);
    let tensor_11 = tensor_10.add_scalar(9);
    let tensor_12 = tensor_8.div(tensor_11);

    tensor_12.backward();
    assert!(false);
}
