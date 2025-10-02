fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mid_data = &mid::data("mykey")?;

    println!("MID data: {}", serde_json::to_string_pretty(mid_data)?);

    // Check if running under Rosetta
    if mid::is_running_under_rosetta() {
        println!("Running under Rosetta");
    } else {
        println!("Not running under Rosetta");
    }

    Ok(())
}
