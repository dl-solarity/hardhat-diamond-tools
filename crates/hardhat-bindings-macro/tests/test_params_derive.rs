use hardhat_bindings_macro::TaskParameter;

#[test]
fn test_params_derive() {
    #[derive(TaskParameter, Default)]
    struct Params {
        /// Small doc
        pub flag: bool,
        /// Big doc
        ///
        /// With multiple lines
        pub flag1: bool,
        /// Big doc
        ///
        /// With multiple lines
        ///
        /// And even more
        pub name: String,
        pub optional: Option<String>,
        pub variadic: Vec<String>,
        pub optional_variadic: Option<Vec<String>>,
    }
}
