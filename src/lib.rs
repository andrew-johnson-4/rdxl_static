pub use rdxl_static_macros::*;

pub struct DotHtml {
   filepath: String,
   content: String,
}

pub enum Dot {
   Html(DotHtml)
}

pub fn build() {
}
