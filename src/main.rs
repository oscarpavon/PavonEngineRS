

#[path = "./engine/windows_manager/windows_manager.rs"]
mod windows_manager;
use crate::windows_manager::manager;

#[path = "./engine/renderer/renderer.rs"]
mod renderer;


fn main() {
    println!("Welcome to Pavon Engine");
    renderer::pe_renderer_init();
    //manager::pe_window_manager_loop();
    
}
