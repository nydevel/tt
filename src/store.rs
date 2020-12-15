use dirs::home_dir;
use std::fs;

pub mod store2 {
    //Ceate directory for app settign and saves
    pub fn dir_init() {
        let mut hd = super::home_dir().unwrap();
        hd.push(".tt");
        super::fs::create_dir_all(hd).expect("Can't create settings directory in homedir");
    }

    pub fn save() {}
}
