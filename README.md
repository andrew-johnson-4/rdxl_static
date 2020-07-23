# rdxl_static

[![Crates.IO](https://img.shields.io/crates/v/rdxl_static.svg)](https://crates.rs/crates/rdxl_static)
[![Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/rdxl_static/)
[![Build Nightly](https://github.com/andrew-johnson-4/rdxl_static/workflows/BuildNightly/badge.svg)](https://github.com/andrew-johnson-4/rdxl_static)
[![Build](https://github.com/andrew-johnson-4/rdxl_static/workflows/Build/badge.svg)](https://github.com/andrew-johnson-4/rdxl_static)

Utility Crate to Compile Static Sites based on RDXL macros

```rust
#[dot_template]
pub fn custom_template(title: String, description: String, xhtml: String) -> String {
   xhtml!(
     <html>
       <head>
         <title>{{ title }}</title>
         <meta name="description" content={{description}}/>
       </head>
       <body>{{ xhtml }}</body>
     </html>
   )
}

#[dot]
fn this_function_is_a_webpage() -> String {
   dot_html!(
      <p>This webpage uses the default HTML Template.</p>
   )
}

#[dot]
fn this_function_is_also_a_webpage() -> String {
   dot_html!(
      template=custom_template,
      title="Hello World",
      description="Classic Cinematic Drama Movie Reviews",
      <p>This year was not a good year for Cinema.</p>
   )
}
```

There is a [template for starting new sites](https://github.com/andrew-johnson-4/rdxl_static_template) with rdxl_static.
