use yew::prelude::*;

pub struct Header {
    title: String
}

pub enum Msg {
    Signup,
    Login,
    Logout
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub title: String
}

impl Default for Props {
    fn default() -> Self {
        Props { title: "Hondana - краудсорсинговая библиотека".to_string() }
    }
}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props:Self::Properties, _: ComponentLink<Self>) -> Self {
        Header { title: props.title }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        // TODO: implement login/logout
        false
    }
}

impl Renderable<Header> for Header {
    fn view(&self) -> Html<Self> {
        html!{
            <nav class="uk-navbar-container",uk-navbar="",>
                <div class="uk-navbar-item",>
                    <h2>{ &self.title }</h2>
                </div>
                <div class="uk-navbar-right",>
                    <ul class="uk-navbar-nav",>
                        <li>
                            <a href="#",>{ "Записаться" }</a>
                        </li>
                        <li>
                            <a href="#",>{ "Войти" }</a>
                        </li>
                    </ul>
                </div>
            </nav>
        }
    }
}
