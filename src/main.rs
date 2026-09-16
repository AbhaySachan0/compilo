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


fn get_dependencies(contents: &str) -> Vec<&str> {
    let contents = contents.replace("\\\n", " ");
    let parts: Vec<&str> = contents.split(':').collect();

    if parts.len()<2 {
        return Vec::new();
    }

    let dependencies = parts[1];
    dependencies.split_whitespace().map(String::from).collect()
}

fn get_dependency_path(object: &Path) -> PathBuf {
    let mut dependency = object.to_path_buf();
    dependency.set_extension("d");

    dependency
}

// tells if the file need to compile again
fn needs_rebuild(source: &Path, object: &Path) -> bool {
    if !object.exists() {
        return true;
    }
    let source_time = fs::metadata(source)
        .unwrap()
        .modified()
        .unwrap();

    let object_time = fs::metadata(object)
        .unwrap()
        .modified()
        .unwrap();

    if source_time > object_time {
        return true;
    }

    let dependency_path = get_dependency_path(object);
    if !dependency_path.exists() {
        return true;
    }

    let contents = fs::read_to_string(&dependency_path).unwrap();

    let dependencies = get_dependencies(&contents);

    for dependency in dependencies {
        let dependency_path = Path::new(dependency);
        let dependency_time = fs::metadata(dependency_path)
            .unwrap()
            .modified()
            .unwrap();

        if dependency_time > object_time {
            return true;
        }
    }

    false
}

// function to compile every file
fn compile(source: &Path, output: &Path, compiler: &str) {
    println!("Compiling {:?}", source);

    let status = Command::new(compiler)
        .arg("-MMD")
        .arg("-MP")
        .arg("-c")
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

                    if needs_rebuild(&path, &output) {
                        compile(&path, &output, compiler);
                    } else {
                        println!("Skipping {:?}", path);
                    }


                }
            }

        }

    }

}
