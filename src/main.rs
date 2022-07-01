mod map; // Add module to project
mod player;

// Create custom prelude to allow easy access in other modules
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*; // crate accesses the root of our tree which is main
    pub use crate::player::*;
}

use prelude::*; // Import everything inside the prelude

struct State {
    map: Map,
}

impl State {
    fn new() -> Self {
        Self { map: Map::new() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("A World Apart: A Rust Roguelike")
        .with_fps_cap(30.0)
        .build()?;
    main_loop(context, State::new())
}
