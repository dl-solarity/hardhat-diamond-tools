use crate::bindings;

pub trait TaskParameter {
    fn add_params_to_task(
        task: bindings::config::ConfigurableTaskDefinition,
    ) -> bindings::config::ConfigurableTaskDefinition;
}
