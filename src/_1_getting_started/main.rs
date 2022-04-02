use clap::{arg, Command};

mod macros;
mod shader;

#[cfg(feature = "chapter-1")]
mod _1_getting_started;

#[cfg(feature = "chapter-1")]
use _1_getting_started::*;

fn main() {
    let matches = Command::new("LearnOpenGL")
        .version("1.0")
        .arg(arg!([TUT]).help(
            "Call with the number of the tutorial, e.g. `1_2_2` for _2_2_hello_triangle_indexed.rs",
        ).default_value("1_6_2"))
        .get_matches();

    match matches
        .value_of("TUT")
        .expect("'tut' is required and parsing will fail if its missing")
    {
        #[cfg(feature = "chapter-1")]
        "1_2_2" => main_1_2_2(),
        #[cfg(feature = "chapter-1")]
        "1_2_3" => main_1_2_3_ex1(),
        #[cfg(feature = "chapter-1")]
        "1_3_1" => main_1_3_1(),
        #[cfg(feature = "chapter-1")]
        "1_3_2" => main_1_3_2(),
        #[cfg(feature = "chapter-1")]
        "1_3_3" => main_1_3_3(),
        #[cfg(feature = "chapter-1")]
        "1_4_1" => main_1_4_1(),
        #[cfg(feature = "chapter-1")]
        "1_4_2" => main_1_4_2(),
        #[cfg(feature = "chapter-1")]
        "1_4_6" => main_1_4_6(),
        #[cfg(feature = "chapter-1")]
        "1_5_1" => main_1_5_1(),
        #[cfg(feature = "chapter-1")]
        "1_5_2" => main_1_5_2(),
        #[cfg(feature = "chapter-1")]
        "1_6_1" => main_1_6_1(),
        #[cfg(feature = "chapter-1")]
        "1_6_2" => main_1_6_2(),
        _ => println!("Unknown tutorial id"),
    }
}
