//! Подагенты.

use crate::requests::Response;
use eva::service;

use crate::types::{ChatId, agent::Agent};

#[service(result = Response)]
pub trait Agents {
    #[args(derive(Default))]
    fn list(
        /// Подагенты этого тимлида; `None` — все.
        chat: Option<ChatId>,
    ) -> Struct! {
        agents: Vec<Agent>,
    };
    embed! {
        code!(Err => []);
    }
}
