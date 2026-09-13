use std::fs;
use std::process::Command;
use std::path::PathBuf;
use std::path::Path;

// decide the compiler
fn get_compiler(extension: &str) -> Option<&str> {
    if extension=="cpp" {
        return Some("g++")
    }

    if extension=="c" {
        return Some("gcc")
    }

    None
}

// get the output of the path
fn get_output_path(source: &PathBuf) -> PathBuf {

    let filename = source.file_stem().unwrap(); // gets the filename 

    let mut output = PathBuf::from("test_project/build");

    output.push(filename);
    output.set_extension("o");

    output
}

// function to compile every file
fn compile(source: &Path, output: &Path, compiler: &str) {
    println!("Compiling {:?}", source);

    let status = Command::new(compiler)
        .args([
            "-c",
        ])
        .arg(source)
        .arg("-o")
        .arg(output)
        .status()
        .unwrap();
    println!("Finished : {:?}\n", status);
}



fn main() {
    println!("Compilo build System\n");

    fs::create_dir_all("test_project/build").unwrap();

    let entries = fs::read_dir("test_project/src").unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if let Some(extension) = path.extension() {
            if let Some(extension) = extension.to_str() {

                if let Some(compiler) = get_compiler(extension) {
                    // println!("{:?} -> {}", path, compiler);
                    // println!("Found: {:?}", path);
                    // println!("compiling: {}", compiler);
                    //
                    let output = get_output_path(&path);
                    compile(&path, &output, compiler);


                }
            }

        }

    }

}
