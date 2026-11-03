//! Хендлеры `cron.*` (`08 §1.7`): только чтение и только хозяину;
//! мутации джоб — тулы модели.

use eva_protocol::{
    errors::{Aux, ext::*},
    requests::{
        Cron::{list, runs},
        Response,
    },
};

use crate::{
    components::Components,
    convert,
    db::Queries as _,
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
    let this::Args {} = args;

    let db = db_conn.acquire(shared).await?;
    let rows = db.run(|c| c.cron().jobs()).await?;

    Ok(this::Ok {
        jobs: rows.into_iter().map(convert::cron::job).collect(),
    })
}

#[eva::endpoint(runs)]
pub async fn Runs<C: Components>(state: &mut State<C>, args: this::Args) -> this::Output {
    let State {
        shared,
        caller,
        db_conn,
        ..
    } = state;
    Policy::new(caller).owner()?;
    let this::Args { job, limit, after } = args;

    let db = db_conn.acquire(shared).await?;
    let rows = db
        .run(move |c| -> Response<_, this::Err> {
            if let Some(after) = &after {
                c.cron()
                    .run_exists(after)?
                    .true_or(Aux::NotFound(format!("cron run {after}")))?;
            }
            Ok(c.cron()
                .runs(job.as_ref(), after.as_ref(), limit.into_inner())?)
        })
        .await?;

    Ok(this::Ok {
        found: rows.into_iter().map(convert::cron::run).collect(),
    })
}
