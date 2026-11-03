//! Реализации сервисов следуют метаданным протокола и именам обработчиков.

use eva::handling::CallStep;

use crate::{components::Components, handling, session::Session};

macro_rules! decorate {
    ($service:ident, $method:ident; $handler:expr) => {
        $handler
    };
}

macro_rules! project {
    ($(
        $accessor:ident {
            trait: $Trait:path,
            client: $Client:path,
            methods: {$(
                $method:ident {
                    flags: [$($flag:ident),*],
                    lifetimes: [$($lt:lifetime),*],
                    args: $Args:ty,
                    ok: $Ok:ty,
                    err: $Err:ty,
                    output: $Output:ty,
                    docs: [$($doc:literal),*],
                    $($method_extra:tt)*
                }
            )*},
            $($extra:tt)*
        }
    )*) => { eva::paste! {
        $(impl<C: Components> $Trait for Session<C> {
            $(fn $method<$($lt),*>(&mut self) -> impl CallStep<$Args, Output = $Output> {
                self.with_state(decorate!($accessor, $method; handling::$accessor::[<$method:camel>]))
            })*
        })*
    } };
}

eva_core::for_each_op!(project);
