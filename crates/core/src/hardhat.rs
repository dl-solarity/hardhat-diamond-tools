use ethabi::Contract as Abi;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HardhatArtifact {
    #[serde(rename = "_format")]
    pub format: String,
    pub contract_name: String,
    pub source_name: String,
    pub abi: Abi,
    pub bytecode: String,
    pub deployed_bytecode: String,
}
