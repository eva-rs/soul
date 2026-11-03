//! Модули, реестр сервисов и токены клиентов.

use crate::requests::Response;
use eva::service;

use crate::{
    errors,
    types::{
        ModuleId, PersonId,
        module::{Module, ServiceAddress, ServiceKey},
    },
};

#[service(result = Response)]
pub trait Modules {
    #[args(derive(Default))]
    fn list() -> Struct! {
        modules: Vec<Module>,
    };
    embed! {
        code!(Err => []);
    }

    #[flags(mutating)]
    #[errors(errors::modules::Taken)]
    fn add(
        module: ModuleId,
        address: ServiceAddress,
        /// `None` — soul генерирует ключ сам.
        key: Option<ServiceKey>,
    ) -> Struct! {
        /// Ключ, сгенерированный soul; показывается один раз.
        key: Option<ServiceKey>,
    };
    embed! {
        code!(Err => [Taken]);
    }

    #[flags(mutating)]
    #[errors(errors::modules::ServiceNotFound)]
    fn remove(module: ModuleId) -> Struct! {};
    embed! {
        code!(Err => [ServiceNotFound]);
    }

    #[flags(mutating)]
    fn token_issue(
        module: ModuleId,
        /// Заполняет только обёртка тула: актор тура.
        issued_by: Option<PersonId>,
    ) -> Struct! {
        /// Токен клиента; показывается один раз, soul хранит только хэш.
        token: String,
    };
    embed! {
        code!(Err => []);
    }

    #[flags(mutating)]
    #[errors(errors::modules::NotFound)]
    fn token_revoke(module: ModuleId) -> Struct! {};
    embed! {
        code!(Err => [NotFound]);
    }
}
