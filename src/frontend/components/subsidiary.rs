use yew::prelude::*;

use super::super::data::Subsidiary;

pub struct SubsidiaryList {
    title: String,
    subsidiaries: Vec<Subsidiary>
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub title: String,
    pub items: Vec<String>
}

impl Default for Props {
    fn default() -> Self {
        Props {
            title: "Филиалы".to_string(),
            items: vec![]
        }
    }
}

impl Component for SubsidiaryList {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        SubsidiaryList {
            title: props.title,
            subsidiaries: props.items.iter().map(|s| Subsidiary {address: s.to_string()}).collect()
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<SubsidiaryList> for SubsidiaryList {
    fn view(&self) -> Html<Self> {
        html!{
            <div class="uk-width-1-4 uk-card uk-card-body uk-background-muted",>
                <h4>{&self.title }</h4>
                <ul class="uk-list",>
                    { for self.subsidiaries.iter().map(render_subsidiary) }
                </ul>
            </div>
        }
    }

}

fn render_subsidiary(s: &Subsidiary) -> Html<SubsidiaryList> {
    html!{
        <li><a href="#",>{&s.address}</a></li>
    }
}
