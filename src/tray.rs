
use image::{GenericImageView, ImageReader};
use ksni::Icon;
use ksni::menu::*;


pub struct TrayIcon {
    on_click: Box<(dyn FnMut() -> () + Send)>,
    path: String,
    custom_color: Option<[u8;4]>
}

impl TrayIcon {
    pub fn spawn<F: FnMut() + Send + 'static> (path: String, on_click: F, custom_rgba8: Option<[u8;4]>) {
        let tray =  TrayIcon{
            on_click: Box::new(on_click),
            path: path,
            custom_color: custom_rgba8,
        };
        let service = ksni::TrayService::new(tray);
        let _handle = service.handle();
        service.spawn();
    }
}

impl ksni::Tray for TrayIcon {
    fn icon_pixmap(&self) -> Vec<Icon> {
        let image = ImageReader::open(&self.path)
            .unwrap()
            .decode()
            .unwrap();
        let (width, height) = image.dimensions();
        let mut image_rgba8 = image.to_rgba8().to_vec();
        if self.custom_color.is_some() {
            for chunk in image_rgba8.chunks_exact_mut(4) {
                if chunk[3] != 0 {
                    chunk.copy_from_slice(&self.custom_color.unwrap());
                }
            }
        }
        vec![Icon {
            data: image_rgba8,
            width: width as i32,
            height: height as i32
        }]
    }

    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        (self.on_click)();   
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        vec![ StandardItem {
            label: "Exit".to_string(),
            activate: Box::new( |_| {
                std::process::exit(0);
            }),
            ..Default::default()
        }.into()]
    }
    
}
