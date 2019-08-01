extern crate yew;
extern crate yew_patterns;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use yew::prelude::*;

mod sending_component;
mod receiving_component;


use sending_component::SendingComponent;
use receiving_component::ReceivingComponent;
use sending_component::Props;


pub struct Model {

}

impl Component for Model {
    type Properties = ();
    type Message = ();
    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Model {
        Model{}
    }
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}


impl Renderable<Model> for Model {
    fn view(&self) -> Html<Model> {
        let props_hello = Props{s:"Hello World".to_string()};
        let props_goodbye = Props{s:"Goodbye World".to_string()};
        html!{
            <div>
                <SendingComponent: with props_hello, />
                <SendingComponent: with props_goodbye, />
                <ReceivingComponent: />
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}