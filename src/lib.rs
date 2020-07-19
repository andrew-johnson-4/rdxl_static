pub use rdxl_static_macros::dot_html;

pub struct DotHtml {
   filepath: String,
   content: String,
}

pub enum Dot {
   Html(DotHtml)
}

pub fn build() {
}
