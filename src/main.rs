mod graphics;
mod math;
mod object_loader;

use anyhow::{bail, Result};
use graphics::App;
use object_loader::Object;
use std::env;
use std::fs;
use std::str::FromStr;
use winit::event_loop::EventLoop;

const OBJ_COLOR: (u8, u8, u8) = (56, 38, 51);
const BG_COLOR: (u8, u8, u8) = (146, 189, 163);

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("This program expect one argument");
    }
    let file = fs::read_to_string(&args[1])?;
    let object = match Object::from_str(&file) {
        Ok(object) => object,
        Err(_) => bail!("Failed to parse the obj file"),
    };

    //println!("{object:?}");

    let event_loop = EventLoop::new()?;
    let mut app = App::new(&event_loop, &object)?;

    event_loop.run_app(&mut app)?;

    Ok(())
}
