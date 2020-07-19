pub use rdxl_static_macros::*;

pub struct DotHtml {
   content: String,
}

pub enum Dot {
   Html(DotHtml)
}

pub struct DotLhs {
   filepath: String,
}

fn build_dirs(dir: &std::path::Path) -> std::io::Result<Vec<(DotLhs,Dot)>> {
    let mut dots = Vec::new();
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                build_dirs(&path)?;
            } else {
                let _syntax = syn::parse_file(&path.to_str().unwrap_or("[non-unicode pathname]"))
                             .expect(&format!("Unable to parse file: {:?}", path));
                println!("extract dots from file: {:?}", path);
                //extract dots from file
            }
        }
    }
    Ok(dots)
}

pub fn build() {
   let _dots = build_dirs(&std::path::Path::new("src")).expect("could not extract dots from source directory");
   std::fs::File::create("src/site.rs").expect("could not create src/site.rs");
}
