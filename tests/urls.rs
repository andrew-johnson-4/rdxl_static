
#[test]
fn url1() {
   assert_eq!(
      rdxl_static::url_from_path_parts(std::path::Path::new("src/lib.rs"), "index".to_string(), "html".to_string()),
      "/index.html"
   )
}
