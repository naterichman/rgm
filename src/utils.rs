use crate::repo::Repos;
use std::cmp::Eq;
use std::marker::Copy;
use std::process;
use std::path::PathBuf;
use std::fs::{create_dir, write};
use lazy_static::lazy_static;
use log::info;

pub fn toggle_item_in_vec<T: Eq + Copy>(list: &mut Vec<T>, item: T) {
    if list.contains(&item) {
        remove_item_from_vec(list, item)
    } else {
        set_item_in_vec(list, item)
    }
}

pub fn set_item_in_vec<T: Eq + Copy>(list: &mut Vec<T>, item: T) {
    list.push(item);
}

pub fn remove_item_from_vec<T: Eq + Copy>(list: &mut Vec<T>, item: T) {
    list.retain(|&x| x != item);
}

pub fn get_repos_or_exit() -> Repos {
    let repos = Repos::load();
    match repos {
        Ok(r) => r,
        Err(_) => process::exit(1),
    }
}

lazy_static! {
    static ref RGM_DIR: PathBuf = {
        let mut home = dirs::home_dir().unwrap();
        home.push(".rgm");
        if home.exists() {
            if ! home.is_dir(){
                panic!("$HOME/.rgm exists but is not a directory.")
            }
        } else {
            create_dir(&home);
        }
        home
    };
}

pub fn log_file() -> PathBuf {
    let mut home = RGM_DIR.clone();
    home.push("rgm.log");
    home
}

pub fn config_file() -> PathBuf {
    let mut home = RGM_DIR.clone();
    home.push("rgm.conf");
    home
}

pub fn shell_file() -> PathBuf {
    let mut home = RGM_DIR.clone();
    home.push("rgm.sh");
    home
}

pub fn clear_shell_file() {
    let shell = shell_file();
    write(&shell, "");
}

pub fn zsh_init() {
    let zsh_function = r#"
# Welcome to RGM!

function rgm(){
    rgm-bin "$@"
    source $HOME/.rgm/rgm.sh
}

# To init rgm, add the following line to your $HOME/.zshrc:
#
# eval "$(rgm-bin init zsh)"
    "#;
    println!("{}", zsh_function);
}

pub fn bash_init() {
    let zsh_function = r#"
# Welcome to RGM!

function rgm(){
    rgm-bin "$@"
    source $HOME/.rgm/rgm.sh
}

# To init rgm, add the following line to your $HOME/.bashrc:
3
# eval "$(rgm-bin init bash)"
    "#;
    println!("{}", zsh_function);
}
