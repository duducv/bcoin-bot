use enigo::{Enigo, MouseButton, MouseControllable};

use crate::smooth_movement::{smoothly_move_to};

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Element {
  pub  id: i32,
  pub  position_x: i32,
  pub  position_y: i32,
  pub  matching_probability: f32,
  pub  matching_probability_minimal: f32
}

impl Element {
   pub fn new(id: i32, position_x: i32, position_y: i32, matching_probability: f32, matching_probability_minimal: f32) -> Element {
        Element {
            id,
            position_x,
            position_y,
            matching_probability,
            matching_probability_minimal,
        }
    }

    pub fn go_to_location(&self, mouse: &mut Enigo,x_compensation: i32, y_compensation: i32, mouse_speed: u64) {
        smoothly_move_to(mouse, self.position_x + x_compensation, self.position_y + y_compensation, mouse_speed)
    }

    pub fn go_to_location_and_click(&self, mouse: &mut Enigo, x_compensation: i32, y_compesation: i32,  mouse_speed: u64) {
        smoothly_move_to(mouse, self.position_x + x_compensation, self.position_y + y_compesation, mouse_speed);
        mouse.mouse_click(enigo::MouseButton::Left);
    }

    pub fn slide_down(&self, mouse: &mut Enigo, y_movement: i32) {
        mouse.mouse_move_to(self.position_x, self.position_y);
        mouse.mouse_down(enigo::MouseButton::Left);
        for y in 0..y_movement {
            std::thread::sleep(std::time::Duration::from_millis(50));
            mouse.mouse_move_relative(0, -y);
        }

        mouse.mouse_up(enigo::MouseButton::Left);
        
        
    }

    pub fn to_owned(&self) -> &Element {
        self
        }
    }


    