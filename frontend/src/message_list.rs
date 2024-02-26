use common::ChatMessage;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub messages: Vec<ChatMessage>,
}

#[function_component(MessageList)]
pub fn message_list(props: &Props) -> Html {
    html! {props.messages.iter().map(|m| html!{

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
