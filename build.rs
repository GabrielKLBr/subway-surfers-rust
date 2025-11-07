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

fn main() {
    let assets = Path::new("assets");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = PathBuf::from(&out_dir).join("../../../").join(assets);

    if dest.exists() {
        fs::remove_dir_all(&dest).unwrap();
    }

    fs::create_dir_all(&dest).unwrap();
    copy_dir_all(assets, &dest).unwrap();
    println!("cargo:rerun-if-changed=assets");

    let cpp_file = Path::new("native/webview.cpp");
    let obj_file = Path::new("native/webview.obj");

    // Verifica se precisa recompilar
    let needs_rebuild = match (fs::metadata(cpp_file), fs::metadata(obj_file)) {
        (Ok(cpp_meta), Ok(obj_meta)) => {
            cpp_meta.modified().unwrap() > obj_meta.modified().unwrap()
        }
        _ => true, // Se o .obj não existir, compila
    };

    if needs_rebuild {
        println!("cargo:warning=Recompilando {}", cpp_file.display());
        cc::Build::new()
            .cpp(true)
            .file(cpp_file)
            .static_crt(true)
            .flag("/std:c++17")
            .compile("webview");
    } else {
        println!("cargo:warning=Usando objeto existente {}", obj_file.display());
    }

    // Garante rebuild se o .cpp mudar
    println!("cargo:rerun-if-changed=native/webview.cpp");
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=ole32");
    println!("cargo:rustc-link-lib=shlwapi");
    println!("cargo:rustc-link-lib=shell32");
    println!("cargo:rustc-link-lib=version");
    println!("cargo:rustc-link-lib=dwmapi");
    println!("cargo:rustc-link-lib=user32");
}