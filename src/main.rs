use speedway_match_tracker::smt_app::App;
use speedway_match_tracker::tui::Tui;
use std::error;

//Convert result to general result
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[tokio::main]
async fn main() -> AppResult<()> {
    let mut app = App::default();
    app.fetch_data().await;

    let mut tui = Tui::new()?;
    tui.enter()?;

    while app.is_running {
        tui.draw(&mut app)?;
    }

    tui.exit()?;

    Ok(())
}
