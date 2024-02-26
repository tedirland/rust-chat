use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub users: Vec<String>,
}

#[function_component(UsersList)]
pub fn users_list(props: &Props) -> Html {
    html! {}
}
