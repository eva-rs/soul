//! Хендлеры `hands_grants.*` (`07 §3.1`): постоянные разрешения рук,
//! только хозяину. Выдаёт их резолв `permission` с `allow_always`.

use eva_protocol::{
    errors::{self, ext::*},
    requests::HandsGrants::{list, revoke},
};

use crate::{
    components::Components,
    convert,
    db::{Db as _, Queries as _, hands_grants::GrantFilter},
    state::{Policy, State},
};

#[eva::endpoint(list)]
pub async fn List<C: Components>(state: &mut State<C>, args: this::Args) -> this::Output {
    let State {
        shared,
        caller,
        db_conn,
        ..
    } = state;
    Policy::new(caller).owner()?;
    let this::Args {
        endpoint,
        surface,
        chat,
    } = args;
    let filter = GrantFilter {
        endpoint,
        surface,
        chat,
    };

    let db = db_conn.acquire(shared).await?;
    let rows = db.run(move |c| c.hands_grants().list(&filter)).await?;

    Ok(this::Ok {
        grants: rows
            .into_iter()
            .map(convert::hands_grant::from_db)
            .collect(),
    })
}

#[eva::endpoint(revoke)]
pub async fn Revoke<C: Components>(state: &mut State<C>, args: this::Args) -> this::Output {
    let State { shared, caller, .. } = state;
    Policy::new(caller).owner()?;
    let this::Args { id } = args;

    let deleted = shared
        .db()
        .write(move |tx| tx.hands_grants().delete(id))
        .await
        .aux_err()?;
    if !deleted {
        return Err(errors::hands_grants::NotFound { id }.spec());
    }

    log::info!(grant:% = id, caller = caller.label(); "hands grant revoked");
    Ok(this::Ok {})
}
