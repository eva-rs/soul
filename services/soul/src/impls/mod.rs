//! Реализации сервисов следуют метаданным протокола и именам обработчиков.

use crate::{components::Components, handling, session::Session};

macro_rules! decorate {
    ($service:ident, $method:ident; $handler:expr) => {
        $handler
    };
}

#[eva::implement(eva_core::for_each_op, for<C: Components> Session<C>)]
macro_rules! project {
    ($this:ident, $accessor:ident, $method:ident, $Args:ty) => {
        $this.with_state(decorate!($accessor, $method; handling::$accessor::[<$method:camel>]))
    };
}
