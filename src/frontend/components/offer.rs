use yew::prelude::*;

pub struct OfferBuyForm {
    text: String,
    on_submit: Option<Callback<()>>,
}

pub enum Msg {
    OpenForm,
    CloseForm
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

impl Component for OfferBuyForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        OfferBuyForm {
            text: "Если нужной вам книги нет ни в одном из филиалов, предложите её к закупке.".to_string(),
            on_submit: props.on_submit
        }
    }


    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<OfferBuyForm> for OfferBuyForm {
    fn view(&self) -> Html<Self> {
        html! {
            <div class=("uk-card", "uk-card-default", "uk-card-body"),>
                <p>{&self.text}</p>
                <button class=("uk-button", "uk-button-primary"),>{"Предложить книгу"}</button>
            </div>
        }
    }
}
