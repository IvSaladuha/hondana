use yew::prelude::*;

mod data;

pub struct BookFormComponent {
    initial: Option<data::Book>,
    onsubmit: Option<Callback<()>>
}

impl Component for BookFormComponent {
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        NewBookFormComponent {}
    }
}

impl Renderable<BookFormComponent> for NewBookFormComponent {
    fn view(&self) -> Html<Self> {
        html! {
            <form>
                <label>{"Название"}<input type="text" name="title"/></label>
                <label>{"Автор"}<input type="text" name="author"/></label>
                <label>{"ISBN"}<input type="text" name="isbn"/></label>
                <button onclick=|_| Msg::Submit>{"Сохранить"}</button>
            </form>
        }
    }
}
