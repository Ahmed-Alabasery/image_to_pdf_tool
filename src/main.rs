use std::env;
use std::process::Command;

fn main() {
    // Check if ImageMagick is installed by running 'convert --version'
    // This ensures that the required tool is available before proceeding
    if Command::new("convert").arg("--version").output().is_err() {
        eprintln!("Error: ImageMagick is not installed. Install it with 'sudo apt install imagemagick' (Debian/Ubuntu) or 'sudo pacman -S imagemagick' (Arch). ");
        std::process::exit(1); // Exit the program with an error code
    }
    
    // Collect command-line arguments, skipping the first one (program name)
    let args: Vec<String> = env::args().skip(1).collect();
    
    // Ensure that at least one image file is provided
    if args.is_empty() {
        eprintln!("Usage: {} image1.jpg image2.png ...", env::args().next().unwrap());
        std::process::exit(1); // Exit with error if no arguments are given
    }
    
    // Define the output PDF filename
    let output_pdf = "output.pdf";
    
    // Execute the ImageMagick 'convert' command to combine images into a single PDF
    let status = Command::new("convert")
        .args(&args) // Pass all image filenames as arguments
        .arg(output_pdf) // Specify the output file
        .status() // Run the command and capture the status
        .expect("Failed to execute convert command"); // Panic if execution fails
    
    // Check if the command was successful
    if status.success() {
        println!("Conversion successful! PDF saved as '{}'", output_pdf);
    } else {
        eprintln!("Error: Conversion failed."); // Print error if conversion fails
        std::process::exit(1); // Exit with an error code
    }
}

