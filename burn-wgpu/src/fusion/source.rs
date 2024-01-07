use crate::{
    codegen::ComputeShader,
    kernel::{DynamicKernelSource, SourceTemplate},
};

#[derive(new, Clone)]
pub struct FusedKernelSource {
    pub(crate) id: String,
    pub(crate) shader: ComputeShader,
}

impl DynamicKernelSource for FusedKernelSource {
    fn source(&self) -> SourceTemplate {
        SourceTemplate::new(self.shader.to_string())
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}
