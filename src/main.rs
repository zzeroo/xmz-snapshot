#![doc(html_logo_url = "https://raw.githubusercontent.com/zzeroo/xmz-snapshot/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/zzeroo/xmz-snapshot/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
#![feature(stmt_expr_attributes)]

#[macro_use] extern crate clap;

mod cmd;

fn main() {
    println!("{:?}", cmd::read_command());
}
