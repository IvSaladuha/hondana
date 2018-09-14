use yew::prelude::*;

pub struct SearchForm {
    title: String,
    pattern: String,
    on_submit: Option<Callback<()>>
}

pub enum Msg {
    Submit,
    ChangePattern(String)
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub on_submit: Option<Callback<()>>
}

impl Default for Props {
    fn default() -> Self {
        Props { on_submit: None }
    }
}

impl Component for SearchForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        SearchForm {
            title: "Поиск".to_string(),
            pattern: "".to_string(),
            on_submit: props.on_submit
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<SearchForm> for SearchForm {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="uk-width-3-4 uk-card uk-card-body uk-background-muted",>
                <h4>{&self.title}</h4>
                {"По названию или автору"}
                <form>
                    <input class="uk-input", type="text", placeholder={"Название (часть) или фамилия автора"},/>
                </form>
                <a href="#search-by-theme-modal", uk-toggle="",>{"По теме"}</a>
                <SearchThemesModal />
            </div>
        }
    }
}

pub struct SearchThemesModal {
    themes: Vec<String>
}

impl Component for SearchThemesModal {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        SearchThemesModal {themes: vec!["Педагогика", "Иностранные языки", "Программирование"].iter().map(|s| s.to_string()).collect()}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

fn render_theme(t: &String) -> Html<SearchThemesModal> {
    html! {
        <li><a href="#",>{t}</a></li>
    }
}

impl Renderable<SearchThemesModal> for SearchThemesModal {
    fn view(&self) -> Html<Self> {
        html! {
            <div id="search-by-theme-modal", uk-modal="",>
                <div class=("uk-modal-dialog", "uk-modal-body"),>
                    <h2 class="uk-modal-title",>{"Темы"}</h2>
                    <ul class="uk-list",>
                        {for self.themes.iter().map(render_theme)}
                    </ul>
                    <button class="uk-button uk-modal-close",>{"Закрыть"}</button>
                </div>
            </div>
        }
    }
}
