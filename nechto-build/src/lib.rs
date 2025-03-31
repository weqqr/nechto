use std::path::Path;
use std::process::Command;

pub fn build_shaders(source_prefix: &str, output_prefix: &str, paths: &[&str]) {
    std::fs::create_dir_all(output_prefix).unwrap();

    for path in paths {
        println!("cargo::rerun-if-changed={}", path);

        let source = Path::new(source_prefix).join(path);
        let output = Path::new(output_prefix).join(path).with_extension("spv");

        build_slang_file(&source, &output);
    }
}

fn build_slang_file(source: &Path, output: &Path) {
    let cmd = Command::new("slangc")
        .arg(source)
        .arg("-fvk-use-entrypoint-name") // set OpEntryPoint to the expected value
        .arg("-o")
        .arg(output)
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
        }
    }
}

pub fn build_scripts(source_dir: impl AsRef<Path>, output_dir: impl AsRef<Path>) {
    std::fs::create_dir_all(output_dir.as_ref()).unwrap();

    for entry in std::fs::read_dir(source_dir).unwrap() {
        let entry = entry.unwrap();

        let output_path = output_dir.as_ref().join(entry.file_name());

        if entry.file_type().unwrap().is_dir() {
            build_scripts(entry.path(), output_path);
        } else {
            std::fs::copy(entry.path(), output_path).unwrap();
        }
    }
}

fn print_text_as_error(text: &str) {
    for line in text.lines() {
        println!("cargo::error={}", line);
    }
}
