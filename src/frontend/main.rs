#[macro_use]
extern crate yew;
use yew::prelude::*;

struct Book {
    author: String,
    title: String,
    isbn: Option<String>
}

struct Model {
    books: Vec<Book>,
}

enum Msg {
    AddNewBook((String, String, Option<String>)),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { books: vec![] }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddNewBook((author, title, isbn)) => {
                self.books.push(Book {author, title, isbn});
                true
            }
        }
    }

}

fn render_book(b: &Book) -> Html<Model> {
    html!{
        <tr>
            <td>{ &b.author }</td>
            <td>{ &b.title }</td>
            <td></td>
        </tr>
    }
}
impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <table>
                <thead>
                    <tr>
                        <th>{ "Автор" }</th>
                        <th>{ "Название" }</th>
                        <th>{ "Описание" }</th>
                    </tr>
                </thead>
                <tbody>
                    {for self.books.iter().map(move |b| render_book(b))}
                </tbody>
            </table>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
