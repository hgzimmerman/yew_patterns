use yew::prelude::*;
use yew_patterns::Sender;
use receiving_component::InterComponentMessage;

pub struct SendingComponent {
    sender: Sender<InterComponentMessage>,
    string: String
}

pub enum Msg {
    SetReceiver,
    ClearReceiver,
}

impl Default for Msg {
    fn default() -> Self {
        Msg::ClearReceiver
    }
}

#[derive(Properties)]
pub struct Props {
    #[props(required)]
    pub s: String
}


impl Component for SendingComponent {
    type Properties = Props;
    type Message = Msg;
    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> SendingComponent {
        let sender = Sender::create(&mut link);
        SendingComponent{
            sender,
            string: props.s
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetReceiver => {
                self.sender.send(InterComponentMessage::Set(self.string.clone()))
            }
            Msg::ClearReceiver => {
                self.sender.send(InterComponentMessage::Clear)
            }
        }
        true
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.string = props.s;
        true
    }
}


impl Renderable<SendingComponent> for SendingComponent {
    fn view(&self) -> Html<SendingComponent> {
        html!{
            <div>
                <button onclick=|_| Msg::SetReceiver ,>
                    {format!("Send Message '{}'", self.string)}
                </button>
                <button onclick=|_| Msg::ClearReceiver ,>
                    {"Clear"}
                </button>
            </div>
        }
    }
}