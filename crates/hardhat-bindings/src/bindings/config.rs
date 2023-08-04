use js_sys::Promise;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsValue,
};

use super::runtime::HardhatRuntimeEnvironment;

/// The [`ActionType`] bindings for Rust.
///
/// NOTE: First value if the arguments of the task defined by plugin developer, that's why
/// we use `any` (see [`JsValue`]).
///
/// [`ActionType`]: https://github.com/NomicFoundation/hardhat/blob/main/packages/hardhat-core/src/types/runtime.ts#L161
pub type ActionType = Closure<dyn Fn(JsValue, HardhatRuntimeEnvironment) -> Promise>;

/// The [`ConfigurableTaskDefinition`] bindings for Rust.
///
/// [`ConfigurableTaskDefinition`]: https://github.com/NomicFoundation/hardhat/blob/main/packages/hardhat-core/src/types/runtime.ts#L43C18-L43C44
#[wasm_bindgen(module = "hardhat/config")]
extern "C" {
    pub type ConfigurableTaskDefinition;

    #[wasm_bindgen(method, js_name = "setDescription")]
    pub fn set_description(
        this: &ConfigurableTaskDefinition,
        description: &str,
    ) -> ConfigurableTaskDefinition;

    #[wasm_bindgen(method, js_name = "setAction")]
    pub fn set_action(
        this: &ConfigurableTaskDefinition,
        action: &ActionType,
    ) -> ConfigurableTaskDefinition;

    #[wasm_bindgen(method, js_name = "addParam")]
    pub fn add_param(
        this: &ConfigurableTaskDefinition,
        name: &str,
        description: &str,
        default_value: JsValue,
    ) -> ConfigurableTaskDefinition;

    #[wasm_bindgen(method, js_name = "addOptionalParam")]
    pub fn add_optional_param(
        this: &ConfigurableTaskDefinition,
        name: &str,
        description: &str,
        default_value: JsValue,
    ) -> ConfigurableTaskDefinition;

    #[wasm_bindgen(method, js_name = "addVariadicPositionalParam")]
    pub fn add_variadic_positional_param(
        this: &ConfigurableTaskDefinition,
        name: &str,
        description: &str,
        default_value: JsValue,
    ) -> ConfigurableTaskDefinition;

    #[wasm_bindgen(method, js_name = "addOptionalVariadicPositionalParam")]
    pub fn add_optional_variadic_positional_param(
        this: &ConfigurableTaskDefinition,
        name: &str,
        description: &str,
        default_value: JsValue,
    ) -> ConfigurableTaskDefinition;

    #[wasm_bindgen(method, js_name = "addFlag")]
    pub fn add_flag(
        this: &ConfigurableTaskDefinition,
        name: &str,
        description: &str,
        default_value: bool,
    ) -> ConfigurableTaskDefinition;
}

#[wasm_bindgen(module = "hardhat/config")]
extern "C" {
    #[wasm_bindgen(js_name = "task")]
    pub fn task(name: &str, description: &str) -> ConfigurableTaskDefinition;
}
