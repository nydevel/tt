use dirs::home_dir;
use std::fs;
use std::fs::File;
use std::path::Path;

const DEFAULT_DIR: &str = ".tt";

/**
 * Configuring store
 */
pub mod config {
    //Ceate directory for app settign and saves
    pub fn init() {
        config_dir_check();
    }

    // Check if dir exist, create if not
    fn config_dir_check() {
        let mut home_dir = super::home_dir().unwrap();
        home_dir.push(super::DEFAULT_DIR);
        let path = super::Path::new(&home_dir);

        if !path.is_dir() {
            config_dir_init();
        }
    }

    // Create config dir
    fn config_dir_init() {
        let mut home_dir = super::home_dir().unwrap();
        home_dir.push(super::DEFAULT_DIR);
        super::fs::create_dir_all(home_dir).expect("Can't create settings directory in homedir");
    }
}

/**
 * Working with file system and files
 */
pub mod orm {
    /**
     * Create file with timestamp in name
     */
    pub fn create(name: &String) {
        let mut file_path = super::home_dir().unwrap();
        file_path.push(super::DEFAULT_DIR);
        file_path.push(name);

        super::File::create(file_path).unwrap();
    }
}
