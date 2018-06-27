use yew::prelude::*;
use yew_patterns::Receiver;
use yew::agent::Transferable;


pub struct ReceivingComponent {
    /// Value to display
    value: Option<String>,
    /// Allows this component to receive messages from other components.
    _receiver: Receiver<InterComponentMessage>
}


#[derive(Serialize, Deserialize, Clone)]
pub enum InterComponentMessage {
    Set(String),
    Clear,
}

impl Transferable for InterComponentMessage {}


impl Component for ReceivingComponent {
    type Properties = ();
    type Message = InterComponentMessage;
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> ReceivingComponent {
        // create() can be called to create Receivers that have the same message type as the channel type.
        // new() works as well if the Message and the channel type aren't the same.

        ReceivingComponent {
            value: None,
            _receiver: Receiver::create(&link)
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            InterComponentMessage::Set(string) => self.value = Some(string),
            InterComponentMessage::Clear => self.value = None
        }
        true
    }
}


impl Renderable<ReceivingComponent> for ReceivingComponent {
    fn view(&self) -> Html<ReceivingComponent> {

        let value = match self.value {
            Some(ref value) => value.as_str(),
            None => "None"
        };


        html!{
            <div>
                {"Receiving Component: "}
                <br/>
                {value}
            </div>
        }
    }
}