//! Крон: только чтение; мутации — тулы модели.

use crate::requests::Response;
use eva::service;

use crate::{
    requests::RunsLimit,
    types::{
        CronJobId, RunId,
        cron::{Job, Run},
    },
};

#[service(result = Response)]
pub trait Cron {
    #[args(derive(Default))]
    fn list() -> Struct! {
        jobs: Vec<Job>,
    };
    embed! {
        code!(Err => []);
    }

    #[args(derive(Default))]
    fn runs(
        job: Option<CronJobId>,
        #[serde(default)] limit: RunsLimit,
        /// Keyset по `(started, run_id)`.
        after: Option<RunId>,
    ) -> Struct! {
        /// Новейшие первыми.
        found: Vec<Run>,
    };
    embed! {
        code!(Err => []);
    }
}
