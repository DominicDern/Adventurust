extern crate iced;

mod actor;
mod encounter;
mod roll;

use actor::Actor;
use encounter::Encounter;

use iced::{
    executor,
    widget::{button, column, container, row, text, Row},
    window, Application, Command, Element, Length, Settings, Theme,
};

fn main() {
    // State::run(Settings {
    //     window: window::Settings {
    //         size: (500, 800),
    //         ..window::Settings::default()
    //     },
    //     ..Settings::default()
    // })
    // .unwrap();
    let teag = Actor::new("Teagan", -1, 10, 1, 1, 1);
    let ben = Actor::new("Ben", 1, 11, 1, 1, 1);
    let jake = Actor::new("Jacob", 2, 12, 1, 1, 1);
    let kate = Actor::new("Katie", 0, 13, 1, 1, 1);

    let mut encounter = Encounter::new();
    encounter.add_actor(teag);
    encounter.add_actor(ben);
    encounter.add_actor(jake);
    encounter.add_actor(kate);

    println!("{:#?}", encounter);
    println!("{:#?}", encounter.get_current());

    encounter.end_turn();

    println!("{:#?}", encounter.get_current());
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
            actor: Actor::new("temp", 10, 10, 1, 1, 1),
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
        let controls = column![
            text(self.actor.health()),
            button("Damage").on_press(Message::Damage(10)),
            button("Heal").on_press(Message::Heal(10))
        ];
        container(controls)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
