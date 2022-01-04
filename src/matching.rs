use enigo::{Enigo};
use crate::{config::Config, element::Element, flow::*, smooth_movement::smoothly_move_to, util::ScreenName};
use opencv::{imgproc, prelude::*};

pub fn matching_elements(
    check_rest: &mut bool, 
    mouse: &mut Enigo,
    actual_screen: &mut ScreenName,
    total_heroes: i32,
    sent_to_work: &mut i32,
    scan_attempt: &mut i32,
    config: &Config,
    screenshot: &Mat,
    connect_img: &Mat,
    metamask_no_hover_img: &Mat,
    metamask_blue_sign_img: &Mat,
    hero_img: &Mat,
    treasure_hunt_img: &Mat,
    green_bar_img: &Mat,
    close_heroes_screen_img: &Mat,
    go_back_arrow_img: &Mat,
    common_text_img: &Mat,
    home_img: &Mat,
    new_map_img: &Mat,
    ok_img: &Mat,
    captcha_ask_robo_img: &Mat,
    ) {

       match actual_screen {
           ScreenName::Connect => {
            let connect_element =  match_element(screenshot, connect_img, 0.99);
            let metamask_element =  match_element(screenshot, metamask_no_hover_img, 0.99);
            let metamask_blue_sign_element = match_element(screenshot, metamask_blue_sign_img, 0.97);
           
            let elements = vec![
                connect_element,
                metamask_element,
                metamask_blue_sign_element,
            ];

            
            let matched_elements: Vec<&Element> = elements.iter().filter(|x| {
                x.matching_probability > x.matching_probability_minimal
            }).collect();



           connect_page_control_flow(mouse, actual_screen, matched_elements, &connect_element, &metamask_element, &metamask_blue_sign_element, config)
           },
           _ => {
            let hero_element =  match_element(screenshot, hero_img, 0.99);
            let treasure_hunt_element =  match_element(screenshot, treasure_hunt_img, 0.99);
            let green_bar_element = match_element(screenshot, green_bar_img, 0.99);
            let close_heroes_screen_element = match_element(screenshot, close_heroes_screen_img, 0.99);
            let go_back_arrow_element = match_element(screenshot, go_back_arrow_img, 0.99);
            let common_text_element = match_element(screenshot, common_text_img, 0.99);
            let home_element = match_element(screenshot, home_img, 0.99);

            let new_map_element =  match_element(screenshot, new_map_img, 0.99);
            let ok_element = match_element(screenshot, ok_img, 0.99);
            let captcha_ask_robot_element = match_element(screenshot, captcha_ask_robo_img, 0.99);
            let elements = vec![
                hero_element,
                treasure_hunt_element,
                green_bar_element,
                close_heroes_screen_element,
                go_back_arrow_element,
                common_text_element,
                home_element,
                new_map_element,
                ok_element,
                captcha_ask_robot_element,
            ];
            let matched_elements: Vec<&Element> = elements.iter().filter(|x| {
                x.matching_probability > x.matching_probability_minimal
            }).collect();
            game_page_control_flow(
                check_rest, 
                mouse, 
                actual_screen, 
                total_heroes, 
                sent_to_work,
                scan_attempt,
                matched_elements, 
                &hero_element, 
                &treasure_hunt_element, 
                &green_bar_element, 
                &close_heroes_screen_element,
                &go_back_arrow_element,
                &common_text_element,
                &home_element,
                &new_map_element,
                &ok_element,
                &captcha_ask_robot_element,
                config
            );
           }
       }
     

}


pub fn match_element(screenshot: &Mat, template: &Mat, threshold: f32) -> Element {
    let mut match_result = Mat::default();
    imgproc::match_template(screenshot, template, &mut match_result, 3, &Mat::default()).expect("error at match template");
    let match_result_buffer: Vec<Vec<f32>> = match_result.to_vec_2d().expect("Error at creating matching result buffer");
    let mut max_value: f32 = 0.0;
    let mut max_value_index = 0;
    match_result_buffer.iter().flatten().enumerate().for_each(|x| {
        if *x.1 > max_value {
            max_value = *x.1;
            max_value_index = x.0;
        };
    });

    let px_max = max_value_index as i32 % match_result.mat_size()[1];
    let py_max = max_value_index as i32 / match_result.mat_size()[1];

    Element::new(1, px_max , py_max, max_value, threshold)
    
}

pub fn match_multiples_elements(screenshot: &Mat, template: &Mat, threshold: f32) -> Vec<Element> {
    let mut match_result = Mat::default();
    imgproc::match_template(&screenshot, &template, &mut match_result, 3, &Mat::default()).expect("error at match multiples items");
    let match_result_buffer: Vec<Vec<f32>> = match_result.to_vec_2d().expect("Error at creating multiple matching result buffer");
    let elements: Vec<Element> = match_result_buffer.iter().flatten().enumerate().filter( |x | {
        *x.1 > threshold
    }).map( |y | {
        let px_max = y.0 as i32 % match_result.mat_size()[1];
        let py_max = y.0 as i32 / match_result.mat_size()[1];
        Element::new(1, px_max, py_max, *y.1, threshold)
    }).collect();

    // let mut normalized_elements: Vec<Element> = Vec::new();

    // for x in 0..elements.len() {
    //     if x < elements.len() - 1 && elements[x+1].position_y - elements[x].position_y < 5 {
    //         normalized_elements.push(elements[x]);
    //     }
    // }
    elements
}

pub fn filter_matched_elements_by_region(elements: Vec<Element>, min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Element {

    let best_match: Vec<Element> = elements.iter().map(|element| *element).filter( |element | {
      element.position_x >= min_x && element.position_x <= max_x && element.position_y >= min_y && element.position_y <= max_y 
    }).collect();

    let mut best_element_match: Element = Element{id: 0, position_x: 0, position_y: 0, matching_probability: 0.0, matching_probability_minimal: 0.0};

    if best_match.len() > 0 {
        for x in best_match {
        
            if x.matching_probability > best_element_match.matching_probability {
                best_element_match = x
            }
    
        }
    }


   best_element_match

}

