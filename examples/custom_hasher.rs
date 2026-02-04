use std::{collections::HashMap, time::Instant};

use rustc_hash::FxBuildHasher;
use tomldir::{Config, Value};

fn main() -> tomldir::Result<()> {
    let path = "examples/example.toml";
    let iterations = 1000;

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = Config::from_file(path)?;
    }
    let default_time = start.elapsed();

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = Config::<HashMap<String, Value, FxBuildHasher>>::from_file_with(path)?;
    }
    let fx_time = start.elapsed();

    let cfg = Config::<HashMap<String, Value, FxBuildHasher>>::from_file_with(path)?;

    println!("Default HashMap: {:?}", default_time);
    println!("FxHashMap:       {:?}", fx_time);
    println!(
        "Speedup:         {:.1}x\n",
        default_time.as_secs_f64() / fx_time.as_secs_f64()
    );

    println!("Runners: {}", cfg.get_int("concurrent").unwrap());
    println!(
        "First runner: {}",
        cfg.get_string("runners[0].name").unwrap()
    );
    println!(
        "Docker image: {}",
        cfg.get_string("runners[1].docker.image").unwrap()
    );

    Ok(())
}
