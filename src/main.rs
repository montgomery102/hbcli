use std::process::{Command, Stdio};
use std::fs;
use std::path::{Path, PathBuf};
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <input_path> <output_path> [preset_path]", args[0]);
        println!("Note: preset_path is optional. If left off Handbrake will encode with default “Normal” Preset.")
        return Ok(());
    }

    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);
    let preset_path = if args.len() >= 4 {
        Some(Path::new(&args[3]))
    } else {
        None
    };

    if !input_path.exists() || !input_path.is_dir() {
        eprintln!("Error: Input path '{}' does not exist or is not a directory.", input_path.display());
        return Ok(());
    }

    if !output_path.exists() || !output_path.is_dir() {
        eprintln!("Error: Output path '{}' does not exist or is not a directory.", output_path.display());
        return Ok(());
    }

    if !preset_path.exists() || !preset_path.is_file() {
        eprintln!("Error: Preset file '{}' does not exist or is not a file.", preset_path.display());
        return Ok(());
    }

    //println!("Input Directory: {:?}", input_path);
    //println!("Output Directory: {:?}", output_path);
    //println!("Preset File: {:?}", preset_path);

    let mut files_processed = 0;
    let mut input_files = Vec::new();
    find_files(input_path, &mut input_files);

    for input_file in input_files {
        let file_size = input_file.metadata()?.len();
        if file_size > 400 * 1024 * 1024 {
            let input_file_name = input_file.file_name().unwrap().to_string_lossy();
            let output_file_name = format!("{}.mp4", input_file_name.to_string().replace(".mkv", ""));
            let output_file_path = output_path.join(&output_file_name);

            //println!("Processing file: {:?}", input_file);

            let output = Command::new("C:\\Program Files\\HandBrake\\HandBrakeCLI.exe")
                .arg("-i").arg(&input_file)
                .arg("-o").arg(&output_file_path)
                .arg("--preset-import-gui").arg(&preset_path)
                .output()
                .expect("Failed to start HandBrakeCLI");

            if !output.status.success() {
                eprintln!("HandBrakeCLI failed with: {:?}. Error output:", output.status.code());
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            } else {
                println!("Successfully converted {:?} to {:?}", input_file, output_file_path);
                files_processed += 1;
            }
        } else {
            //println!("Skipping file {:?} because it is smaller than 400MB.", input_file);
        }
    }
if files_processed == 0 {
    eprintln!("No files were processed.");
    std::process::exit(1);
}

    println!("Processed {} files.", files_processed);

    Ok(())
}

fn find_files(dir: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    files.push(path);
                } else if path.is_dir() {
                    find_files(&path, files);
                }
            }
        }
    }
}
