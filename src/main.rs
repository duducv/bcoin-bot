use image::{self, RgbaImage};
use enigo::{self, Enigo};
use scrap::{self, Capturer, Display};
use bcbot::{config::Config, matching::{matching_elements}, util::{*}};
use std::{borrow::Borrow, error::Error, thread::{self}};
use std::time::Duration;
use std::io::ErrorKind::WouldBlock;
use opencv::{imgcodecs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let arguments: Vec<String> = std::env::args().collect();
    let config = &Config::from_args(arguments);
    println!("cheking credentials");
    let is_token_valid = check_token(String::from(&config.token)).await;

    if !is_token_valid {
        println!("invalid or expired credentials, try log in again in few seconds");
        return Ok(())
    }
    

    //importing target element assets and covert to OpenCV elements
    let target_connect_img = imgcodecs::imread("images-target/connect.png", 0).expect("Couldn't find connect image");
    let metamask_connect_img = imgcodecs::imread("images-target/select-wallet-1-no-hover.png", 0).expect("Couldn't find connect image");
    let metamask_blue_sign_img =  imgcodecs::imread("images-target/select-wallet-2.png", 0).expect("Couldn't find connect image");
    let hero_img = imgcodecs::imread("images-target/hero-icon.png", 0).expect("Couldn't find connect image");
    let treasure_hunt_img = imgcodecs::imread("images-target/treasure-hunt-icon.png", 0).expect("Couldn't find treasure hunt image");
    let green_bar_img = imgcodecs::imread("images-target/green-bar.png", 0).expect("Couldn't  image");
    let close_heroes_screen_img = imgcodecs::imread("images-target/x.png", 0).expect("Couldn't  image");
    let go_back_arrow_img = imgcodecs::imread("images-target/go-back-arrow.png", 0).expect("Couldn't  image");
    let common_img = imgcodecs::imread("images-target/common-text.png", 0).expect("Couldn't image");
    let home_img = imgcodecs::imread("images-target/home.png", 0).expect("Couldn't image");
    let new_map_img =  imgcodecs::imread("images-target/new-map.png", 0).expect("Couldn't image");
    let ok_img =  imgcodecs::imread("images-target/ok.png", 0).expect("Couldn't image");
    let captcha_ask_robot =  imgcodecs::imread("images-target/are_you_robot.png", 0).expect("Couldn't image");


    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    let mut check_rest = true;
    let mut mouse = Enigo::new();
    let total_heroes = 11;
    let mut sent_to_work = 0;
    let mut scan_attempt = 1;
    let mut actual_screen = ScreenName::Connect;
    println!("the bot will start looking for game elements in {} seconds. Please remeber to unlock metamask before.", config.start_delay);
    println!("wait after connect {:?}", config.after_click_connect_orange_btn_delay);
    thread::sleep(std::time::Duration::from_secs(config.start_delay));

    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("Looking for game elements.");
        let buffer = match capturer.frame() {
            Ok(buffer) => {
                buffer
            },
            Err(error) => {
                if error.kind() == WouldBlock {
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };

        let new_buff=  buffer.to_vec();
        let bgra_image = RgbaImage::from_vec(w as u32, h as u32, new_buff).unwrap();
        let normalized_rgba_image = convert_bgra_to_rgba(bgra_image);
        normalized_rgba_image.save_with_format("tmp/output.png", image::ImageFormat::Png).expect("couldn't find screenshot");
        let screenshot = imgcodecs::imread("tmp/output.png", 0).expect("Couldn't find connect image");
        
        matching_elements(
            &mut check_rest, 
            &mut mouse, 
            &mut actual_screen,
            total_heroes,
            &mut sent_to_work,
            &mut scan_attempt,
            config.clone().borrow(),
            screenshot.borrow(),
            target_connect_img.borrow(),
            metamask_connect_img.borrow(),
            metamask_blue_sign_img.borrow(),
            hero_img.borrow(),
            treasure_hunt_img.borrow(),
            &green_bar_img.borrow(),
            close_heroes_screen_img.borrow(),
            go_back_arrow_img.borrow(),
            common_img.borrow(),
            home_img.borrow(),
            new_map_img.borrow(),
            &ok_img.borrow(),
            captcha_ask_robot.borrow(),
        );
   }
}