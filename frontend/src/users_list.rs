use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub users: Vec<String>,
}

#[function_component(UsersList)]
pub fn users_list(props: &Props) -> Html {
    html! {
    <table class="table table-bordered table-dark">
    <tbody>
    {props.users.iter()
        .map(|username| html!{<tr><td class="text-center">{username}</td></tr>})
        .collect::<Html>()}
        </tbody>
    </table>
    }
}
