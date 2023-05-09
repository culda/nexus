pub use aggregation_router_v5::*;
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
pub mod aggregation_router_v5 {
    #[rustfmt::skip]
    const __ABI: &str = "[\n  {\n    \"inputs\": [\n      { \"internalType\": \"contract IWETH\", \"name\": \"weth\", \"type\": \"address\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  { \"inputs\": [], \"name\": \"AccessDenied\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"AdvanceNonceFailed\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"AlreadyFilled\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"ArbitraryStaticCallFailed\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"BadPool\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"BadSignature\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"ETHTransferFailed\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"ETHTransferFailed\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"EmptyPools\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"EthDepositRejected\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"GetAmountCallFailed\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"IncorrectDataLength\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"InsufficientBalance\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"InvalidMsgValue\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"InvalidMsgValue\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"InvalidatedOrder\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"MakingAmountExceeded\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"MakingAmountTooLow\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"OnlyOneAmountShouldBeZero\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"OrderExpired\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"PermitLengthTooLow\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"PredicateIsNotTrue\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"PrivateOrder\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"RFQBadSignature\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"RFQPrivateOrder\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"RFQSwapWithZeroAmount\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"RFQZeroTargetIsForbidden\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"ReentrancyDetected\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"RemainingAmountIsZero\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"ReservesCallFailed\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"ReturnAmountIsNotEnough\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"SafePermitBadLength\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"SafeTransferFailed\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"SafeTransferFromFailed\", \"type\": \"error\" },\n  {\n    \"inputs\": [\n      { \"internalType\": \"bool\", \"name\": \"success\", \"type\": \"bool\" },\n      { \"internalType\": \"bytes\", \"name\": \"res\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"SimulationResults\",\n    \"type\": \"error\"\n  },\n  { \"inputs\": [], \"name\": \"SwapAmountTooLarge\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"SwapWithZeroAmount\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"TakingAmountExceeded\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"TakingAmountIncreased\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"TakingAmountTooHigh\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"TransferFromMakerToTakerFailed\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"TransferFromTakerToMakerFailed\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"UnknownOrder\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"WrongAmount\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"WrongGetter\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"ZeroAddress\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"ZeroMinReturn\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"ZeroReturnAmount\", \"type\": \"error\" },\n  { \"inputs\": [], \"name\": \"ZeroTargetIsForbidden\", \"type\": \"error\" },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"maker\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"newNonce\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"NonceIncreased\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"maker\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"orderHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"remainingRaw\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"OrderCanceled\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"maker\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"orderHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"remaining\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"OrderFilled\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"orderHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"makingAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"OrderFilledRFQ\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"previousOwner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OwnershipTransferred\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"uint8\", \"name\": \"amount\", \"type\": \"uint8\" }],\n    \"name\": \"advanceNonce\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"offsets\", \"type\": \"uint256\" },\n      { \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"and\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"target\", \"type\": \"address\" },\n      { \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"arbitraryStaticCall\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          { \"internalType\": \"uint256\", \"name\": \"salt\", \"type\": \"uint256\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"makerAsset\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"takerAsset\",\n            \"type\": \"address\"\n          },\n          { \"internalType\": \"address\", \"name\": \"maker\", \"type\": \"address\" },\n          { \"internalType\": \"address\", \"name\": \"receiver\", \"type\": \"address\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"allowedSender\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"makingAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"takingAmount\",\n            \"type\": \"uint256\"\n          },\n          { \"internalType\": \"uint256\", \"name\": \"offsets\", \"type\": \"uint256\" },\n          { \"internalType\": \"bytes\", \"name\": \"interactions\", \"type\": \"bytes\" }\n        ],\n        \"internalType\": \"struct OrderLib.Order\",\n        \"name\": \"order\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"cancelOrder\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"orderRemaining\",\n        \"type\": \"uint256\"\n      },\n      { \"internalType\": \"bytes32\", \"name\": \"orderHash\", \"type\": \"bytes32\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"orderInfo\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"cancelOrderRFQ\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"orderInfo\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"additionalMask\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"cancelOrderRFQ\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          { \"internalType\": \"uint256\", \"name\": \"salt\", \"type\": \"uint256\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"makerAsset\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"takerAsset\",\n            \"type\": \"address\"\n          },\n          { \"internalType\": \"address\", \"name\": \"maker\", \"type\": \"address\" },\n          { \"internalType\": \"address\", \"name\": \"receiver\", \"type\": \"address\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"allowedSender\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"makingAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"takingAmount\",\n            \"type\": \"uint256\"\n          },\n          { \"internalType\": \"uint256\", \"name\": \"offsets\", \"type\": \"uint256\" },\n          { \"internalType\": \"bytes\", \"name\": \"interactions\", \"type\": \"bytes\" }\n        ],\n        \"internalType\": \"struct OrderLib.Order\",\n        \"name\": \"order\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"checkPredicate\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract IClipperExchangeInterface\",\n        \"name\": \"clipperExchange\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"srcToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"dstToken\",\n        \"type\": \"address\"\n      },\n      { \"internalType\": \"uint256\", \"name\": \"inputAmount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"outputAmount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"goodUntil\", \"type\": \"uint256\" },\n      { \"internalType\": \"bytes32\", \"name\": \"r\", \"type\": \"bytes32\" },\n      { \"internalType\": \"bytes32\", \"name\": \"vs\", \"type\": \"bytes32\" }\n    ],\n    \"name\": \"clipperSwap\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"returnAmount\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract IClipperExchangeInterface\",\n        \"name\": \"clipperExchange\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"srcToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"dstToken\",\n        \"type\": \"address\"\n      },\n      { \"internalType\": \"uint256\", \"name\": \"inputAmount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"outputAmount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"goodUntil\", \"type\": \"uint256\" },\n      { \"internalType\": \"bytes32\", \"name\": \"r\", \"type\": \"bytes32\" },\n      { \"internalType\": \"bytes32\", \"name\": \"vs\", \"type\": \"bytes32\" }\n    ],\n    \"name\": \"clipperSwapTo\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"returnAmount\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract IClipperExchangeInterface\",\n        \"name\": \"clipperExchange\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"srcToken\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"dstToken\",\n        \"type\": \"address\"\n      },\n      { \"internalType\": \"uint256\", \"name\": \"inputAmount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"outputAmount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"goodUntil\", \"type\": \"uint256\" },\n      { \"internalType\": \"bytes32\", \"name\": \"r\", \"type\": \"bytes32\" },\n      { \"internalType\": \"bytes32\", \"name\": \"vs\", \"type\": \"bytes32\" },\n      { \"internalType\": \"bytes\", \"name\": \"permit\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"clipperSwapToWithPermit\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"returnAmount\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"destroy\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"value\", \"type\": \"uint256\" },\n      { \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"eq\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          { \"internalType\": \"uint256\", \"name\": \"salt\", \"type\": \"uint256\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"makerAsset\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"takerAsset\",\n            \"type\": \"address\"\n          },\n          { \"internalType\": \"address\", \"name\": \"maker\", \"type\": \"address\" },\n          { \"internalType\": \"address\", \"name\": \"receiver\", \"type\": \"address\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"allowedSender\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"makingAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"takingAmount\",\n            \"type\": \"uint256\"\n          },\n          { \"internalType\": \"uint256\", \"name\": \"offsets\", \"type\": \"uint256\" },\n          { \"internalType\": \"bytes\", \"name\": \"interactions\", \"type\": \"bytes\" }\n        ],\n        \"internalType\": \"struct OrderLib.Order\",\n        \"name\": \"order\",\n        \"type\": \"tuple\"\n      },\n      { \"internalType\": \"bytes\", \"name\": \"signature\", \"type\": \"bytes\" },\n      { \"internalType\": \"bytes\", \"name\": \"interaction\", \"type\": \"bytes\" },\n      { \"internalType\": \"uint256\", \"name\": \"makingAmount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"takingAmount\", \"type\": \"uint256\" },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"skipPermitAndThresholdAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"fillOrder\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" },\n      { \"internalType\": \"bytes32\", \"name\": \"\", \"type\": \"bytes32\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          { \"internalType\": \"uint256\", \"name\": \"info\", \"type\": \"uint256\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"makerAsset\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"takerAsset\",\n            \"type\": \"address\"\n          },\n          { \"internalType\": \"address\", \"name\": \"maker\", \"type\": \"address\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"allowedSender\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"makingAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"takingAmount\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct OrderRFQLib.OrderRFQ\",\n        \"name\": \"order\",\n        \"type\": \"tuple\"\n      },\n      { \"internalType\": \"bytes\", \"name\": \"signature\", \"type\": \"bytes\" },\n      { \"internalType\": \"uint256\", \"name\": \"flagsAndAmount\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"fillOrderRFQ\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" },\n      { \"internalType\": \"bytes32\", \"name\": \"\", \"type\": \"bytes32\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          { \"internalType\": \"uint256\", \"name\": \"info\", \"type\": \"uint256\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"makerAsset\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"takerAsset\",\n            \"type\": \"address\"\n          },\n          { \"internalType\": \"address\", \"name\": \"maker\", \"type\": \"address\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"allowedSender\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"makingAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"takingAmount\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct OrderRFQLib.OrderRFQ\",\n        \"name\": \"order\",\n        \"type\": \"tuple\"\n      },\n      { \"internalType\": \"bytes32\", \"name\": \"r\", \"type\": \"bytes32\" },\n      { \"internalType\": \"bytes32\", \"name\": \"vs\", \"type\": \"bytes32\" },\n      { \"internalType\": \"uint256\", \"name\": \"flagsAndAmount\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"fillOrderRFQCompact\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"filledMakingAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"filledTakingAmount\",\n        \"type\": \"uint256\"\n      },\n      { \"internalType\": \"bytes32\", \"name\": \"orderHash\", \"type\": \"bytes32\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          { \"internalType\": \"uint256\", \"name\": \"info\", \"type\": \"uint256\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"makerAsset\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"takerAsset\",\n            \"type\": \"address\"\n          },\n          { \"internalType\": \"address\", \"name\": \"maker\", \"type\": \"address\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"allowedSender\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"makingAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"takingAmount\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct OrderRFQLib.OrderRFQ\",\n        \"name\": \"order\",\n        \"type\": \"tuple\"\n      },\n      { \"internalType\": \"bytes\", \"name\": \"signature\", \"type\": \"bytes\" },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"flagsAndAmount\",\n        \"type\": \"uint256\"\n      },\n      { \"internalType\": \"address\", \"name\": \"target\", \"type\": \"address\" }\n    ],\n    \"name\": \"fillOrderRFQTo\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"filledMakingAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"filledTakingAmount\",\n        \"type\": \"uint256\"\n      },\n      { \"internalType\": \"bytes32\", \"name\": \"orderHash\", \"type\": \"bytes32\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          { \"internalType\": \"uint256\", \"name\": \"info\", \"type\": \"uint256\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"makerAsset\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"takerAsset\",\n            \"type\": \"address\"\n          },\n          { \"internalType\": \"address\", \"name\": \"maker\", \"type\": \"address\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"allowedSender\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"makingAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"takingAmount\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct OrderRFQLib.OrderRFQ\",\n        \"name\": \"order\",\n        \"type\": \"tuple\"\n      },\n      { \"internalType\": \"bytes\", \"name\": \"signature\", \"type\": \"bytes\" },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"flagsAndAmount\",\n        \"type\": \"uint256\"\n      },\n      { \"internalType\": \"address\", \"name\": \"target\", \"type\": \"address\" },\n      { \"internalType\": \"bytes\", \"name\": \"permit\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"fillOrderRFQToWithPermit\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" },\n      { \"internalType\": \"bytes32\", \"name\": \"\", \"type\": \"bytes32\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          { \"internalType\": \"uint256\", \"name\": \"salt\", \"type\": \"uint256\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"makerAsset\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"takerAsset\",\n            \"type\": \"address\"\n          },\n          { \"internalType\": \"address\", \"name\": \"maker\", \"type\": \"address\" },\n          { \"internalType\": \"address\", \"name\": \"receiver\", \"type\": \"address\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"allowedSender\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"makingAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"takingAmount\",\n            \"type\": \"uint256\"\n          },\n          { \"internalType\": \"uint256\", \"name\": \"offsets\", \"type\": \"uint256\" },\n          { \"internalType\": \"bytes\", \"name\": \"interactions\", \"type\": \"bytes\" }\n        ],\n        \"internalType\": \"struct OrderLib.Order\",\n        \"name\": \"order_\",\n        \"type\": \"tuple\"\n      },\n      { \"internalType\": \"bytes\", \"name\": \"signature\", \"type\": \"bytes\" },\n      { \"internalType\": \"bytes\", \"name\": \"interaction\", \"type\": \"bytes\" },\n      { \"internalType\": \"uint256\", \"name\": \"makingAmount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"takingAmount\", \"type\": \"uint256\" },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"skipPermitAndThresholdAmount\",\n        \"type\": \"uint256\"\n      },\n      { \"internalType\": \"address\", \"name\": \"target\", \"type\": \"address\" }\n    ],\n    \"name\": \"fillOrderTo\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"actualMakingAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"actualTakingAmount\",\n        \"type\": \"uint256\"\n      },\n      { \"internalType\": \"bytes32\", \"name\": \"orderHash\", \"type\": \"bytes32\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          { \"internalType\": \"uint256\", \"name\": \"salt\", \"type\": \"uint256\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"makerAsset\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"takerAsset\",\n            \"type\": \"address\"\n          },\n          { \"internalType\": \"address\", \"name\": \"maker\", \"type\": \"address\" },\n          { \"internalType\": \"address\", \"name\": \"receiver\", \"type\": \"address\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"allowedSender\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"makingAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"takingAmount\",\n            \"type\": \"uint256\"\n          },\n          { \"internalType\": \"uint256\", \"name\": \"offsets\", \"type\": \"uint256\" },\n          { \"internalType\": \"bytes\", \"name\": \"interactions\", \"type\": \"bytes\" }\n        ],\n        \"internalType\": \"struct OrderLib.Order\",\n        \"name\": \"order\",\n        \"type\": \"tuple\"\n      },\n      { \"internalType\": \"bytes\", \"name\": \"signature\", \"type\": \"bytes\" },\n      { \"internalType\": \"bytes\", \"name\": \"interaction\", \"type\": \"bytes\" },\n      { \"internalType\": \"uint256\", \"name\": \"makingAmount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"takingAmount\", \"type\": \"uint256\" },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"skipPermitAndThresholdAmount\",\n        \"type\": \"uint256\"\n      },\n      { \"internalType\": \"address\", \"name\": \"target\", \"type\": \"address\" },\n      { \"internalType\": \"bytes\", \"name\": \"permit\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"fillOrderToWithPermit\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" },\n      { \"internalType\": \"bytes32\", \"name\": \"\", \"type\": \"bytes32\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"value\", \"type\": \"uint256\" },\n      { \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"gt\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          { \"internalType\": \"uint256\", \"name\": \"salt\", \"type\": \"uint256\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"makerAsset\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"takerAsset\",\n            \"type\": \"address\"\n          },\n          { \"internalType\": \"address\", \"name\": \"maker\", \"type\": \"address\" },\n          { \"internalType\": \"address\", \"name\": \"receiver\", \"type\": \"address\" },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"allowedSender\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"makingAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"takingAmount\",\n            \"type\": \"uint256\"\n          },\n          { \"internalType\": \"uint256\", \"name\": \"offsets\", \"type\": \"uint256\" },\n          { \"internalType\": \"bytes\", \"name\": \"interactions\", \"type\": \"bytes\" }\n        ],\n        \"internalType\": \"struct OrderLib.Order\",\n        \"name\": \"order\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"hashOrder\",\n    \"outputs\": [{ \"internalType\": \"bytes32\", \"name\": \"\", \"type\": \"bytes32\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"increaseNonce\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"maker\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"slot\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"invalidatorForOrderRFQ\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"value\", \"type\": \"uint256\" },\n      { \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"lt\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"name\": \"nonce\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"makerAddress\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"makerNonce\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"nonceEquals\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"offsets\", \"type\": \"uint256\" },\n      { \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"or\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"bytes32\", \"name\": \"orderHash\", \"type\": \"bytes32\" }\n    ],\n    \"name\": \"remaining\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"bytes32\", \"name\": \"orderHash\", \"type\": \"bytes32\" }\n    ],\n    \"name\": \"remainingRaw\",\n    \"outputs\": [{ \"internalType\": \"uint256\", \"name\": \"\", \"type\": \"uint256\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32[]\",\n        \"name\": \"orderHashes\",\n        \"type\": \"bytes32[]\"\n      }\n    ],\n    \"name\": \"remainingsRaw\",\n    \"outputs\": [\n      { \"internalType\": \"uint256[]\", \"name\": \"\", \"type\": \"uint256[]\" }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"renounceOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"contract IERC20\", \"name\": \"token\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"rescueFunds\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"target\", \"type\": \"address\" },\n      { \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"simulate\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract IAggregationExecutor\",\n        \"name\": \"executor\",\n        \"type\": \"address\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"contract IERC20\",\n            \"name\": \"srcToken\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"contract IERC20\",\n            \"name\": \"dstToken\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address payable\",\n            \"name\": \"srcReceiver\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address payable\",\n            \"name\": \"dstReceiver\",\n            \"type\": \"address\"\n          },\n          { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"minReturnAmount\",\n            \"type\": \"uint256\"\n          },\n          { \"internalType\": \"uint256\", \"name\": \"flags\", \"type\": \"uint256\" }\n        ],\n        \"internalType\": \"struct GenericRouter.SwapDescription\",\n        \"name\": \"desc\",\n        \"type\": \"tuple\"\n      },\n      { \"internalType\": \"bytes\", \"name\": \"permit\", \"type\": \"bytes\" },\n      { \"internalType\": \"bytes\", \"name\": \"data\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"swap\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"returnAmount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"spentAmount\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"time\", \"type\": \"uint256\" }\n    ],\n    \"name\": \"timestampBelow\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"timeNonceAccount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"timestampBelowAndNonceEquals\",\n    \"outputs\": [{ \"internalType\": \"bool\", \"name\": \"\", \"type\": \"bool\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"newOwner\", \"type\": \"address\" }\n    ],\n    \"name\": \"transferOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"minReturn\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256[]\", \"name\": \"pools\", \"type\": \"uint256[]\" }\n    ],\n    \"name\": \"uniswapV3Swap\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"returnAmount\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"int256\", \"name\": \"amount0Delta\", \"type\": \"int256\" },\n      { \"internalType\": \"int256\", \"name\": \"amount1Delta\", \"type\": \"int256\" },\n      { \"internalType\": \"bytes\", \"name\": \"\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"uniswapV3SwapCallback\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"minReturn\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256[]\", \"name\": \"pools\", \"type\": \"uint256[]\" }\n    ],\n    \"name\": \"uniswapV3SwapTo\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"returnAmount\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"srcToken\",\n        \"type\": \"address\"\n      },\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"minReturn\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256[]\", \"name\": \"pools\", \"type\": \"uint256[]\" },\n      { \"internalType\": \"bytes\", \"name\": \"permit\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"uniswapV3SwapToWithPermit\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"returnAmount\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"srcToken\",\n        \"type\": \"address\"\n      },\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"minReturn\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256[]\", \"name\": \"pools\", \"type\": \"uint256[]\" }\n    ],\n    \"name\": \"unoswap\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"returnAmount\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"srcToken\",\n        \"type\": \"address\"\n      },\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"minReturn\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256[]\", \"name\": \"pools\", \"type\": \"uint256[]\" }\n    ],\n    \"name\": \"unoswapTo\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"returnAmount\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"contract IERC20\",\n        \"name\": \"srcToken\",\n        \"type\": \"address\"\n      },\n      { \"internalType\": \"uint256\", \"name\": \"amount\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"minReturn\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256[]\", \"name\": \"pools\", \"type\": \"uint256[]\" },\n      { \"internalType\": \"bytes\", \"name\": \"permit\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"unoswapToWithPermit\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"returnAmount\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  { \"stateMutability\": \"payable\", \"type\": \"receive\" }\n]";
    ///The parsed JSON ABI of the contract.
    pub static AGGREGATIONROUTERV5_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct AggregationRouterV5<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AggregationRouterV5<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AggregationRouterV5<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AggregationRouterV5<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AggregationRouterV5<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(AggregationRouterV5))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AggregationRouterV5<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                AGGREGATIONROUTERV5_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `advanceNonce` (0x72c244a8) function
        pub fn advance_nonce(
            &self,
            amount: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 194, 68, 168], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `and` (0xbfa75143) function
        pub fn and(
            &self,
            offsets: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([191, 167, 81, 67], (offsets, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `arbitraryStaticCall` (0xbf15fcd8) function
        pub fn arbitrary_static_call(
            &self,
            target: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([191, 21, 252, 216], (target, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancelOrder` (0x2d9a56f6) function
        pub fn cancel_order(
            &self,
            order: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, (::ethers::core::types::U256, [u8; 32])>
        {
            self.0
                .method_hash([45, 154, 86, 246], (order,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancelOrderRFQ` (0x825caba1) function
        pub fn cancel_order_rfq(
            &self,
            order_info: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 92, 171, 161], order_info)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cancelOrderRFQ` (0xbddccd35) function
        pub fn cancel_order_rfq_with_additional_mask(
            &self,
            order_info: ::ethers::core::types::U256,
            additional_mask: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 220, 205, 53], (order_info, additional_mask))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkPredicate` (0x6c838250) function
        pub fn check_predicate(
            &self,
            order: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([108, 131, 130, 80], (order,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clipperSwap` (0x84bd6d29) function
        pub fn clipper_swap(
            &self,
            clipper_exchange: ::ethers::core::types::Address,
            src_token: ::ethers::core::types::Address,
            dst_token: ::ethers::core::types::Address,
            input_amount: ::ethers::core::types::U256,
            output_amount: ::ethers::core::types::U256,
            good_until: ::ethers::core::types::U256,
            r: [u8; 32],
            vs: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [132, 189, 109, 41],
                    (
                        clipper_exchange,
                        src_token,
                        dst_token,
                        input_amount,
                        output_amount,
                        good_until,
                        r,
                        vs,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clipperSwapTo` (0x093d4fa5) function
        pub fn clipper_swap_to(
            &self,
            clipper_exchange: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            src_token: ::ethers::core::types::Address,
            dst_token: ::ethers::core::types::Address,
            input_amount: ::ethers::core::types::U256,
            output_amount: ::ethers::core::types::U256,
            good_until: ::ethers::core::types::U256,
            r: [u8; 32],
            vs: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [9, 61, 79, 165],
                    (
                        clipper_exchange,
                        recipient,
                        src_token,
                        dst_token,
                        input_amount,
                        output_amount,
                        good_until,
                        r,
                        vs,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clipperSwapToWithPermit` (0xc805a666) function
        pub fn clipper_swap_to_with_permit(
            &self,
            clipper_exchange: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            src_token: ::ethers::core::types::Address,
            dst_token: ::ethers::core::types::Address,
            input_amount: ::ethers::core::types::U256,
            output_amount: ::ethers::core::types::U256,
            good_until: ::ethers::core::types::U256,
            r: [u8; 32],
            vs: [u8; 32],
            permit: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [200, 5, 166, 102],
                    (
                        clipper_exchange,
                        recipient,
                        src_token,
                        dst_token,
                        input_amount,
                        output_amount,
                        good_until,
                        r,
                        vs,
                        permit,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `destroy` (0x83197ef0) function
        pub fn destroy(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([131, 25, 126, 240], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eq` (0x6fe7b0ba) function
        pub fn eq(
            &self,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([111, 231, 176, 186], (value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fillOrder` (0x62e238bb) function
        pub fn fill_order(
            &self,
            order: Order,
            signature: ::ethers::core::types::Bytes,
            interaction: ::ethers::core::types::Bytes,
            making_amount: ::ethers::core::types::U256,
            taking_amount: ::ethers::core::types::U256,
            skip_permit_and_threshold_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                [u8; 32],
            ),
        > {
            self.0
                .method_hash(
                    [98, 226, 56, 187],
                    (
                        order,
                        signature,
                        interaction,
                        making_amount,
                        taking_amount,
                        skip_permit_and_threshold_amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fillOrderRFQ` (0x3eca9c0a) function
        pub fn fill_order_rfq(
            &self,
            order: OrderRFQ,
            signature: ::ethers::core::types::Bytes,
            flags_and_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                [u8; 32],
            ),
        > {
            self.0
                .method_hash([62, 202, 156, 10], (order, signature, flags_and_amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fillOrderRFQCompact` (0x9570eeee) function
        pub fn fill_order_rfq_compact(
            &self,
            order: OrderRFQ,
            r: [u8; 32],
            vs: [u8; 32],
            flags_and_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                [u8; 32],
            ),
        > {
            self.0
                .method_hash([149, 112, 238, 238], (order, r, vs, flags_and_amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fillOrderRFQTo` (0x5a099843) function
        pub fn fill_order_rfq_to(
            &self,
            order: OrderRFQ,
            signature: ::ethers::core::types::Bytes,
            flags_and_amount: ::ethers::core::types::U256,
            target: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                [u8; 32],
            ),
        > {
            self.0
                .method_hash(
                    [90, 9, 152, 67],
                    (order, signature, flags_and_amount, target),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fillOrderRFQToWithPermit` (0x70ccbd31) function
        pub fn fill_order_rfq_to_with_permit(
            &self,
            order: OrderRFQ,
            signature: ::ethers::core::types::Bytes,
            flags_and_amount: ::ethers::core::types::U256,
            target: ::ethers::core::types::Address,
            permit: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                [u8; 32],
            ),
        > {
            self.0
                .method_hash(
                    [112, 204, 189, 49],
                    (order, signature, flags_and_amount, target, permit),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fillOrderTo` (0xe5d7bde6) function
        pub fn fill_order_to(
            &self,
            order: Order,
            signature: ::ethers::core::types::Bytes,
            interaction: ::ethers::core::types::Bytes,
            making_amount: ::ethers::core::types::U256,
            taking_amount: ::ethers::core::types::U256,
            skip_permit_and_threshold_amount: ::ethers::core::types::U256,
            target: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                [u8; 32],
            ),
        > {
            self.0
                .method_hash(
                    [229, 215, 189, 230],
                    (
                        order,
                        signature,
                        interaction,
                        making_amount,
                        taking_amount,
                        skip_permit_and_threshold_amount,
                        target,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fillOrderToWithPermit` (0xd365c695) function
        pub fn fill_order_to_with_permit(
            &self,
            order: Order,
            signature: ::ethers::core::types::Bytes,
            interaction: ::ethers::core::types::Bytes,
            making_amount: ::ethers::core::types::U256,
            taking_amount: ::ethers::core::types::U256,
            skip_permit_and_threshold_amount: ::ethers::core::types::U256,
            target: ::ethers::core::types::Address,
            permit: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                [u8; 32],
            ),
        > {
            self.0
                .method_hash(
                    [211, 101, 198, 149],
                    (
                        order,
                        signature,
                        interaction,
                        making_amount,
                        taking_amount,
                        skip_permit_and_threshold_amount,
                        target,
                        permit,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gt` (0x4f38e2b8) function
        pub fn gt(
            &self,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([79, 56, 226, 184], (value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hashOrder` (0x37e7316f) function
        pub fn hash_order(
            &self,
            order: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([55, 231, 49, 111], (order,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseNonce` (0xc53a0292) function
        pub fn increase_nonce(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([197, 58, 2, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `invalidatorForOrderRFQ` (0x56f16124) function
        pub fn invalidator_for_order_rfq(
            &self,
            maker: ::ethers::core::types::Address,
            slot: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([86, 241, 97, 36], (maker, slot))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lt` (0xca4ece22) function
        pub fn lt(
            &self,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([202, 78, 206, 34], (value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonce` (0x70ae92d2) function
        pub fn nonce(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 174, 146, 210], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonceEquals` (0xcf6fc6e3) function
        pub fn nonce_equals(
            &self,
            maker_address: ::ethers::core::types::Address,
            maker_nonce: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([207, 111, 198, 227], (maker_address, maker_nonce))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `or` (0x74261145) function
        pub fn or(
            &self,
            offsets: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([116, 38, 17, 69], (offsets, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `remaining` (0xbc1ed74c) function
        pub fn remaining(
            &self,
            order_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([188, 30, 215, 76], order_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `remainingRaw` (0x7e54f092) function
        pub fn remaining_raw(
            &self,
            order_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 84, 240, 146], order_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `remainingsRaw` (0x942461bb) function
        pub fn remainings_raw(
            &self,
            order_hashes: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([148, 36, 97, 187], order_hashes)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rescueFunds` (0x78e3214f) function
        pub fn rescue_funds(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 227, 33, 79], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulate` (0xbd61951d) function
        pub fn simulate(
            &self,
            target: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 97, 149, 29], (target, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0x12aa3caf) function
        pub fn swap(
            &self,
            executor: ::ethers::core::types::Address,
            desc: SwapDescription,
            permit: ::ethers::core::types::Bytes,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([18, 170, 60, 175], (executor, desc, permit, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `timestampBelow` (0x63592c2b) function
        pub fn timestamp_below(
            &self,
            time: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([99, 89, 44, 43], time)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `timestampBelowAndNonceEquals` (0x2cc2878d) function
        pub fn timestamp_below_and_nonce_equals(
            &self,
            time_nonce_account: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([44, 194, 135, 141], time_nonce_account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3Swap` (0xe449022e) function
        pub fn uniswap_v3_swap(
            &self,
            amount: ::ethers::core::types::U256,
            min_return: ::ethers::core::types::U256,
            pools: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([228, 73, 2, 46], (amount, min_return, pools))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: ::ethers::core::types::I256,
            amount_1_delta: ::ethers::core::types::I256,
            p2: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3SwapTo` (0xbc80f1a8) function
        pub fn uniswap_v3_swap_to(
            &self,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            min_return: ::ethers::core::types::U256,
            pools: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([188, 128, 241, 168], (recipient, amount, min_return, pools))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3SwapToWithPermit` (0x2521b930) function
        pub fn uniswap_v3_swap_to_with_permit(
            &self,
            recipient: ::ethers::core::types::Address,
            src_token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            min_return: ::ethers::core::types::U256,
            pools: ::std::vec::Vec<::ethers::core::types::U256>,
            permit: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [37, 33, 185, 48],
                    (recipient, src_token, amount, min_return, pools, permit),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unoswap` (0x0502b1c5) function
        pub fn unoswap(
            &self,
            src_token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            min_return: ::ethers::core::types::U256,
            pools: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([5, 2, 177, 197], (src_token, amount, min_return, pools))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unoswapTo` (0xf78dc253) function
        pub fn unoswap_to(
            &self,
            recipient: ::ethers::core::types::Address,
            src_token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            min_return: ::ethers::core::types::U256,
            pools: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [247, 141, 194, 83],
                    (recipient, src_token, amount, min_return, pools),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unoswapToWithPermit` (0x3c15fd91) function
        pub fn unoswap_to_with_permit(
            &self,
            recipient: ::ethers::core::types::Address,
            src_token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            min_return: ::ethers::core::types::U256,
            pools: ::std::vec::Vec<::ethers::core::types::U256>,
            permit: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [60, 21, 253, 145],
                    (recipient, src_token, amount, min_return, pools, permit),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `NonceIncreased` event
        pub fn nonce_increased_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NonceIncreasedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OrderCanceled` event
        pub fn order_canceled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OrderCanceledFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OrderFilled` event
        pub fn order_filled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OrderFilledFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OrderFilledRFQ` event
        pub fn order_filled_rfq_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OrderFilledRFQFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AggregationRouterV5Events>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for AggregationRouterV5<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AccessDenied` with signature `AccessDenied()` and selector `0x4ca88867`
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
    #[etherror(name = "AccessDenied", abi = "AccessDenied()")]
    pub struct AccessDenied;
    ///Custom Error type `AdvanceNonceFailed` with signature `AdvanceNonceFailed()` and selector `0xbd71636d`
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
    #[etherror(name = "AdvanceNonceFailed", abi = "AdvanceNonceFailed()")]
    pub struct AdvanceNonceFailed;
    ///Custom Error type `AlreadyFilled` with signature `AlreadyFilled()` and selector `0x41a26a63`
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
    #[etherror(name = "AlreadyFilled", abi = "AlreadyFilled()")]
    pub struct AlreadyFilled;
    ///Custom Error type `ArbitraryStaticCallFailed` with signature `ArbitraryStaticCallFailed()` and selector `0x1f1b8f61`
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
    #[etherror(
        name = "ArbitraryStaticCallFailed",
        abi = "ArbitraryStaticCallFailed()"
    )]
    pub struct ArbitraryStaticCallFailed;
    ///Custom Error type `BadPool` with signature `BadPool()` and selector `0xb2c02722`
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
    #[etherror(name = "BadPool", abi = "BadPool()")]
    pub struct BadPool;
    ///Custom Error type `BadSignature` with signature `BadSignature()` and selector `0x5cd5d233`
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
    #[etherror(name = "BadSignature", abi = "BadSignature()")]
    pub struct BadSignature;
    ///Custom Error type `ETHTransferFailed` with signature `ETHTransferFailed()` and selector `0xb12d13eb`
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
    #[etherror(name = "ETHTransferFailed", abi = "ETHTransferFailed()")]
    pub struct ETHTransferFailed;
    ///Custom Error type `EmptyPools` with signature `EmptyPools()` and selector `0x67e7c0f6`
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
    #[etherror(name = "EmptyPools", abi = "EmptyPools()")]
    pub struct EmptyPools;
    ///Custom Error type `EthDepositRejected` with signature `EthDepositRejected()` and selector `0x1b10b0f9`
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
    #[etherror(name = "EthDepositRejected", abi = "EthDepositRejected()")]
    pub struct EthDepositRejected;
    ///Custom Error type `GetAmountCallFailed` with signature `GetAmountCallFailed()` and selector `0x110b8e73`
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
    #[etherror(name = "GetAmountCallFailed", abi = "GetAmountCallFailed()")]
    pub struct GetAmountCallFailed;
    ///Custom Error type `IncorrectDataLength` with signature `IncorrectDataLength()` and selector `0xef356d7a`
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
    #[etherror(name = "IncorrectDataLength", abi = "IncorrectDataLength()")]
    pub struct IncorrectDataLength;
    ///Custom Error type `InsufficientBalance` with signature `InsufficientBalance()` and selector `0xf4d678b8`
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
    #[etherror(name = "InsufficientBalance", abi = "InsufficientBalance()")]
    pub struct InsufficientBalance;
    ///Custom Error type `InvalidMsgValue` with signature `InvalidMsgValue()` and selector `0x1841b4e1`
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
    #[etherror(name = "InvalidMsgValue", abi = "InvalidMsgValue()")]
    pub struct InvalidMsgValue;
    ///Custom Error type `InvalidatedOrder` with signature `InvalidatedOrder()` and selector `0xf71fbda2`
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
    #[etherror(name = "InvalidatedOrder", abi = "InvalidatedOrder()")]
    pub struct InvalidatedOrder;
    ///Custom Error type `MakingAmountExceeded` with signature `MakingAmountExceeded()` and selector `0xaa34b696`
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
    #[etherror(name = "MakingAmountExceeded", abi = "MakingAmountExceeded()")]
    pub struct MakingAmountExceeded;
    ///Custom Error type `MakingAmountTooLow` with signature `MakingAmountTooLow()` and selector `0x481ea392`
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
    #[etherror(name = "MakingAmountTooLow", abi = "MakingAmountTooLow()")]
    pub struct MakingAmountTooLow;
    ///Custom Error type `OnlyOneAmountShouldBeZero` with signature `OnlyOneAmountShouldBeZero()` and selector `0x00e2a522`
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
    #[etherror(
        name = "OnlyOneAmountShouldBeZero",
        abi = "OnlyOneAmountShouldBeZero()"
    )]
    pub struct OnlyOneAmountShouldBeZero;
    ///Custom Error type `OrderExpired` with signature `OrderExpired()` and selector `0xc56873ba`
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
    #[etherror(name = "OrderExpired", abi = "OrderExpired()")]
    pub struct OrderExpired;
    ///Custom Error type `PermitLengthTooLow` with signature `PermitLengthTooLow()` and selector `0xd9e1c6dc`
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
    #[etherror(name = "PermitLengthTooLow", abi = "PermitLengthTooLow()")]
    pub struct PermitLengthTooLow;
    ///Custom Error type `PredicateIsNotTrue` with signature `PredicateIsNotTrue()` and selector `0xb6629c02`
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
    #[etherror(name = "PredicateIsNotTrue", abi = "PredicateIsNotTrue()")]
    pub struct PredicateIsNotTrue;
    ///Custom Error type `PrivateOrder` with signature `PrivateOrder()` and selector `0xd4dfdafe`
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
    #[etherror(name = "PrivateOrder", abi = "PrivateOrder()")]
    pub struct PrivateOrder;
    ///Custom Error type `RFQBadSignature` with signature `RFQBadSignature()` and selector `0x17c2b1f1`
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
    #[etherror(name = "RFQBadSignature", abi = "RFQBadSignature()")]
    pub struct RFQBadSignature;
    ///Custom Error type `RFQPrivateOrder` with signature `RFQPrivateOrder()` and selector `0xe8c66321`
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
    #[etherror(name = "RFQPrivateOrder", abi = "RFQPrivateOrder()")]
    pub struct RFQPrivateOrder;
    ///Custom Error type `RFQSwapWithZeroAmount` with signature `RFQSwapWithZeroAmount()` and selector `0x07b6e79f`
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
    #[etherror(name = "RFQSwapWithZeroAmount", abi = "RFQSwapWithZeroAmount()")]
    pub struct RFQSwapWithZeroAmount;
    ///Custom Error type `RFQZeroTargetIsForbidden` with signature `RFQZeroTargetIsForbidden()` and selector `0x692e45e0`
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
    #[etherror(name = "RFQZeroTargetIsForbidden", abi = "RFQZeroTargetIsForbidden()")]
    pub struct RFQZeroTargetIsForbidden;
    ///Custom Error type `ReentrancyDetected` with signature `ReentrancyDetected()` and selector `0xc5f2be51`
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
    #[etherror(name = "ReentrancyDetected", abi = "ReentrancyDetected()")]
    pub struct ReentrancyDetected;
    ///Custom Error type `RemainingAmountIsZero` with signature `RemainingAmountIsZero()` and selector `0xecef3664`
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
    #[etherror(name = "RemainingAmountIsZero", abi = "RemainingAmountIsZero()")]
    pub struct RemainingAmountIsZero;
    ///Custom Error type `ReservesCallFailed` with signature `ReservesCallFailed()` and selector `0x85cd58dc`
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
    #[etherror(name = "ReservesCallFailed", abi = "ReservesCallFailed()")]
    pub struct ReservesCallFailed;
    ///Custom Error type `ReturnAmountIsNotEnough` with signature `ReturnAmountIsNotEnough()` and selector `0xf32bec2f`
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
    #[etherror(name = "ReturnAmountIsNotEnough", abi = "ReturnAmountIsNotEnough()")]
    pub struct ReturnAmountIsNotEnough;
    ///Custom Error type `SafePermitBadLength` with signature `SafePermitBadLength()` and selector `0x68275857`
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
    #[etherror(name = "SafePermitBadLength", abi = "SafePermitBadLength()")]
    pub struct SafePermitBadLength;
    ///Custom Error type `SafeTransferFailed` with signature `SafeTransferFailed()` and selector `0xfb7f5079`
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
    #[etherror(name = "SafeTransferFailed", abi = "SafeTransferFailed()")]
    pub struct SafeTransferFailed;
    ///Custom Error type `SafeTransferFromFailed` with signature `SafeTransferFromFailed()` and selector `0xf4059071`
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
    #[etherror(name = "SafeTransferFromFailed", abi = "SafeTransferFromFailed()")]
    pub struct SafeTransferFromFailed;
    ///Custom Error type `SimulationResults` with signature `SimulationResults(bool,bytes)` and selector `0x1934afc8`
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
    #[etherror(name = "SimulationResults", abi = "SimulationResults(bool,bytes)")]
    pub struct SimulationResults {
        pub success: bool,
        pub res: ::ethers::core::types::Bytes,
    }
    ///Custom Error type `SwapAmountTooLarge` with signature `SwapAmountTooLarge()` and selector `0xcf0b4d3a`
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
    #[etherror(name = "SwapAmountTooLarge", abi = "SwapAmountTooLarge()")]
    pub struct SwapAmountTooLarge;
    ///Custom Error type `SwapWithZeroAmount` with signature `SwapWithZeroAmount()` and selector `0xfba5a276`
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
    #[etherror(name = "SwapWithZeroAmount", abi = "SwapWithZeroAmount()")]
    pub struct SwapWithZeroAmount;
    ///Custom Error type `TakingAmountExceeded` with signature `TakingAmountExceeded()` and selector `0x7f902a93`
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
    #[etherror(name = "TakingAmountExceeded", abi = "TakingAmountExceeded()")]
    pub struct TakingAmountExceeded;
    ///Custom Error type `TakingAmountIncreased` with signature `TakingAmountIncreased()` and selector `0x939c4204`
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
    #[etherror(name = "TakingAmountIncreased", abi = "TakingAmountIncreased()")]
    pub struct TakingAmountIncreased;
    ///Custom Error type `TakingAmountTooHigh` with signature `TakingAmountTooHigh()` and selector `0xfb8ae129`
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
    #[etherror(name = "TakingAmountTooHigh", abi = "TakingAmountTooHigh()")]
    pub struct TakingAmountTooHigh;
    ///Custom Error type `TransferFromMakerToTakerFailed` with signature `TransferFromMakerToTakerFailed()` and selector `0x70a03f48`
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
    #[etherror(
        name = "TransferFromMakerToTakerFailed",
        abi = "TransferFromMakerToTakerFailed()"
    )]
    pub struct TransferFromMakerToTakerFailed;
    ///Custom Error type `TransferFromTakerToMakerFailed` with signature `TransferFromTakerToMakerFailed()` and selector `0x478a5205`
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
    #[etherror(
        name = "TransferFromTakerToMakerFailed",
        abi = "TransferFromTakerToMakerFailed()"
    )]
    pub struct TransferFromTakerToMakerFailed;
    ///Custom Error type `UnknownOrder` with signature `UnknownOrder()` and selector `0xb838de96`
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
    #[etherror(name = "UnknownOrder", abi = "UnknownOrder()")]
    pub struct UnknownOrder;
    ///Custom Error type `WrongAmount` with signature `WrongAmount()` and selector `0x49986e73`
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
    #[etherror(name = "WrongAmount", abi = "WrongAmount()")]
    pub struct WrongAmount;
    ///Custom Error type `WrongGetter` with signature `WrongGetter()` and selector `0xbec74c85`
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
    #[etherror(name = "WrongGetter", abi = "WrongGetter()")]
    pub struct WrongGetter;
    ///Custom Error type `ZeroAddress` with signature `ZeroAddress()` and selector `0xd92e233d`
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
    #[etherror(name = "ZeroAddress", abi = "ZeroAddress()")]
    pub struct ZeroAddress;
    ///Custom Error type `ZeroMinReturn` with signature `ZeroMinReturn()` and selector `0x0262dde4`
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
    #[etherror(name = "ZeroMinReturn", abi = "ZeroMinReturn()")]
    pub struct ZeroMinReturn;
    ///Custom Error type `ZeroReturnAmount` with signature `ZeroReturnAmount()` and selector `0x28ebf247`
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
    #[etherror(name = "ZeroReturnAmount", abi = "ZeroReturnAmount()")]
    pub struct ZeroReturnAmount;
    ///Custom Error type `ZeroTargetIsForbidden` with signature `ZeroTargetIsForbidden()` and selector `0xb0c4d05f`
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
    #[etherror(name = "ZeroTargetIsForbidden", abi = "ZeroTargetIsForbidden()")]
    pub struct ZeroTargetIsForbidden;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AggregationRouterV5Errors {
        AccessDenied(AccessDenied),
        AdvanceNonceFailed(AdvanceNonceFailed),
        AlreadyFilled(AlreadyFilled),
        ArbitraryStaticCallFailed(ArbitraryStaticCallFailed),
        BadPool(BadPool),
        BadSignature(BadSignature),
        ETHTransferFailed(ETHTransferFailed),
        EmptyPools(EmptyPools),
        EthDepositRejected(EthDepositRejected),
        GetAmountCallFailed(GetAmountCallFailed),
        IncorrectDataLength(IncorrectDataLength),
        InsufficientBalance(InsufficientBalance),
        InvalidMsgValue(InvalidMsgValue),
        InvalidatedOrder(InvalidatedOrder),
        MakingAmountExceeded(MakingAmountExceeded),
        MakingAmountTooLow(MakingAmountTooLow),
        OnlyOneAmountShouldBeZero(OnlyOneAmountShouldBeZero),
        OrderExpired(OrderExpired),
        PermitLengthTooLow(PermitLengthTooLow),
        PredicateIsNotTrue(PredicateIsNotTrue),
        PrivateOrder(PrivateOrder),
        RFQBadSignature(RFQBadSignature),
        RFQPrivateOrder(RFQPrivateOrder),
        RFQSwapWithZeroAmount(RFQSwapWithZeroAmount),
        RFQZeroTargetIsForbidden(RFQZeroTargetIsForbidden),
        ReentrancyDetected(ReentrancyDetected),
        RemainingAmountIsZero(RemainingAmountIsZero),
        ReservesCallFailed(ReservesCallFailed),
        ReturnAmountIsNotEnough(ReturnAmountIsNotEnough),
        SafePermitBadLength(SafePermitBadLength),
        SafeTransferFailed(SafeTransferFailed),
        SafeTransferFromFailed(SafeTransferFromFailed),
        SimulationResults(SimulationResults),
        SwapAmountTooLarge(SwapAmountTooLarge),
        SwapWithZeroAmount(SwapWithZeroAmount),
        TakingAmountExceeded(TakingAmountExceeded),
        TakingAmountIncreased(TakingAmountIncreased),
        TakingAmountTooHigh(TakingAmountTooHigh),
        TransferFromMakerToTakerFailed(TransferFromMakerToTakerFailed),
        TransferFromTakerToMakerFailed(TransferFromTakerToMakerFailed),
        UnknownOrder(UnknownOrder),
        WrongAmount(WrongAmount),
        WrongGetter(WrongGetter),
        ZeroAddress(ZeroAddress),
        ZeroMinReturn(ZeroMinReturn),
        ZeroReturnAmount(ZeroReturnAmount),
        ZeroTargetIsForbidden(ZeroTargetIsForbidden),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AggregationRouterV5Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AccessDenied as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AccessDenied(decoded));
            }
            if let Ok(decoded) =
                <AdvanceNonceFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AdvanceNonceFailed(decoded));
            }
            if let Ok(decoded) = <AlreadyFilled as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AlreadyFilled(decoded));
            }
            if let Ok(decoded) =
                <ArbitraryStaticCallFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ArbitraryStaticCallFailed(decoded));
            }
            if let Ok(decoded) = <BadPool as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BadPool(decoded));
            }
            if let Ok(decoded) = <BadSignature as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BadSignature(decoded));
            }
            if let Ok(decoded) = <ETHTransferFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ETHTransferFailed(decoded));
            }
            if let Ok(decoded) = <ETHTransferFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ETHTransferFailed(decoded));
            }
            if let Ok(decoded) = <EmptyPools as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EmptyPools(decoded));
            }
            if let Ok(decoded) =
                <EthDepositRejected as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EthDepositRejected(decoded));
            }
            if let Ok(decoded) =
                <GetAmountCallFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAmountCallFailed(decoded));
            }
            if let Ok(decoded) =
                <IncorrectDataLength as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncorrectDataLength(decoded));
            }
            if let Ok(decoded) =
                <InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientBalance(decoded));
            }
            if let Ok(decoded) = <InvalidMsgValue as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidMsgValue(decoded));
            }
            if let Ok(decoded) = <InvalidMsgValue as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidMsgValue(decoded));
            }
            if let Ok(decoded) = <InvalidatedOrder as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidatedOrder(decoded));
            }
            if let Ok(decoded) =
                <MakingAmountExceeded as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MakingAmountExceeded(decoded));
            }
            if let Ok(decoded) =
                <MakingAmountTooLow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MakingAmountTooLow(decoded));
            }
            if let Ok(decoded) =
                <OnlyOneAmountShouldBeZero as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnlyOneAmountShouldBeZero(decoded));
            }
            if let Ok(decoded) = <OrderExpired as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OrderExpired(decoded));
            }
            if let Ok(decoded) =
                <PermitLengthTooLow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PermitLengthTooLow(decoded));
            }
            if let Ok(decoded) =
                <PredicateIsNotTrue as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PredicateIsNotTrue(decoded));
            }
            if let Ok(decoded) = <PrivateOrder as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PrivateOrder(decoded));
            }
            if let Ok(decoded) = <RFQBadSignature as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RFQBadSignature(decoded));
            }
            if let Ok(decoded) = <RFQPrivateOrder as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RFQPrivateOrder(decoded));
            }
            if let Ok(decoded) =
                <RFQSwapWithZeroAmount as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RFQSwapWithZeroAmount(decoded));
            }
            if let Ok(decoded) =
                <RFQZeroTargetIsForbidden as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RFQZeroTargetIsForbidden(decoded));
            }
            if let Ok(decoded) =
                <ReentrancyDetected as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReentrancyDetected(decoded));
            }
            if let Ok(decoded) =
                <RemainingAmountIsZero as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemainingAmountIsZero(decoded));
            }
            if let Ok(decoded) =
                <ReservesCallFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReservesCallFailed(decoded));
            }
            if let Ok(decoded) =
                <ReturnAmountIsNotEnough as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReturnAmountIsNotEnough(decoded));
            }
            if let Ok(decoded) =
                <SafePermitBadLength as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafePermitBadLength(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafeTransferFailed(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafeTransferFromFailed(decoded));
            }
            if let Ok(decoded) = <SimulationResults as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SimulationResults(decoded));
            }
            if let Ok(decoded) =
                <SwapAmountTooLarge as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapAmountTooLarge(decoded));
            }
            if let Ok(decoded) =
                <SwapWithZeroAmount as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapWithZeroAmount(decoded));
            }
            if let Ok(decoded) =
                <TakingAmountExceeded as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TakingAmountExceeded(decoded));
            }
            if let Ok(decoded) =
                <TakingAmountIncreased as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TakingAmountIncreased(decoded));
            }
            if let Ok(decoded) =
                <TakingAmountTooHigh as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TakingAmountTooHigh(decoded));
            }
            if let Ok(decoded) =
                <TransferFromMakerToTakerFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferFromMakerToTakerFailed(decoded));
            }
            if let Ok(decoded) =
                <TransferFromTakerToMakerFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferFromTakerToMakerFailed(decoded));
            }
            if let Ok(decoded) = <UnknownOrder as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnknownOrder(decoded));
            }
            if let Ok(decoded) = <WrongAmount as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WrongAmount(decoded));
            }
            if let Ok(decoded) = <WrongGetter as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WrongGetter(decoded));
            }
            if let Ok(decoded) = <ZeroAddress as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroAddress(decoded));
            }
            if let Ok(decoded) = <ZeroMinReturn as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroMinReturn(decoded));
            }
            if let Ok(decoded) = <ZeroReturnAmount as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ZeroReturnAmount(decoded));
            }
            if let Ok(decoded) =
                <ZeroTargetIsForbidden as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ZeroTargetIsForbidden(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AggregationRouterV5Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AccessDenied(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AdvanceNonceFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AlreadyFilled(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ArbitraryStaticCallFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BadPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BadSignature(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ETHTransferFailed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EmptyPools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EthDepositRejected(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAmountCallFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncorrectDataLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMsgValue(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidatedOrder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MakingAmountExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MakingAmountTooLow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnlyOneAmountShouldBeZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OrderExpired(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PermitLengthTooLow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PredicateIsNotTrue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrivateOrder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RFQBadSignature(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RFQPrivateOrder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RFQSwapWithZeroAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RFQZeroTargetIsForbidden(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReentrancyDetected(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemainingAmountIsZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReservesCallFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReturnAmountIsNotEnough(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafePermitBadLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFromFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulationResults(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SwapAmountTooLarge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapWithZeroAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TakingAmountExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TakingAmountIncreased(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TakingAmountTooHigh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFromMakerToTakerFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFromTakerToMakerFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnknownOrder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WrongAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WrongGetter(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZeroAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZeroMinReturn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZeroReturnAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ZeroTargetIsForbidden(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for AggregationRouterV5Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AccessDenied as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <AdvanceNonceFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AlreadyFilled as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ArbitraryStaticCallFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BadPool as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <BadSignature as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ETHTransferFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ETHTransferFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EmptyPools as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <EthDepositRejected as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <GetAmountCallFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <IncorrectDataLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMsgValue as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMsgValue as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidatedOrder as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MakingAmountExceeded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MakingAmountTooLow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyOneAmountShouldBeZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OrderExpired as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <PermitLengthTooLow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PredicateIsNotTrue as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PrivateOrder as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <RFQBadSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RFQPrivateOrder as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RFQSwapWithZeroAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RFQZeroTargetIsForbidden as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ReentrancyDetected as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RemainingAmountIsZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ReservesCallFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ReturnAmountIsNotEnough as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafePermitBadLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeTransferFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeTransferFromFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SimulationResults as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SwapAmountTooLarge as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SwapWithZeroAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TakingAmountExceeded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TakingAmountIncreased as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TakingAmountTooHigh as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransferFromMakerToTakerFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransferFromTakerToMakerFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnknownOrder as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <WrongAmount as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <WrongGetter as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ZeroAddress as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ZeroMinReturn as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ZeroReturnAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ZeroTargetIsForbidden as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AggregationRouterV5Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccessDenied(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdvanceNonceFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::AlreadyFilled(element) => ::core::fmt::Display::fmt(element, f),
                Self::ArbitraryStaticCallFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::BadPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::BadSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::ETHTransferFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::EmptyPools(element) => ::core::fmt::Display::fmt(element, f),
                Self::EthDepositRejected(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountCallFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncorrectDataLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidMsgValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidatedOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakingAmountExceeded(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakingAmountTooLow(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyOneAmountShouldBeZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::OrderExpired(element) => ::core::fmt::Display::fmt(element, f),
                Self::PermitLengthTooLow(element) => ::core::fmt::Display::fmt(element, f),
                Self::PredicateIsNotTrue(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrivateOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::RFQBadSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::RFQPrivateOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::RFQSwapWithZeroAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::RFQZeroTargetIsForbidden(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReentrancyDetected(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemainingAmountIsZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReservesCallFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReturnAmountIsNotEnough(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafePermitBadLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFromFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulationResults(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapAmountTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapWithZeroAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::TakingAmountExceeded(element) => ::core::fmt::Display::fmt(element, f),
                Self::TakingAmountIncreased(element) => ::core::fmt::Display::fmt(element, f),
                Self::TakingAmountTooHigh(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFromMakerToTakerFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFromTakerToMakerFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnknownOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongGetter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroMinReturn(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroReturnAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroTargetIsForbidden(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AggregationRouterV5Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AccessDenied> for AggregationRouterV5Errors {
        fn from(value: AccessDenied) -> Self {
            Self::AccessDenied(value)
        }
    }
    impl ::core::convert::From<AdvanceNonceFailed> for AggregationRouterV5Errors {
        fn from(value: AdvanceNonceFailed) -> Self {
            Self::AdvanceNonceFailed(value)
        }
    }
    impl ::core::convert::From<AlreadyFilled> for AggregationRouterV5Errors {
        fn from(value: AlreadyFilled) -> Self {
            Self::AlreadyFilled(value)
        }
    }
    impl ::core::convert::From<ArbitraryStaticCallFailed> for AggregationRouterV5Errors {
        fn from(value: ArbitraryStaticCallFailed) -> Self {
            Self::ArbitraryStaticCallFailed(value)
        }
    }
    impl ::core::convert::From<BadPool> for AggregationRouterV5Errors {
        fn from(value: BadPool) -> Self {
            Self::BadPool(value)
        }
    }
    impl ::core::convert::From<BadSignature> for AggregationRouterV5Errors {
        fn from(value: BadSignature) -> Self {
            Self::BadSignature(value)
        }
    }
    impl ::core::convert::From<ETHTransferFailed> for AggregationRouterV5Errors {
        fn from(value: ETHTransferFailed) -> Self {
            Self::ETHTransferFailed(value)
        }
    }
    impl ::core::convert::From<EmptyPools> for AggregationRouterV5Errors {
        fn from(value: EmptyPools) -> Self {
            Self::EmptyPools(value)
        }
    }
    impl ::core::convert::From<EthDepositRejected> for AggregationRouterV5Errors {
        fn from(value: EthDepositRejected) -> Self {
            Self::EthDepositRejected(value)
        }
    }
    impl ::core::convert::From<GetAmountCallFailed> for AggregationRouterV5Errors {
        fn from(value: GetAmountCallFailed) -> Self {
            Self::GetAmountCallFailed(value)
        }
    }
    impl ::core::convert::From<IncorrectDataLength> for AggregationRouterV5Errors {
        fn from(value: IncorrectDataLength) -> Self {
            Self::IncorrectDataLength(value)
        }
    }
    impl ::core::convert::From<InsufficientBalance> for AggregationRouterV5Errors {
        fn from(value: InsufficientBalance) -> Self {
            Self::InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<InvalidMsgValue> for AggregationRouterV5Errors {
        fn from(value: InvalidMsgValue) -> Self {
            Self::InvalidMsgValue(value)
        }
    }
    impl ::core::convert::From<InvalidatedOrder> for AggregationRouterV5Errors {
        fn from(value: InvalidatedOrder) -> Self {
            Self::InvalidatedOrder(value)
        }
    }
    impl ::core::convert::From<MakingAmountExceeded> for AggregationRouterV5Errors {
        fn from(value: MakingAmountExceeded) -> Self {
            Self::MakingAmountExceeded(value)
        }
    }
    impl ::core::convert::From<MakingAmountTooLow> for AggregationRouterV5Errors {
        fn from(value: MakingAmountTooLow) -> Self {
            Self::MakingAmountTooLow(value)
        }
    }
    impl ::core::convert::From<OnlyOneAmountShouldBeZero> for AggregationRouterV5Errors {
        fn from(value: OnlyOneAmountShouldBeZero) -> Self {
            Self::OnlyOneAmountShouldBeZero(value)
        }
    }
    impl ::core::convert::From<OrderExpired> for AggregationRouterV5Errors {
        fn from(value: OrderExpired) -> Self {
            Self::OrderExpired(value)
        }
    }
    impl ::core::convert::From<PermitLengthTooLow> for AggregationRouterV5Errors {
        fn from(value: PermitLengthTooLow) -> Self {
            Self::PermitLengthTooLow(value)
        }
    }
    impl ::core::convert::From<PredicateIsNotTrue> for AggregationRouterV5Errors {
        fn from(value: PredicateIsNotTrue) -> Self {
            Self::PredicateIsNotTrue(value)
        }
    }
    impl ::core::convert::From<PrivateOrder> for AggregationRouterV5Errors {
        fn from(value: PrivateOrder) -> Self {
            Self::PrivateOrder(value)
        }
    }
    impl ::core::convert::From<RFQBadSignature> for AggregationRouterV5Errors {
        fn from(value: RFQBadSignature) -> Self {
            Self::RFQBadSignature(value)
        }
    }
    impl ::core::convert::From<RFQPrivateOrder> for AggregationRouterV5Errors {
        fn from(value: RFQPrivateOrder) -> Self {
            Self::RFQPrivateOrder(value)
        }
    }
    impl ::core::convert::From<RFQSwapWithZeroAmount> for AggregationRouterV5Errors {
        fn from(value: RFQSwapWithZeroAmount) -> Self {
            Self::RFQSwapWithZeroAmount(value)
        }
    }
    impl ::core::convert::From<RFQZeroTargetIsForbidden> for AggregationRouterV5Errors {
        fn from(value: RFQZeroTargetIsForbidden) -> Self {
            Self::RFQZeroTargetIsForbidden(value)
        }
    }
    impl ::core::convert::From<ReentrancyDetected> for AggregationRouterV5Errors {
        fn from(value: ReentrancyDetected) -> Self {
            Self::ReentrancyDetected(value)
        }
    }
    impl ::core::convert::From<RemainingAmountIsZero> for AggregationRouterV5Errors {
        fn from(value: RemainingAmountIsZero) -> Self {
            Self::RemainingAmountIsZero(value)
        }
    }
    impl ::core::convert::From<ReservesCallFailed> for AggregationRouterV5Errors {
        fn from(value: ReservesCallFailed) -> Self {
            Self::ReservesCallFailed(value)
        }
    }
    impl ::core::convert::From<ReturnAmountIsNotEnough> for AggregationRouterV5Errors {
        fn from(value: ReturnAmountIsNotEnough) -> Self {
            Self::ReturnAmountIsNotEnough(value)
        }
    }
    impl ::core::convert::From<SafePermitBadLength> for AggregationRouterV5Errors {
        fn from(value: SafePermitBadLength) -> Self {
            Self::SafePermitBadLength(value)
        }
    }
    impl ::core::convert::From<SafeTransferFailed> for AggregationRouterV5Errors {
        fn from(value: SafeTransferFailed) -> Self {
            Self::SafeTransferFailed(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromFailed> for AggregationRouterV5Errors {
        fn from(value: SafeTransferFromFailed) -> Self {
            Self::SafeTransferFromFailed(value)
        }
    }
    impl ::core::convert::From<SimulationResults> for AggregationRouterV5Errors {
        fn from(value: SimulationResults) -> Self {
            Self::SimulationResults(value)
        }
    }
    impl ::core::convert::From<SwapAmountTooLarge> for AggregationRouterV5Errors {
        fn from(value: SwapAmountTooLarge) -> Self {
            Self::SwapAmountTooLarge(value)
        }
    }
    impl ::core::convert::From<SwapWithZeroAmount> for AggregationRouterV5Errors {
        fn from(value: SwapWithZeroAmount) -> Self {
            Self::SwapWithZeroAmount(value)
        }
    }
    impl ::core::convert::From<TakingAmountExceeded> for AggregationRouterV5Errors {
        fn from(value: TakingAmountExceeded) -> Self {
            Self::TakingAmountExceeded(value)
        }
    }
    impl ::core::convert::From<TakingAmountIncreased> for AggregationRouterV5Errors {
        fn from(value: TakingAmountIncreased) -> Self {
            Self::TakingAmountIncreased(value)
        }
    }
    impl ::core::convert::From<TakingAmountTooHigh> for AggregationRouterV5Errors {
        fn from(value: TakingAmountTooHigh) -> Self {
            Self::TakingAmountTooHigh(value)
        }
    }
    impl ::core::convert::From<TransferFromMakerToTakerFailed> for AggregationRouterV5Errors {
        fn from(value: TransferFromMakerToTakerFailed) -> Self {
            Self::TransferFromMakerToTakerFailed(value)
        }
    }
    impl ::core::convert::From<TransferFromTakerToMakerFailed> for AggregationRouterV5Errors {
        fn from(value: TransferFromTakerToMakerFailed) -> Self {
            Self::TransferFromTakerToMakerFailed(value)
        }
    }
    impl ::core::convert::From<UnknownOrder> for AggregationRouterV5Errors {
        fn from(value: UnknownOrder) -> Self {
            Self::UnknownOrder(value)
        }
    }
    impl ::core::convert::From<WrongAmount> for AggregationRouterV5Errors {
        fn from(value: WrongAmount) -> Self {
            Self::WrongAmount(value)
        }
    }
    impl ::core::convert::From<WrongGetter> for AggregationRouterV5Errors {
        fn from(value: WrongGetter) -> Self {
            Self::WrongGetter(value)
        }
    }
    impl ::core::convert::From<ZeroAddress> for AggregationRouterV5Errors {
        fn from(value: ZeroAddress) -> Self {
            Self::ZeroAddress(value)
        }
    }
    impl ::core::convert::From<ZeroMinReturn> for AggregationRouterV5Errors {
        fn from(value: ZeroMinReturn) -> Self {
            Self::ZeroMinReturn(value)
        }
    }
    impl ::core::convert::From<ZeroReturnAmount> for AggregationRouterV5Errors {
        fn from(value: ZeroReturnAmount) -> Self {
            Self::ZeroReturnAmount(value)
        }
    }
    impl ::core::convert::From<ZeroTargetIsForbidden> for AggregationRouterV5Errors {
        fn from(value: ZeroTargetIsForbidden) -> Self {
            Self::ZeroTargetIsForbidden(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "NonceIncreased", abi = "NonceIncreased(address,uint256)")]
    pub struct NonceIncreasedFilter {
        #[ethevent(indexed)]
        pub maker: ::ethers::core::types::Address,
        pub new_nonce: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "OrderCanceled", abi = "OrderCanceled(address,bytes32,uint256)")]
    pub struct OrderCanceledFilter {
        #[ethevent(indexed)]
        pub maker: ::ethers::core::types::Address,
        pub order_hash: [u8; 32],
        pub remaining_raw: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "OrderFilled", abi = "OrderFilled(address,bytes32,uint256)")]
    pub struct OrderFilledFilter {
        #[ethevent(indexed)]
        pub maker: ::ethers::core::types::Address,
        pub order_hash: [u8; 32],
        pub remaining: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "OrderFilledRFQ", abi = "OrderFilledRFQ(bytes32,uint256)")]
    pub struct OrderFilledRFQFilter {
        pub order_hash: [u8; 32],
        pub making_amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AggregationRouterV5Events {
        NonceIncreasedFilter(NonceIncreasedFilter),
        OrderCanceledFilter(OrderCanceledFilter),
        OrderFilledFilter(OrderFilledFilter),
        OrderFilledRFQFilter(OrderFilledRFQFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for AggregationRouterV5Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = NonceIncreasedFilter::decode_log(log) {
                return Ok(AggregationRouterV5Events::NonceIncreasedFilter(decoded));
            }
            if let Ok(decoded) = OrderCanceledFilter::decode_log(log) {
                return Ok(AggregationRouterV5Events::OrderCanceledFilter(decoded));
            }
            if let Ok(decoded) = OrderFilledFilter::decode_log(log) {
                return Ok(AggregationRouterV5Events::OrderFilledFilter(decoded));
            }
            if let Ok(decoded) = OrderFilledRFQFilter::decode_log(log) {
                return Ok(AggregationRouterV5Events::OrderFilledRFQFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(AggregationRouterV5Events::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AggregationRouterV5Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::NonceIncreasedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OrderCanceledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OrderFilledFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OrderFilledRFQFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<NonceIncreasedFilter> for AggregationRouterV5Events {
        fn from(value: NonceIncreasedFilter) -> Self {
            Self::NonceIncreasedFilter(value)
        }
    }
    impl ::core::convert::From<OrderCanceledFilter> for AggregationRouterV5Events {
        fn from(value: OrderCanceledFilter) -> Self {
            Self::OrderCanceledFilter(value)
        }
    }
    impl ::core::convert::From<OrderFilledFilter> for AggregationRouterV5Events {
        fn from(value: OrderFilledFilter) -> Self {
            Self::OrderFilledFilter(value)
        }
    }
    impl ::core::convert::From<OrderFilledRFQFilter> for AggregationRouterV5Events {
        fn from(value: OrderFilledRFQFilter) -> Self {
            Self::OrderFilledRFQFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for AggregationRouterV5Events {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `advanceNonce` function with signature `advanceNonce(uint8)` and selector `0x72c244a8`
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
    #[ethcall(name = "advanceNonce", abi = "advanceNonce(uint8)")]
    pub struct AdvanceNonceCall {
        pub amount: u8,
    }
    ///Container type for all input parameters for the `and` function with signature `and(uint256,bytes)` and selector `0xbfa75143`
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
    #[ethcall(name = "and", abi = "and(uint256,bytes)")]
    pub struct AndCall {
        pub offsets: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `arbitraryStaticCall` function with signature `arbitraryStaticCall(address,bytes)` and selector `0xbf15fcd8`
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
        name = "arbitraryStaticCall",
        abi = "arbitraryStaticCall(address,bytes)"
    )]
    pub struct ArbitraryStaticCallCall {
        pub target: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `cancelOrder` function with signature `cancelOrder((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes))` and selector `0x2d9a56f6`
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
        name = "cancelOrder",
        abi = "cancelOrder((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes))"
    )]
    pub struct CancelOrderCall {
        pub order: Order,
    }
    ///Container type for all input parameters for the `cancelOrderRFQ` function with signature `cancelOrderRFQ(uint256)` and selector `0x825caba1`
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
    #[ethcall(name = "cancelOrderRFQ", abi = "cancelOrderRFQ(uint256)")]
    pub struct CancelOrderRFQCall {
        pub order_info: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `cancelOrderRFQ` function with signature `cancelOrderRFQ(uint256,uint256)` and selector `0xbddccd35`
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
    #[ethcall(name = "cancelOrderRFQ", abi = "cancelOrderRFQ(uint256,uint256)")]
    pub struct CancelOrderRfqWithAdditionalMaskCall {
        pub order_info: ::ethers::core::types::U256,
        pub additional_mask: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `checkPredicate` function with signature `checkPredicate((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes))` and selector `0x6c838250`
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
        name = "checkPredicate",
        abi = "checkPredicate((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes))"
    )]
    pub struct CheckPredicateCall {
        pub order: Order,
    }
    ///Container type for all input parameters for the `clipperSwap` function with signature `clipperSwap(address,address,address,uint256,uint256,uint256,bytes32,bytes32)` and selector `0x84bd6d29`
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
        name = "clipperSwap",
        abi = "clipperSwap(address,address,address,uint256,uint256,uint256,bytes32,bytes32)"
    )]
    pub struct ClipperSwapCall {
        pub clipper_exchange: ::ethers::core::types::Address,
        pub src_token: ::ethers::core::types::Address,
        pub dst_token: ::ethers::core::types::Address,
        pub input_amount: ::ethers::core::types::U256,
        pub output_amount: ::ethers::core::types::U256,
        pub good_until: ::ethers::core::types::U256,
        pub r: [u8; 32],
        pub vs: [u8; 32],
    }
    ///Container type for all input parameters for the `clipperSwapTo` function with signature `clipperSwapTo(address,address,address,address,uint256,uint256,uint256,bytes32,bytes32)` and selector `0x093d4fa5`
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
        name = "clipperSwapTo",
        abi = "clipperSwapTo(address,address,address,address,uint256,uint256,uint256,bytes32,bytes32)"
    )]
    pub struct ClipperSwapToCall {
        pub clipper_exchange: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub src_token: ::ethers::core::types::Address,
        pub dst_token: ::ethers::core::types::Address,
        pub input_amount: ::ethers::core::types::U256,
        pub output_amount: ::ethers::core::types::U256,
        pub good_until: ::ethers::core::types::U256,
        pub r: [u8; 32],
        pub vs: [u8; 32],
    }
    ///Container type for all input parameters for the `clipperSwapToWithPermit` function with signature `clipperSwapToWithPermit(address,address,address,address,uint256,uint256,uint256,bytes32,bytes32,bytes)` and selector `0xc805a666`
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
        name = "clipperSwapToWithPermit",
        abi = "clipperSwapToWithPermit(address,address,address,address,uint256,uint256,uint256,bytes32,bytes32,bytes)"
    )]
    pub struct ClipperSwapToWithPermitCall {
        pub clipper_exchange: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub src_token: ::ethers::core::types::Address,
        pub dst_token: ::ethers::core::types::Address,
        pub input_amount: ::ethers::core::types::U256,
        pub output_amount: ::ethers::core::types::U256,
        pub good_until: ::ethers::core::types::U256,
        pub r: [u8; 32],
        pub vs: [u8; 32],
        pub permit: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `destroy` function with signature `destroy()` and selector `0x83197ef0`
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
    #[ethcall(name = "destroy", abi = "destroy()")]
    pub struct DestroyCall;
    ///Container type for all input parameters for the `eq` function with signature `eq(uint256,bytes)` and selector `0x6fe7b0ba`
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
    #[ethcall(name = "eq", abi = "eq(uint256,bytes)")]
    pub struct EqCall {
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `fillOrder` function with signature `fillOrder((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes),bytes,bytes,uint256,uint256,uint256)` and selector `0x62e238bb`
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
        name = "fillOrder",
        abi = "fillOrder((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes),bytes,bytes,uint256,uint256,uint256)"
    )]
    pub struct FillOrderCall {
        pub order: Order,
        pub signature: ::ethers::core::types::Bytes,
        pub interaction: ::ethers::core::types::Bytes,
        pub making_amount: ::ethers::core::types::U256,
        pub taking_amount: ::ethers::core::types::U256,
        pub skip_permit_and_threshold_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `fillOrderRFQ` function with signature `fillOrderRFQ((uint256,address,address,address,address,uint256,uint256),bytes,uint256)` and selector `0x3eca9c0a`
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
        name = "fillOrderRFQ",
        abi = "fillOrderRFQ((uint256,address,address,address,address,uint256,uint256),bytes,uint256)"
    )]
    pub struct FillOrderRFQCall {
        pub order: OrderRFQ,
        pub signature: ::ethers::core::types::Bytes,
        pub flags_and_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `fillOrderRFQCompact` function with signature `fillOrderRFQCompact((uint256,address,address,address,address,uint256,uint256),bytes32,bytes32,uint256)` and selector `0x9570eeee`
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
        name = "fillOrderRFQCompact",
        abi = "fillOrderRFQCompact((uint256,address,address,address,address,uint256,uint256),bytes32,bytes32,uint256)"
    )]
    pub struct FillOrderRFQCompactCall {
        pub order: OrderRFQ,
        pub r: [u8; 32],
        pub vs: [u8; 32],
        pub flags_and_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `fillOrderRFQTo` function with signature `fillOrderRFQTo((uint256,address,address,address,address,uint256,uint256),bytes,uint256,address)` and selector `0x5a099843`
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
        name = "fillOrderRFQTo",
        abi = "fillOrderRFQTo((uint256,address,address,address,address,uint256,uint256),bytes,uint256,address)"
    )]
    pub struct FillOrderRFQToCall {
        pub order: OrderRFQ,
        pub signature: ::ethers::core::types::Bytes,
        pub flags_and_amount: ::ethers::core::types::U256,
        pub target: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `fillOrderRFQToWithPermit` function with signature `fillOrderRFQToWithPermit((uint256,address,address,address,address,uint256,uint256),bytes,uint256,address,bytes)` and selector `0x70ccbd31`
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
        name = "fillOrderRFQToWithPermit",
        abi = "fillOrderRFQToWithPermit((uint256,address,address,address,address,uint256,uint256),bytes,uint256,address,bytes)"
    )]
    pub struct FillOrderRFQToWithPermitCall {
        pub order: OrderRFQ,
        pub signature: ::ethers::core::types::Bytes,
        pub flags_and_amount: ::ethers::core::types::U256,
        pub target: ::ethers::core::types::Address,
        pub permit: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `fillOrderTo` function with signature `fillOrderTo((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes),bytes,bytes,uint256,uint256,uint256,address)` and selector `0xe5d7bde6`
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
        name = "fillOrderTo",
        abi = "fillOrderTo((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes),bytes,bytes,uint256,uint256,uint256,address)"
    )]
    pub struct FillOrderToCall {
        pub order: Order,
        pub signature: ::ethers::core::types::Bytes,
        pub interaction: ::ethers::core::types::Bytes,
        pub making_amount: ::ethers::core::types::U256,
        pub taking_amount: ::ethers::core::types::U256,
        pub skip_permit_and_threshold_amount: ::ethers::core::types::U256,
        pub target: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `fillOrderToWithPermit` function with signature `fillOrderToWithPermit((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes),bytes,bytes,uint256,uint256,uint256,address,bytes)` and selector `0xd365c695`
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
        name = "fillOrderToWithPermit",
        abi = "fillOrderToWithPermit((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes),bytes,bytes,uint256,uint256,uint256,address,bytes)"
    )]
    pub struct FillOrderToWithPermitCall {
        pub order: Order,
        pub signature: ::ethers::core::types::Bytes,
        pub interaction: ::ethers::core::types::Bytes,
        pub making_amount: ::ethers::core::types::U256,
        pub taking_amount: ::ethers::core::types::U256,
        pub skip_permit_and_threshold_amount: ::ethers::core::types::U256,
        pub target: ::ethers::core::types::Address,
        pub permit: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `gt` function with signature `gt(uint256,bytes)` and selector `0x4f38e2b8`
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
    #[ethcall(name = "gt", abi = "gt(uint256,bytes)")]
    pub struct GtCall {
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `hashOrder` function with signature `hashOrder((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes))` and selector `0x37e7316f`
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
        name = "hashOrder",
        abi = "hashOrder((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes))"
    )]
    pub struct HashOrderCall {
        pub order: Order,
    }
    ///Container type for all input parameters for the `increaseNonce` function with signature `increaseNonce()` and selector `0xc53a0292`
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
    #[ethcall(name = "increaseNonce", abi = "increaseNonce()")]
    pub struct IncreaseNonceCall;
    ///Container type for all input parameters for the `invalidatorForOrderRFQ` function with signature `invalidatorForOrderRFQ(address,uint256)` and selector `0x56f16124`
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
        name = "invalidatorForOrderRFQ",
        abi = "invalidatorForOrderRFQ(address,uint256)"
    )]
    pub struct InvalidatorForOrderRFQCall {
        pub maker: ::ethers::core::types::Address,
        pub slot: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `lt` function with signature `lt(uint256,bytes)` and selector `0xca4ece22`
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
    #[ethcall(name = "lt", abi = "lt(uint256,bytes)")]
    pub struct LtCall {
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `nonce` function with signature `nonce(address)` and selector `0x70ae92d2`
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
    #[ethcall(name = "nonce", abi = "nonce(address)")]
    pub struct NonceCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `nonceEquals` function with signature `nonceEquals(address,uint256)` and selector `0xcf6fc6e3`
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
    #[ethcall(name = "nonceEquals", abi = "nonceEquals(address,uint256)")]
    pub struct NonceEqualsCall {
        pub maker_address: ::ethers::core::types::Address,
        pub maker_nonce: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `or` function with signature `or(uint256,bytes)` and selector `0x74261145`
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
    #[ethcall(name = "or", abi = "or(uint256,bytes)")]
    pub struct OrCall {
        pub offsets: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `remaining` function with signature `remaining(bytes32)` and selector `0xbc1ed74c`
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
    #[ethcall(name = "remaining", abi = "remaining(bytes32)")]
    pub struct RemainingCall {
        pub order_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `remainingRaw` function with signature `remainingRaw(bytes32)` and selector `0x7e54f092`
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
    #[ethcall(name = "remainingRaw", abi = "remainingRaw(bytes32)")]
    pub struct RemainingRawCall {
        pub order_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `remainingsRaw` function with signature `remainingsRaw(bytes32[])` and selector `0x942461bb`
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
    #[ethcall(name = "remainingsRaw", abi = "remainingsRaw(bytes32[])")]
    pub struct RemainingsRawCall {
        pub order_hashes: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `rescueFunds` function with signature `rescueFunds(address,uint256)` and selector `0x78e3214f`
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
    #[ethcall(name = "rescueFunds", abi = "rescueFunds(address,uint256)")]
    pub struct RescueFundsCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `simulate` function with signature `simulate(address,bytes)` and selector `0xbd61951d`
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
    #[ethcall(name = "simulate", abi = "simulate(address,bytes)")]
    pub struct SimulateCall {
        pub target: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `swap` function with signature `swap(address,(address,address,address,address,uint256,uint256,uint256),bytes,bytes)` and selector `0x12aa3caf`
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
        abi = "swap(address,(address,address,address,address,uint256,uint256,uint256),bytes,bytes)"
    )]
    pub struct SwapCall {
        pub executor: ::ethers::core::types::Address,
        pub desc: SwapDescription,
        pub permit: ::ethers::core::types::Bytes,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `timestampBelow` function with signature `timestampBelow(uint256)` and selector `0x63592c2b`
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
    #[ethcall(name = "timestampBelow", abi = "timestampBelow(uint256)")]
    pub struct TimestampBelowCall {
        pub time: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `timestampBelowAndNonceEquals` function with signature `timestampBelowAndNonceEquals(uint256)` and selector `0x2cc2878d`
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
        name = "timestampBelowAndNonceEquals",
        abi = "timestampBelowAndNonceEquals(uint256)"
    )]
    pub struct TimestampBelowAndNonceEqualsCall {
        pub time_nonce_account: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `uniswapV3Swap` function with signature `uniswapV3Swap(uint256,uint256,uint256[])` and selector `0xe449022e`
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
        name = "uniswapV3Swap",
        abi = "uniswapV3Swap(uint256,uint256,uint256[])"
    )]
    pub struct UniswapV3SwapCall {
        pub amount: ::ethers::core::types::U256,
        pub min_return: ::ethers::core::types::U256,
        pub pools: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `uniswapV3SwapCallback` function with signature `uniswapV3SwapCallback(int256,int256,bytes)` and selector `0xfa461e33`
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
        name = "uniswapV3SwapCallback",
        abi = "uniswapV3SwapCallback(int256,int256,bytes)"
    )]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0_delta: ::ethers::core::types::I256,
        pub amount_1_delta: ::ethers::core::types::I256,
        pub p2: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `uniswapV3SwapTo` function with signature `uniswapV3SwapTo(address,uint256,uint256,uint256[])` and selector `0xbc80f1a8`
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
        name = "uniswapV3SwapTo",
        abi = "uniswapV3SwapTo(address,uint256,uint256,uint256[])"
    )]
    pub struct UniswapV3SwapToCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub min_return: ::ethers::core::types::U256,
        pub pools: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `uniswapV3SwapToWithPermit` function with signature `uniswapV3SwapToWithPermit(address,address,uint256,uint256,uint256[],bytes)` and selector `0x2521b930`
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
        name = "uniswapV3SwapToWithPermit",
        abi = "uniswapV3SwapToWithPermit(address,address,uint256,uint256,uint256[],bytes)"
    )]
    pub struct UniswapV3SwapToWithPermitCall {
        pub recipient: ::ethers::core::types::Address,
        pub src_token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub min_return: ::ethers::core::types::U256,
        pub pools: ::std::vec::Vec<::ethers::core::types::U256>,
        pub permit: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `unoswap` function with signature `unoswap(address,uint256,uint256,uint256[])` and selector `0x0502b1c5`
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
    #[ethcall(name = "unoswap", abi = "unoswap(address,uint256,uint256,uint256[])")]
    pub struct UnoswapCall {
        pub src_token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub min_return: ::ethers::core::types::U256,
        pub pools: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `unoswapTo` function with signature `unoswapTo(address,address,uint256,uint256,uint256[])` and selector `0xf78dc253`
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
        name = "unoswapTo",
        abi = "unoswapTo(address,address,uint256,uint256,uint256[])"
    )]
    pub struct UnoswapToCall {
        pub recipient: ::ethers::core::types::Address,
        pub src_token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub min_return: ::ethers::core::types::U256,
        pub pools: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `unoswapToWithPermit` function with signature `unoswapToWithPermit(address,address,uint256,uint256,uint256[],bytes)` and selector `0x3c15fd91`
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
        name = "unoswapToWithPermit",
        abi = "unoswapToWithPermit(address,address,uint256,uint256,uint256[],bytes)"
    )]
    pub struct UnoswapToWithPermitCall {
        pub recipient: ::ethers::core::types::Address,
        pub src_token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub min_return: ::ethers::core::types::U256,
        pub pools: ::std::vec::Vec<::ethers::core::types::U256>,
        pub permit: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AggregationRouterV5Calls {
        AdvanceNonce(AdvanceNonceCall),
        And(AndCall),
        ArbitraryStaticCall(ArbitraryStaticCallCall),
        CancelOrder(CancelOrderCall),
        CancelOrderRFQ(CancelOrderRFQCall),
        CancelOrderRfqWithAdditionalMask(CancelOrderRfqWithAdditionalMaskCall),
        CheckPredicate(CheckPredicateCall),
        ClipperSwap(ClipperSwapCall),
        ClipperSwapTo(ClipperSwapToCall),
        ClipperSwapToWithPermit(ClipperSwapToWithPermitCall),
        Destroy(DestroyCall),
        Eq(EqCall),
        FillOrder(FillOrderCall),
        FillOrderRFQ(FillOrderRFQCall),
        FillOrderRFQCompact(FillOrderRFQCompactCall),
        FillOrderRFQTo(FillOrderRFQToCall),
        FillOrderRFQToWithPermit(FillOrderRFQToWithPermitCall),
        FillOrderTo(FillOrderToCall),
        FillOrderToWithPermit(FillOrderToWithPermitCall),
        Gt(GtCall),
        HashOrder(HashOrderCall),
        IncreaseNonce(IncreaseNonceCall),
        InvalidatorForOrderRFQ(InvalidatorForOrderRFQCall),
        Lt(LtCall),
        Nonce(NonceCall),
        NonceEquals(NonceEqualsCall),
        Or(OrCall),
        Owner(OwnerCall),
        Remaining(RemainingCall),
        RemainingRaw(RemainingRawCall),
        RemainingsRaw(RemainingsRawCall),
        RenounceOwnership(RenounceOwnershipCall),
        RescueFunds(RescueFundsCall),
        Simulate(SimulateCall),
        Swap(SwapCall),
        TimestampBelow(TimestampBelowCall),
        TimestampBelowAndNonceEquals(TimestampBelowAndNonceEqualsCall),
        TransferOwnership(TransferOwnershipCall),
        UniswapV3Swap(UniswapV3SwapCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
        UniswapV3SwapTo(UniswapV3SwapToCall),
        UniswapV3SwapToWithPermit(UniswapV3SwapToWithPermitCall),
        Unoswap(UnoswapCall),
        UnoswapTo(UnoswapToCall),
        UnoswapToWithPermit(UnoswapToWithPermitCall),
    }
    impl ::ethers::core::abi::AbiDecode for AggregationRouterV5Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AdvanceNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AdvanceNonce(decoded));
            }
            if let Ok(decoded) = <AndCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::And(decoded));
            }
            if let Ok(decoded) =
                <ArbitraryStaticCallCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ArbitraryStaticCall(decoded));
            }
            if let Ok(decoded) = <CancelOrderCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CancelOrder(decoded));
            }
            if let Ok(decoded) =
                <CancelOrderRFQCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CancelOrderRFQ(decoded));
            }
            if let Ok(decoded) =
                <CancelOrderRfqWithAdditionalMaskCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::CancelOrderRfqWithAdditionalMask(decoded));
            }
            if let Ok(decoded) =
                <CheckPredicateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckPredicate(decoded));
            }
            if let Ok(decoded) = <ClipperSwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClipperSwap(decoded));
            }
            if let Ok(decoded) = <ClipperSwapToCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ClipperSwapTo(decoded));
            }
            if let Ok(decoded) =
                <ClipperSwapToWithPermitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ClipperSwapToWithPermit(decoded));
            }
            if let Ok(decoded) = <DestroyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Destroy(decoded));
            }
            if let Ok(decoded) = <EqCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Eq(decoded));
            }
            if let Ok(decoded) = <FillOrderCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FillOrder(decoded));
            }
            if let Ok(decoded) = <FillOrderRFQCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FillOrderRFQ(decoded));
            }
            if let Ok(decoded) =
                <FillOrderRFQCompactCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FillOrderRFQCompact(decoded));
            }
            if let Ok(decoded) =
                <FillOrderRFQToCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FillOrderRFQTo(decoded));
            }
            if let Ok(decoded) =
                <FillOrderRFQToWithPermitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FillOrderRFQToWithPermit(decoded));
            }
            if let Ok(decoded) = <FillOrderToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FillOrderTo(decoded));
            }
            if let Ok(decoded) =
                <FillOrderToWithPermitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FillOrderToWithPermit(decoded));
            }
            if let Ok(decoded) = <GtCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Gt(decoded));
            }
            if let Ok(decoded) = <HashOrderCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HashOrder(decoded));
            }
            if let Ok(decoded) = <IncreaseNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncreaseNonce(decoded));
            }
            if let Ok(decoded) =
                <InvalidatorForOrderRFQCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidatorForOrderRFQ(decoded));
            }
            if let Ok(decoded) = <LtCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Lt(decoded));
            }
            if let Ok(decoded) = <NonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonce(decoded));
            }
            if let Ok(decoded) = <NonceEqualsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NonceEquals(decoded));
            }
            if let Ok(decoded) = <OrCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Or(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <RemainingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Remaining(decoded));
            }
            if let Ok(decoded) = <RemainingRawCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemainingRaw(decoded));
            }
            if let Ok(decoded) = <RemainingsRawCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemainingsRaw(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <RescueFundsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RescueFunds(decoded));
            }
            if let Ok(decoded) = <SimulateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Simulate(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded) =
                <TimestampBelowCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TimestampBelow(decoded));
            }
            if let Ok(decoded) =
                <TimestampBelowAndNonceEqualsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TimestampBelowAndNonceEquals(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UniswapV3SwapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UniswapV3Swap(decoded));
            }
            if let Ok(decoded) =
                <UniswapV3SwapCallbackCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            if let Ok(decoded) =
                <UniswapV3SwapToCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UniswapV3SwapTo(decoded));
            }
            if let Ok(decoded) =
                <UniswapV3SwapToWithPermitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UniswapV3SwapToWithPermit(decoded));
            }
            if let Ok(decoded) = <UnoswapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unoswap(decoded));
            }
            if let Ok(decoded) = <UnoswapToCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnoswapTo(decoded));
            }
            if let Ok(decoded) =
                <UnoswapToWithPermitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnoswapToWithPermit(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AggregationRouterV5Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AdvanceNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::And(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ArbitraryStaticCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CancelOrder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CancelOrderRFQ(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CancelOrderRfqWithAdditionalMask(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckPredicate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClipperSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClipperSwapTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClipperSwapToWithPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Destroy(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Eq(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FillOrder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FillOrderRFQ(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FillOrderRFQCompact(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FillOrderRFQTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FillOrderRFQToWithPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FillOrderTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FillOrderToWithPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Gt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HashOrder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncreaseNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidatorForOrderRFQ(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Lt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NonceEquals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Or(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Remaining(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemainingRaw(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemainingsRaw(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RescueFunds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Simulate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TimestampBelow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TimestampBelowAndNonceEquals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UniswapV3Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV3SwapTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UniswapV3SwapToWithPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unoswap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnoswapTo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnoswapToWithPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AggregationRouterV5Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdvanceNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::And(element) => ::core::fmt::Display::fmt(element, f),
                Self::ArbitraryStaticCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelOrderRFQ(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelOrderRfqWithAdditionalMask(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CheckPredicate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClipperSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClipperSwapTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClipperSwapToWithPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Destroy(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eq(element) => ::core::fmt::Display::fmt(element, f),
                Self::FillOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::FillOrderRFQ(element) => ::core::fmt::Display::fmt(element, f),
                Self::FillOrderRFQCompact(element) => ::core::fmt::Display::fmt(element, f),
                Self::FillOrderRFQTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::FillOrderRFQToWithPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::FillOrderTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::FillOrderToWithPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Gt(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidatorForOrderRFQ(element) => ::core::fmt::Display::fmt(element, f),
                Self::Lt(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::NonceEquals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Or(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Remaining(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemainingRaw(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemainingsRaw(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RescueFunds(element) => ::core::fmt::Display::fmt(element, f),
                Self::Simulate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::TimestampBelow(element) => ::core::fmt::Display::fmt(element, f),
                Self::TimestampBelowAndNonceEquals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapCallback(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapToWithPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unoswap(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnoswapTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnoswapToWithPermit(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdvanceNonceCall> for AggregationRouterV5Calls {
        fn from(value: AdvanceNonceCall) -> Self {
            Self::AdvanceNonce(value)
        }
    }
    impl ::core::convert::From<AndCall> for AggregationRouterV5Calls {
        fn from(value: AndCall) -> Self {
            Self::And(value)
        }
    }
    impl ::core::convert::From<ArbitraryStaticCallCall> for AggregationRouterV5Calls {
        fn from(value: ArbitraryStaticCallCall) -> Self {
            Self::ArbitraryStaticCall(value)
        }
    }
    impl ::core::convert::From<CancelOrderCall> for AggregationRouterV5Calls {
        fn from(value: CancelOrderCall) -> Self {
            Self::CancelOrder(value)
        }
    }
    impl ::core::convert::From<CancelOrderRFQCall> for AggregationRouterV5Calls {
        fn from(value: CancelOrderRFQCall) -> Self {
            Self::CancelOrderRFQ(value)
        }
    }
    impl ::core::convert::From<CancelOrderRfqWithAdditionalMaskCall> for AggregationRouterV5Calls {
        fn from(value: CancelOrderRfqWithAdditionalMaskCall) -> Self {
            Self::CancelOrderRfqWithAdditionalMask(value)
        }
    }
    impl ::core::convert::From<CheckPredicateCall> for AggregationRouterV5Calls {
        fn from(value: CheckPredicateCall) -> Self {
            Self::CheckPredicate(value)
        }
    }
    impl ::core::convert::From<ClipperSwapCall> for AggregationRouterV5Calls {
        fn from(value: ClipperSwapCall) -> Self {
            Self::ClipperSwap(value)
        }
    }
    impl ::core::convert::From<ClipperSwapToCall> for AggregationRouterV5Calls {
        fn from(value: ClipperSwapToCall) -> Self {
            Self::ClipperSwapTo(value)
        }
    }
    impl ::core::convert::From<ClipperSwapToWithPermitCall> for AggregationRouterV5Calls {
        fn from(value: ClipperSwapToWithPermitCall) -> Self {
            Self::ClipperSwapToWithPermit(value)
        }
    }
    impl ::core::convert::From<DestroyCall> for AggregationRouterV5Calls {
        fn from(value: DestroyCall) -> Self {
            Self::Destroy(value)
        }
    }
    impl ::core::convert::From<EqCall> for AggregationRouterV5Calls {
        fn from(value: EqCall) -> Self {
            Self::Eq(value)
        }
    }
    impl ::core::convert::From<FillOrderCall> for AggregationRouterV5Calls {
        fn from(value: FillOrderCall) -> Self {
            Self::FillOrder(value)
        }
    }
    impl ::core::convert::From<FillOrderRFQCall> for AggregationRouterV5Calls {
        fn from(value: FillOrderRFQCall) -> Self {
            Self::FillOrderRFQ(value)
        }
    }
    impl ::core::convert::From<FillOrderRFQCompactCall> for AggregationRouterV5Calls {
        fn from(value: FillOrderRFQCompactCall) -> Self {
            Self::FillOrderRFQCompact(value)
        }
    }
    impl ::core::convert::From<FillOrderRFQToCall> for AggregationRouterV5Calls {
        fn from(value: FillOrderRFQToCall) -> Self {
            Self::FillOrderRFQTo(value)
        }
    }
    impl ::core::convert::From<FillOrderRFQToWithPermitCall> for AggregationRouterV5Calls {
        fn from(value: FillOrderRFQToWithPermitCall) -> Self {
            Self::FillOrderRFQToWithPermit(value)
        }
    }
    impl ::core::convert::From<FillOrderToCall> for AggregationRouterV5Calls {
        fn from(value: FillOrderToCall) -> Self {
            Self::FillOrderTo(value)
        }
    }
    impl ::core::convert::From<FillOrderToWithPermitCall> for AggregationRouterV5Calls {
        fn from(value: FillOrderToWithPermitCall) -> Self {
            Self::FillOrderToWithPermit(value)
        }
    }
    impl ::core::convert::From<GtCall> for AggregationRouterV5Calls {
        fn from(value: GtCall) -> Self {
            Self::Gt(value)
        }
    }
    impl ::core::convert::From<HashOrderCall> for AggregationRouterV5Calls {
        fn from(value: HashOrderCall) -> Self {
            Self::HashOrder(value)
        }
    }
    impl ::core::convert::From<IncreaseNonceCall> for AggregationRouterV5Calls {
        fn from(value: IncreaseNonceCall) -> Self {
            Self::IncreaseNonce(value)
        }
    }
    impl ::core::convert::From<InvalidatorForOrderRFQCall> for AggregationRouterV5Calls {
        fn from(value: InvalidatorForOrderRFQCall) -> Self {
            Self::InvalidatorForOrderRFQ(value)
        }
    }
    impl ::core::convert::From<LtCall> for AggregationRouterV5Calls {
        fn from(value: LtCall) -> Self {
            Self::Lt(value)
        }
    }
    impl ::core::convert::From<NonceCall> for AggregationRouterV5Calls {
        fn from(value: NonceCall) -> Self {
            Self::Nonce(value)
        }
    }
    impl ::core::convert::From<NonceEqualsCall> for AggregationRouterV5Calls {
        fn from(value: NonceEqualsCall) -> Self {
            Self::NonceEquals(value)
        }
    }
    impl ::core::convert::From<OrCall> for AggregationRouterV5Calls {
        fn from(value: OrCall) -> Self {
            Self::Or(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for AggregationRouterV5Calls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RemainingCall> for AggregationRouterV5Calls {
        fn from(value: RemainingCall) -> Self {
            Self::Remaining(value)
        }
    }
    impl ::core::convert::From<RemainingRawCall> for AggregationRouterV5Calls {
        fn from(value: RemainingRawCall) -> Self {
            Self::RemainingRaw(value)
        }
    }
    impl ::core::convert::From<RemainingsRawCall> for AggregationRouterV5Calls {
        fn from(value: RemainingsRawCall) -> Self {
            Self::RemainingsRaw(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for AggregationRouterV5Calls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RescueFundsCall> for AggregationRouterV5Calls {
        fn from(value: RescueFundsCall) -> Self {
            Self::RescueFunds(value)
        }
    }
    impl ::core::convert::From<SimulateCall> for AggregationRouterV5Calls {
        fn from(value: SimulateCall) -> Self {
            Self::Simulate(value)
        }
    }
    impl ::core::convert::From<SwapCall> for AggregationRouterV5Calls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<TimestampBelowCall> for AggregationRouterV5Calls {
        fn from(value: TimestampBelowCall) -> Self {
            Self::TimestampBelow(value)
        }
    }
    impl ::core::convert::From<TimestampBelowAndNonceEqualsCall> for AggregationRouterV5Calls {
        fn from(value: TimestampBelowAndNonceEqualsCall) -> Self {
            Self::TimestampBelowAndNonceEquals(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for AggregationRouterV5Calls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCall> for AggregationRouterV5Calls {
        fn from(value: UniswapV3SwapCall) -> Self {
            Self::UniswapV3Swap(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for AggregationRouterV5Calls {
        fn from(value: UniswapV3SwapCallbackCall) -> Self {
            Self::UniswapV3SwapCallback(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapToCall> for AggregationRouterV5Calls {
        fn from(value: UniswapV3SwapToCall) -> Self {
            Self::UniswapV3SwapTo(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapToWithPermitCall> for AggregationRouterV5Calls {
        fn from(value: UniswapV3SwapToWithPermitCall) -> Self {
            Self::UniswapV3SwapToWithPermit(value)
        }
    }
    impl ::core::convert::From<UnoswapCall> for AggregationRouterV5Calls {
        fn from(value: UnoswapCall) -> Self {
            Self::Unoswap(value)
        }
    }
    impl ::core::convert::From<UnoswapToCall> for AggregationRouterV5Calls {
        fn from(value: UnoswapToCall) -> Self {
            Self::UnoswapTo(value)
        }
    }
    impl ::core::convert::From<UnoswapToWithPermitCall> for AggregationRouterV5Calls {
        fn from(value: UnoswapToWithPermitCall) -> Self {
            Self::UnoswapToWithPermit(value)
        }
    }
    ///Container type for all return fields from the `and` function with signature `and(uint256,bytes)` and selector `0xbfa75143`
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
    pub struct AndReturn(pub bool);
    ///Container type for all return fields from the `arbitraryStaticCall` function with signature `arbitraryStaticCall(address,bytes)` and selector `0xbf15fcd8`
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
    pub struct ArbitraryStaticCallReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `cancelOrder` function with signature `cancelOrder((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes))` and selector `0x2d9a56f6`
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
    pub struct CancelOrderReturn {
        pub order_remaining: ::ethers::core::types::U256,
        pub order_hash: [u8; 32],
    }
    ///Container type for all return fields from the `checkPredicate` function with signature `checkPredicate((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes))` and selector `0x6c838250`
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
    pub struct CheckPredicateReturn(pub bool);
    ///Container type for all return fields from the `clipperSwap` function with signature `clipperSwap(address,address,address,uint256,uint256,uint256,bytes32,bytes32)` and selector `0x84bd6d29`
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
    pub struct ClipperSwapReturn {
        pub return_amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `clipperSwapTo` function with signature `clipperSwapTo(address,address,address,address,uint256,uint256,uint256,bytes32,bytes32)` and selector `0x093d4fa5`
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
    pub struct ClipperSwapToReturn {
        pub return_amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `clipperSwapToWithPermit` function with signature `clipperSwapToWithPermit(address,address,address,address,uint256,uint256,uint256,bytes32,bytes32,bytes)` and selector `0xc805a666`
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
    pub struct ClipperSwapToWithPermitReturn {
        pub return_amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `eq` function with signature `eq(uint256,bytes)` and selector `0x6fe7b0ba`
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
    pub struct EqReturn(pub bool);
    ///Container type for all return fields from the `fillOrder` function with signature `fillOrder((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes),bytes,bytes,uint256,uint256,uint256)` and selector `0x62e238bb`
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
    pub struct FillOrderReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub [u8; 32],
    );
    ///Container type for all return fields from the `fillOrderRFQ` function with signature `fillOrderRFQ((uint256,address,address,address,address,uint256,uint256),bytes,uint256)` and selector `0x3eca9c0a`
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
    pub struct FillOrderRFQReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub [u8; 32],
    );
    ///Container type for all return fields from the `fillOrderRFQCompact` function with signature `fillOrderRFQCompact((uint256,address,address,address,address,uint256,uint256),bytes32,bytes32,uint256)` and selector `0x9570eeee`
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
    pub struct FillOrderRFQCompactReturn {
        pub filled_making_amount: ::ethers::core::types::U256,
        pub filled_taking_amount: ::ethers::core::types::U256,
        pub order_hash: [u8; 32],
    }
    ///Container type for all return fields from the `fillOrderRFQTo` function with signature `fillOrderRFQTo((uint256,address,address,address,address,uint256,uint256),bytes,uint256,address)` and selector `0x5a099843`
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
    pub struct FillOrderRFQToReturn {
        pub filled_making_amount: ::ethers::core::types::U256,
        pub filled_taking_amount: ::ethers::core::types::U256,
        pub order_hash: [u8; 32],
    }
    ///Container type for all return fields from the `fillOrderRFQToWithPermit` function with signature `fillOrderRFQToWithPermit((uint256,address,address,address,address,uint256,uint256),bytes,uint256,address,bytes)` and selector `0x70ccbd31`
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
    pub struct FillOrderRFQToWithPermitReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub [u8; 32],
    );
    ///Container type for all return fields from the `fillOrderTo` function with signature `fillOrderTo((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes),bytes,bytes,uint256,uint256,uint256,address)` and selector `0xe5d7bde6`
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
    pub struct FillOrderToReturn {
        pub actual_making_amount: ::ethers::core::types::U256,
        pub actual_taking_amount: ::ethers::core::types::U256,
        pub order_hash: [u8; 32],
    }
    ///Container type for all return fields from the `fillOrderToWithPermit` function with signature `fillOrderToWithPermit((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes),bytes,bytes,uint256,uint256,uint256,address,bytes)` and selector `0xd365c695`
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
    pub struct FillOrderToWithPermitReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub [u8; 32],
    );
    ///Container type for all return fields from the `gt` function with signature `gt(uint256,bytes)` and selector `0x4f38e2b8`
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
    pub struct GtReturn(pub bool);
    ///Container type for all return fields from the `hashOrder` function with signature `hashOrder((uint256,address,address,address,address,address,uint256,uint256,uint256,bytes))` and selector `0x37e7316f`
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
    pub struct HashOrderReturn(pub [u8; 32]);
    ///Container type for all return fields from the `invalidatorForOrderRFQ` function with signature `invalidatorForOrderRFQ(address,uint256)` and selector `0x56f16124`
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
    pub struct InvalidatorForOrderRFQReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lt` function with signature `lt(uint256,bytes)` and selector `0xca4ece22`
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
    pub struct LtReturn(pub bool);
    ///Container type for all return fields from the `nonce` function with signature `nonce(address)` and selector `0x70ae92d2`
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
    pub struct NonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `nonceEquals` function with signature `nonceEquals(address,uint256)` and selector `0xcf6fc6e3`
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
    pub struct NonceEqualsReturn(pub bool);
    ///Container type for all return fields from the `or` function with signature `or(uint256,bytes)` and selector `0x74261145`
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
    pub struct OrReturn(pub bool);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `remaining` function with signature `remaining(bytes32)` and selector `0xbc1ed74c`
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
    pub struct RemainingReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `remainingRaw` function with signature `remainingRaw(bytes32)` and selector `0x7e54f092`
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
    pub struct RemainingRawReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `remainingsRaw` function with signature `remainingsRaw(bytes32[])` and selector `0x942461bb`
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
    pub struct RemainingsRawReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `swap` function with signature `swap(address,(address,address,address,address,uint256,uint256,uint256),bytes,bytes)` and selector `0x12aa3caf`
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
        pub return_amount: ::ethers::core::types::U256,
        pub spent_amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `timestampBelow` function with signature `timestampBelow(uint256)` and selector `0x63592c2b`
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
    pub struct TimestampBelowReturn(pub bool);
    ///Container type for all return fields from the `timestampBelowAndNonceEquals` function with signature `timestampBelowAndNonceEquals(uint256)` and selector `0x2cc2878d`
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
    pub struct TimestampBelowAndNonceEqualsReturn(pub bool);
    ///Container type for all return fields from the `uniswapV3Swap` function with signature `uniswapV3Swap(uint256,uint256,uint256[])` and selector `0xe449022e`
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
    pub struct UniswapV3SwapReturn {
        pub return_amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `uniswapV3SwapTo` function with signature `uniswapV3SwapTo(address,uint256,uint256,uint256[])` and selector `0xbc80f1a8`
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
    pub struct UniswapV3SwapToReturn {
        pub return_amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `uniswapV3SwapToWithPermit` function with signature `uniswapV3SwapToWithPermit(address,address,uint256,uint256,uint256[],bytes)` and selector `0x2521b930`
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
    pub struct UniswapV3SwapToWithPermitReturn {
        pub return_amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `unoswap` function with signature `unoswap(address,uint256,uint256,uint256[])` and selector `0x0502b1c5`
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
    pub struct UnoswapReturn {
        pub return_amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `unoswapTo` function with signature `unoswapTo(address,address,uint256,uint256,uint256[])` and selector `0xf78dc253`
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
    pub struct UnoswapToReturn {
        pub return_amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `unoswapToWithPermit` function with signature `unoswapToWithPermit(address,address,uint256,uint256,uint256[],bytes)` and selector `0x3c15fd91`
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
    pub struct UnoswapToWithPermitReturn {
        pub return_amount: ::ethers::core::types::U256,
    }
    ///`SwapDescription(address,address,address,address,uint256,uint256,uint256)`
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
    pub struct SwapDescription {
        pub src_token: ::ethers::core::types::Address,
        pub dst_token: ::ethers::core::types::Address,
        pub src_receiver: ::ethers::core::types::Address,
        pub dst_receiver: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub min_return_amount: ::ethers::core::types::U256,
        pub flags: ::ethers::core::types::U256,
    }
    ///`Order(uint256,address,address,address,address,address,uint256,uint256,uint256,bytes)`
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
    pub struct Order {
        pub salt: ::ethers::core::types::U256,
        pub maker_asset: ::ethers::core::types::Address,
        pub taker_asset: ::ethers::core::types::Address,
        pub maker: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub allowed_sender: ::ethers::core::types::Address,
        pub making_amount: ::ethers::core::types::U256,
        pub taking_amount: ::ethers::core::types::U256,
        pub offsets: ::ethers::core::types::U256,
        pub interactions: ::ethers::core::types::Bytes,
    }
    ///`OrderRFQ(uint256,address,address,address,address,uint256,uint256)`
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
    pub struct OrderRFQ {
        pub info: ::ethers::core::types::U256,
        pub maker_asset: ::ethers::core::types::Address,
        pub taker_asset: ::ethers::core::types::Address,
        pub maker: ::ethers::core::types::Address,
        pub allowed_sender: ::ethers::core::types::Address,
        pub making_amount: ::ethers::core::types::U256,
        pub taking_amount: ::ethers::core::types::U256,
    }
}
