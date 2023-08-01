use crate::bindings;

pub fn task(name: &str, description: &str) -> ConfigurableTaskDefinition {
    bindings::config::task(name, description).into()
}

pub struct ConfigurableTaskDefinition {
    inner: bindings::config::ConfigurableTaskDefinition,
}

impl ConfigurableTaskDefinition {
    pub fn set_action(self, action: &bindings::config::ActionType) -> Self {
        self.inner.set_action(action).into()
    }
}

impl From<bindings::config::ConfigurableTaskDefinition> for ConfigurableTaskDefinition {
    fn from(inner: bindings::config::ConfigurableTaskDefinition) -> Self {
        Self { inner }
    }
}
