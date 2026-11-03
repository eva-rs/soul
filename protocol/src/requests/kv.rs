//! KV модуля, last-write-wins.

use crate::requests::Response;
use eva::service;

use crate::{requests::Limit, types::Value};

#[service(result = Response)]
pub trait Kv {
    fn get(
        key: String,
    ) -> Struct! {
        value: Option<Value>,
    };
    embed! {
        code!(Err => []);
    }

    #[flags(mutating)]
    fn set(key: String, value: Value) -> Struct! {};
    embed! {
        code!(Err => []);
    }

    #[flags(mutating)]
    fn del(key: String) -> Struct! {};
    embed! {
        code!(Err => []);
    }

    #[args(derive(Default))]
    fn list(
        #[serde(default)] prefix: String,
        #[serde(default)] limit: Limit,
        /// Ключи по возрастанию, keyset по ключу.
        after: Option<String>,
    ) -> Struct! {
        keys: Vec<String>,
    };
    embed! {
        code!(Err => []);
    }
}
