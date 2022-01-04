#[derive(Debug, Clone)]
pub struct Config {
    pub start_delay: u64,
    pub heroes_page_first_action_delay: u64,
    pub treasure_hunt_first_action_delay: u64,
    pub after_sent_to_work_delay: u64,
    pub after_close_heroes_page_delay: u64,
    pub check_for_heroes_able_to_work_delay: u64,
    pub after_click_metamask_sign_blue_btn_delay: u64,
    pub after_click_metamask_connect_delay: u64,
    pub after_click_connect_orange_btn_delay: u64,
    pub wait_for_page_refresh_delay: u64,
    pub heroes_page_slide_attempt: u64,
    pub token: String
}

impl Config {
    pub fn from_args(args: Vec<String>) -> Config {
        Config { 
            start_delay:  convert_string_to_u64(args[1].to_owned()),
            heroes_page_first_action_delay: convert_string_to_u64(args[2].to_owned()),
            treasure_hunt_first_action_delay:convert_string_to_u64(args[3].to_owned()),
            after_sent_to_work_delay: convert_string_to_u64(args[4].to_owned()),
            after_close_heroes_page_delay: convert_string_to_u64(args[5].to_owned()),
            check_for_heroes_able_to_work_delay: convert_string_to_u64(args[6].to_owned()),
            after_click_metamask_sign_blue_btn_delay: convert_string_to_u64(args[7].to_owned()),
            after_click_metamask_connect_delay:  convert_string_to_u64(args[8].to_owned()),
            after_click_connect_orange_btn_delay: convert_string_to_u64(args[9].to_owned()),
            wait_for_page_refresh_delay: convert_string_to_u64(args[10].to_owned()),
            heroes_page_slide_attempt: convert_string_to_u64(args[11].to_owned()),
            token: args[12].to_owned()
        }
    }
}

fn convert_string_to_u64(str: String) -> u64 {
     str.parse().expect("invalid argument `{}`: expected number")
}