use clap::{Parser};
use std::fs;
use std::path::PathBuf;
use piston_window::PistonWindow;

extern crate piston_window;

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct Cli {
    /// Path of the iNES file to be set in the emulator
    #[clap(short = 'p', long = "path", value_name = "FILE")]
    file: String
}

fn main() {
    let cli = Cli::parse();

    let path = PathBuf::from(cli.file);
    let f = match fs::canonicalize(&path) {
        Ok(file) => file,
        Err(error) => panic!("{:?}", error)
    };
    println!("{:?}", f);

    let width  = nes::RENDER_SCREEN_AREA_WIDTH as u32;
    let height = nes::RENDER_SCREEN_AREA_HEIGHT as u32;

    let mut window: PistonWindow = piston_window::WindowSettings::new("Nes", (width, height))
        .exit_on_esc(true)
        .graphics_api(piston_window::OpenGL::V3_2)
        .build()
        .unwrap();

    while let Some(e) = window.next(){

    }
}
