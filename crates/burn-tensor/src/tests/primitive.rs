#[burn_tensor_testgen::testgen(primitive)]
mod tests {
    use super::*;
    use burn_tensor::{backend::Backend, DType, Element, Shape};

    #[test]
    fn should_support_float_dtype() {
        let tensor = TestTensor::<2>::from([[0.0, -1.0, 2.0], [3.0, 4.0, -5.0]]).into_primitive();

        assert_eq!(burn_tensor::Primitive::shape(&tensor), Shape::new([2, 3]));
        assert_eq!(
            burn_tensor::Primitive::dtype(&tensor),
            <TestBackend as Backend>::FloatElem::dtype() // default float elem type
        );
    }

    #[test]
    fn should_support_int_dtype() {
        let tensor = TestTensorInt::<2>::from([[0, -1, 2], [3, 4, -5]]).into_primitive();

        assert_eq!(burn_tensor::Primitive::shape(&tensor), Shape::new([2, 3]));
        assert_eq!(
            burn_tensor::Primitive::dtype(&tensor),
            <TestBackend as Backend>::IntElem::dtype() // default int elem type
        );
    }

    #[test]
    fn should_support_bool_dtype() {
        let tensor =
            TestTensorBool::<2>::from([[false, true, true], [false, false, true]]).into_primitive();

        assert_eq!(burn_tensor::Primitive::shape(&tensor), Shape::new([2, 3]));
        assert_eq!(burn_tensor::Primitive::dtype(&tensor), DType::Bool);
    }
}
