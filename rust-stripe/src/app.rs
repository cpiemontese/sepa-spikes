use sauron::prelude::*;

#[derive(Debug)]
pub enum Msg {
    Click,
}

pub struct App {
    click_count: i32,
}

impl App {
    pub fn new() -> Self {
        App { click_count: 0 }
    }
}

impl Application<Msg> for App {
    fn view(&self) -> Node<Msg> {
        node! {
            <main>
                <h1>"Minimal example"</h1>
                <div class="some-class" id="some-id" {attr("data-id", 1)}>
                    <input class="client"
                            type="button"
                            value="Click me!"
                            key=1
                            on_click={|_| {
                                Msg::Click
                            }}
                    />
                    <div>{text(format!("Clicked: {}", self.click_count))}</div>
                    <input type="text" value={self.click_count}/>
                </div>
            </main>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Click => self.click_count += 1,
        }
        Cmd::none()
    }
}
