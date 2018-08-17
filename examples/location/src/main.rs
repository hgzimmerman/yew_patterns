#[macro_use]
extern crate yew;
extern crate yew_patterns;
extern crate stdweb;

use yew::prelude::*;
use yew_patterns::location_service::LocationService;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}


pub struct Model {
    location_service: LocationService
}
pub enum Msg {
    SetLocation
}

impl Component for Model {
    type Properties = ();
    type Message = Msg;
    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Model {
        Model {
            location_service: LocationService{}
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetLocation => {
                let location = "https://www.google.com".to_string();
                self.location_service.set_location(location)
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Model> {
        html!{
            <div>
                <button onclick=|_| Msg::SetLocation, > 
                    {"Go to google"}
                </button>
            </div>
        }
    }
}
