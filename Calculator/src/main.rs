use iced::widget::{button, column, row, text};
use iced::{Alignment, Element, Sandbox, Settings, Length};
use iced::alignment;

pub fn main() -> iced::Result {
    Calculator::run(Settings::default())
}

struct Calculator {
    value: String,
    prev_value: String,
    operator: String,
    prev_operator: String,
    next_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    EnterNumber(String),
    EnterOperator(String),
    EqualOperator,
    ClearOperator,
    PercentOperator,
    ToggleSign
}

impl Sandbox for Calculator {
    type Message = Message;

    fn new() -> Self {
        Self { 
            value: String::new(),
            prev_value: String::new(),
            operator: String::new(),
            prev_operator: String::new(),
            next_value: String::new()
         }
    }

    fn title(&self) -> String {
        String::from("Calculator - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::EnterNumber(value) => {
                if self.operator == String::from("") {
                    self.value = format!("{}{}", self.prev_value, value);
                    self.prev_value = format!("{}{}", self.prev_value, value);
                } else if self.operator == String::from("=") {
                    self.prev_value.clear();
                    self.next_value.clear();
                    self.operator.clear();

                    self.value = format!("{}{}", self.prev_value, value);
                    self.prev_value = format!("{}{}", self.prev_value, value);
                } else {
                    self.value = format!("{}{}", self.next_value, value);
                    self.next_value = format!("{}{}", self.next_value, value);
                } 
            }
            Message::EnterOperator(value) => {
                if self.operator != String::from("") && self.operator != String::from("=") {
                    self.prev_value = calculate(self.prev_value.clone(), self.next_value.clone(), self.operator.clone());
                    self.value = self.prev_value.clone();
                    self.next_value.clear();
                } else if self.operator == String::from("=") {
                    
                }
                self.operator = value.clone();
            }
            Message::EqualOperator => {
                if self.operator == '='.to_string() {
                    self.value = calculate(self.prev_value.clone(), self.next_value.clone(), self.prev_operator.clone());
                    self.prev_value = self.value.clone();
                } else {
                    self.prev_operator = self.operator.clone();
                    self.value = calculate(self.prev_value.clone(), self.next_value.clone(), self.operator.clone());
                    self.prev_value = self.value.clone();
                    self.operator = '='.to_string();
                    self.next_value.clear();
                }
            }
            Message::ClearOperator => {
                self.value.clear();
                self.prev_value.clear();
                self.next_value.clear();
                self.operator.clear();
            }
            Message::PercentOperator => {
                if self.next_value != String::from("") {
                    let num1: f64 = self.next_value.parse().expect("Failed to parse number");
                    self.next_value = (num1 * 0.01).to_string();
                    self.value = calculate(self.prev_value.clone(), self.next_value.clone(), self.operator.clone());
                } else if self.prev_value != String::from("") {
                    let num1: f64 = self.prev_value.parse().expect("Failed to parse number");
                    self.prev_value = (num1 * 0.01).to_string();
                    self.value = self.prev_value.clone();
                }
            }
            Message::ToggleSign => {
                if self.next_value != String::from("") {
                    let num1: f64 = self.next_value.parse().expect("Failed to parse number");
                    self.next_value = (num1 * -1.00).to_string();
                    self.value = calculate(self.prev_value.clone(), self.next_value.clone(), self.operator.clone());
                } else if self.prev_value != String::from("") {
                    let num1: f64 = self.prev_value.parse().expect("Failed to parse number");
                    self.prev_value = (num1 * -1.00).to_string();
                    self.value = self.prev_value.clone();
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let result = row![
            text(self.value.clone())
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Right)    
        ]
        .spacing(10)
        .padding(10);
        let first_row = row![
            button(centered_text("AC")).on_press(Message::ClearOperator),
            button(centered_text("+/-")).on_press(Message::ToggleSign),
            button(centered_text("%")).on_press(Message::PercentOperator),
            button(centered_text("/")).on_press(Message::EnterOperator("/".to_string()))
        ]
        .spacing(10)
        .padding(10);

        let second_row = row![
            button(centered_text("7")).on_press(Message::EnterNumber("7".to_string())),
            button(centered_text("8")).on_press(Message::EnterNumber("8".to_string())),
            button(centered_text("9")).on_press(Message::EnterNumber("9".to_string())),
            button(centered_text("X")).on_press(Message::EnterOperator("X".to_string()))
        ]
        .spacing(10)        
        .padding(10);

        let third_row = row![
            button(centered_text("4")).on_press(Message::EnterNumber("4".to_string())),
            button(centered_text("5")).on_press(Message::EnterNumber("5".to_string())),
            button(centered_text("6")).on_press(Message::EnterNumber("6".to_string())),
            button(centered_text("-")).on_press(Message::EnterOperator("-".to_string()))
        ]
        .spacing(10)        
        .padding(10);

        let fourth_row = row![
            button(centered_text("1")).on_press(Message::EnterNumber("1".to_string())),
            button(centered_text("2")).on_press(Message::EnterNumber("2".to_string())),
            button(centered_text("3")).on_press(Message::EnterNumber("3".to_string())),
            button(centered_text("+")).on_press(Message::EnterOperator("+".to_string()))
        ]
        .spacing(10)        
        .padding(10);

        let fifth_row = row![
            button(centered_text("0")).on_press(Message::EnterNumber("0".to_string())),
            button(centered_text(".")).on_press(Message::EnterNumber(".".to_string())),
            button(centered_text("=")).on_press(Message::EqualOperator)
        ]
        .spacing(10)        
        .padding(10);

        column![
            result,
            first_row,
            second_row,
            third_row,
            fourth_row,
            fifth_row
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

fn centered_text(content: &str) -> Element<'_, Message> {
    text(content)
        .width(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Center)
        .into()
}

fn calculate(prev_value: String, next_value: String, operator: String) -> String {
    let num1: f64 = prev_value.parse().expect("Failed to parse number");
    let num2: f64 = next_value.parse().expect("Failed to parse number");
    
    let value = match operator.as_str() {
        "+" => (num1 + num2).to_string(),
        "-" => (num1 - num2).to_string(),
        "X" => (num1 * num2).to_string(),
        "/" => {
            if num2 == 0.0 {
                panic!("not a number")
            }
            (num1 / num2).to_string()
        },
        _ => {
            panic!("invalid opearator");
        }
    };
    value
}