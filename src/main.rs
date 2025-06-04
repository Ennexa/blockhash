///src/main.rs

use std::process;
use image::DynamicImage;
use clap::Parser;
use blockhash::{blockhash16, blockhash64, blockhash144, blockhash256};

/// Calculate and print the blockhash of one or more images.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // clap will use version from Cargo.toml
struct Args {
    /// Path(s) to the image file(s)
    #[arg(required = true, num_args = 1..)] // Mark as required and allow one or more values
    image_paths: Vec<String>,

    /// Create hash of size N^2 bits.
    #[arg(long = "bits", short = 'b', default_value_t = 16, value_name = "N", help = "Create hash of size N^2 bits (N can be 4, 8, 12, or 16). Default: 16 (256 bits)")]
    n_value: usize,
}

fn calculate_hash_for_image(image_path: &str, n_value: usize) -> Option<String> {
    // Open the image
    let img: DynamicImage = match image::open(image_path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error opening image '{}': {}", image_path, e);
            return None; // Return None if image opening fails
        }
    };

    // Calculate the blockhash based on the specified N value
    match n_value {
        4 => Some(blockhash16(&img).to_string()), // N=4, so 4*4 = 16 bits
        8 => Some(blockhash64(&img).to_string()), // N=8, so 8*8 = 64 bits
        12 => Some(blockhash144(&img).to_string()), // N=12, so 12*12 = 144 bits
        16 => Some(blockhash256(&img).to_string()), // N=16, so 16*16 = 256 bits
        _ => {
            // This case should ideally be caught by clap's validation or initial arg parsing if we restricted values
            // but as a safeguard:
            eprintln!("Error: Invalid value for N (provided via --bits or -b). Allowed N values are 4, 8, 12, or 16.");
            process::exit(1);
        }
    }
}


fn main() {
    let args = Args::parse();

    // Validate N value once at the beginning
    if ![4, 8, 12, 16].contains(&args.n_value) {
        eprintln!("Error: Invalid value for N ({}). Allowed N values are 4 (16 bits), 8 (64 bits), 12 (144 bits), or 16 (256 bits).", args.n_value);
        process::exit(1);
    }

    for image_path in &args.image_paths {
        if let Some(hash_string) = calculate_hash_for_image(image_path, args.n_value) {
            println!("{}  {}", hash_string, image_path);
        }
    }
}
