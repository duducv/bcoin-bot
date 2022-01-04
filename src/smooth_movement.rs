use std::time::Duration;
use enigo::{Enigo, MouseControllable, MouseButton};

pub fn smoothly_move_to(mouse: &mut Enigo, x_pos: i32, y_pos: i32, movement_in_microsecs: u64) {
    if x_pos > y_pos {
        for movement in (0..=x_pos).step_by(6){
            if movement < y_pos {
                std::thread::sleep(Duration::from_micros(movement_in_microsecs * 60));
                mouse.mouse_move_to(movement, movement);
        
            } else {
                std::thread::sleep(Duration::from_micros(movement_in_microsecs * 60));
                mouse.mouse_move_to(movement, y_pos);
            }
            
        }
    } else {
        for movement in (0..=x_pos).step_by(6) {
            if movement < x_pos {
                std::thread::sleep(Duration::from_micros(movement_in_microsecs * 60));
                mouse.mouse_move_to(movement, movement);
                
            } else {
                std::thread::sleep(Duration::from_millis(movement_in_microsecs * 60));
                mouse.mouse_move_to(x_pos, movement);
                
            }
            
        }
    }
}
