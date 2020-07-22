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

pub fn dot_to_file(cf: &str, dat: &str) -> std::io::Result<()> {
   let p = format!("www{}", cf);
   let pr = p.rfind("/").unwrap();
   let pd = p[..pr].to_string();
   let p = std::path::Path::new(&p);
   std::fs::create_dir_all(pd)?;
   let mut f = std::fs::File::create(p)?;
   std::io::Write::write_all(&mut f, dat.as_bytes())?;
   Ok(())
}

pub fn url_from_path_parts(p: &std::path::Path, i: String, dot: String) -> String {
   let p = format!("{:?}", p);
   let pl = p.find("/").unwrap();
   let pr = p.find("/").unwrap();
   let p = p[pl..pr+1].to_string();
   format!("{}{}.{}", p, i, dot)
}
pub fn fn_from_path_parts(p: &std::path::Path, i: String) -> String {
   let p = format!("{:?}", p);
   let pl = p.find("/").unwrap();
   let pr = p.find("/").unwrap();
   let p = p[pl..pr+1].to_string().replace("/","::");
   format!("crate{}{}", p, i)
}

fn build_dirs(dir: &std::path::Path) -> std::io::Result<Vec<(String,String)>> {
    use std::io::Read;
    let mut dots = Vec::new();
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                dots.append(&mut build_dirs(&path)?);
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
                         let cf = fn_from_path_parts(&path, f.sig.ident.to_string());
                         dots.push((u, cf));
                      }
                   }
                }
            }
        }
    }
    Ok(dots)
}

pub fn build() {
   let dots = build_dirs(&std::path::Path::new("src")).expect("could not extract dots from source directory");

   let mut site = std::fs::File::create("src/site.rs").expect("could not create src/site.rs");
   std::io::Write::write_all(&mut site, "pub fn run() {\n".as_bytes()).unwrap();
   for (u,cf) in dots.iter() {
      std::io::Write::write_all(&mut site, format!("    rdxl_static::dot_to_file(\"{}\",&{}().content).unwrap();\n",u,cf).as_bytes()).unwrap();
      println!("cargo:warning=dot-fn {} {}", u, cf);
   }
   std::io::Write::write_all(&mut site, "}\n".as_bytes()).unwrap();
}
