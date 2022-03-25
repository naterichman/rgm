use std::process;
use std::cmp::Eq;
use std::marker::Copy;
use crate::repo::Repos;

pub fn toggle_item_in_vec<T: Eq + Copy>(list: &mut Vec<T>, item: T){
    if list.contains(&item) {
        remove_item_from_vec(list, item)
    } else {
        set_item_in_vec(list, item)
    }
}

pub fn set_item_in_vec<T: Eq + Copy>(list: &mut Vec<T>, item: T){
    list.push(item);
}

pub fn remove_item_from_vec<T: Eq + Copy>(list: &mut Vec<T>, item: T){
    list.retain(|&x| x != item);
}

pub fn get_repos_or_exit() -> Repos {
    let repos = Repos::load();
    match repos {
        Ok(r) => r,
        Err(_) => process::exit(1)
    }
}
