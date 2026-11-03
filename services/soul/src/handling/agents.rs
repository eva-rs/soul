//! Хендлеры `agents.*` (`07 §2.3`): только хозяину.

use eva_protocol::requests::Agents::list;

use crate::{
    components::Components,
    convert,
    db::Queries as _,
    state::{Policy, State},
};

/// `chat` — тимлид; несуществующий чат — пустой список: подагенты гибнут с ним каскадом.
#[eva::endpoint(list)]
pub async fn List<C: Components>(state: &mut State<C>, args: this::Args) -> this::Output {
    let State {
        shared,
        caller,
        db_conn,
        ..
    } = state;
    Policy::new(caller).owner()?;
    let this::Args { chat } = args;

    let db = db_conn.acquire(shared).await?;
    let rows = db.run(move |c| c.agents().list(chat)).await?;

    Ok(this::Ok {
        agents: rows.into_iter().map(convert::agent::from_db).collect(),
    })
}
