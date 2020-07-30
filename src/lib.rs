pub use rdxl_static_macros::*;

pub fn write_to_file(fp: &str, dat: &str) -> std::io::Result<()> {
   if let Some(pr) = fp.rfind("/") {
      let pd = fp[..pr].to_string();
      std::fs::create_dir_all(pd)?;
   }
   let p = std::path::Path::new(&fp);
   let mut f = std::fs::File::create(p)?;
   std::io::Write::write_all(&mut f, dat.as_bytes())?;
   Ok(())
}
