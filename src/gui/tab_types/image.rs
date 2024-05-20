use crate::gui::tab_types::TabStruct;
use egui::Ui;
use egui_extras::RetainedImage;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct ImageTab {
    #[serde(skip)]
    image: Option<RetainedImage>,
    pub file_path: String,
}

#[typetag::serde]
impl TabStruct for ImageTab {
    fn show_interface(&mut self) -> bool {
        false
    }
    fn plot(&mut self, ui: &mut Ui) {
        if self.image.is_none() {
            let image_bytes = match std::fs::read(&self.file_path) {
                Ok(bytes) => bytes,
                Err(_err) => {
                    // eprintln!("Error reading image: {}", err);
                    return;
                }
            };
            self.image = Some(
                match RetainedImage::from_image_bytes(self.file_path.clone(), &image_bytes) {
                    Ok(image) => image,
                    Err(err) => {
                        eprintln!("Error loading image: {}", err);
                        return;
                    }
                },
            );
        }
        if let Some(image) = &self.image {
            image.show(ui);
        }
    }

    fn title(&self) -> String {
        self.file_path
            .clone()
            .replace('\\', "/")
            .split('/')
            .last()
            .unwrap()
            .to_string()
    }
}
