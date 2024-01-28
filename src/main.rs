extern crate iced;

mod actor;
mod encounter;
mod inventory;
mod item;
mod roll;

use actor::Actor;
use encounter::Encounter;

use iced::{
    executor,
    widget::{button, column, container, Space, row, text, Row, Rule},
    window, Application, Command, Element, Length, Settings, Theme,
};

fn main() {
    State::run(Settings {
        window: window::Settings {
            size: (1800, 920),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
    .unwrap();
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Heal(u16),
    Damage(u16),
    Redraw,
}

struct State {
    actor: Actor,
}

impl Application for State {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (State, Command<Self::Message>) {
        let tracker_application = State {
            actor: Actor::default(),
        };

        (tracker_application, Command::none())
    }

    fn title(&self) -> String {
        String::from("tracker app")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Heal(_) => {
                self.actor.heal(1);
                println!("Healing logic");
            }
            Message::Damage(_) => {
                self.actor.damage(1);
                println!("Damage logic");
            }
            Message::Redraw => {}
        };
        Command::perform(async {}, |_| Message::Redraw)
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Theme>> {
        // let actor_elements: Vec<Element<_>> = self
        //     .tracker
        //     .actors()
        //     .iter()
        //     .map(|actor| column![text(actor.get_name()), text(actor.health().to_string()),].into())
        //     .collect();

        // let non_active_actors: Row<'_, Message> = Row::with_children(actor_elements);
        let controls = row![
            column![text("menu")].width(Length::FillPortion(2)),
            Rule::vertical(100),
            column![
                text("Initiative tabs").height(Length::FillPortion(1)),
                Rule::horizontal(100.0),
                text("Actor sheet").height(Length::FillPortion(5)),
                Rule::horizontal(100),
                text("Actor actions").height(Length::FillPortion(3)),
            ].width(Length::FillPortion(7)),
            Rule::vertical(100),
            column![text("world actions")].width(Length::FillPortion(2))
        ];
        container(controls)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
