use iced::alignment::{self, Alignment};
use iced::event::{self, Event};
use iced::font::{self, Font};
use iced::keyboard::{self, KeyCode, Modifiers};
use iced::subscription;
use iced::theme::{self, Theme};
use iced::widget::{
    self, button, checkbox, column, container, row, scrollable, text,
    text_input, Text,
};
use iced::window;
use iced::{Application, Element};
use iced::{Color, Command, Length, Settings, Subscription};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::task::{TaskManager, TaskMessage};
use crate::storage::{SavedState, LoadError, SaveError};

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

pub fn main() -> iced::Result {
    TodoAppUI::run(Settings {
        window: window::Settings {
            size: (500, 800),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug)]
pub enum TodoAppUI {
    Loading,
    Loaded(State),
}
#[derive(Debug, Default)]
pub struct State {
    description_input: String,
    tasks: Vec<TaskManager>,
    dirty: bool,
    saving: bool,
    filter: Filter,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<SavedState, LoadError>),
    DescriptionInputChanged(String),
    CreateTask,
    // TaskCompleted(usize),
    // TaskDeleted(usize),
    // TaskEdited(usize, String),
    TaskMessage(usize, TaskMessage),
    Saved(Result<(), SaveError>),
    FontLoaded(Result<(), font::Error>),
    FilterChanged(Filter),
}

impl Application for TodoAppUI  {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (TodoAppUI, Command<Message>) {
        (
            TodoAppUI::Loading,
            Command::batch(vec![
                font::load(include_bytes!("../fonts/icons.ttf").as_slice())
                    .map(Message::FontLoaded),
                Command::perform(SavedState::load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Todo App")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            TodoAppUI::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = TodoAppUI::Loaded(State {
                            description_input: state.description_input,
                            tasks: state.tasks,
                            ..State::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = TodoAppUI::Loaded(State::default());
                    }
                    _ => {}
                }
                Command::none()
            }
            TodoAppUI::Loaded(state) => {
                let mut saved = false;

                let command = match message {
                    Message::DescriptionInputChanged(description) => {
                        // update the description input value
                        state.description_input = description;

                        Command::none()
                    }
                    Message::CreateTask => {
                        // add a new task using the description input value
                        // if let Some(description) = self.description_input.value() {
                        //     self.task_manager.add_task(description);
                        //     // clear the input after adding the task
                        //     self.description_input = self.description_input.set_value(String::new());
                        // }
                        if !state.description_input.is_empty() {
                            println!("用户输入 {}", state.description_input.clone());
                            state.tasks.push(TaskManager::new(state.description_input.clone()));
                            state.description_input.clear()
                        }

                        Command::none()
                    }
                    // Message::TaskCompleted(id) => {
                    //     self.task_manager.complete_task(id);
                    // }
                    Message::TaskMessage(id, TaskMessage::TaskDeleted) => {
                        state.tasks.remove(id);
                        Command::none()
                    }
                    Message::FilterChanged(filter) => {
                        state.filter = filter;

                        Command::none()
                    }
                    Message::TaskMessage(i, new_description) => {
                        if let Some(task) = state.tasks.get_mut(i) {
                            let should_focus = matches!(new_description, TaskMessage::TaskUpdate);
                            task.update(new_description);

                            if should_focus {
                                let id = TaskManager::text_input_id(i);
                                Command::batch(vec![
                                    text_input::focus(id.clone()),
                                    text_input::select_all(id),
                                ])
                            } else {
                                Command::none()
                            }
                        } else {
                            Command::none()
                        }
                    }
                    Message::Saved(_) => {
                        state.saving = false;
                        saved = true;

                        Command::none()
                    }
                    _ =>  Command::none(),
                };

                if !saved {
                    state.dirty = true;
                }

                let save = if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Command::perform(
                        SavedState {
                            description_input: state.description_input.clone(),
                            tasks: state.tasks.clone(),
                        }
                        .save(),
                        Message::Saved,
                    )
                } else {
                    Command::none()
                };

                Command::batch(vec![command, save])
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            TodoAppUI::Loading => loading_message(),
            TodoAppUI::Loaded(State {
                description_input,
                filter,
                tasks,
                ..
            }) => {
                let title = text("todos")
                    .width(Length::Fill)
                    .size(100)
                    .style(Color::from([0.5, 0.5, 0.5]))
                    .horizontal_alignment(alignment::Horizontal::Center);

                let input = text_input("Add anything what you want to be done in future", description_input)
                    .id(INPUT_ID.clone())
                    .on_input(Message::DescriptionInputChanged)
                    .on_submit(Message::CreateTask)
                    .padding(15)
                    .size(30);

                let controls = view_controls(tasks, *filter);
                let filtered_tasks = tasks.iter().filter(|task| filter.matches(task));

                let tasks: Element<_> = if filtered_tasks.count() > 0 {
                    column(
                        tasks
                            .iter()
                            .enumerate()
                            .filter(|(_, task)| filter.matches(task))
                            .map(|(i, task)| {
                                task.view(i).map(move |message| {
                                    Message::TaskMessage(i, message)
                                })
                            })
                            .collect(),
                    )
                    .spacing(10)
                    .into()
                } else {
                    empty_message(match filter {
                        Filter::All => "No Task",
                        Filter::Active => "No Active Task",
                        Filter::Completed => {
                            "No Completed Task"
                        }
                    })
                };

                let content = column![title, input, controls, tasks]
                    .spacing(20)
                    .max_width(800);
                
                scrollable(
                    container(content)
                        .width(Length::Fill)
                        .padding(40)
                        .center_x(),
                )
                .into()
            }
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize,
)]
pub enum Filter {
    #[default]
    All,
    Active,
    Completed,
}

impl Filter {
    fn matches(&self, task: &TaskManager) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !task.completed,
            Filter::Completed => task.completed,
        }
    }
}

pub fn view_controls(tasks: &[TaskManager], current_filter: Filter) -> Element<Message> {
    let tasks_left = tasks.iter().filter(|task| !task.completed).count();

    let filter_button = |label, filter, current_filter| {
        let label = text(label);

        let button = button(label).style(if filter == current_filter {
            theme::Button::Primary
        } else {
            theme::Button::Text
        });

        button.on_press(Message::FilterChanged(filter)).padding(8)
    };

    row![
        text(format!(
            "{} {} left",
            tasks_left,
            if tasks_left == 1 { "task " } else { "tasks" }
        ))
        .width(Length::Fill),
        row![
            filter_button("All", Filter::All, current_filter),
            filter_button("Active", Filter::Active, current_filter),
            filter_button("Completed", Filter::Completed, current_filter),
        ]
        .width(Length::Shrink)
        .spacing(10)
    ]
    .spacing(20)
    .align_items(Alignment::Center)
    .into()
}


pub fn loading_message<'a>() -> Element<'a, Message> {
    container(
        text("Loading...")
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}

pub fn empty_message(message: &str) -> Element<'_, Message> {
    container(
        text(message)
            .width(Length::Fill)
            .size(25)
            .horizontal_alignment(alignment::Horizontal::Center)
            .style(Color::from([0.7, 0.7, 0.7])),
    )
    .width(Length::Fill)
    .height(200)
    .center_y()
    .into()
}