use iced::window;
use iced::{Settings};

mod task;
mod ui;
mod storage;

fn main() -> iced::Result {
    // let mut task_manager = task::TaskManager::new();
    // storage::load_tasks(&mut task_manager);

    // let mut app = ui::TodoAppUI::new();

    // app.run();
    // ui::TodoAppUI::run(Settings {
    //     window: window::Settings {
    //         size: (500, 800),
    //         ..window::Settings::default()
    //     },
    //     ..Settings::default()
    // })
    ui::main()
}