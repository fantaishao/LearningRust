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

use crate::task::{TaskManager, TaskMessage};

use crate::storage::{SavedState, LoadError};

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
    // pub task_manager: TaskManager,
    // pub description_input: TextInput,
    // pub add_button: button::State,
    // pub tasks_scrollable: Scrollable,
}

#[derive(Debug, Default)]
pub struct State {
    description_input: String,
    tasks: Vec<TaskManager>,
    dirty: bool,
    saving: bool,
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
}

impl Application for TodoAppUI  {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (TodoAppUI, Command<Message>) {
        // let mut task_manager = TaskManager::new();
        // task_manager.load_tasks();
        // TodoAppUI{
        //     task_manager,
        //     description_input: TextInput::new(),
        //     add_button: button::State::new(),
        //     tasks_scrollable: Scrollable::new(),
        // }
        (
            TodoAppUI::Loading,
            Command::batch(vec![
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

                match message {
                    Message::DescriptionInputChanged(description) => {
                        // update the description input value
                        state.description_input = description;
                    }
                    Message::CreateTask => {
                        // add a new task using the description input value
                        // if let Some(description) = self.description_input.value() {
                        //     self.task_manager.add_task(description);
                        //     // clear the input after adding the task
                        //     self.description_input = self.description_input.set_value(String::new());
                        // }
                        if !state.description_input.is_empty() {
                            state.tasks.push(TaskManager::new(state.description_input.clone()));
                            state.description_input.clear()
                        }
                    }
                    // Message::TaskCompleted(id) => {
                    //     self.task_manager.complete_task(id);
                    // }
                    Message::TaskMessage(id, TaskMessage::TaskDeleted) => {
                        state.tasks.remove(id);
                    }
                    Message::TaskMessage(id, new_description) => {
                        if let Some(task) = state.tasks.get_mut(id) {
                            let should_focus = matches!(new_description, TaskMessage::TaskUpdate);
                            task.update(new_description);
                        }
                    }
                    _ => {}
                };
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            TodoAppUI::Loading => loading_message(),
            TodoAppUI::Loaded(State {
                description_input,
                tasks,
                ..
            }) => {
                let title = text("todos")
                    .width(Length::Fill)
                    .size(100)
                    .style(Color::from([0.5, 0.5, 0.5]))
                    .horizontal_alignment(alignment::Horizontal::Center);

                let content = column![title]
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
        // define ui layout
        // let content = Column::new()
        //     .spacing(20)
        //     .align_items(Alignment::Center)
        //     .push(
        //         TextInput::new(
        //             &mut self.description_input,
        //             "Enter task description",
        //             "",
        //             Message::DescriptionInputChanged,
        //         )
        //         .padding(10)
        //         .size(30)
        //     )
        //     .push(
        //         Button::new(&mut self.add_button, Text::new("Add Task"))
        //             .on_press(Message::CreateTask)
        //             .padding(10)
        //             .width(Length::Fill)
        //             .style(Default::default()),
        //     );

        //     let tasks_columns = self.task_manager.tasks.iter().fold(
        //         Column::new().spacing(10),
        //         |column, (id, task)| {
        //             let complete_button = Button::new(&mut button::State::new(), Text::new("Complete"))
        //             .on_press(Message::TaskCompleted(*id))
        //             .padding(10);

        //             let edit_button = Button::new(&mut button::State::new(), Text::new("Edit"))
        //                 .on_press(Message::TaskEdited(*id, task.description.clone()))
        //                 .padding(10);

        //             let delete_button = Button::new(&mut button::State::new(), Text::new("Delete"))
        //                 .on_press(Message::TaskDeleted(*id))
        //                 .padding(10);

        //             let task_row = Row::new()
        //                 .push(Text::new(&format!("ID: {}", id)).width(Length::Fill))
        //                 .push(Text::new(&format!("Description: {}", task.description)).width(Length::Fill))
        //                 .push(complete_button)
        //                 .push(edit_button)
        //                 .push(delete_button);

        //             column.push(task_row)
        //         },
        //     );

        //     let tasks_content = Container::new(
        //         Scrollable::new(&mut self.tasks_scrollable)
        //             .push(tasks_columns)
        //             .width(Length::Fill)
        //             .height(Length::Fill),
        //     )
        //     .width(Length::Fill)
        //     .height(Length::Fill);

        //     // return the content wrapped in a container
        //     Column::new()
        //         .spacing(20)
        //         .align_items(Alignment::Center)
        //         .push(content)
        //         .push(tasks_content)
        //         .into()
    }
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