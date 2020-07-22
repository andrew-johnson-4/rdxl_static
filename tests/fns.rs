
#[test]
fn fn1() {
   assert_eq!(
      rdxl_static::fn_from_path_parts(std::path::Path::new("src/lib.rs"), "index".to_string()),
      "crate::index"
   )
}
