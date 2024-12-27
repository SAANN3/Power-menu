pub struct LocalAssets {}
use std::{env, fs::create_dir, path};

use include_dir::{include_dir, Dir};

impl LocalAssets {
    pub fn extract_assets() {
        // seems like there is no known way to make dioxus work without extracting assets :/
        const ASSETS: Dir = include_dir!("./assets");
        let mut path = env::current_exe().unwrap();
        path.pop();
        let path = path.join("assets");
        if !path.exists() {
            create_dir(&path).expect(&format!("Can't create folder: {:?}", &path));
            ASSETS.extract(&path).expect(&format!("Can't extract assets to {:?}", &path));
        }
    }

    pub fn get_path(relative: String) -> String {
        let mut path = env::current_exe().unwrap();
        path.pop();
        let path = path.join(relative);
        path.to_str().unwrap().to_string()
    }
}