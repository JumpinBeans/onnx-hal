mod hal;

fn main() -> anyhow::Result<()> {
    let mut ai = hal::device::get_best_hal();
    println!("Using: {}", ai.device_name());

    ai.load_model("models/placeholder.onnx")?;
    let result = ai.infer(&[1.0, 2.0, 3.0])?;
    println!("Output: {:?}", result);

    Ok(())
}
