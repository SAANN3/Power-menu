#![allow(non_snake_case)]


mod tray;
mod windowItem;
mod assets;

use dioxus::prelude::*;
use dioxus_desktop::{use_window, window, WindowBuilder};
use dioxus_logger::tracing::Level;
use std::{env, fs, process::Command, str::FromStr, sync::mpsc::channel, time::Duration};
use tray::TrayIcon;
use windowItem::WindowItem;
use assets::LocalAssets;
use css_color::Srgb;


fn main() {
    LocalAssets::extract_assets();
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 2 && args[1] == "--open" {
        open_window();
    } else {
        let (tx, rx) = channel();

        let current_color: Option<[u8;4]> = {
            let path = LocalAssets::get_path("./assets/main.css".to_string());
            let file = fs::read_to_string(path).unwrap();
            let color = file
                .split_once("--global-color:")
                .unwrap().1
                .split_once(";")
                .unwrap().0
                .replace(" ", "");
            let srgb_color = Srgb::from_str(&color)
                .expect(&format!("Error parsing color: '{color}', if the color looks wrong make sure that main.css has something like this `--global-color: YOUR_COLOR;`"));


            Some([
                (srgb_color.alpha    * (255 as f32)) as u8,
                (srgb_color.red  * (255 as f32)) as u8,
                (srgb_color.green   * (255 as f32)) as u8,
                (srgb_color.blue  * (255 as f32)) as u8,
            ])
        };


        TrayIcon::spawn(
            LocalAssets::get_path("./assets/icon.png".to_string()),
            move || {
                tx.send(0).unwrap();
            },
            current_color
        );
        loop {
            let _ = rx.recv();
            open_process();
            while rx.try_recv().is_ok() {
                //flush all recieved events that are piled up, while main window was open
            }
        }   
    }
    
}

fn open_process() {
    let mut window = Command::new(env::current_exe()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
    );
    window.arg("--open");
    let _ = window.status();
}

fn open_window() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    let window = WindowBuilder::new()
        .with_decorations(false);
    let config = dioxus_desktop::Config::new()
        .with_disable_context_menu(true)
        .with_window(window);
    dioxus::LaunchBuilder::new()
        .with_cfg(config)
        .launch(App);  
}

pub struct HoverProps {
    inside: bool,
    text: String,
}


#[component]
fn App() -> Element {
    let selected_pos: Signal<Option<i64>> = use_signal(|| None);
    let mut current_text = use_signal(|| "".to_string());
    let onhover = move |event: HoverProps| {
        if selected_pos.read().is_some() {
            return;
        }
        if event.inside {
            *current_text.write() = event.text;
        } else if current_text() == event.text {
            *current_text.write() = "".to_string();
        }
    };
    use_effect(move || {
        
        use_window().set_fullscreen(true);
    });

    use_effect(move || { 
        if selected_pos.read().is_some() {
            spawn(async move {
                tokio::time::sleep(Duration::from_secs(1)).await;
                match selected_pos.read().unwrap() {
                    1 => {
                        Command::new("reboot")
                            .spawn()
                            .expect("reboot")
                            .wait()
                            .unwrap();
                    },
                    0 =>  {
                        Command::new("shutdown")
                            .arg("now")
                            .spawn()
                            .expect("shutdown now")
                            .wait()
                            .unwrap();
                    },
                    -1 => {
                        Command::new("loginctl")
                            .arg("terminate-session")
                            .arg(env::var_os("XDG_SESSION_ID").unwrap())
                            .spawn()
                            .expect("loginctl terminate-session")
                            .wait()
                            .unwrap();
                    },
                    _ => {
                        println!("Nothing to see here 0_0");
                    },
                }
            });
        };
    });

    rsx! {
        link { rel: "stylesheet", href: "./assets/main.css", }
        div { 
            class: "vbox", height: "100%", width: "95%",
            div { class: "hbox", width: "100%", margin: "10%",
                WindowItem {image: "./assets/restart.svg",  text: "Reboot",     position: 1,  onhover: onhover, selected: selected_pos}
                WindowItem {image: "./assets/shutdown.svg", text: "Shutdown",   position: 0,  onhover: onhover, selected: selected_pos}
                WindowItem {image: "./assets/logout.svg",   text: "Log Out",    position: -1, onhover: onhover, selected: selected_pos}
            }

            div {
                class: "text-div",
                width: if current_text() != "" {"100%"} else {"0%"},
                color: if current_text() != "" {"rgba(0, 0, 0, 1)"} else {"rgba(0, 0, 0, 0)"},
                "{current_text}"  
            }
        }
        div {
            class: "bottom-corner",
            transform: if selected_pos.read().is_some() {"translateY(100vh)"} else {""}, 
            button { onclick: move |_| window().close(), 
                "Cancel"
            }
        }
    }
}
