
#[test]
fn dot1() {
   if std::path::Path::new("www").exists() {
      std::fs::remove_dir_all("www").unwrap();
   }
   rdxl_static::dot_to_file("/index.html", "crate::index").unwrap();
}
