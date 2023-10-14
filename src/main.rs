extern crate regex;
mod german;
mod multi_lang;
mod entities;
mod icons;
mod read_line;
use std::env;
mod tests;
mod thisis;
use crate::tests::run_tests;
use crate::multi_lang::Lang;
use crate::multi_lang::print_update_at;
use crate::multi_lang::print_type_exit_to_exit;

fn main() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const APPNAME: &str = env!("CARGO_PKG_NAME");
    let mut lang: Lang = Lang::German;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1].eq("french") {
            lang=Lang::French;
        } else if args[1].eq("spanish") {
            lang=Lang::Spanish;
        }
    }
    
    println!("############################");
    println!("#");
    println!("# {} v.{}", APPNAME, VERSION);
    print_update_at("14.10.2023", lang);
    println!("#");
    println!("############################");
    println!("");
    print_type_exit_to_exit(lang);
    run_tests(lang)
}
