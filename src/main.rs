extern crate yew;

use yew::prelude::*;

struct Model {
    hello: String
}

enum Msg {
    DoIt
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            hello: "".to_string()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                self.hello = "hello world".to_string();
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    <button onclick=|_| Msg::DoIt,>{ "Click me!" }</button>
                </div>
                <div>
                  <p>{&self.hello}</p>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
