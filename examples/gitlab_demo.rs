use tomldir::Config;

fn main() -> tomldir::Result<()> {
    println!("--- Loading GitLab Runner Config ---");
    // In a real app you'd use Config::from_file("examples/example.toml")
    // But for `cargo run --example` the CWD varies, so we'll include bytes for
    // stability or just assume CWD. Let's assume CWD is project root as per
    // usual `cargo run`.
    let cfg = Config::from_file("examples/example.toml")?;

    // Access generic properties
    if let Some(concurrent) = cfg.get_int("concurrent") {
        println!("Global concurrency limit: {}", concurrent);
    }

    println!("\n--- Exploring Flattened Keys ---");
    // Flatten to HashMap to easily inspect keys
    let flat = cfg.flatten();

    // We can iterate over the keys to find runners.
    // Since we know the structure, let's look for specific runners.

    // 1. Shell Runner (first one)
    println!("\n[Runner 0: Shell]");
    println!("Name: {}", flat.get("runners[0].name").unwrap());
    println!("Executor: {}", flat.get("runners[0].executor").unwrap());

    // 2. Docker Runner (second one)
    println!("\n[Runner 1: Docker]");
    println!("Name: {}", flat.get("runners[1].name").unwrap());
    println!("Image: {}", flat.get("runners[1].docker.image").unwrap());

    // 3. SSH Runner (third one)
    println!("\n[Runner 2: SSH]");
    println!("Name: {}", flat.get("runners[2].name").unwrap());
    println!("Host: {}", flat.get("runners[2].ssh.host").unwrap());
    println!("Port: {}", flat.get("runners[2].ssh.port").unwrap());

    println!("\n--- Full Flattened Dump (sorted) ---");
    // Sort for display
    let mut keys: Vec<_> = flat.keys().collect();
    keys.sort();
    for key in keys {
        println!("{} = {}", key, flat.get(key).unwrap());
    }

    Ok(())
}
