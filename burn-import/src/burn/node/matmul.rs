use super::{Node, NodeCodegen};
use crate::burn::{Scope, TensorType, Type};
use burn::record::PrecisionSettings;
use proc_macro2::TokenStream;
use quote::quote;
use serde::Serialize;

#[derive(Debug, Clone, new)]
pub struct MatmulNode {
    pub lhs: TensorType,
    pub rhs: TensorType,
    pub output: TensorType,
}

impl Serialize for MatmulNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_none()
    }
}

impl<PS: PrecisionSettings> NodeCodegen<PS> for MatmulNode {
    fn output_types(&self) -> Vec<Type> {
        vec![Type::Tensor(self.output.clone())]
    }

    fn input_types(&self) -> Vec<Type> {
        vec![
            Type::Tensor(self.lhs.clone()),
            Type::Tensor(self.rhs.clone()),
        ]
    }

    fn forward(&self, scope: &mut Scope, node_position: usize) -> TokenStream {
        let lhs = scope.use_owned_tensor(&self.lhs.name, node_position);
        let rhs = scope.use_owned_tensor(&self.rhs.name, node_position);
        let output = &self.output.name;

        quote! {
            let #output = #lhs.matmul(#rhs);
        }
    }

    fn into_node(self) -> Node<PS> {
        Node::Matmul(self)
    }
}

#[cfg(test)]
mod tests {
    use burn::record::FullPrecisionSettings;

    use super::*;
    use crate::burn::{
        graph::Graph,
        node::{matmul::MatmulNode, test::assert_tokens},
        TensorType,
    };

    #[test]
    fn test_codegen_two_nodes() {
        let mut graph = Graph::<FullPrecisionSettings>::default();

        graph.register(MatmulNode::new(
            TensorType::new("tensor1", 4),
            TensorType::new("tensor2", 4),
            TensorType::new("tensor3", 4),
        ));

        let expected = quote! {
            use burn::{
                module::Module,
                tensor::{backend::Backend, Tensor},
            };

            #[derive(Module, Debug)]
            pub struct Model <B: Backend>{}

            impl<B: Backend> Model <B> {
                pub fn new_with(record: ModelRecord<B>) -> Self {
                    Self { }
                }

                pub fn forward(&self, tensor1: Tensor<B, 4>, tensor2: Tensor<B, 4>) -> Tensor<B, 4> {
                    let tensor3 = tensor1.matmul(tensor2);

                    tensor3
                }
            }
        };

        assert_tokens(graph.codegen(), expected);
    }
}
