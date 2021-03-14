use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting font serialization");
    let start = Instant::now();
    firecore_font_builder::build_font("fonts", "output/fonts.bin")?;
    println!("Finished serializing fonts in {}ms.", start.elapsed().as_millis());
    Ok(())
}