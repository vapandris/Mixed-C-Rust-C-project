use std::path::Path;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write, Read};

fn main() {
    let out_path = Path::new("./src/bindings.rs");
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&out_path);

    match file {
        Err(error) => {
            println!("cargo:warning={error}");
        }

        Ok(_) => {
            let types_dir = Path::new("Types");
            let math_dir = Path::new("Math");

            let bindings = bindgen::Builder::default()
                .header(types_dir.join("types.h").to_str().unwrap())
                .header(math_dir.join("add.h").to_str().unwrap())
                .header(math_dir.join("mult.h").to_str().unwrap())
                .header(math_dir.join("pow.h").to_str().unwrap())
                //.blocklist_function("mult") // Comment out this line if you'd like to generate mult
                .clang_arg(format!("-I{}", types_dir.to_str().unwrap()))
                .clang_arg(format!("-I{}", math_dir.to_str().unwrap()))
                .generate();

            match bindings {
                Err(error) => {
                    println!("cargo:warning={error}");
                }

                Ok(bindings) => {
                    bindings.write_to_file(out_path)
                        .expect(format!("Couldn't write to {}", out_path.to_str().unwrap()).as_str());

                    prepend_to_file(
                        out_path.to_str().unwrap(),
                        "#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]\n"
                    ).expect("Couldn't append to the start of file.");
                }
            }

            cc::Build::new()
                .file(math_dir.join("add.c").to_str().unwrap())
                .file(math_dir.join("pow.c").to_str().unwrap())
                .include(math_dir)
                .include(types_dir)
                .compile("Math");
        }
    }
}

fn prepend_to_file(filename: &str, content: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)?;

    let mut old_content = Vec::new();
    file.read_to_end(&mut old_content)?;

    file.seek(SeekFrom::Start(0))?;
    file.write_all(content.as_bytes())?;
    file.write_all(&old_content)?;

    Ok(())
}