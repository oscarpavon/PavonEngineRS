use std::io::BufRead;



/*#[path = "./engine/windows_manager/windows_manager.rs"]
mod windows_manager;
use crate::windows_manager::manager;

#[path = "./engine/renderer/rendeder.rs"]
mod renderer;

use crate::renderer::pe_renderer;
*/

fn main() {
    println!("Welcome to Pavon Engine");
    //pe_renderer::pe_renderer_init();
    //manager::pe_window_manager_loop();
    
    let mut _line = String::new();
    println!(":");
    let _enter = std::io::stdin().read_line(&mut _line).unwrap();
    println!("Your enter: {}",_line);
    loop{

    }
}
