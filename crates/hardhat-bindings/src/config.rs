use std::{future::Future, marker::PhantomData};

use ::wasm_bindgen::prelude::JsValue;
use js_sys::Promise;
use serde::de::DeserializeOwned;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen_futures::future_to_promise;

use crate::{
    bindings::{self},
    params::TaskParameter,
    HardhatRuntimeEnvironment,
};

pub fn task<Action, Args, F, E>(
    name: &str,
    description: &str,
) -> ConfigurableTaskDefinition<Action, Args, F, E>
where
    Args: TaskParameter + DeserializeOwned,
    Action: Fn(Args, HardhatRuntimeEnvironment) -> F,
    F: Future<Output = Result<(), E>>,
    E: std::error::Error + 'static,
{
    bindings::config::task(name, description).into()
}

pub struct ConfigurableTaskDefinition<Action, Args, F, E>
where
    Action: Fn(Args, HardhatRuntimeEnvironment) -> F,
    Args: TaskParameter + DeserializeOwned,
    F: Future<Output = Result<(), E>>,
    E: std::error::Error + 'static,
{
    inner: bindings::config::ConfigurableTaskDefinition,

    _action: PhantomData<Action>,
    _args: PhantomData<Args>,
    _f: PhantomData<F>,
    _e: PhantomData<E>,
}

impl<Args, Action, F, E> ConfigurableTaskDefinition<Action, Args, F, E>
where
    Args: TaskParameter + DeserializeOwned + 'static,
    Action: (Fn(Args, HardhatRuntimeEnvironment) -> F) + Copy + 'static,
    F: Future<Output = Result<(), E>>,
    E: std::error::Error + 'static,
{
    pub fn set_action(self, action: Action) -> bindings::config::ActionType {
        let conf = self.add_params();

        let action = Closure::new({
            move |args: JsValue, hre: bindings::runtime::HardhatRuntimeEnvironment| -> Promise {
                let hre = hre.into();

                let args: Args = match serde_wasm_bindgen::from_value(args) {
                    Ok(args) => args,
                    Err(err) => return ::js_sys::Promise::reject(&err.into()),
                };

                future_to_promise(async move {
                    action(args, hre)
                        .await
                        .map_err(|err| JsValue::from(err.to_string()))?;

                    Ok(JsValue::UNDEFINED)
                })
            }
        });

        conf.inner.set_action(&action);

        action
    }

    fn add_params(self) -> Self {
        Args::add_params_to_task(self.inner).into()
    }
}

impl<Args, Action, F, E> From<bindings::config::ConfigurableTaskDefinition>
    for ConfigurableTaskDefinition<Action, Args, F, E>
where
    Args: TaskParameter + DeserializeOwned,
    Action: Fn(Args, HardhatRuntimeEnvironment) -> F,
    F: Future<Output = Result<(), E>>,
    E: std::error::Error + 'static,
{
    fn from(inner: bindings::config::ConfigurableTaskDefinition) -> Self {
        Self {
            inner,
            _action: PhantomData,
            _args: PhantomData,
            _f: PhantomData,
            _e: PhantomData,
        }
    }
}
