#![allow(clippy::unnecessary_wraps)]

//Comment this out for dead code errors
#![allow(dead_code)]

use std::env;
use std::path;

use ggez::*;

mod main_state;
mod world;
mod utilities;
mod graphics_engine;

// 0---------------------PROGRAM MAIN------------------------------------------0

pub fn main() -> GameResult {
    //added in to add resources dir
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    //End of Resource Directory Stuff
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        //Tell context builder where to find the resources for our game
        .add_resource_path(resource_dir)
        // Next we set up the window. This title will be displayed in the title bar of the window.
        .window_setup(ggez::conf::WindowSetup::default().title("Titanium").vsync(true))
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(1008.0, 960.0)
        );
    // And finally we attempt to build the context and create the window. If it fails, we panic with the message
    // "Failed to build ggez context"
    let (mut ctx, event_loop) = cb.build()?;
    let state = main_state::ms::MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
