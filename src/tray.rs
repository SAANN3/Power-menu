use dioxus::prelude::*;
use image::{GenericImageView, ImageReader};
use ksni::Icon;
use ksni::menu::*;

pub struct TrayIcon {
    on_click: Box<(dyn FnMut() -> () + Send)>,
    path: String,
}

impl TrayIcon {
    pub fn spawn<F: FnMut() + Send + 'static> (path: String, on_click: F) {
        let tray =  TrayIcon{
            on_click: Box::new(on_click),
            path: path,
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
        vec![Icon {
            data: image.to_rgba8().into_vec(),
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
