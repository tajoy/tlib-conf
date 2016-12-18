
#![crate_name = "tlib_conf"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![doc(html_logo_url = "http://tajoy.net/img/logo.jpg",
       html_favicon_url = "https://doc.rust-lang.org/favicon.ico",
       html_root_url = "http://tajoy.net/",
       html_playground_url = "https://play.rust-lang.org/",
       test(attr(allow(unused_variables), deny(warnings))))]

pub use conf::*;

pub mod configer;
mod conf;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
