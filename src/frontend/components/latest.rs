use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct NowReading {
    who: String,
    what: String
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub items: Vec<NowReading>
}

impl Default for Props {
    fn default() -> Self {
        Props {
            items: vec![]
        }
    }
}

pub struct NowReadingList {
    items: Vec<NowReading>
}

impl Component for NowReadingList {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        NowReadingList {items: props.items}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        // Read-only, so no update.
        false
    }
}

fn view_now_reading_item(item: &NowReading) -> Html<NowReadingList> {
    html! {
        <li>
            {&item.who} {"читает"} {&item.what}
        </li>
    }
}

impl Renderable<NowReadingList> for NowReadingList {
    fn view(&self) -> Html<Self> {
        html! {
            <div class=("uk-width-1-4", "uk-card", "uk-card-body"),>
                <h4 class="uk-heading-line",>{ "Сейчас читают" }</h4>
                <ul class="uk-list",>
                    { for self.items.iter().map(|item| {
                        view_now_reading_item(item)
                    })}
                </ul>
            </div>
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Latest {
    img_url: String,
    caption: String,
    link: String
}

#[derive(PartialEq, Clone)]
pub struct LatestsProps {
    pub items: Vec<Latest>
}

impl Default for LatestsProps {
    fn default() -> Self {
        LatestsProps { items: vec![] }
    }
}

pub struct LatestsScrollNav {
    items: Vec<Latest>
}

impl Component for LatestsScrollNav {
    type Message = ();
    type Properties = LatestsProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        LatestsScrollNav {items: props.items}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        // Read-only, so no update.
        false
    }
}

fn render_latest(l: &Latest) -> Html<LatestsScrollNav> {
    html! {
        <li>
            <img src={&l.img_url}, alt="", uk-cover="",/>
            <div class="uk-overlay uk-light uk-overlay-default uk-position-bottom",>
                <p>{&l.caption}</p>
            </div>
        </li>
    }
}

impl Renderable<LatestsScrollNav> for LatestsScrollNav {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="uk-width-3-4",>
                <div class=("uk-position-relative", "uk-visible-toggle", "uk-light"), uk-slideshow="",>
                    <ul class="uk-slideshow-items",>
                        { for self.items.iter().map(render_latest) }
                    </ul>
                    <a class="uk-slidenav-large uk-position-center-left uk-position-small uk-hidden-hover", href="#", uk-slidenav-previous="", uk-slideshow-item="previous",></a>
                    <a class="uk-slidenav-large uk-position-center-right uk-position-small uk-hidden-hover", href="#", uk-slidenav-next="", uk-slideshow-item="next",></a>
                </div>
            </div>
        }
    }
}
