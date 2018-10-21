#[macro_use]
extern crate yew;
use yew::prelude::*;
use yew::virtual_dom::vnode::VNode;
use yew::virtual_dom::VList;
use yew::services::ConsoleService;

mod data;
mod components;

use components::{
    offer::OfferBuyForm,
    header::Header,
    latest::{LatestsScrollNav, NowReadingList},
    search::SearchForm,
    subsidiary::SubsidiaryList
};
use data::Book;

struct Model {
    books: Vec<Book>,
    console: ConsoleService
}

enum Msg {
    SubmitSearchForm,
    AddNewBook((String, String, Option<String>)),
    SubmitOfferBuyForm,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            books: vec![],
            console: ConsoleService::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddNewBook((author, title, isbn)) => {
                self.books.push(Book {author, title, isbn});
                true
            },
            Msg::SubmitSearchForm => true,
            Msg::SubmitOfferBuyForm => true
        }
    }

}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let subsidiary_list = vec!["Парковая, дом 3", "Парковая, дом 6"];
        let address_list: Vec<String> = subsidiary_list.into_iter().map(|s| s.to_owned()).collect();
        let title = ""; //env.APP_TITLE

        html! {
            <div class=("uk-container", "uk-container-center", "uk-margin-top", "uk-margin-bottom"),>
                <nav class="uk-navbar-container",uk-navbar="",>
                    <Header: title=title, />
                </nav>
                <div class=("uk-grid", "uk-card", "uk-card-body"),>
                    <SearchForm: on_submit=|_| Msg::SubmitSearchForm, />
                    <SubsidiaryList: title="Филиалы", items=address_list, />
                </div>
                <div class="uk-grid",>
                    <LatestsScrollNav:items=vec![], />
                    <NowReadingList:items=vec![], />
                </div>
                <div class=("uk-card", "uk-card-default", "uk-card-body"),>
                    <OfferBuyForm:on_submit=|_| Msg::SubmitOfferBuyForm, />
                </div>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    let model = Model { books: vec![], console: ConsoleService::new() };
    let mut con = ConsoleService::new();
    con.log("hello!");
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
