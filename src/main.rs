use wry::application::dpi::{LogicalPosition, LogicalSize, Position, Size};
use wry::{
    application::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
    Result,
};

use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1", author = "Roland Fredenhagen <dev@modprog.de>")]
struct Opts {
    url: String,
    #[clap(short, long, default_value = "500")]
    width: f64,
    #[clap(short, long, default_value = "500")]
    height: f64,
    #[clap(short, requires("y"))]
    x: Option<f64>,
    #[clap(short, requires("x"))]
    y: Option<f64>,
    /// Trys to make the Backround Transparent via CSS
    #[clap(short, long)]
    clear: bool,
    /// Set custom Title
    #[clap(short, long, default_value = "Glass Page")]
    title: String,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let event_loop = EventLoop::new();
    let mut window = WindowBuilder::new()
        .with_decorations(false)
        .with_transparent(true)
        .with_always_on_top(true)
        .with_title(opts.title)
        .with_inner_size(Size::Logical(LogicalSize::new(opts.width, opts.height)));

    if let (Some(x), Some(y)) = (opts.x, opts.y) {
        window = window.with_position(Position::Logical(LogicalPosition::new(x, y)))
    }

    let window = window.build(&event_loop).unwrap();

    let mut webview = WebViewBuilder::new(window)?
        .with_transparent(true)
        .with_url(&opts.url)?;
    if opts.clear {
        webview = webview.with_initialization_script(
            r#"
            document.addEventListener("DOMContentLoaded", ()=> {
                let glassPageStyleSheet = document.createElement('style');
                document.head.appendChild(glassPageStyleSheet);
                glassPageStyleSheet.sheet.insertRule("body {background: transparent !important}");
            });
            "#,
        );
    }
    let webview = webview.build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {
                let _ = webview.resize();
            }
        }
    });
}
