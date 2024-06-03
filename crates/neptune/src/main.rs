mod engine;

use engine::*;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub fn main() {
    let engine = engine::Engine::new();
}
