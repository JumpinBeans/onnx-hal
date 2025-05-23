use crate::hal::{onnx_cpu::OnnxCpuHal, AiHal};

pub fn get_best_hal() -> Box<dyn AiHal> {
    // Eventually detect platform/NPU
    Box::new(OnnxCpuHal::new("models/placeholder.onnx").unwrap())
}
