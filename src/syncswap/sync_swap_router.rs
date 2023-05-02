pub use sync_swap_router::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod sync_swap_router {
    #[rustfmt::skip]
    const __ABI: &str = "[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_vault\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_wETH\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"ApproveFailed\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"Expired\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NotEnoughLiquidityMinted\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"TooLittleReceived\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"TransferFromFailed\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"pool\",\n        \"type\": \"address\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"token\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct SyncSwapRouter.TokenInput[]\",\n        \"name\": \"inputs\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"minLiquidity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"callback\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"callbackData\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"addLiquidity\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"pool\",\n        \"type\": \"address\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"token\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct SyncSwapRouter.TokenInput[]\",\n        \"name\": \"inputs\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"minLiquidity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"callback\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"callbackData\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"addLiquidity2\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"pool\",\n        \"type\": \"address\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"token\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct SyncSwapRouter.TokenInput[]\",\n        \"name\": \"inputs\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"minLiquidity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"callback\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"callbackData\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"token\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"approveAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"deadline\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"v\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"r\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"s\",\n            \"type\": \"bytes32\"\n          }\n        ],\n        \"internalType\": \"struct IRouter.SplitPermitParams[]\",\n        \"name\": \"permits\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"name\": \"addLiquidityWithPermit\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"pool\",\n        \"type\": \"address\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"token\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct SyncSwapRouter.TokenInput[]\",\n        \"name\": \"inputs\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"minLiquidity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"callback\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"callbackData\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"token\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"approveAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"deadline\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"v\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"r\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"s\",\n            \"type\": \"bytes32\"\n          }\n        ],\n        \"internalType\": \"struct IRouter.SplitPermitParams[]\",\n        \"name\": \"permits\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"name\": \"addLiquidityWithPermit2\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"pool\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"minAmounts\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"callback\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"callbackData\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"burnLiquidity\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"token\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct IPool.TokenAmount[]\",\n        \"name\": \"amounts\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"pool\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"minAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"callback\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"callbackData\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"burnLiquiditySingle\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"token\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct IPool.TokenAmount\",\n        \"name\": \"amountOut\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"pool\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"minAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"callback\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"callbackData\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"approveAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"deadline\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"signature\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct IRouter.ArrayPermitParams\",\n        \"name\": \"permit\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"burnLiquiditySingleWithPermit\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"token\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct IPool.TokenAmount\",\n        \"name\": \"amountOut\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"pool\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"minAmounts\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"callback\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"callbackData\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"approveAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"deadline\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"signature\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct IRouter.ArrayPermitParams\",\n        \"name\": \"permit\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"burnLiquidityWithPermit\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"token\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct IPool.TokenAmount[]\",\n        \"name\": \"amounts\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_factory\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"createPool\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"enteredPools\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"enteredPoolsLength\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"isPoolEntered\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes[]\",\n        \"name\": \"data\",\n        \"type\": \"bytes[]\"\n      }\n    ],\n    \"name\": \"multicall\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes[]\",\n        \"name\": \"results\",\n        \"type\": \"bytes[]\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"v\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"r\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"s\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"selfPermit\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"signature\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"selfPermit2\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"signature\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"selfPermit2IfNecessary\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"nonce\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"expiry\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"v\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"r\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"s\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"selfPermitAllowed\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"nonce\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"expiry\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"v\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"r\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"s\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"selfPermitAllowedIfNecessary\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"v\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"r\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"s\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"selfPermitIfNecessary\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"stakingPool\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"onBehalf\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"stake\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"pool\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"bytes\",\n                \"name\": \"data\",\n                \"type\": \"bytes\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"callback\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"bytes\",\n                \"name\": \"callbackData\",\n                \"type\": \"bytes\"\n              }\n            ],\n            \"internalType\": \"struct IRouter.SwapStep[]\",\n            \"name\": \"steps\",\n            \"type\": \"tuple[]\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"tokenIn\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amountIn\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct IRouter.SwapPath[]\",\n        \"name\": \"paths\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountOutMin\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"swap\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"token\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct IPool.TokenAmount\",\n        \"name\": \"amountOut\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"pool\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"bytes\",\n                \"name\": \"data\",\n                \"type\": \"bytes\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"callback\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"bytes\",\n                \"name\": \"callbackData\",\n                \"type\": \"bytes\"\n              }\n            ],\n            \"internalType\": \"struct IRouter.SwapStep[]\",\n            \"name\": \"steps\",\n            \"type\": \"tuple[]\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"tokenIn\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amountIn\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct IRouter.SwapPath[]\",\n        \"name\": \"paths\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amountOutMin\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"token\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"approveAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"deadline\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint8\",\n            \"name\": \"v\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"r\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"s\",\n            \"type\": \"bytes32\"\n          }\n        ],\n        \"internalType\": \"struct IRouter.SplitPermitParams\",\n        \"name\": \"permit\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"swapWithPermit\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"token\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct IPool.TokenAmount\",\n        \"name\": \"amountOut\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"vault\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"wETH\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]";
    ///The parsed JSON ABI of thse contract.
    pub static SYNCSWAPROUTER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct SyncSwapRouter<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SyncSwapRouter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SyncSwapRouter<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SyncSwapRouter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SyncSwapRouter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(SyncSwapRouter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SyncSwapRouter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SYNCSWAPROUTER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `addLiquidity` (0x6cbe96fa) function
        pub fn add_liquidity(
            &self,
            pool: ::ethers::core::types::Address,
            inputs: ::std::vec::Vec<TokenInput>,
            data: ::ethers::core::types::Bytes,
            min_liquidity: ::ethers::core::types::U256,
            callback: ::ethers::core::types::Address,
            callback_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [108, 190, 150, 250],
                    (pool, inputs, data, min_liquidity, callback, callback_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addLiquidity2` (0x94ec6d78) function
        pub fn add_liquidity_2(
            &self,
            pool: ::ethers::core::types::Address,
            inputs: ::std::vec::Vec<TokenInput>,
            data: ::ethers::core::types::Bytes,
            min_liquidity: ::ethers::core::types::U256,
            callback: ::ethers::core::types::Address,
            callback_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [148, 236, 109, 120],
                    (pool, inputs, data, min_liquidity, callback, callback_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addLiquidityWithPermit` (0xc4b3fc40) function
        pub fn add_liquidity_with_permit(
            &self,
            pool: ::ethers::core::types::Address,
            inputs: ::std::vec::Vec<TokenInput>,
            data: ::ethers::core::types::Bytes,
            min_liquidity: ::ethers::core::types::U256,
            callback: ::ethers::core::types::Address,
            callback_data: ::ethers::core::types::Bytes,
            permits: ::std::vec::Vec<SplitPermitParams>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [196, 179, 252, 64],
                    (
                        pool,
                        inputs,
                        data,
                        min_liquidity,
                        callback,
                        callback_data,
                        permits,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addLiquidityWithPermit2` (0xced78795) function
        pub fn add_liquidity_with_permit_2(
            &self,
            pool: ::ethers::core::types::Address,
            inputs: ::std::vec::Vec<TokenInput>,
            data: ::ethers::core::types::Bytes,
            min_liquidity: ::ethers::core::types::U256,
            callback: ::ethers::core::types::Address,
            callback_data: ::ethers::core::types::Bytes,
            permits: ::std::vec::Vec<SplitPermitParams>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [206, 215, 135, 149],
                    (
                        pool,
                        inputs,
                        data,
                        min_liquidity,
                        callback,
                        callback_data,
                        permits,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burnLiquidity` (0xad271fa3) function
        pub fn burn_liquidity(
            &self,
            pool: ::ethers::core::types::Address,
            liquidity: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            min_amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            callback: ::ethers::core::types::Address,
            callback_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<TokenAmount>> {
            self.0
                .method_hash(
                    [173, 39, 31, 163],
                    (pool, liquidity, data, min_amounts, callback, callback_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burnLiquiditySingle` (0x53c43f15) function
        pub fn burn_liquidity_single(
            &self,
            pool: ::ethers::core::types::Address,
            liquidity: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            min_amount: ::ethers::core::types::U256,
            callback: ::ethers::core::types::Address,
            callback_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, TokenAmount> {
            self.0
                .method_hash(
                    [83, 196, 63, 21],
                    (pool, liquidity, data, min_amount, callback, callback_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burnLiquiditySingleWithPermit` (0x7d10c9d6) function
        pub fn burn_liquidity_single_with_permit(
            &self,
            pool: ::ethers::core::types::Address,
            liquidity: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            min_amount: ::ethers::core::types::U256,
            callback: ::ethers::core::types::Address,
            callback_data: ::ethers::core::types::Bytes,
            permit: ArrayPermitParams,
        ) -> ::ethers::contract::builders::ContractCall<M, TokenAmount> {
            self.0
                .method_hash(
                    [125, 16, 201, 214],
                    (
                        pool,
                        liquidity,
                        data,
                        min_amount,
                        callback,
                        callback_data,
                        permit,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burnLiquidityWithPermit` (0x353766c6) function
        pub fn burn_liquidity_with_permit(
            &self,
            pool: ::ethers::core::types::Address,
            liquidity: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
            min_amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            callback: ::ethers::core::types::Address,
            callback_data: ::ethers::core::types::Bytes,
            permit: ArrayPermitParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<TokenAmount>> {
            self.0
                .method_hash(
                    [53, 55, 102, 198],
                    (
                        pool,
                        liquidity,
                        data,
                        min_amounts,
                        callback,
                        callback_data,
                        permit,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createPool` (0x9dd41df2) function
        pub fn create_pool(
            &self,
            factory: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([157, 212, 29, 242], (factory, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enteredPools` (0x2b4abadb) function
        pub fn entered_pools(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([43, 74, 186, 219], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enteredPoolsLength` (0xb956b3fb) function
        pub fn entered_pools_length(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([185, 86, 179, 251], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPoolEntered` (0x4f25b858) function
        pub fn is_pool_entered(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([79, 37, 184, 88], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multicall` (0xac9650d8) function
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([172, 150, 80, 216], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermit` (0xf3995c67) function
        pub fn self_permit(
            &self,
            token: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 153, 92, 103], (token, value, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermit2` (0x6cc781cd) function
        pub fn self_permit_2(
            &self,
            token: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 199, 129, 205], (token, value, deadline, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermit2IfNecessary` (0x688ee44c) function
        pub fn self_permit_2_if_necessary(
            &self,
            token: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            signature: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 142, 228, 76], (token, value, deadline, signature))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermitAllowed` (0x4659a494) function
        pub fn self_permit_allowed(
            &self,
            token: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
            expiry: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 89, 164, 148], (token, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermitAllowedIfNecessary` (0xa4a78f0c) function
        pub fn self_permit_allowed_if_necessary(
            &self,
            token: ::ethers::core::types::Address,
            nonce: ::ethers::core::types::U256,
            expiry: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 167, 143, 12], (token, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selfPermitIfNecessary` (0xc2e3140a) function
        pub fn self_permit_if_necessary(
            &self,
            token: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 227, 20, 10], (token, value, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stake` (0x6291027c) function
        pub fn stake(
            &self,
            staking_pool: ::ethers::core::types::Address,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            on_behalf: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 145, 2, 124], (staking_pool, token, amount, on_behalf))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0x2cc4081e) function
        pub fn swap(
            &self,
            paths: ::std::vec::Vec<SwapPath>,
            amount_out_min: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, TokenAmount> {
            self.0
                .method_hash([44, 196, 8, 30], (paths, amount_out_min, deadline))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapWithPermit` (0xe84d494b) function
        pub fn swap_with_permit(
            &self,
            paths: ::std::vec::Vec<SwapPath>,
            amount_out_min: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            permit: SplitPermitParams,
        ) -> ::ethers::contract::builders::ContractCall<M, TokenAmount> {
            self.0
                .method_hash([232, 77, 73, 75], (paths, amount_out_min, deadline, permit))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vault` (0xfbfa77cf) function
        pub fn vault(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([251, 250, 119, 207], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wETH` (0xf2428621) function
        pub fn w_eth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([242, 66, 134, 33], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for SyncSwapRouter<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ApproveFailed` with signature `ApproveFailed()` and selector `0x3e3f8f73`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ApproveFailed", abi = "ApproveFailed()")]
    pub struct ApproveFailed;
    ///Custom Error type `Expired` with signature `Expired()` and selector `0x203d82d8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Expired", abi = "Expired()")]
    pub struct Expired;
    ///Custom Error type `NotEnoughLiquidityMinted` with signature `NotEnoughLiquidityMinted()` and selector `0x249942be`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotEnoughLiquidityMinted", abi = "NotEnoughLiquidityMinted()")]
    pub struct NotEnoughLiquidityMinted;
    ///Custom Error type `TooLittleReceived` with signature `TooLittleReceived()` and selector `0xc9f52c71`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "TooLittleReceived", abi = "TooLittleReceived()")]
    pub struct TooLittleReceived;
    ///Custom Error type `TransferFromFailed` with signature `TransferFromFailed()` and selector `0x7939f424`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "TransferFromFailed", abi = "TransferFromFailed()")]
    pub struct TransferFromFailed;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SyncSwapRouterErrors {
        ApproveFailed(ApproveFailed),
        Expired(Expired),
        NotEnoughLiquidityMinted(NotEnoughLiquidityMinted),
        TooLittleReceived(TooLittleReceived),
        TransferFromFailed(TransferFromFailed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SyncSwapRouterErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ApproveFailed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ApproveFailed(decoded));
            }
            if let Ok(decoded) = <Expired as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Expired(decoded));
            }
            if let Ok(decoded) =
                <NotEnoughLiquidityMinted as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NotEnoughLiquidityMinted(decoded));
            }
            if let Ok(decoded) = <TooLittleReceived as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TooLittleReceived(decoded));
            }
            if let Ok(decoded) =
                <TransferFromFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferFromFailed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SyncSwapRouterErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ApproveFailed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Expired(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotEnoughLiquidityMinted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TooLittleReceived(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFromFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for SyncSwapRouterErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <ApproveFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Expired as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NotEnoughLiquidityMinted as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <TooLittleReceived as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <TransferFromFailed as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for SyncSwapRouterErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApproveFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::Expired(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotEnoughLiquidityMinted(element) => ::core::fmt::Display::fmt(element, f),
                Self::TooLittleReceived(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFromFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for SyncSwapRouterErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ApproveFailed> for SyncSwapRouterErrors {
        fn from(value: ApproveFailed) -> Self {
            Self::ApproveFailed(value)
        }
    }
    impl ::core::convert::From<Expired> for SyncSwapRouterErrors {
        fn from(value: Expired) -> Self {
            Self::Expired(value)
        }
    }
    impl ::core::convert::From<NotEnoughLiquidityMinted> for SyncSwapRouterErrors {
        fn from(value: NotEnoughLiquidityMinted) -> Self {
            Self::NotEnoughLiquidityMinted(value)
        }
    }
    impl ::core::convert::From<TooLittleReceived> for SyncSwapRouterErrors {
        fn from(value: TooLittleReceived) -> Self {
            Self::TooLittleReceived(value)
        }
    }
    impl ::core::convert::From<TransferFromFailed> for SyncSwapRouterErrors {
        fn from(value: TransferFromFailed) -> Self {
            Self::TransferFromFailed(value)
        }
    }
    ///Container type for all input parameters for the `addLiquidity` function with signature `addLiquidity(address,(address,uint256)[],bytes,uint256,address,bytes)` and selector `0x6cbe96fa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "addLiquidity",
        abi = "addLiquidity(address,(address,uint256)[],bytes,uint256,address,bytes)"
    )]
    pub struct AddLiquidityCall {
        pub pool: ::ethers::core::types::Address,
        pub inputs: ::std::vec::Vec<TokenInput>,
        pub data: ::ethers::core::types::Bytes,
        pub min_liquidity: ::ethers::core::types::U256,
        pub callback: ::ethers::core::types::Address,
        pub callback_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `addLiquidity2` function with signature `addLiquidity2(address,(address,uint256)[],bytes,uint256,address,bytes)` and selector `0x94ec6d78`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "addLiquidity2",
        abi = "addLiquidity2(address,(address,uint256)[],bytes,uint256,address,bytes)"
    )]
    pub struct AddLiquidity2Call {
        pub pool: ::ethers::core::types::Address,
        pub inputs: ::std::vec::Vec<TokenInput>,
        pub data: ::ethers::core::types::Bytes,
        pub min_liquidity: ::ethers::core::types::U256,
        pub callback: ::ethers::core::types::Address,
        pub callback_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `addLiquidityWithPermit` function with signature `addLiquidityWithPermit(address,(address,uint256)[],bytes,uint256,address,bytes,(address,uint256,uint256,uint8,bytes32,bytes32)[])` and selector `0xc4b3fc40`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "addLiquidityWithPermit",
        abi = "addLiquidityWithPermit(address,(address,uint256)[],bytes,uint256,address,bytes,(address,uint256,uint256,uint8,bytes32,bytes32)[])"
    )]
    pub struct AddLiquidityWithPermitCall {
        pub pool: ::ethers::core::types::Address,
        pub inputs: ::std::vec::Vec<TokenInput>,
        pub data: ::ethers::core::types::Bytes,
        pub min_liquidity: ::ethers::core::types::U256,
        pub callback: ::ethers::core::types::Address,
        pub callback_data: ::ethers::core::types::Bytes,
        pub permits: ::std::vec::Vec<SplitPermitParams>,
    }
    ///Container type for all input parameters for the `addLiquidityWithPermit2` function with signature `addLiquidityWithPermit2(address,(address,uint256)[],bytes,uint256,address,bytes,(address,uint256,uint256,uint8,bytes32,bytes32)[])` and selector `0xced78795`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "addLiquidityWithPermit2",
        abi = "addLiquidityWithPermit2(address,(address,uint256)[],bytes,uint256,address,bytes,(address,uint256,uint256,uint8,bytes32,bytes32)[])"
    )]
    pub struct AddLiquidityWithPermit2Call {
        pub pool: ::ethers::core::types::Address,
        pub inputs: ::std::vec::Vec<TokenInput>,
        pub data: ::ethers::core::types::Bytes,
        pub min_liquidity: ::ethers::core::types::U256,
        pub callback: ::ethers::core::types::Address,
        pub callback_data: ::ethers::core::types::Bytes,
        pub permits: ::std::vec::Vec<SplitPermitParams>,
    }
    ///Container type for all input parameters for the `burnLiquidity` function with signature `burnLiquidity(address,uint256,bytes,uint256[],address,bytes)` and selector `0xad271fa3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "burnLiquidity",
        abi = "burnLiquidity(address,uint256,bytes,uint256[],address,bytes)"
    )]
    pub struct BurnLiquidityCall {
        pub pool: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub min_amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub callback: ::ethers::core::types::Address,
        pub callback_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `burnLiquiditySingle` function with signature `burnLiquiditySingle(address,uint256,bytes,uint256,address,bytes)` and selector `0x53c43f15`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "burnLiquiditySingle",
        abi = "burnLiquiditySingle(address,uint256,bytes,uint256,address,bytes)"
    )]
    pub struct BurnLiquiditySingleCall {
        pub pool: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub min_amount: ::ethers::core::types::U256,
        pub callback: ::ethers::core::types::Address,
        pub callback_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `burnLiquiditySingleWithPermit` function with signature `burnLiquiditySingleWithPermit(address,uint256,bytes,uint256,address,bytes,(uint256,uint256,bytes))` and selector `0x7d10c9d6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "burnLiquiditySingleWithPermit",
        abi = "burnLiquiditySingleWithPermit(address,uint256,bytes,uint256,address,bytes,(uint256,uint256,bytes))"
    )]
    pub struct BurnLiquiditySingleWithPermitCall {
        pub pool: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub min_amount: ::ethers::core::types::U256,
        pub callback: ::ethers::core::types::Address,
        pub callback_data: ::ethers::core::types::Bytes,
        pub permit: ArrayPermitParams,
    }
    ///Container type for all input parameters for the `burnLiquidityWithPermit` function with signature `burnLiquidityWithPermit(address,uint256,bytes,uint256[],address,bytes,(uint256,uint256,bytes))` and selector `0x353766c6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "burnLiquidityWithPermit",
        abi = "burnLiquidityWithPermit(address,uint256,bytes,uint256[],address,bytes,(uint256,uint256,bytes))"
    )]
    pub struct BurnLiquidityWithPermitCall {
        pub pool: ::ethers::core::types::Address,
        pub liquidity: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
        pub min_amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub callback: ::ethers::core::types::Address,
        pub callback_data: ::ethers::core::types::Bytes,
        pub permit: ArrayPermitParams,
    }
    ///Container type for all input parameters for the `createPool` function with signature `createPool(address,bytes)` and selector `0x9dd41df2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "createPool", abi = "createPool(address,bytes)")]
    pub struct CreatePoolCall {
        pub factory: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `enteredPools` function with signature `enteredPools(address,uint256)` and selector `0x2b4abadb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "enteredPools", abi = "enteredPools(address,uint256)")]
    pub struct EnteredPoolsCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `enteredPoolsLength` function with signature `enteredPoolsLength(address)` and selector `0xb956b3fb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "enteredPoolsLength", abi = "enteredPoolsLength(address)")]
    pub struct EnteredPoolsLengthCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isPoolEntered` function with signature `isPoolEntered(address,address)` and selector `0x4f25b858`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isPoolEntered", abi = "isPoolEntered(address,address)")]
    pub struct IsPoolEnteredCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "multicall", abi = "multicall(bytes[])")]
    pub struct MulticallCall {
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `selfPermit` function with signature `selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xf3995c67`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "selfPermit",
        abi = "selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitCall {
        pub token: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `selfPermit2` function with signature `selfPermit2(address,uint256,uint256,bytes)` and selector `0x6cc781cd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "selfPermit2",
        abi = "selfPermit2(address,uint256,uint256,bytes)"
    )]
    pub struct SelfPermit2Call {
        pub token: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `selfPermit2IfNecessary` function with signature `selfPermit2IfNecessary(address,uint256,uint256,bytes)` and selector `0x688ee44c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "selfPermit2IfNecessary",
        abi = "selfPermit2IfNecessary(address,uint256,uint256,bytes)"
    )]
    pub struct SelfPermit2IfNecessaryCall {
        pub token: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `selfPermitAllowed` function with signature `selfPermitAllowed(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0x4659a494`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "selfPermitAllowed",
        abi = "selfPermitAllowed(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitAllowedCall {
        pub token: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub expiry: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `selfPermitAllowedIfNecessary` function with signature `selfPermitAllowedIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xa4a78f0c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "selfPermitAllowedIfNecessary",
        abi = "selfPermitAllowedIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitAllowedIfNecessaryCall {
        pub token: ::ethers::core::types::Address,
        pub nonce: ::ethers::core::types::U256,
        pub expiry: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `selfPermitIfNecessary` function with signature `selfPermitIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xc2e3140a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "selfPermitIfNecessary",
        abi = "selfPermitIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitIfNecessaryCall {
        pub token: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `stake` function with signature `stake(address,address,uint256,address)` and selector `0x6291027c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "stake", abi = "stake(address,address,uint256,address)")]
    pub struct StakeCall {
        pub staking_pool: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub on_behalf: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `swap` function with signature `swap(((address,bytes,address,bytes)[],address,uint256)[],uint256,uint256)` and selector `0x2cc4081e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "swap",
        abi = "swap(((address,bytes,address,bytes)[],address,uint256)[],uint256,uint256)"
    )]
    pub struct SwapCall {
        pub paths: ::std::vec::Vec<SwapPath>,
        pub amount_out_min: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapWithPermit` function with signature `swapWithPermit(((address,bytes,address,bytes)[],address,uint256)[],uint256,uint256,(address,uint256,uint256,uint8,bytes32,bytes32))` and selector `0xe84d494b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "swapWithPermit",
        abi = "swapWithPermit(((address,bytes,address,bytes)[],address,uint256)[],uint256,uint256,(address,uint256,uint256,uint8,bytes32,bytes32))"
    )]
    pub struct SwapWithPermitCall {
        pub paths: ::std::vec::Vec<SwapPath>,
        pub amount_out_min: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub permit: SplitPermitParams,
    }
    ///Container type for all input parameters for the `vault` function with signature `vault()` and selector `0xfbfa77cf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "vault", abi = "vault()")]
    pub struct VaultCall;
    ///Container type for all input parameters for the `wETH` function with signature `wETH()` and selector `0xf2428621`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "wETH", abi = "wETH()")]
    pub struct WethCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SyncSwapRouterCalls {
        AddLiquidity(AddLiquidityCall),
        AddLiquidity2(AddLiquidity2Call),
        AddLiquidityWithPermit(AddLiquidityWithPermitCall),
        AddLiquidityWithPermit2(AddLiquidityWithPermit2Call),
        BurnLiquidity(BurnLiquidityCall),
        BurnLiquiditySingle(BurnLiquiditySingleCall),
        BurnLiquiditySingleWithPermit(BurnLiquiditySingleWithPermitCall),
        BurnLiquidityWithPermit(BurnLiquidityWithPermitCall),
        CreatePool(CreatePoolCall),
        EnteredPools(EnteredPoolsCall),
        EnteredPoolsLength(EnteredPoolsLengthCall),
        IsPoolEntered(IsPoolEnteredCall),
        Multicall(MulticallCall),
        SelfPermit(SelfPermitCall),
        SelfPermit2(SelfPermit2Call),
        SelfPermit2IfNecessary(SelfPermit2IfNecessaryCall),
        SelfPermitAllowed(SelfPermitAllowedCall),
        SelfPermitAllowedIfNecessary(SelfPermitAllowedIfNecessaryCall),
        SelfPermitIfNecessary(SelfPermitIfNecessaryCall),
        Stake(StakeCall),
        Swap(SwapCall),
        SwapWithPermit(SwapWithPermitCall),
        Vault(VaultCall),
        Weth(WethCall),
    }
    impl ::ethers::core::abi::AbiDecode for SyncSwapRouterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddLiquidity(decoded));
            }
            if let Ok(decoded) = <AddLiquidity2Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddLiquidity2(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityWithPermitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddLiquidityWithPermit(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityWithPermit2Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddLiquidityWithPermit2(decoded));
            }
            if let Ok(decoded) = <BurnLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BurnLiquidity(decoded));
            }
            if let Ok(decoded) =
                <BurnLiquiditySingleCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BurnLiquiditySingle(decoded));
            }
            if let Ok(decoded) =
                <BurnLiquiditySingleWithPermitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BurnLiquiditySingleWithPermit(decoded));
            }
            if let Ok(decoded) =
                <BurnLiquidityWithPermitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BurnLiquidityWithPermit(decoded));
            }
            if let Ok(decoded) = <CreatePoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreatePool(decoded));
            }
            if let Ok(decoded) = <EnteredPoolsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EnteredPools(decoded));
            }
            if let Ok(decoded) =
                <EnteredPoolsLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EnteredPoolsLength(decoded));
            }
            if let Ok(decoded) = <IsPoolEnteredCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsPoolEntered(decoded));
            }
            if let Ok(decoded) = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded) = <SelfPermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SelfPermit(decoded));
            }
            if let Ok(decoded) = <SelfPermit2Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SelfPermit2(decoded));
            }
            if let Ok(decoded) =
                <SelfPermit2IfNecessaryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SelfPermit2IfNecessary(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitAllowedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SelfPermitAllowed(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitAllowedIfNecessaryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SelfPermitAllowedIfNecessary(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitIfNecessaryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SelfPermitIfNecessary(decoded));
            }
            if let Ok(decoded) = <StakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Stake(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded) =
                <SwapWithPermitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapWithPermit(decoded));
            }
            if let Ok(decoded) = <VaultCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Vault(decoded));
            }
            if let Ok(decoded) = <WethCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Weth(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SyncSwapRouterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddLiquidity2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddLiquidityWithPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddLiquidityWithPermit2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BurnLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BurnLiquiditySingle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BurnLiquiditySingleWithPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BurnLiquidityWithPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreatePool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnteredPools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EnteredPoolsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPoolEntered(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Multicall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SelfPermit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SelfPermit2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SelfPermit2IfNecessary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfPermitAllowed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SelfPermitAllowedIfNecessary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SelfPermitIfNecessary(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Stake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SwapWithPermit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Vault(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Weth(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SyncSwapRouterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddLiquidity2(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddLiquidityWithPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddLiquidityWithPermit2(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnLiquiditySingle(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnLiquiditySingleWithPermit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BurnLiquidityWithPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnteredPools(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnteredPoolsLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPoolEntered(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfPermit2(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfPermit2IfNecessary(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfPermitAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfPermitAllowedIfNecessary(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SelfPermitIfNecessary(element) => ::core::fmt::Display::fmt(element, f),
                Self::Stake(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapWithPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Vault(element) => ::core::fmt::Display::fmt(element, f),
                Self::Weth(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddLiquidityCall> for SyncSwapRouterCalls {
        fn from(value: AddLiquidityCall) -> Self {
            Self::AddLiquidity(value)
        }
    }
    impl ::core::convert::From<AddLiquidity2Call> for SyncSwapRouterCalls {
        fn from(value: AddLiquidity2Call) -> Self {
            Self::AddLiquidity2(value)
        }
    }
    impl ::core::convert::From<AddLiquidityWithPermitCall> for SyncSwapRouterCalls {
        fn from(value: AddLiquidityWithPermitCall) -> Self {
            Self::AddLiquidityWithPermit(value)
        }
    }
    impl ::core::convert::From<AddLiquidityWithPermit2Call> for SyncSwapRouterCalls {
        fn from(value: AddLiquidityWithPermit2Call) -> Self {
            Self::AddLiquidityWithPermit2(value)
        }
    }
    impl ::core::convert::From<BurnLiquidityCall> for SyncSwapRouterCalls {
        fn from(value: BurnLiquidityCall) -> Self {
            Self::BurnLiquidity(value)
        }
    }
    impl ::core::convert::From<BurnLiquiditySingleCall> for SyncSwapRouterCalls {
        fn from(value: BurnLiquiditySingleCall) -> Self {
            Self::BurnLiquiditySingle(value)
        }
    }
    impl ::core::convert::From<BurnLiquiditySingleWithPermitCall> for SyncSwapRouterCalls {
        fn from(value: BurnLiquiditySingleWithPermitCall) -> Self {
            Self::BurnLiquiditySingleWithPermit(value)
        }
    }
    impl ::core::convert::From<BurnLiquidityWithPermitCall> for SyncSwapRouterCalls {
        fn from(value: BurnLiquidityWithPermitCall) -> Self {
            Self::BurnLiquidityWithPermit(value)
        }
    }
    impl ::core::convert::From<CreatePoolCall> for SyncSwapRouterCalls {
        fn from(value: CreatePoolCall) -> Self {
            Self::CreatePool(value)
        }
    }
    impl ::core::convert::From<EnteredPoolsCall> for SyncSwapRouterCalls {
        fn from(value: EnteredPoolsCall) -> Self {
            Self::EnteredPools(value)
        }
    }
    impl ::core::convert::From<EnteredPoolsLengthCall> for SyncSwapRouterCalls {
        fn from(value: EnteredPoolsLengthCall) -> Self {
            Self::EnteredPoolsLength(value)
        }
    }
    impl ::core::convert::From<IsPoolEnteredCall> for SyncSwapRouterCalls {
        fn from(value: IsPoolEnteredCall) -> Self {
            Self::IsPoolEntered(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for SyncSwapRouterCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<SelfPermitCall> for SyncSwapRouterCalls {
        fn from(value: SelfPermitCall) -> Self {
            Self::SelfPermit(value)
        }
    }
    impl ::core::convert::From<SelfPermit2Call> for SyncSwapRouterCalls {
        fn from(value: SelfPermit2Call) -> Self {
            Self::SelfPermit2(value)
        }
    }
    impl ::core::convert::From<SelfPermit2IfNecessaryCall> for SyncSwapRouterCalls {
        fn from(value: SelfPermit2IfNecessaryCall) -> Self {
            Self::SelfPermit2IfNecessary(value)
        }
    }
    impl ::core::convert::From<SelfPermitAllowedCall> for SyncSwapRouterCalls {
        fn from(value: SelfPermitAllowedCall) -> Self {
            Self::SelfPermitAllowed(value)
        }
    }
    impl ::core::convert::From<SelfPermitAllowedIfNecessaryCall> for SyncSwapRouterCalls {
        fn from(value: SelfPermitAllowedIfNecessaryCall) -> Self {
            Self::SelfPermitAllowedIfNecessary(value)
        }
    }
    impl ::core::convert::From<SelfPermitIfNecessaryCall> for SyncSwapRouterCalls {
        fn from(value: SelfPermitIfNecessaryCall) -> Self {
            Self::SelfPermitIfNecessary(value)
        }
    }
    impl ::core::convert::From<StakeCall> for SyncSwapRouterCalls {
        fn from(value: StakeCall) -> Self {
            Self::Stake(value)
        }
    }
    impl ::core::convert::From<SwapCall> for SyncSwapRouterCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<SwapWithPermitCall> for SyncSwapRouterCalls {
        fn from(value: SwapWithPermitCall) -> Self {
            Self::SwapWithPermit(value)
        }
    }
    impl ::core::convert::From<VaultCall> for SyncSwapRouterCalls {
        fn from(value: VaultCall) -> Self {
            Self::Vault(value)
        }
    }
    impl ::core::convert::From<WethCall> for SyncSwapRouterCalls {
        fn from(value: WethCall) -> Self {
            Self::Weth(value)
        }
    }
    ///Container type for all return fields from the `addLiquidity` function with signature `addLiquidity(address,(address,uint256)[],bytes,uint256,address,bytes)` and selector `0x6cbe96fa`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AddLiquidityReturn {
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `addLiquidity2` function with signature `addLiquidity2(address,(address,uint256)[],bytes,uint256,address,bytes)` and selector `0x94ec6d78`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AddLiquidity2Return {
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `addLiquidityWithPermit` function with signature `addLiquidityWithPermit(address,(address,uint256)[],bytes,uint256,address,bytes,(address,uint256,uint256,uint8,bytes32,bytes32)[])` and selector `0xc4b3fc40`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AddLiquidityWithPermitReturn {
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `addLiquidityWithPermit2` function with signature `addLiquidityWithPermit2(address,(address,uint256)[],bytes,uint256,address,bytes,(address,uint256,uint256,uint8,bytes32,bytes32)[])` and selector `0xced78795`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AddLiquidityWithPermit2Return {
        pub liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `burnLiquidity` function with signature `burnLiquidity(address,uint256,bytes,uint256[],address,bytes)` and selector `0xad271fa3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BurnLiquidityReturn {
        pub amounts: ::std::vec::Vec<TokenAmount>,
    }
    ///Container type for all return fields from the `burnLiquiditySingle` function with signature `burnLiquiditySingle(address,uint256,bytes,uint256,address,bytes)` and selector `0x53c43f15`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BurnLiquiditySingleReturn {
        pub amount_out: TokenAmount,
    }
    ///Container type for all return fields from the `burnLiquiditySingleWithPermit` function with signature `burnLiquiditySingleWithPermit(address,uint256,bytes,uint256,address,bytes,(uint256,uint256,bytes))` and selector `0x7d10c9d6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BurnLiquiditySingleWithPermitReturn {
        pub amount_out: TokenAmount,
    }
    ///Container type for all return fields from the `burnLiquidityWithPermit` function with signature `burnLiquidityWithPermit(address,uint256,bytes,uint256[],address,bytes,(uint256,uint256,bytes))` and selector `0x353766c6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BurnLiquidityWithPermitReturn {
        pub amounts: ::std::vec::Vec<TokenAmount>,
    }
    ///Container type for all return fields from the `createPool` function with signature `createPool(address,bytes)` and selector `0x9dd41df2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CreatePoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `enteredPools` function with signature `enteredPools(address,uint256)` and selector `0x2b4abadb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct EnteredPoolsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `enteredPoolsLength` function with signature `enteredPoolsLength(address)` and selector `0xb956b3fb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct EnteredPoolsLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `isPoolEntered` function with signature `isPoolEntered(address,address)` and selector `0x4f25b858`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsPoolEnteredReturn(pub bool);
    ///Container type for all return fields from the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MulticallReturn {
        pub results: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all return fields from the `swap` function with signature `swap(((address,bytes,address,bytes)[],address,uint256)[],uint256,uint256)` and selector `0x2cc4081e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SwapReturn {
        pub amount_out: TokenAmount,
    }
    ///Container type for all return fields from the `swapWithPermit` function with signature `swapWithPermit(((address,bytes,address,bytes)[],address,uint256)[],uint256,uint256,(address,uint256,uint256,uint8,bytes32,bytes32))` and selector `0xe84d494b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SwapWithPermitReturn {
        pub amount_out: TokenAmount,
    }
    ///Container type for all return fields from the `vault` function with signature `vault()` and selector `0xfbfa77cf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct VaultReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `wETH` function with signature `wETH()` and selector `0xf2428621`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct WethReturn(pub ::ethers::core::types::Address);
    ///`TokenAmount(address,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TokenAmount {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///`ArrayPermitParams(uint256,uint256,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ArrayPermitParams {
        pub approve_amount: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub signature: ::ethers::core::types::Bytes,
    }
    ///`SplitPermitParams(address,uint256,uint256,uint8,bytes32,bytes32)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SplitPermitParams {
        pub token: ::ethers::core::types::Address,
        pub approve_amount: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///`SwapPath((address,bytes,address,bytes)[],address,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SwapPath {
        pub steps: ::std::vec::Vec<SwapStep>,
        pub token_in: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///`SwapStep(address,bytes,address,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SwapStep {
        pub pool: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
        pub callback: ::ethers::core::types::Address,
        pub callback_data: ::ethers::core::types::Bytes,
    }
    ///`TokenInput(address,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TokenInput {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
}
