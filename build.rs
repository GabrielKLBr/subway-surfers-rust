extern crate winres;

use std::path::{Path, PathBuf};
use std::fs;

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            copy_dir_all(&path, &dest_path)?; // recursão limpa usando Path
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

fn check_debug_build() {
    let main_path = Path::new("src/main.rs");
    let code = fs::read_to_string(main_path).expect("Falha ao ler src/main.rs");
    let profile = std::env::var("PROFILE").unwrap();
    let is_release = profile == "release";
    let desired_attr = if is_release {
        "#![windows_subsystem = \"windows\"]"
    } else {
        "#![windows_subsystem = \"console\"]"
    };

    let mut lines: Vec<&str> = code.lines().collect();
    if let Some(first_line) = lines.first() {
        if first_line.starts_with("#![windows_subsystem") {
            lines.remove(0);
        }
    }

    let mut new_code = String::new();
    new_code.push_str(desired_attr);
    new_code.push('\n');
    new_code.push_str(&lines.join("\n"));
    fs::write(main_path, new_code).expect("Falha ao sobrescrever src/main.rs");
    println!("cargo:rerun-if-changed=src/main.rs");
}

fn add_executable_icon() {
    if cfg!(target_os = "windows") {    
        winres::WindowsResource::new()
            .set_icon_with_id("favicon.ico", "6969")
            .compile().unwrap();    
    } else {
        println!("cargo:warning=This build script in not configured to add icons for this os.");
    }
}

fn main() {
    let assets = Path::new("assets");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = PathBuf::from(&out_dir).join("../../../").join(assets);

    if dest.exists() {
        fs::remove_dir_all(&dest).unwrap();
    }

    fs::create_dir_all(&dest).unwrap();
    copy_dir_all(assets, &dest).unwrap();
    check_debug_build();
    println!("cargo:rerun-if-changed=assets");

    add_executable_icon();

    let cpp_file = Path::new("native/webview.cpp");
    let obj_file = Path::new("native/webview.obj");

    //Checks if needs rebuild
    let needs_rebuild = match (fs::metadata(cpp_file), fs::metadata(obj_file)) {
        (Ok(cpp_meta), Ok(obj_meta)) => {
            cpp_meta.modified().unwrap() > obj_meta.modified().unwrap()
        }
        _ => true, //If the .obj file doesen't exists, it compiles
    };

    if needs_rebuild {
        println!("cargo:warning=Recompiling {}", cpp_file.display());
        cc::Build::new()
            .cpp(true)
            .file(cpp_file)
            .static_crt(true)
            .flag("/std:c++17")
            .compile("webview");
    } else {
        println!("cargo:warning=Using existing object {}", obj_file.display());
    }

    println!("cargo:rerun-if-changed=native/webview.cpp");
    println!("cargo:rustc-link-lib=static=advapi32");
    println!("cargo:rustc-link-lib=static=ole32");
    println!("cargo:rustc-link-lib=static=shlwapi");
    println!("cargo:rustc-link-lib=static=shell32");
    println!("cargo:rustc-link-lib=static=version");
    println!("cargo:rustc-link-lib=static=dwmapi");
    println!("cargo:rustc-link-lib=static=user32");
}