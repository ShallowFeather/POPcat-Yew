use wasm_bindgen::prelude::*;
use yew::prelude::*;
use js_sys::Promise;
use gloo_console as console;

extern crate web_sys;
struct Model {
    link: ComponentLink<Self>,
    value: i64,
    sounds: web_sys::HtmlAudioElement,
}

enum Msg {
    AddOne,
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let sounds = web_sys::HtmlAudioElement::new_with_src("../pop.wav").expect("");
        console::log!("ok");
        Self {
            link,
            value: 0,
            sounds,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                self.sounds.set_current_time(0.0);
                let _ = self.sounds.play().expect("failed to play audio");
                console::log!("add one and has sounds");
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <h1>{ "POPCAT" }</h1>
                <p>{ "score:" }{ self.value }</p>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}