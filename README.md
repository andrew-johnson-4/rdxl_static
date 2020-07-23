# rdxl_static
Utility Crate to Compile Static Sites based on RDXL macros

```rust
#[dot]
fn this_function_is_a_webpage() -> DotHtml {
   dot_html!(
      <p>This webpage uses the default HTML Template.</p>
   )
}

#[dot]
fn this_function_is_also_a_webpage() -> DotHtml {
   dot_html!(
      template="other_template.html",
      title="Hello World",
      description="Classic Cinematic Drama Movie Reviews",
      <p>This year was not a good year for Cinema.</p>
   )
}
```

There is a [template for starting new sites](https://github.com/andrew-johnson-4/rdxl_static_template) with rdxl_static.
