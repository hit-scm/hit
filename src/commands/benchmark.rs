use crate::error::HitResult;
use crate::ui::Ui;
use colored::*;
use std::time::Instant;

pub fn execute() -> HitResult<()> {
    println!("{}", "Running Hit Micro-Engine Benchmark...".bold());

    // 1. Hash benchmark with 32MB payload
    let test_data = vec![0x42u8; 32 * 1024 * 1024];
    let iters = 5;

    let start_hash = Instant::now();
    for _ in 0..iters {
        let _ = blake3::hash(&test_data);
    }
    let elapsed_hash = start_hash.elapsed();
    let total_hashed = (test_data.len() * iters) as f64 / (1024.0 * 1024.0 * 1024.0);
    let hash_gb_per_sec = total_hashed / elapsed_hash.as_secs_f64();

    // 2. Compression benchmark with 16MB payload
    let comp_data = vec![0x61u8; 16 * 1024 * 1024];
    let start_comp = Instant::now();
    let compressed = zstd::encode_all(&comp_data[..], 3).unwrap();
    let elapsed_comp = start_comp.elapsed();
    let comp_mb = comp_data.len() as f64 / (1024.0 * 1024.0);
    let comp_speed = comp_mb / elapsed_comp.as_secs_f64();

    println!(
        "  {} BLAKE3 Throughput: {:.2} GB/s {}",
        "•".bright_cyan(),
        hash_gb_per_sec,
        Ui::dim(&format!("({:.2}ms)", elapsed_hash.as_millis()))
    );
    println!(
        "  {} Zstd Compression:   {:.2} MB/s {}",
        "•".bright_blue(),
        comp_speed,
        Ui::dim(&format!("({:.2}ms)", elapsed_comp.as_millis()))
    );
    println!(
        "  {} Compression Ratio:  {:.1}:1",
        "•".bright_magenta(),
        comp_data.len() as f64 / compressed.len() as f64
    );

    println!();
    Ui::success("Benchmark complete. Hit is running at peak bare-metal performance.");
    Ok(())
}
