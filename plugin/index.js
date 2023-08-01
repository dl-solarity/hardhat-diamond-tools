const { task } = require("hardhat/config");
const { names_action, merge_artifacts_action } = require("./pkg");

task("names", "prints names of the contracts")
    .setAction(async (args, hre) => {
        await names_action(args, hre, null)
    });

task("merge")
    .addOptionalParam("outDir")
    .addOptionalParam("outContractName")
    .addOptionalParam("filter")
    .setAction(async (args, hre) => {
        await merge_artifacts_action(args, hre, null)
    });
