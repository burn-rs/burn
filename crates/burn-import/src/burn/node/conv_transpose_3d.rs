use super::{Node, NodeCodegen, SerializationBackend};
use crate::burn::{BurnImports, OtherType, Scope, TensorType, ToTokens, Type};
use burn::{
    module::{ConstantRecord, Param, ParamId},
    nn::conv::{ConvTranspose3dConfig, ConvTranspose3dRecord},
    record::{PrecisionSettings, Record},
    tensor::{Tensor, TensorData},
};
use proc_macro2::TokenStream;
use quote::quote;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ConvTranspose3dNode {
    pub field: OtherType,
    pub input: TensorType,
    pub output: TensorType,
    pub data_weights: TensorData,
    pub data_bias: Option<TensorData>,
    pub config: ConvTranspose3dConfig,
}

impl ConvTranspose3dNode {
    pub fn new<S: AsRef<str>>(
        name: S,
        input: TensorType,
        output: TensorType,
        data_weights: TensorData,
        data_bias: Option<TensorData>,
        config: ConvTranspose3dConfig,
    ) -> Self {
        Self {
            field: OtherType::new(
                name,
                quote! {
                    ConvTranspose3d<B>
                },
            ),
            input,
            output,
            data_weights,
            data_bias,
            config,
        }
    }
}

impl<PS: PrecisionSettings> NodeCodegen<PS> for ConvTranspose3dNode {
    fn input_types(&self) -> Vec<Type> {
        vec![Type::Tensor(self.input.clone())]
    }
    fn output_types(&self) -> Vec<Type> {
        vec![Type::Tensor(self.output.clone())]
    }
    fn field_type(&self) -> Option<Type> {
        Some(Type::Other(self.field.clone()))
    }

    fn field_init(&self) -> Option<TokenStream> {
        let name = &self.field.name;
        let channels = self.config.channels.to_tokens();
        let kernel_size = self.config.kernel_size.to_tokens();
        let stride = self.config.stride.to_tokens();
        let dilation = self.config.dilation.to_tokens();
        let groups = self.config.groups.to_tokens();
        let padding = self.config.padding.to_tokens();
        let padding_out = self.config.padding_out.to_tokens();
        let bias = self.config.bias;

        let tokens = quote! {
            let #name = ConvTranspose3dConfig::new(#channels, #kernel_size)
                .with_stride(#stride)
                .with_padding(#padding)
                .with_padding_out(#padding_out)
                .with_dilation(#dilation)
                .with_groups(#groups)
                .with_bias(#bias)
                .init(device);
        };

        Some(tokens)
    }

    fn field_serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let device = Default::default();
        let record = ConvTranspose3dRecord::<SerializationBackend> {
            weight: Param::initialized(
                ParamId::new(),
                Tensor::from_data(
                    self.data_weights.clone().convert::<PS::FloatElem>(),
                    &device,
                ),
            ),
            bias: self.data_bias.as_ref().map(|bias| {
                Param::initialized(
                    ParamId::new(),
                    Tensor::from_data(bias.clone().convert::<PS::FloatElem>(), &device),
                )
            }),
            stride: [ConstantRecord::new(); 3],
            kernel_size: [ConstantRecord::new(); 3],
            dilation: [ConstantRecord::new(); 3],
            groups: ConstantRecord::new(),
            padding: [ConstantRecord::new(); 3],
            padding_out: [ConstantRecord::new(); 3],
            channels: [ConstantRecord::new(); 2],
        };

        let item = Record::into_item::<PS>(record);
        item.serialize(serializer)
    }

    fn forward(&self, scope: &mut Scope, node_position: usize) -> TokenStream {
        let input = scope.tensor_use_owned(&self.input, node_position);
        let output = &self.output.name;
        let field = &self.field.name;

        quote! {
            let #output = self.#field.forward(#input);
        }
    }
    fn register_imports(&self, imports: &mut BurnImports) {
        imports.register("burn::nn::conv::ConvTranspose3d");
        imports.register("burn::nn::conv::ConvTranspose3dConfig");
    }

    fn into_node(self) -> Node<PS> {
        Node::ConvTranspose3d(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::burn::{
        graph::BurnGraph,
        node::{conv_transpose_3d::ConvTranspose3dNode, test::assert_tokens},
        TensorType,
    };
    use burn::{nn::conv::ConvTranspose3dConfig, record::FullPrecisionSettings};

    #[test]
    fn test_codegen() {
        let mut graph = BurnGraph::<FullPrecisionSettings>::default();

        graph.register(ConvTranspose3dNode::new(
            "conv_transpose_3d",
            TensorType::new_float("input", 5),
            TensorType::new_float("output", 5),
            TensorData::from([2f32]),
            None,
            ConvTranspose3dConfig::new([3, 3], [3, 3, 3]).with_padding([0, 0, 0]),
        ));

        graph.register_input_output(vec!["input".to_string()], vec!["output".to_string()]);

        let expected = quote! {
            use burn::{
                module::Module,
                tensor::{backend::Backend, Tensor},
            };
            use burn::nn::conv::ConvTranspose3d;
            use burn::nn::conv::ConvTranspose3dConfig;

            #[derive(Module, Debug)]
            pub struct Model <B: Backend> {
                conv_transpose_3d: ConvTranspose3d<B>,
                phantom: core::marker::PhantomData<B>,
                device: burn::module::Ignored<B::Device>,
            }

            impl<B: Backend> Model <B> {
                #[allow(unused_variables)]
                pub fn new(device: &B::Device) -> Self {
                    let conv_transpose_3d = ConvTranspose3dConfig::new([3, 3], [3, 3, 3])
                        .with_stride([1, 1, 1])
                        .with_padding([0, 0, 0])
                        .with_padding_out([0, 0, 0])
                        .with_dilation([1, 1, 1])
                        .with_groups(1)
                        .with_bias(true)
                        .init(device);

                    Self {
                        conv_transpose_3d,
                        phantom: core::marker::PhantomData,
                        device: burn::module::Ignored(device.clone()),
                    }
                }
                #[allow(clippy::let_and_return, clippy::approx_constant)]
                pub fn forward(&self, input: Tensor<B, 5>) -> Tensor<B, 5> {
                    let output = self.conv_transpose_3d.forward(input);

                    output
                }
            }
        };

        assert_tokens(graph.codegen(), expected);
    }
}
