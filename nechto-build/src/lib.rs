use std::path::Path;
use std::process::Command;

pub fn build_shaders(source_prefix: &str, output_prefix: &str, paths: &[&str]) {
    std::fs::create_dir_all(output_prefix).unwrap();

    for path in paths {
        println!("cargo::rerun-if-changed={}", path);

        let cmd = Command::new("slangc")
            .arg(Path::new(source_prefix).join(path))
            .arg("-fvk-use-entrypoint-name") // set OpEntryPoint to the expected value
            .arg("-o")
            .arg(Path::new(output_prefix).join(path).with_extension("spv"))
            .output();

        match cmd {
            Ok(output) => {
                if !output.stdout.is_empty() {
                    print_text_as_error(&String::from_utf8(output.stdout).unwrap());
                }

                if !output.stderr.is_empty() {
                    print_text_as_error(&String::from_utf8(output.stderr).unwrap());
                }
            }
            Err(err) => {
                print_text_as_error(&format!("unable to run Slang: {}", err));
                return;
            }
        }
    }
}

fn print_text_as_error(text: &str) {
    for line in text.lines() {
        println!("cargo::error={}", line);
    }
}
