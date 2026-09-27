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

    let contents = match fs::read_to_string("compilo.toml") {
        Ok(contents) => contents,
        Err(_) => {
            println!("Error: compilo.toml not found.");
            println!("Run `compilo init` to create a project.");
            std::process::exit(1);
        }
    };
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

fn link(objects: &[PathBuf], output: &Path, linker: &str) {
    println!("Linking -> {:?}", output);

    let status = Command::new(linker)
        .args(objects)
        .arg("-o")
        .arg(output)
        .status()
        .unwrap();

    if !status.success() {
        println!("Linking failed..");
    }
}

fn init_project(is_c: bool, project_name: &str) {
    let project_dir = PathBuf::from(project_name);
    fs::create_dir_all(project_dir.join("src")).unwrap();
    fs::create_dir_all(project_dir.join("build")).unwrap();

    let config = format!(
        r#"[project]
name = "{}"
source = "src"
build = "build"
"#, 
        project_name
    );

    fs::write(
        project_dir.join("compilo.toml"),
        config,
    )
    .unwrap();

    if is_c {
        fs::write(
            project_dir.join("src/main.c"),
            r#"#include <stdio.h>

int main() {
    printf("Hello from Compilo!\n");
    return 0;
}
"#,
        )
        .unwrap();

        println!("Created C project: {}", project_name);
    } else {
        fs::write(
            project_dir.join("src/main.cpp"),
            r#"#include <iostream>

int main() {
    std::cout << "Hello from Compilo!\n";
    return 0;
}
"#,
        )
        .unwrap();

        println!("Created C++ project: {}", project_name);
    }

    println!("Created {}", project_name);
    println!("Created {}/compilo.toml", project_name);
    println!("Created {}/src/", project_name);
    println!("Created {}/build/", project_name);}

fn build_project() {

    println!("Building project...");
    let config = read_config();

    println!("Project: {}", config.project.name);
    let source_dir = PathBuf::from(&config.project.source);
    let build_dir = PathBuf::from(&config.project.build);

    fs::create_dir_all(&build_dir).unwrap();

    let mut objects = Vec::new();
    let mut linker = "g++";
    let entries = fs::read_dir(&source_dir).unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if let Some(extension) = path.extension() {
            if let Some(extension) = extension.to_str() {

                if let Some(compiler) = get_compiler(extension) {

                    if extension == "c" {
                        linker = "gcc";
                    }
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
    let executable = build_dir.join(&config.project.name);

    if needs_link(&objects, &executable) {
        link(&objects, &executable, linker);
    } else {
        println!("Skipping linking..");
    }

}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] =="init" {
        let is_c = args.len() >2 && args[2] == "--c";
        let project_name;

        if is_c {
            if args.len() < 4 {
                println!("Usage: compilo init --c <porject-name>");
                return;
            }
            project_name = &args[3];
        } else {
            if args.len() < 3 {
                println!("Usage: compilo init --c <porject-name>");
                return;
            }

            project_name = &args[2];
        }
        init_project(is_c, project_name);
        return;
    }

    build_project();
}
