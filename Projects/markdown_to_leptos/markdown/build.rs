use walkdir::WalkDir;

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");

    println!("cargo:rerun-if-changed=../Docs/");

    // Watch every file in the folder recursively
    for entry in WalkDir::new("../Docs").into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            println!("cargo:rerun-if-changed={}", entry.path().display());
        }
    }
}
