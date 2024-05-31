use iced::{
    widget::{
        button::{self, Button},
        Column, Container, Text,
    },
    time,theme::{self, Theme},
    Alignment, Application, Command, Element, Settings, Subscription,
    Color, Point, Rectangle
};

pub fn main() -> iced::Result {
    let settings = Settings{
        window: iced::window::Settings{
        size: (iced::Size::new(200.0, 200.0)),

        resizable: false,
        ..Default::default()
        },
        ..Settings::default()
    };
    Pomodoro::run(settings)
}

struct Pomodoro {
    duration: usize,
    timer_state: TimerState,
    start_button: button::State,
    reset_button: button::State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimerState {
    Idle,
    Running,
    Paused,
}

#[derive(Debug, Clone)]
enum Message {
    Start,
    Tick,
    Reset,
}

impl Application for Pomodoro {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = ();
    type Theme = theme::Theme; // 使用 Iced 的 Basic 主题

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                duration: 1500,
                timer_state: TimerState::Idle,
                start_button: button::State::new(),
                reset_button: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Pomodoro")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Start => {
                if self.timer_state == TimerState::Idle || self.timer_state == TimerState::Paused {
                    self.timer_state = TimerState::Running;
                } else {
                    self.timer_state = TimerState::Paused;
                }
            }
            Message::Tick => {
                if self.duration > 0 && self.timer_state == TimerState::Running {
                    self.duration -= 1;
                }
            }
            Message::Reset => {
                self.duration = 1500;
                self.timer_state = TimerState::Idle;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        // Same view code as before
        let start_text = match self.timer_state {
            TimerState::Running => "Pause",
            _ => "Start",
        };

        let start_button = Button::new(Text::new(start_text)).on_press(Message::Start);

        let reset_button = Button::new(Text::new("Reset")).on_press(Message::Reset);

        let time_display = Text::new(format!(
            "{:02}:{:02}",
            self.duration / 60,
            self.duration % 60
        ))
        .size(50);

        let content = Column::new()
            .align_items(Alignment::Center)
            .push(time_display)
            .push(start_button)
            .push(reset_button)
            .spacing(20);

        Container::new(content)
            .width(200)
            .height(200)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        match self.timer_state {
            TimerState::Running => {
                time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick)
            }
            _ => Subscription::none(),
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
