
use crate::egui::{Key, InputState};
use crate::virtual_codes::VIRTUAL_CODES;

pub fn is_key_pressed(i: &InputState, key: char) -> bool {
    if let Some(key_char) = VIRTUAL_CODES.get(&key) {
        return is_key_pressed_for_code(i, *key_char);
    } 
    
    println!("Virtual code is not defined for {}", key);
    return false;
}

pub fn _is_key_released(i: &InputState, key: char) -> bool {
    if let Some(key_char) = VIRTUAL_CODES.get(&key) {
        return _is_key_released_for_code(i, *key_char);
    } 
    
    println!("Virtual code is not defined for {}", key);
    return false;
}

pub fn is_key_down(i: &InputState, key: char) -> bool {
    if let Some(key_char) = VIRTUAL_CODES.get(&key) {
        return is_key_down_for_code(i, *key_char);
    } 
    
    println!("Virtual code is not defined for {}", key);
    return false;
}

pub fn is_key_pressed_for_code(i: &InputState, key: Key) -> bool {
	i.key_pressed(key)
}

pub fn _is_key_released_for_code(i: &InputState, key: Key) -> bool {
	i.key_released(key)
}

pub fn is_key_down_for_code(i: &InputState, key: Key) -> bool {
	i.key_down(key)
}