//! Хендлеры `models.*` (`01 §8.2`, `11 §3`).

use eva_protocol::requests::Models::list;

use crate::{
    components::Components,
    convert,
    state::{Policy, State},
};

#[eva::endpoint(list)]
pub async fn List<C: Components>(state: &mut State<C>, args: this::Args) -> this::Output {
    let State { shared, caller, .. } = state;
    Policy::new(caller).owner()?;
    let this::Args {} = args;

    Ok(this::Ok {
        models: convert::model::from_registry(&shared.models()),
    })
}
