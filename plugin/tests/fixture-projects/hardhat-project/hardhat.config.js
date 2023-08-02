require("../../../index")

module.exports = {
  solidity: {
    compilers: [
      {
        version: "0.8.20",
      },
    ],
  },
  diamondMerge: {
    outDir: "artifacts/merged",
    outContractName: "DiamondProxy",
    filter: {
      include: [
        "getB",
      ]
    },
  },
};
