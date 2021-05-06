use wry::{Application, Attributes, Result};

use clap::Clap;

#[derive(Clap)]
#[clap(version, author = "Roland Fredenhagen <dev@modprog.de>")]
struct Opts {
    url: String,
    #[clap(short, long, default_value = "500")]
    width: f64,
    #[clap(short, long, default_value = "500")]
    height: f64,
    #[clap(short)]
    x: Option<f64>,
    #[clap(short)]
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
    let mut app = Application::new()?;

    let attributes = Attributes {
        decorations: false,
        transparent: true,
        x: opts.x,
        y: opts.y,
        width: opts.width,
        height: opts.height,
        always_on_top: true,
        title: opts.title,
        initialization_scripts: if opts.clear {
            vec![
                r#"
                document.addEventListener("DOMContentLoaded", ()=> {
                    let glassPageStyleSheet = document.createElement('style');
                    document.head.appendChild(glassPageStyleSheet);
                    glassPageStyleSheet.sheet.insertRule("body {background: transparent !important}");
                });
                "#.to_string(),
    ]
        } else {
            vec![]
        },
        url: Some(opts.url),
        ..Default::default()
    };

    app.add_window(attributes)?;
    app.run();
    Ok(())
}
