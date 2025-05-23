pub mod onnx_cpu;
pub mod device;

pub trait AiHal {
    fn load_model(&mut self, path: &str) -> anyhow::Result<()>;
    fn infer(&self, input: &[f32]) -> anyhow::Result<Vec<f32>>;
    fn device_name(&self) -> String;
}
