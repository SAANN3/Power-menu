
# Power menu ![Power menu](readme/icon.svg)

![alt text](readme/preview.gif)

Power menu built in rust with dioxus!
## How to build
```bash
git clone https://github.com/SAANN3/power-menu.git
cd power-menu
cargo build 
```
Builded binary will be located in ```target/debug/power-menu```

After that you can move binary anywhere

## Usage

Add it to autostart and then, when you need to launch it, click on the power icon in the system tray

## Styling
Open ```assets/main.css``` and edit the css class
```css
:root {
    --global-color: aliceblue; // icons, text, borders,  basically all 
    --global-bg-color: #white; // background color
}
```


#### !! If styling doesn't applying, make sure that you have deleted  'assets' folder in place, where binary is located. !!

For example, to run app, you need to write
```bash
cargo run
```
But if you want to reset current stylings, you need to remove assets folder
```bash
rm -r target/debug/assets && cargo run
```