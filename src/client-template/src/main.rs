use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

pub struct TemplateGame;

impl SimpleState for TemplateGame {}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let path = app_root.join("resources").join("display.ron");
    println!("{}", path.as_path().display().to_string());
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0])
                )
                .with_plugin(RenderFlat2D::default()),
        )?;
    let mut game = Application::new("./", TemplateGame, game_data)?;

    game.run();

    Ok(())
}
