//! Хендлеры `sleep.*` (`08 §2.8`): журнал ночей, только хозяину.

use eva_protocol::{
    errors::{self, Aux, ext::*},
    requests::{
        Response,
        Sleep::{cancel, phase_runs, runs, start, status},
    },
};

use crate::{
    components::Components,
    convert,
    db::{DbError, Queries as _},
    soul::Soul,
    state::{Policy, State},
    subsystems::{sleep::Sleep, turns::driver::Driver},
};

#[eva::endpoint(status)]
pub async fn Status<C: Components>(state: &mut State<C>, args: this::Args) -> this::Output {
    let State {
        shared,
        caller,
        db_conn,
        ..
    } = state;
    Policy::new(caller).sleep_control()?;
    let this::Args {} = args;
    let db = db_conn.acquire(shared).await?;
    let last_run_at = db.run(|c| c.sleep().last_started()).await?;
    Ok(this::Ok {
        enabled: shared.cfg().sleep.enabled,
        active: shared.sleep().current(),
        last_run_at,
    })
}

#[eva::endpoint(start)]
pub async fn Start<C: Components>(state: &mut State<C>, args: this::Args) -> this::Output {
    Policy::new(&state.caller).sleep_control()?;
    Sleep::new(
        Soul::from_shared(state.shared.clone()),
        Driver::new(state.shared.clone()),
    )
    .start(args.dry_run)
    .await
}

#[eva::endpoint(cancel)]
pub async fn Cancel<C: Components>(state: &mut State<C>, args: this::Args) -> this::Output {
    Policy::new(&state.caller).sleep_control()?;
    state
        .shared
        .sleep()
        .cancel(&args.run)
        .map_err(|error| error.spec())?;
    Ok(this::Ok { run: args.run })
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
    let this::Args { limit, after } = args;

    let db = db_conn.acquire(shared).await?;
    let rows = db
        .run(move |c| -> Response<_, this::Err> {
            if let Some(after) = &after {
                c.sleep()
                    .exists(after)?
                    .true_or(Aux::NotFound(format!("sleep run {after}")))?;
            }
            Ok(c.sleep().runs(after.as_ref(), limit.into_inner())?)
        })
        .await?;

    Ok(this::Ok {
        found: rows.into_iter().map(convert::sleep::run).collect(),
    })
}

#[eva::endpoint(phase_runs)]
pub async fn PhaseRuns<C: Components>(state: &mut State<C>, args: this::Args) -> this::Output {
    let State {
        shared,
        caller,
        db_conn,
        ..
    } = state;
    Policy::new(caller).owner()?;
    let this::Args { run } = args;

    let db = db_conn.acquire(shared).await?;
    let rows = db
        .run(move |c| -> Response<_, this::Err> {
            c.sleep()
                .exists(&run)?
                .true_or(errors::sleep::RunNotFound { run: run.clone() }.spec())?;
            Ok(c.sleep().phases(&run)?)
        })
        .await?;

    let phases = rows
        .into_iter()
        .map(convert::sleep::phase)
        .collect::<Result<_, DbError>>()?;
    Ok(this::Ok { phases })
}
