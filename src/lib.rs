pub use rdxl_static_macros::*;

pub fn dot_to_file<P: Into<String>, D: Into<String>>(p: P, dat: D) -> std::io::Result<()> {
   let p: String = p.into();
   let dat: String = dat.into();
   let pr = p.rfind("/").expect("dot_to_file expected a / in directory path");
   let pd = p[..pr].to_string();
   let p = std::path::Path::new(&p);
   std::fs::create_dir_all(pd)?;
   let mut f = std::fs::File::create(p)?;
   std::io::Write::write_all(&mut f, dat.as_bytes())?;
   Ok(())
}
