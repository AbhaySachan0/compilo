use std::fs;

fn main() {
    println!("Compilo build System");

    let entries = fs::read_dir("test_project/src").unwrap();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension() == Some("cpp".as_ref()) || path.extension() == Some("c".as_ref()){

            println!("Source : {:?}",path);
        }
    }
}
