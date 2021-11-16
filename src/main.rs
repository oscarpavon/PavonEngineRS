

#[path = "./engine/windows_manager/windows_manager.rs"]
mod windows_manager;
use crate::windows_manager::manager;

#[path = "./engine/renderer/rendeder.rs"]
mod renderer;

use crate::renderer::pe_renderer;

fn main() {
    println!("Hello, world!");
    pe_renderer::pe_renderer_init();
    manager::pe_window_manager_loop();

}
