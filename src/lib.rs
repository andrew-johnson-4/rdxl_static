pub use rdxl_static_macros::*;

pub struct DotHtml {
   pub content: String,
}

pub enum Dot {
   Html(DotHtml)
}

pub struct DotLhs {
   pub filepath: String,
}

pub fn url_from_path_parts(p: &std::path::Path, i: String, dot: String) -> String {
   let p = format!("{:?}", p);
   let pl = p.find("/").unwrap();
   let pr = p.find("/").unwrap();
   let p = p[pl..pr+1].to_string();
   format!("{}{}.{}", p, i, dot)
}

fn build_dirs(dir: &std::path::Path) -> std::io::Result<Vec<(DotLhs,Dot)>> {
    use std::io::Read;
    let mut dots = Vec::new();
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                build_dirs(&path)?;
            } else {
                let mut file = std::fs::File::open(&path).expect(&format!("Unable to open file: {:?}", path));
                let mut src = String::new();
                file.read_to_string(&mut src).expect(&format!("Unable to read file: {:?}", path));

                let file_content = syn::parse_file(&src)
                                   .expect(&format!("Unable to parse file: {:?}", path));

                for i in file_content.items.iter() {
                   if let syn::Item::Fn(f) = i {
                      let mut is_dot = false;
                      for at in f.attrs.iter() {
                         if at.path.is_ident("dot") {
                            is_dot = true;
                         }
                      }
                      if is_dot {
                         let u = url_from_path_parts(&path, f.sig.ident.to_string(), "html".to_string());
                         println!("cargo:warning=dot-fn {}", u);
                      }
                   }
                }
                println!("cargo:warning=extract dots from file: {:?}", path);
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
