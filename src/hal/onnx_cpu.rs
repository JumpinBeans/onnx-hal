use onnxruntime::{environment::Environment, session::Session, tensor::OrtOwnedTensor};
use crate::hal::AiHal;

pub struct OnnxCpuHal {
    session: Session<'static>,
}

impl OnnxCpuHal {
    pub fn new(model_path: &str) -> anyhow::Result<Self> {
        let env = Environment::builder().with_name("onnx").build()?;
        let session = env
            .new_session_builder()?
            .with_model_from_file(model_path)?;
        Ok(Self { session })
    }
}

impl AiHal for OnnxCpuHal {
    fn load_model(&mut self, _path: &str) -> anyhow::Result<()> {
        Ok(())
    }

    fn infer(&self, input: &[f32]) -> anyhow::Result<Vec<f32>> {
        // Placeholder inference
        Ok(input.to_vec())
    }

    fn device_name(&self) -> String {
        "ONNX CPU Runtime".into()
    }
}
