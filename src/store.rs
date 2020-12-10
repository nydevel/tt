use dirs::home_dir;
use std::fs;

pub mod store2 {
    pub fn save() -> std::io::Result<()> {
        let hd = super::home_dir();
        hd.push("tt");
        super::fs::create_dir_all(hd)?;
        Ok(())
    }
}
