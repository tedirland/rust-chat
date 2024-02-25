use common::ChatMessage;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct Props {
    messages: Vec<ChatMessage>,
}

#[function_component(MessageList)]
fn message_list(props: &Props) -> Html {
    html! {props.messages.iter().map(|m| html!{
                        // <div class="list-group-item list-group-item-action">
                        //     <div class="d-flex w-100 justify-content-between">
                        //         <h5>{m.author.clone()}</h5>
                        //         <small>{m.created_at.format("Sent at %l %p on %b %-d").to_string()}</small>
                        //     </div>
                        //     <p>
                        //     {m.message.clone()}
                        // </p>
                        // </div>
                        <div class="card" style="width: 18rem;">
                            <div class="card-body">
                              <h5 class="card-title">{m.author.clone()}</h5>
                              <h6 class="card-subtitle mb-2 text-body-secondary">{m.created_at.format("Sent at %l %p on %b %-d").to_string()}</h6>
                              <p class="card-text">{m.message.clone()}</p>
                            </div>
                          </div>

                    }
                    ).collect::<Html>()
    }
}
