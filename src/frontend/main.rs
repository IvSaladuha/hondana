#[macro_use]
extern crate yew;
use yew::prelude::*;

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
    show_edit_form: bool
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
            show_edit_form: false
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

fn render_book(b: &Book) -> Html<Model> {
    html!{
        <tr>
            <td>{ &b.author }</td>
            <td>{ &b.title }</td>
            <td></td>
            <td>{ "Редактировать" }
        </tr>
    }
}
impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let subsidiary_list = vec!["Парковая, дом 3", "Парковая, дом 6"];
        let address_list: Vec<String> = subsidiary_list.into_iter().map(|s| s.to_owned()).collect();

        html! {
            <div class=("uk-container", "uk-container-center", "uk-margin-top", "uk-margin-bottom"),>
                <Header: title="Библиотека Посёлка Программистов", />
                <div class=("uk-grid", "uk-card", "uk-card-body"),>
                    <SearchForm: on_submit=|_| Msg::SubmitSearchForm, />
                    <SubsidiaryList: title="Филиалы", items=address_list, />
                </div>
                <div class="uk-grid",>
                    <LatestsScrollNav:items=vec![], />
                    <NowReadingList:items=vec![], />
                </div>
                <OfferBuyForm:on_submit=|_| Msg::SubmitOfferBuyForm, />
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
