use enigo::{Enigo, MouseControllable};
use image::{Rgba, RgbaImage, DynamicImage, GenericImageView, GenericImage};
use opencv::imgcodecs;
use reqwest::Client;
use scrap::{Capturer, Display, Frame};
use std::{io::ErrorKind::WouldBlock, borrow::Borrow};

use crate::{element::Element, matching::match_element};

  pub fn convert_bgra_to_rgba(bgra_image: RgbaImage) -> RgbaImage {
    let mut new_rgba_image = RgbaImage::new(bgra_image.width(), bgra_image.height());
    for x in 0..bgra_image.width() {
        for y in 0..bgra_image.height() {
          let pixel = bgra_image.get_pixel(x, y);
          new_rgba_image.put_pixel(x, y, Rgba([pixel[2], pixel[1], pixel[0], pixel[3]]) );
        }
      };

      new_rgba_image
}
    

#[derive(Debug)]
pub enum ScreenName {
  Connect,
  Game,
  Heroes
}


pub async fn check_token(token: String) -> bool {
  let client = Client::new();
  let request = client
  .get("https://bombcryptobot.herokuapp.com/auth/check-token")
  .bearer_auth(token);

  match request.send().await {
    Ok(response) => {
      match response.text().await {
        Ok(text) => {
          if text == String::from("true") {
            return true;
          } else {
            return false
          }
        },
        _ => {
          false
        }
      }
    },
    _ => {
      false
    }
    
  }
}

pub fn filter_image_by_min_max_rgba_color(
  screenshot: &mut DynamicImage,
  output_path: String,
  min_x: i32,
  max_x: i32,
  min_y: i32,
  max_y: i32,
) -> String {

  
 
  for x in 0..screenshot.width() {
    for y in 0..screenshot.height() {
        let actual_pixel = screenshot.get_pixel(x, y);

        if actual_pixel[0] < 175
        && actual_pixel[1] < 175
        && actual_pixel[2] < 175
        && actual_pixel[0] > 135
        && actual_pixel[1] > 135
        && actual_pixel[2] > 135
        && x > min_x as u32
        && x < max_x as u32
        && y > min_y as u32
        && y < max_y as u32
        {
            screenshot.put_pixel(x, y, Rgba([0, 255, 0 , 255]))
        } else if actual_pixel[0] < 175
        && actual_pixel[1] < 175
        && actual_pixel[2] < 175
        && actual_pixel[0] > 153 
        && actual_pixel[1] > 153 
        && actual_pixel[2] > 153 {
            screenshot.put_pixel(x, y, Rgba([255, 0, 255 , 255]))
        } else {
            screenshot.put_pixel(x, y, Rgba([0, 0, 0, 255]))
        }
    }
  }

  screenshot.save_with_format(&output_path, image::ImageFormat::Png).unwrap();
  output_path

}

pub fn find_best_match_by_colors(
  screenshot: &mut DynamicImage,
  min_x: i32,
  max_x: i32,
  min_y: i32,
  max_y: i32,
) -> Element {

 
  for x in 0..screenshot.width() {
    for y in 0..screenshot.height() {
        let actual_pixel = screenshot.get_pixel(x, y);

        if actual_pixel[0] < 200
        && actual_pixel[1] < 200
        && actual_pixel[2] < 200
        && actual_pixel[0] > 135
        && actual_pixel[1] > 135
        && actual_pixel[2] > 135
        && x > min_x as u32
        && x < max_x as u32
        && y > min_y as u32
        && y < max_y as u32
        {
            screenshot.put_pixel(x, y, Rgba([0, 255, 0 , 255]))
        } else if actual_pixel[0] < 200
        && actual_pixel[1] < 200
        && actual_pixel[2] < 200
        && actual_pixel[0] > 153 
        && actual_pixel[1] > 153 
        && actual_pixel[2] > 153 {
            screenshot.put_pixel(x, y, Rgba([255, 0, 255 , 255]))
        } else {
            screenshot.put_pixel(x, y, Rgba([0, 0, 0, 255]))
        }
    }
  }

  let mut max_x_y: (i32, i32) = (0, 0);
  let mut max_green = 0;

  for x in 0..screenshot.width() {
    for y in 0..screenshot.height() {
      if x > min_x as u32
      && x < max_x as u32
      && y > min_y as u32
      && y < max_y as u32 {
        let mut total_green = 0;
        for dx in x..x+38 {
          for dy in y..y+70 {
            let actual_pixel = screenshot.get_pixel(dx, dy);
            if actual_pixel == Rgba([0, 255, 0, 255]) {
              total_green += 1;
            }
          }
        }
        if total_green > max_green {
          max_green = total_green;
          max_x_y = (x as i32, y as i32);
        }
      }
    }
  }

  Element::new(0, max_x_y.0, max_x_y.1, 0.99, 0.99)

}


pub fn find_captcha_target(
  screenshot: &mut DynamicImage,
  min_x: i32,
  max_x: i32,
  min_y: i32,
  max_y: i32,
) -> Element {

  let mut matching_locations: Vec<(i32, i32)> = Vec::new();
 
  for x in 0..screenshot.width() {
    for y in 0..screenshot.height() {
        let actual_pixel = screenshot.get_pixel(x, y);

        if actual_pixel[0] < 175
        && actual_pixel[1] < 175
        && actual_pixel[2] < 175
        && actual_pixel[0] > 135
        && actual_pixel[1] > 135
        && actual_pixel[2] > 135
        && x > min_x as u32
        && x < max_x as u32
        && y > min_y as u32
        && y < max_y as u32
        {
            matching_locations.push((x as i32, y as i32))
        } 
    }
  }


  if matching_locations.len() > 0 {
    let mut leftist_position = (1500, 0);
    for x in matching_locations {
      if x.0 < leftist_position.0 {
        leftist_position = x
      }
    }
    let result = Element::new(0, leftist_position.0, leftist_position.1, 1.00, 0.99);
    result
  } else {
    let result = Element::new(0, 0, 0, 0.0, 0.99);
    result
  }
}


