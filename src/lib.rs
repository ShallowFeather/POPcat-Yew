use wasm_bindgen::prelude::*;
use yew::prelude::*;
use js_sys::Promise;
use gloo_console as console;
use yew::html::Scope;
use yew::events::MouseEvent;

extern crate web_sys;
struct Model {
    link: ComponentLink<Self>,
    value: i64,
    sounds: web_sys::HtmlAudioElement,
    src: String,
}

enum Msg {
    AddOne,
    Change,
    Back,
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let sounds = web_sys::HtmlAudioElement::new_with_src("./pop.wav").expect("");
        let img_src = "./popcat1.png".to_string();
        console::log!("ok");
        Self {
            link,
            value: 0,
            sounds,
            src: img_src,
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
            Msg::Change => {
                console::log!("aaa");
                self.src = "./popcat2.png".to_string();
                console::log!("aaa");
                true
            }
            Msg::Back => {
                self.src = "./popcat1.png".to_string();
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
                <img src={ self.src.clone() }
                    onmousedown=self.link.callback(|_| Msg::Change)
                    onmouseup=self.link.callback(|_| Msg::Back)
                    onclick=self.link.callback(|_| Msg::AddOne)
                    height="600px"
                />
            </div>
        }
    }
}



#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}