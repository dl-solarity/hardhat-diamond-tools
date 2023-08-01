use hardhat_bindings_macro::hardhat_action;

#[derive(serde::Deserialize, Default)]
pub struct Args {
    pub contracts: Vec<String>,
}

#[hardhat_action]
pub async fn names_action(
    args: Args,
    hre: HardhatRuntimeEnvironment,
) -> Result<(), Box<dyn Error>> {
    todo!("Implement me")
}