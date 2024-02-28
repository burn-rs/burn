use crate::{
    compute::StaticKernel,
    element::JitElement,
    kernel::{build_info, elemwise_workgroup, KernelSettings, WORKGROUP_DEFAULT},
    kernel_wgsl,
    tensor::JitTensor,
    Runtime,
};
use std::ops::Range;

kernel_wgsl!(
    IndexAssignInplaceRaw,
    "../../template/index/slice_assign_inplace.wgsl"
);

pub(crate) fn slice_assign<R: Runtime, E: JitElement, const D1: usize, const D2: usize>(
    tensor: JitTensor<R, E, D1>,
    indices: [Range<usize>; D2],
    value: JitTensor<R, E, D1>,
) -> JitTensor<R, E, D1> {
    let tensor = match tensor.can_mut() {
        true => tensor,
        false => tensor.copy(),
    };
    let num_elems = tensor.shape.num_elements();
    let mut info = build_info(&[&tensor, &value]);

    for i in 0..D1 {
        let start = indices.get(i).map(|index| index.start).unwrap_or(0);
        info.push(start as u32);
    }

    let info_handle = tensor.client.create(bytemuck::cast_slice(&info));

    let kernel = StaticKernel::<
        KernelSettings<IndexAssignInplaceRaw, E, i32, WORKGROUP_DEFAULT, WORKGROUP_DEFAULT, 1>,
    >::new(elemwise_workgroup(num_elems, WORKGROUP_DEFAULT));

    tensor.client.execute(
        Box::new(kernel),
        &[&tensor.handle, &value.handle, &info_handle],
    );

    tensor
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{ReferenceBackend, TestBackend, TestRuntime};
    use burn_tensor::{Distribution, Tensor};

    #[test]
    fn slice_assign_should_work_with_multiple_workgroups() {
        let tensor =
            Tensor::<TestBackend, 2>::random([6, 256], Distribution::Default, &Default::default());
        let value =
            Tensor::<TestBackend, 2>::random([2, 211], Distribution::Default, &Default::default());
        let indices = [3..5, 45..256];
        let tensor_ref =
            Tensor::<ReferenceBackend, 2>::from_data(tensor.to_data(), &Default::default());
        let value_ref =
            Tensor::<ReferenceBackend, 2>::from_data(value.to_data(), &Default::default());

        let actual = slice_assign::<TestRuntime, _, 2, 2>(
            tensor.into_primitive(),
            indices.clone(),
            value.into_primitive(),
        );
        let expected = tensor_ref.slice_assign(indices, value_ref);

        expected.into_data().assert_approx_eq(
            &Tensor::<TestBackend, 2>::from_primitive(actual).into_data(),
            3,
        );
    }
}
