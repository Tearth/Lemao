use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::RawFont;
use lemao_core::renderer::textures::bmp;
use lemao_core::renderer::textures::RawTexture;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::ffi::OsStr;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

#[derive(Default)]
pub struct AssetsLoader {
    pub textures: Arc<RwLock<HashMap<String, RawTexture>>>,
    pub fonts: Arc<RwLock<HashMap<String, RawFont>>>,
    pub queue: Arc<RwLock<VecDeque<String>>>,

    pub status: Arc<RwLock<String>>,
    pub loaded_assets: Arc<RwLock<u32>>,
    pub total_assets: u32,
}

impl AssetsLoader {
    pub fn set_queue(&mut self, path: &str) -> Result<(), String> {
        if !Path::new(path).is_dir() {
            return Err("Directory not found".to_string());
        }

        self.add_to_queue(path);
        self.loaded_assets = Arc::new(RwLock::new(0));
        self.total_assets = self.queue.read().unwrap().len() as u32;

        Ok(())
    }

    pub fn start_loading(&mut self) {
        let queue = self.queue.clone();
        let textures = self.textures.clone();
        let fonts = self.fonts.clone();
        let status = self.status.clone();
        let loaded_assets = self.loaded_assets.clone();

        thread::spawn(move || {
            let mut queue = queue.write().unwrap();
            while let Some(asset_to_load) = queue.pop_front() {
                let asset_to_load_path = Path::new(&asset_to_load);
                let name = asset_to_load_path.file_stem().unwrap();
                let extension = asset_to_load_path.extension().unwrap();

                *status.write().unwrap() = asset_to_load.clone();

                match extension.to_str().unwrap() {
                    "bmp" => {
                        let texture = bmp::load(&asset_to_load)?;
                        if textures.write().unwrap().insert(name.to_str().unwrap().to_string(), texture).is_some() {
                            return Err("Duplicated asset".to_string());
                        }
                    }
                    "bff" => {
                        let font = bff::load(&asset_to_load)?;
                        if fonts.write().unwrap().insert(name.to_str().unwrap().to_string(), font).is_some() {
                            return Err("Duplicated asset".to_string());
                        }
                    }
                    _ => return Err("Unsupported extension".to_string()),
                };

                *loaded_assets.write().unwrap() += 1;
            }

            Ok(())
        });
    }

    fn add_to_queue(&mut self, path: &str) {
        let path = Path::new(path);

        for file in path.read_dir().unwrap().flatten() {
            let file_path = file.path();
            let file_path = Path::new(file_path.to_str().unwrap());

            if file_path.is_dir() {
                self.add_to_queue(file_path.to_str().unwrap());
            } else if self.is_extension_allowed(file_path.extension().unwrap()) {
                self.queue.write().unwrap().push_back(file_path.to_str().unwrap().to_string());
            }
        }
    }

    fn is_extension_allowed(&self, extension: &OsStr) -> bool {
        let allowed_extension = ["bmp"];
        allowed_extension.contains(&extension.to_str().unwrap())
    }
}
