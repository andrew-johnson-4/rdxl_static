pub use rdxl_static_macros::*;

pub struct DotHtml {
   content: String,
}

pub enum Dot {
   Html(DotHtml)
}

pub fn build() {
}
