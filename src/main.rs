use crate::app::App;

mod app;

fn main() -> color_eyre::Result<()> {
    color_eyre::config::HookBuilder::default()
        .display_env_section(false)
        .display_location_section(cfg!(debug_assertions))
        .install()?;

    let mut app = App::new();

    ratatui::run(|terminal| app.run(terminal))?;

    Ok(())
}
