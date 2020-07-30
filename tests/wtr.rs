use rdxl_static::write_to_file;

#[test]
fn write1() {
   write_to_file("www/foo.txt","aaaa").expect("write_to_file");
}
