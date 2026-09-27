use std::fs;
use std::process::Command;
use std::path::PathBuf;
use std::path::Path;

use serde::Deserialize;

use std::env;

#[derive(Deserialize)]
struct Config {
    project: Project,
}

#[derive(Deserialize)]
struct Project {
    name: String,
    source: String,
    build: String,
}

fn read_config() -> Config {
    let contents = fs::read_to_string("test_project/compilo.toml").unwrap();

    let config: Config = toml::from_str(&contents).unwrap();

    config
}

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
fn get_output_path(source: &Path, build_dir: &Path) -> PathBuf {

    let filename = source.file_stem().unwrap(); // gets the filename 

    let mut output = build_dir.to_path_buf();

    output.push(filename);
    output.set_extension("o");

    output
}


fn get_dependencies(contents: &str) -> Vec<String> {
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
fn needs_rebuild(object: &Path) -> bool {
    if !object.exists() {
        return true;
    }
    let object_time = fs::metadata(object)
        .unwrap()
        .modified()
        .unwrap();

    let dependency_path = get_dependency_path(object);
    if !dependency_path.exists() {
        return true;
    }

    let contents = fs::read_to_string(&dependency_path).unwrap();

    let dependencies = get_dependencies(&contents);

    for dependency in dependencies {
        let dependency_path = Path::new(&dependency);
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

fn needs_link(objects: &[PathBuf], executable: &Path) -> bool {
    if !executable.exists() {
        return true;
    }

    let executable_time = fs::metadata(executable)
        .unwrap()
        .modified()
        .unwrap();

    for object in objects {
        let object_time = fs::metadata(object)
            .unwrap()
            .modified()
            .unwrap();

        if object_time > executable_time {
            return true;
        }
    }
    return false;
}

fn link(objects: &[PathBuf], output: &Path) {
    println!("Linking -> {:?}", output);

    let status = Command::new("g++")
        .args(objects)
        .arg("-o")
        .arg(output)
        .status()
        .unwrap();

    if !status.success() {
        println!("Linking failed..");
    }
}



fn main() {
    println!("Compilo build System\n");

    let config = read_config();

    let project_dir = PathBuf::from("test_project");
    let source_dir = project_dir.join(&config.project.source);
    let build_dir = project_dir.join(&config.project.build);

    fs::create_dir_all(&build_dir).unwrap();

    let mut objects = Vec::new();
    let entries = fs::read_dir(&source_dir).unwrap();

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
                    let output = get_output_path(&path, &build_dir);
                    objects.push(output.clone());

                    if needs_rebuild(&output) {
                        compile(&path, &output, compiler);
                    } else {
                        println!("Skipping {:?}", path);
                    }


                }
            }

        }
    }
    let executable = PathBuf::from("test_project/build/app");

    if needs_link(&objects, &executable) {
        link(&objects, &executable);
    } else {
        println!("Skipping linking..");
    }

}
