use std::time::Instant;

fn main() {
    println!("Starting font serialization");
    let start = Instant::now();
    let _ = firecore_font_builder::compile("fonts");
    println!("Finished serializing fonts in {}ms.", start.elapsed().as_millis());
}