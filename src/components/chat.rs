use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::services::websocket::WebsocketService;
use crate::User;

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://api.dicebear.com/7.x/adventurer-neutral/svg?seed={}",
                                    u
                                ),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let my_username = user.username.borrow().clone();

        html! {
            <div class="flex w-screen">
                // Sidebar users
                <div class="flex-none w-56 h-screen bg-gray-100">
                    <div class="text-xl p-3 font-bold">{"Users"}</div>
                    {
                        self.users.clone().iter().map(|u| {
                            html!{
                                <div class="flex m-3 bg-white rounded-lg p-2 shadow">
                                    <div>
                                        <img class="w-12 h-12 rounded-full" src={u.avatar.clone()} alt="avatar"/>
                                    </div>
                                    <div class="flex-grow p-3">
                                        <div class="text-sm font-medium">{u.name.clone()}</div>
                                        <div class="text-xs text-gray-400">{"Hi there!"}</div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>

                // Area chat
                <div class="grow h-screen flex flex-col">
                    <div class="w-full h-14 border-b-2 border-gray-300 flex items-center px-4">
                        <div class="text-xl font-bold">{"💬 Chat!"}</div>
                    </div>
                    <div class="w-full grow overflow-auto border-b-2 border-gray-300 p-4">
                        {
                            self.messages.iter().map(|m| {
                                let is_mine = m.from == my_username;
                                let avatar = format!(
                                    "https://api.dicebear.com/7.x/adventurer-neutral/svg?seed={}",
                                    m.from
                                );

                                if is_mine {
                                    // Bubble kanan
                                    html!{
                                        <div class="flex flex-row-reverse items-end w-full mb-4">
                                            <img class="w-8 h-8 rounded-full ml-3" src={avatar} alt="avatar"/>
                                            <div class="bg-blue-500 text-white p-3 rounded-tl-lg rounded-tr-lg rounded-bl-lg shadow max-w-xs">
                                                <div class="text-xs font-medium mb-1 text-blue-100">{m.from.clone()}</div>
                                                <div class="text-sm">
                                                    if m.message.ends_with(".gif") {
                                                        <img class="mt-1 rounded" src={m.message.clone()} alt="gif"/>
                                                    } else {
                                                        {m.message.clone()}
                                                    }
                                                </div>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    // Bubble kiri
                                    html!{
                                        <div class="flex flex-row items-end w-full mb-4">
                                            <img class="w-8 h-8 rounded-full mr-3" src={avatar} alt="avatar"/>
                                            <div class="bg-gray-100 text-gray-800 p-3 rounded-tl-lg rounded-tr-lg rounded-br-lg shadow max-w-xs">
                                                <div class="text-xs font-medium mb-1 text-gray-500">{m.from.clone()}</div>
                                                <div class="text-sm">
                                                    if m.message.ends_with(".gif") {
                                                        <img class="mt-1 rounded" src={m.message.clone()} alt="gif"/>
                                                    } else {
                                                        {m.message.clone()}
                                                    }
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }
                            }).collect::<Html>()
                        }
                    </div>

                    // Input area
                    <div class="w-full h-14 flex px-3 items-center">
                        <input
                            ref={self.chat_input.clone()}
                            type="text"
                            placeholder="Message..."
                            class="flex-grow p-3 rounded-l-lg border border-gray-300 focus:outline-none"
                        />
                        <button
                            onclick={submit}
                            class="p-3 bg-blue-600 text-white rounded-r-lg hover:bg-blue-700"
                        >
                            {"➤"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}