use std::any::Any;

use druid_shell::kurbo::{Line, Size};
use druid_shell::piet::{Color, RenderContext};
use druid_shell::{
    Application, Cursor, FileDialogOptions, FileSpec, Menu, MouseEvent, TimerToken, WinHandler,
    WindowHandle, SysMods, HotKey, WindowBuilder,
};

const BG_COLOR: Color = Color::rgb8(0x27, 0x28, 0x22);
const FG_COLOR: Color = Color::rgb8(0xf0, 0xf0, 0xea);

#[derive(Default)]
struct HelloState {
    size: Size,
    handle: WindowHandle,
}

impl WinHandler for HelloState {
    fn connect(&mut self, handle: &WindowHandle) {
        self.handle = handle.clone();
    }

    fn prepare_paint(&mut self) {}

    fn paint(&mut self, piet: &mut druid_shell::piet::Piet, invalid: &druid_shell::Region) {
        let rect = self.size.to_rect();
        piet.fill(rect, &BG_COLOR);
        piet.stroke(Line::new((10.0, 50.0), (90.0, 90.0)), &FG_COLOR, 1.0);
    }

    fn command(&mut self, id: u32) {
        match id {
            0x100 => {
                self.handle.close();
                Application::global().quit();
            }
            0x101 => {
                let options = FileDialogOptions::new().show_hidden().allowed_types(vec![
                    FileSpec::new("Rust Files", &["rs", "toml"]),
                    FileSpec::TEXT,
                    FileSpec::JPG,
                ]);
                self.handle.open_file(options);
            }
            0x102 => {
                let options = FileDialogOptions::new().show_hidden().allowed_types(vec![
                    FileSpec::new("Rust Files", &["rs", "toml"]),
                    FileSpec::TEXT,
                    FileSpec::JPG,
                ]);
                self.handle.save_as(options);
            }
            _ => println!("unexpected id {}", id),
        }
    }

    fn open_file(
        &mut self,
        token: druid_shell::FileDialogToken,
        file: Option<druid_shell::FileInfo>,
    ) {
        println!("open file result: {:?}", file);
    }

    fn save_as(
        &mut self,
        token: druid_shell::FileDialogToken,
        file: Option<druid_shell::FileInfo>,
    ) {
        println!("save file result: {:?}", file);
    }

    fn key_down(&mut self, event: druid_shell::KeyEvent) -> bool {
        println!("keydown: {:?}", event);
        false
    }

    fn key_up(&mut self, event: druid_shell::KeyEvent) {
        println!("keyup: {:?}", event);
    }

    fn wheel(&mut self, event: &druid_shell::MouseEvent) {
        println!("mouse_wheel: {:?}", event);
    }

    fn mouse_move(&mut self, event: &MouseEvent) {
        self.handle.set_cursor(&Cursor::Arrow);
        println!("mouse_move {:?}", event);
    }

    fn mouse_down(&mut self, event: &MouseEvent) {
        println!("mouse_down {:?}", event);
    }

    fn mouse_up(&mut self, event: &MouseEvent) {
        println!("mouse_up {:?}", event);
    }

    fn timer(&mut self, id: TimerToken) {
        println!("timer fired: {:?}", id);
    }

    fn size(&mut self, size: Size) {
        self.size = size;
    }

    fn got_focus(&mut self) {
        println!("Got focus");
    }

    fn lost_focus(&mut self) {
        println!("Lost focus");
    }

    fn request_close(&mut self) {
        println!("REQUEST CLOSE");
        self.handle.close();
    }

    fn destroy(&mut self) {
        println!("DESTROY");
        Application::global().quit()
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn scale(&mut self, scale: druid_shell::Scale) {
        println!("scale: {:?}", scale);
    }

    fn rebuild_resources(&mut self) {
        println!("rebuild_resources");
    }

    fn zoom(&mut self, delta: f64) {
        println!("zoom: delta: {}", delta);
    }

    fn mouse_leave(&mut self) {
        println!("mouse_leave");
    }

    fn idle(&mut self, token: druid_shell::IdleToken) {
        println!("idle {:?}", token);
    }
}

fn main() {
    tracing_subscriber::fmt().init();
    let mut file_menu = Menu::new();
    file_menu.add_item(
        0x100,
        "E&ixt",
        Some(&HotKey::new(SysMods::Cmd, "q")),
        true,
        false,
    );
    file_menu.add_item(
        0x101,
        "O&pen",
        Some(&HotKey::new(SysMods::Cmd, "o")),
        true,
        false,
    );
    file_menu.add_item(
        0x102,
        "S&ave",
        Some(&HotKey::new(SysMods::Cmd, "s")),
        true,
        false,
    );
    let mut menubar = Menu::new();
    menubar.add_dropdown(Menu::new(), "Application", true);
    menubar.add_dropdown(file_menu, "&File", true);

    let app = Application::new().unwrap();
    let mut builder = WindowBuilder::new(app.clone());
    builder.set_handler(Box::new(HelloState::default()));
    builder.set_title("Hello example");
    builder.set_menu(menubar);

    let window = builder.build().unwrap();
    window.show();

    app.run(None);
}
