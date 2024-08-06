use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;
use iced::widget::{ text_input, text, checkbox, row, Text, button };
use iced::theme::{self, Theme};
use iced::alignment::{self, Alignment};
use iced::{Color, Command, Length, Settings, Subscription};
use iced::{Application, Element};
use iced::font::{self, Font};

#[derive(Debug, Clone)]
pub enum TaskState {
    Idle,
    Editing,
}

impl Default for TaskState {
    // 使用 Idle 作为默认状态，不需要显式地指定
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone)]
pub enum TaskMessage {
    Completed(bool),
    TaskUpdate,
    DescriptionEdited(String),
    FinishEdition,
    TaskDeleted,
}

// 定义任务结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskManager {
    pub description: String,
    pub completed: bool,

    #[serde(skip)]
    state: TaskState,
}

impl TaskManager {
    pub fn text_input_id(i: usize) -> text_input::Id {
        text_input::Id::new(format!("task-{i}"))
    }

    pub fn new(description: String) -> Self {
        // TaskManager {
        //     tasks: HashMap::new(),
        //     next_id: 1,
        // }
        TaskManager {
            description,
            completed: false,
            state: TaskState::Idle,
        }
    }

    pub fn update(&mut self, message: TaskMessage) {
        match message {
            TaskMessage::Completed(completed) => {
                self.completed = completed;
            }
            TaskMessage::TaskUpdate => {
                self.state = TaskState::Editing;
            }
            TaskMessage::DescriptionEdited(new_description) => {
                self.description = new_description;
            }
            TaskMessage::TaskDeleted => {}
            TaskMessage::FinishEdition => {
                if !self.description.is_empty() {
                    self.state = TaskState::Idle;
                }
            }
        }
    }

    pub fn view(&self, i: usize) -> Element<TaskMessage> {
        match &self.state {
            TaskState::Idle => {
                let checkbox = checkbox(
                    &self.description,
                    self.completed,
                    TaskMessage::Completed,
                )
                .width(Length::Fill)
                .text_shaping(text::Shaping::Advanced);

                row![
                    checkbox,
                    button(edit_icon())
                        .on_press(TaskMessage::TaskUpdate)
                        .padding(10)
                        .style(theme::Button::Text)
                ]
                .spacing(20)
                .align_items(Alignment::Center)
                .into()
            }
            TaskState::Editing => {
                let text_input = text_input("Describe your task...", &self.description)
                    .id(Self::text_input_id(i))
                    .on_input(TaskMessage::DescriptionEdited)
                    .on_submit(TaskMessage::FinishEdition)
                    .padding(10);
                
                row![
                    text_input,
                    button(
                        row![delete_icon(), "Delete"]
                            .spacing(10)
                            .align_items(Alignment::Center)
                    )
                    .on_press(TaskMessage::TaskDeleted)
                    .padding(10)
                    .style(theme::Button::Destructive)
                ]
                .spacing(20)
                .align_items(Alignment::Center)
                .into()
            }
        }
    }
}

// Fonts
const ICONS: Font = Font::with_name("Iced-Todos-Icons");

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(20)
        .horizontal_alignment(alignment::Horizontal::Center)
}

fn edit_icon() -> Text<'static> {
    icon('\u{F303}')
}

fn delete_icon() -> Text<'static> {
    icon('\u{F1F8}')
}
