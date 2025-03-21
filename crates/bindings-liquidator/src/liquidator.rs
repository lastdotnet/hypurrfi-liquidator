pub use liquidator::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod liquidator {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addLiquidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addLiquidator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_liquidator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approvePool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approvePool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hook"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hook"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Out"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Out"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isLiquidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isLiquidator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtToCover"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapPath"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapVenue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("collateralGain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pool"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recover"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeLiquidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeLiquidator"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_liquidator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("uniswapV3SwapCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "uniswapV3SwapCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Test"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Test"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LIQUIDATOR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0EW__\xFD[P_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x82U`@Q\x90\x91\x82\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3Pa!\xF5\x80a\0\\_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xA6W_5`\xE0\x1C\x80cW\x05\xAEC\x11a\0nW\x80cW\x05\xAEC\x14a\x01]W\x80c\x8D\xA5\xCB[\x14a\x01pW\x80c\x9A{\xFFy\x14a\x01\x82W\x80c\xEF\xB7D\0\x14a\x01\x95W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xA8W\x80c\xFAF\x1E3\x14a\x01\xBBW__\xFD[\x80c\x16\xF0\x11[\x14a\0\xAAW\x80c.C\xC9a\x14a\0\xE2W\x80cBL&[\x14a\x01\x03W\x80cLN\x7Fo\x14a\x01\x18W\x80cR\x9A5o\x14a\x01+W[__\xFD[a\0\xC5s2F{C\xBF\xA6rs\xFC}\xDD\xA0\x99\x9E\xE9\xA1/*\xAA\x08\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xF5a\0\xF06`\x04a\x1B}V[a\x01\xCEV[`@Q\x90\x81R` \x01a\0\xD9V[a\x01\x16a\x01\x116`\x04a\x1C,V[a\x05lV[\0[a\x01\x16a\x01&6`\x04a\x1C,V[a\x06\x1BV[a\x01Ma\x0196`\x04a\x1C,V[`\x02` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xD9V[a\x01\x16a\x01k6`\x04a\x1CGV[a\x06gV[_Ta\0\xC5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x16a\x01\x906`\x04a\x1CqV[a\x07;V[a\x01\x16a\x01\xA36`\x04a\x1C,V[a\t\x05V[a\x01\x16a\x01\xB66`\x04a\x1C,V[a\tNV[a\x01\x16a\x01\xC96`\x04a\x1C\xD6V[a\t\xC1V[_\x80T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x01\xF5WP3_\x90\x81R`\x02` R`@\x90 T`\xFF\x16[a\x02^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FOnly owner or liquidator can cal`D\x82\x01Rn6\x10:44\xB9\x903:\xB71\xBA4\xB7\xB7`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xA2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xC6\x91\x90a\x1D%V[`@Qi\x06\xB6\x97GFV\xE77v\x17`\xB4\x1B` \x82\x01R\x90\x91P`*\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84\x84`@Q` \x01a\x03\x0B\x92\x91\x90a\x1D<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\x03\xBFWa\x03\xBA\x87`@Q\x80`\xC0\x01`@R\x80\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8A\x81R` \x01_\x81RPa\x0B\x16V[a\x04\xECV[`@Qh\x06\x87\x97\x06W'7v\x17`\xBC\x1B` \x82\x01R`)\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84\x84`@Q` \x01a\x04\0\x92\x91\x90a\x1D<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\x04\xAFWa\x03\xBA\x87`@Q\x80`\xC0\x01`@R\x80\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8A\x81R` \x01_\x81RPa\r'V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqInvalid swap venue`p\x1B`D\x82\x01R`d\x01a\x02UV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x050W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05T\x91\x90a\x1D%V[a\x05^\x91\x90a\x1D_V[\x9A\x99PPPPPPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02U\x90a\x1D\x85V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rs2F{C\xBF\xA6rs\xFC}\xDD\xA0\x99\x9E\xE9\xA1/*\xAA\x08`\x04\x82\x01R_\x19`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x05\xF3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x17\x91\x90a\x1D\xBAV[PPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02U\x90a\x1D\x85V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x02` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02U\x90a\x1D\x85V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x06\xCDW`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x06\xC8W=__>=_\xFD[PPPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07\x17W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC8\x91\x90a\x1D\xBAV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fmsg.sender != activeKittenPair\0\0`D\x82\x01R`d\x01a\x02UV[_a\x07\xA2\x82\x84\x01\x84a\x1EAV[\x90P___a\x07\xB3\x84_\x01Qa\x0E\xB5V[` \x87\x01Q`@\x80\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q\x92Qb\xA7\x18\xA9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x91\x84\x16`$\x83\x01R\x92\x90\x92\x16`D\x83\x01R`d\x82\x01R_`\x84\x82\x01R\x92\x95P\x90\x93P\x91Ps2F{C\xBF\xA6rs\xFC}\xDD\xA0\x99\x9E\xE9\xA1/*\xAA\x08\x90b\xA7\x18\xA9\x90`\xA4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08<W__\xFD[PZ\xF1\x15\x80\x15a\x08NW=__>=_\xFD[PPPPa\x08^\x84_\x01Qa\x0E\xF4V[\x15a\x08\x7FW\x83Qa\x08n\x90a\x0F-V[\x84R`\xA0\x84\x01Qa\x08\x7F\x90\x85a\x0B\x16V[`\xA0\x84\x01Q`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x91\x92P\x82\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08\xD5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xF9\x91\x90a\x1D\xBAV[PPPPPPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02U\x90a\x1D\x85V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x02` R`@\x90 \x80T`\xFF\x19\x16\x90UV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\twW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02U\x90a\x1D\x85V[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[_a\t\xCE\x82\x84\x01\x84a\x1EAV[\x90P___a\t\xDF\x84_\x01Qa\x0E\xB5V[\x92P\x92P\x92Pa\n\rs\"\xA9\xB8*l=+\xFBh\xF3$\xB2\xE86\x7F4m\xD6\xF3*a\n\x08\x85\x85\x85a\x0FdV[a\x0F\xCEV[` \x84\x01Q`@\x80\x86\x01Q``\x87\x01Q`\x80\x88\x01Q\x92Qb\xA7\x18\xA9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x91\x84\x16`$\x83\x01R\x92\x90\x92\x16`D\x83\x01R`d\x82\x01R_`\x84\x82\x01Rs2F{C\xBF\xA6rs\xFC}\xDD\xA0\x99\x9E\xE9\xA1/*\xAA\x08\x90b\xA7\x18\xA9\x90`\xA4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n\x8EW__\xFD[PZ\xF1\x15\x80\x15a\n\xA0W=__>=_\xFD[PPPP__\x89\x13a\n\xB2W\x87a\n\xB4V[\x88[\x90Pa\n\xC2\x85_\x01Qa\x0E\xF4V[\x15a\n\xDEW\x84Qa\n\xD2\x90a\x0F-V[\x85Ra\n\xDE\x81\x86a\r'V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x92\x93P\x83\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01a\x08\xB9V[___a\x0B%\x84_\x01Qa\x0E\xB5V[`@Qc\x06\x80\x1C\xC3`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01\x81\x90R\x90\x85\x16`$\x83\x01\x81\x90R`\x01`D\x84\x01R\x94\x97P\x92\x95P\x90\x93P\x91\x11\x15\x90s\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x90ch\x01\xCC0\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x9CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xC0\x91\x90a\x1FPV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90Ua\x0C!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr$\xB7;0\xB64\xB2\x105\xB4\xBA:2\xB7\x1080\xB4\xB9`i\x1B`D\x82\x01R`d\x01a\x02UV[`@\x80Q`\x80\x80\x82\x01\x83R`\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83Rs\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B` \x84\x01R\x90\x88\x01Q\x92\x82\x01\x92\x90\x92R\x90\x84\x16``\x82\x01Ra\x0Cr\x90a\x10\"V[`\xA0\x86\x01R`\x01T`\x01`\x01`\xA0\x1B\x03\x16c\x02,\r\x9F\x82a\x0C\x93W\x87a\x0C\x95V[_[\x83a\x0C\xA0W_a\x0C\xA2V[\x88[0\x89`@Q` \x01a\x0C\xB4\x91\x90a\x1F\x99V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xE2\x94\x93\x92\x91\x90a \nV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\xF9W__\xFD[PZ\xF1\x15\x80\x15a\r\x0BW=__>=_\xFD[PP`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UPPPPPPPPV[___a\r6\x84_\x01Qa\x0E\xB5V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x83\x16\x10_a\rus\"\xA9\xB8*l=+\xFBh\xF3$\xB2\xE86\x7F4m\xD6\xF3*a\rp\x86\x88\x87a\x0FdV[a\x11\xA3V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xB3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xD7\x91\x90a\x1FPV[P`\x01`\x01`\xA0\x1B\x03\x81\x16c\x12\x8A\xCB\x080\x84a\r\xF2\x8Ba 6V[\x86a\x0E\x1BWa\x0E\x16`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a PV[a\x0E+V[a\x0E+d\x01\0\x02v\xA3`\x01a oV[\x8B`@Q` \x01a\x0E<\x91\x90a\x1F\x99V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Ek\x95\x94\x93\x92\x91\x90a \x8EV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E\x86W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xAA\x91\x90a \xD3V[PPPPPPPPPV[_\x80\x80a\x0E\xC2\x84\x82a\x12\x89V[\x92Pa\x0E\xCF\x84`\x14a\x12\xEDV[a\xFF\xFF\x16\x90Pa\x0E\xEBa\x0E\xE4`\x03`\x14a \xF5V[\x85\x90a\x12\x89V[\x91P\x91\x93\x90\x92PV[_a\x0F\x01`\x03`\x14a \xF5V[`\x14a\x0F\x0E`\x03\x82a \xF5V[a\x0F\x18\x91\x90a \xF5V[a\x0F\"\x91\x90a \xF5V[\x82Q\x10\x15\x90P\x91\x90PV[``a\x0F^a\x0F>`\x03`\x14a \xF5V[a\x0FJ`\x03`\x14a \xF5V[\x84Qa\x0FV\x91\x90a!\x08V[\x84\x91\x90a\x13\x97V[\x92\x91PPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x0F\x9EW\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[_a\x0F\xD9\x83\x83a\x11\xA3V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x06\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x1A[\x9D\x98[\x1AY\x08\x1C\x1B\xDB\xDB`\xA2\x1B`D\x82\x01R`d\x01a\x02UV[`@\x80Q`\xE0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R\x82_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c9/7\xE9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x98W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xBC\x91\x90a!\x1BV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x89\x01R\x16`\xA0\x87\x01R\x15\x15`\x80\x86\x01R``\x85\x01R`@\x84\x01R` \x83\x01R\x81R_a\x10\xF5\x84\x83a\x14\xA3V[` \x85\x01Q\x85Q`\x80\x85\x01Q`@Qc\xCCV\xB2\xC5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x15\x15`$\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xCCV\xB2\xC5\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11RW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11v\x91\x90a\x1D%V[\x90Pa\x11\x84\x81a'\x10a!\x08V[a\x11\x90\x83a'\x10a!\x89V[a\x11\x9A\x91\x90a!\xA0V[\x95\x94PPPPPV[_\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x11\xC8W__\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\x1E\xEDC\xDC\xAA.\xFD\xE0g.\xB5qd\x92\0\xA2\x927\xB7\x95\x8E{\x0F\xBDR\xF7_\xA3[~\xC5,`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[_a\x12\x95\x82`\x14a \xF5V[\x83Q\x10\x15a\x12\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R`d\x01a\x02UV[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[_\x81a\x12\xFA\x81`\x03a \xF5V[\x10\x15a\x13<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R`d\x01a\x02UV[a\x13G\x82`\x03a \xF5V[\x83Q\x10\x15a\x13\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R`d\x01a\x02UV[P\x01`\x03\x01Q\x90V[``\x81a\x13\xA5\x81`\x1Fa \xF5V[\x10\x15a\x13\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\x02UV[a\x13\xEE\x82\x84a \xF5V[\x84Q\x10\x15a\x142W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R`d\x01a\x02UV[``\x82\x15\x80\x15a\x14PW`@Q\x91P_\x82R` \x82\x01`@Ra\x14\x9AV[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a\x14\x89W\x80Q\x83R` \x92\x83\x01\x92\x01a\x14qV[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[_\x81`\x80\x01Q\x15a\x16\xBAW_a\x14\xF3`@Q\x80`\xA0\x01`@R\x80\x85`@\x01Q\x81R` \x01\x85``\x01Q\x81R` \x01\x85`\x80\x01Q\x15\x15\x81R` \x01\x85_\x01Q\x81R` \x01\x85` \x01Q\x81RPa\x17.V[\x83Q`@\x85\x01Q\x91\x92P\x90a\x15\x10\x90g\r\xE0\xB6\xB3\xA7d\0\0a!\x89V[a\x15\x1A\x91\x90a!\xA0V[`@\x84\x01R` \x83\x01Q``\x84\x01Qa\x15;\x90g\r\xE0\xB6\xB3\xA7d\0\0a!\x89V[a\x15E\x91\x90a!\xA0V[\x83``\x01\x81\x81RPP__\x84`\xC0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15\x80W\x84``\x01Q\x85`@\x01Qa\x15\x8BV[\x84`@\x01Q\x85``\x01Q[\x91P\x91P\x84`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15\xDBW` \x85\x01Q`@\x87\x01Qa\x15\xCC\x90g\r\xE0\xB6\xB3\xA7d\0\0a!\x89V[a\x15\xD6\x91\x90a!\xA0V[a\x15\xFEV[\x84Q`@\x87\x01Qa\x15\xF4\x90g\r\xE0\xB6\xB3\xA7d\0\0a!\x89V[a\x15\xFE\x91\x90a!\xA0V[`@\x87\x01\x81\x90R_\x90a\x16\x11\x90\x83a!\x08V[\x90P_\x83a\x16W`@Q\x80`\xC0\x01`@R\x80\x85\x81R` \x01\x88\x81R` \x01\x87\x81R` \x01\x8A`\x80\x01Q\x15\x15\x81R` \x01\x8A_\x01Q\x81R` \x01\x8A` \x01Q\x81RPa\x18/V[a\x16a\x91\x90a!\x08V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x87`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x89``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\x94W\x87Qa\x16\x9AV[\x87` \x01Q[a\x16\xA4\x90\x83a!\x89V[a\x16\xAE\x91\x90a!\xA0V[\x95PPPPPPa\x0F^V[__\x83`\xC0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x85``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\xECW\x83``\x01Q\x84`@\x01Qa\x16\xF7V[\x83`@\x01Q\x84``\x01Q[\x91P\x91P\x84`@\x01Q\x81a\x17\x0B\x91\x90a!\x08V[\x82\x86`@\x01Qa\x17\x1B\x91\x90a!\x89V[a\x17%\x91\x90a!\xA0V[\x92PPPa\x0F^V[_\x81`@\x01Q\x15a\x18\x19W``\x82\x01Q\x82Q_\x91\x90a\x17U\x90g\r\xE0\xB6\xB3\xA7d\0\0a!\x89V[a\x17_\x91\x90a!\xA0V[\x90P_\x83`\x80\x01Q\x84` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x17\x7F\x91\x90a!\x89V[a\x17\x89\x91\x90a!\xA0V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a\x17\x9F\x83\x85a!\x89V[a\x17\xA9\x91\x90a!\xA0V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a\x17\xBF\x84\x80a!\x89V[a\x17\xC9\x91\x90a!\xA0V[g\r\xE0\xB6\xB3\xA7d\0\0a\x17\xDC\x86\x80a!\x89V[a\x17\xE6\x91\x90a!\xA0V[a\x17\xF0\x91\x90a \xF5V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a\x18\x05\x82\x84a!\x89V[a\x18\x0F\x91\x90a!\xA0V[\x96\x95PPPPPPV[` \x82\x01Q\x82Qa\x0F^\x91\x90a!\x89V[\x91\x90PV[_\x80[`\xFF\x81\x10\x15a\x19\xEBW_a\x18M\x84`@\x01Q\x85_\x01Qa\x1A\x19V[\x90P\x83` \x01Q\x81\x10\x15a\x19BW_a\x18m\x85`@\x01Q\x86_\x01Qa\x1A\x94V[\x82\x86` \x01Qa\x18}\x91\x90a!\x08V[a\x18\x8F\x90g\r\xE0\xB6\xB3\xA7d\0\0a!\x89V[a\x18\x99\x91\x90a!\xA0V[\x90P\x80_\x03a\x19'W\x84` \x01Q\x82\x03a\x18\xB8WPPPP`@\x01Q\x90V[\x84` \x01Qa\x19\r`@Q\x80`\xA0\x01`@R\x80\x88`@\x01Q`\x01a\x18\xDC\x91\x90a \xF5V[\x81R` \x01\x88_\x01Q\x81R` \x01\x88``\x01Q\x15\x15\x81R` \x01\x88`\x80\x01Q\x81R` \x01\x88`\xA0\x01Q\x81RPa\x17.V[\x11\x15a\x19#W`@\x85\x01Qa\x11\x9A\x90`\x01a \xF5V[P`\x01[\x80\x85`@\x01Qa\x197\x91\x90a \xF5V[`@\x86\x01RPa\x19\xE2V[_a\x19T\x85`@\x01Q\x86_\x01Qa\x1A\x94V[` \x86\x01Qa\x19c\x90\x84a!\x08V[a\x19u\x90g\r\xE0\xB6\xB3\xA7d\0\0a!\x89V[a\x19\x7F\x91\x90a!\xA0V[\x90P\x80_\x03a\x19\xCBW\x84` \x01Q\x82\x14\x80a\x19\xB7WP\x84` \x01Qa\x19\xB5`\x01\x87`@\x01Qa\x19\xAE\x91\x90a!\x08V[\x87Qa\x1A\x19V[\x10[\x15a\x19\xC7WPPPP`@\x01Q\x90V[P`\x01[\x80\x85`@\x01Qa\x19\xDB\x91\x90a!\x08V[`@\x86\x01RP[P`\x01\x01a\x182V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x04/`\xF3\x1B`D\x82\x01R`d\x01a\x02UV[_\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x1A.\x84\x86a!\x89V[a\x1A8\x91\x90a!\xA0V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a\x1AN\x85\x80a!\x89V[a\x1AX\x91\x90a!\xA0V[g\r\xE0\xB6\xB3\xA7d\0\0a\x1Ak\x87\x80a!\x89V[a\x1Au\x91\x90a!\xA0V[a\x1A\x7F\x91\x90a \xF5V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a\x11\x90\x82\x84a!\x89V[_g\r\xE0\xB6\xB3\xA7d\0\0\x83\x81a\x1A\xAA\x82\x80a!\x89V[a\x1A\xB4\x91\x90a!\xA0V[a\x1A\xBE\x91\x90a!\x89V[a\x1A\xC8\x91\x90a!\xA0V[g\r\xE0\xB6\xB3\xA7d\0\0\x80a\x1A\xDC\x85\x80a!\x89V[a\x1A\xE6\x91\x90a!\xA0V[a\x1A\xF1\x86`\x03a!\x89V[a\x1A\xFB\x91\x90a!\x89V[a\x1B\x05\x91\x90a!\xA0V[a\x1B\x0F\x91\x90a \xF5V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B*W__\xFD[PV[\x805a\x18*\x81a\x1B\x16V[__\x83`\x1F\x84\x01\x12a\x1BHW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B_W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1BvW__\xFD[\x92P\x92\x90PV[________`\xC0\x89\x8B\x03\x12\x15a\x1B\x94W__\xFD[\x885a\x1B\x9F\x81a\x1B\x16V[\x97P` \x89\x015a\x1B\xAF\x81a\x1B\x16V[\x96P`@\x89\x015a\x1B\xBF\x81a\x1B\x16V[\x95P``\x89\x015\x94P`\x80\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\xE1W__\xFD[a\x1B\xED\x8B\x82\x8C\x01a\x1B8V[\x90\x95P\x93PP`\xA0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\x0CW__\xFD[a\x1C\x18\x8B\x82\x8C\x01a\x1B8V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[_` \x82\x84\x03\x12\x15a\x1C<W__\xFD[\x815a\x1B\x0F\x81a\x1B\x16V[__`@\x83\x85\x03\x12\x15a\x1CXW__\xFD[\x825a\x1Cc\x81a\x1B\x16V[\x94` \x93\x90\x93\x015\x93PPPV[_____`\x80\x86\x88\x03\x12\x15a\x1C\x85W__\xFD[\x855a\x1C\x90\x81a\x1B\x16V[\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\xB9W__\xFD[a\x1C\xC5\x88\x82\x89\x01a\x1B8V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[____``\x85\x87\x03\x12\x15a\x1C\xE9W__\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\rW__\xFD[a\x1D\x19\x87\x82\x88\x01a\x1B8V[\x95\x98\x94\x97P\x95PPPPV[_` \x82\x84\x03\x12\x15a\x1D5W__\xFD[PQ\x91\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1D~Wa\x1D~a\x1DKV[P\x92\x91PPV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[\x80Q\x80\x15\x15\x81\x14a\x18*W__\xFD[_` \x82\x84\x03\x12\x15a\x1D\xCAW__\xFD[a\x1B\x0F\x82a\x1D\xABV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E\nWa\x1E\na\x1D\xD3V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E9Wa\x1E9a\x1D\xD3V[`@R\x91\x90PV[_` \x82\x84\x03\x12\x15a\x1EQW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1EgW__\xFD[\x82\x01`\xC0\x81\x85\x03\x12\x15a\x1ExW__\xFD[a\x1E\x80a\x1D\xE7V[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\x96W__\xFD[\x82\x01`\x1F\x81\x01\x86\x13a\x1E\xA6W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xC0Wa\x1E\xC0a\x1D\xD3V[a\x1E\xD3`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1E\x10V[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15a\x1E\xE7W__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x84RPPPa\x1F\x0B` \x83\x01a\x1B-V[` \x82\x01Ra\x1F\x1C`@\x83\x01a\x1B-V[`@\x82\x01Ra\x1F-``\x83\x01a\x1B-V[``\x82\x01R`\x80\x82\x81\x015\x90\x82\x01R`\xA0\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x1F`W__\xFD[\x81Qa\x1B\x0F\x81a\x1B\x16V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_\x82Q`\xC0` \x84\x01Ra\x1F\xB4`\xE0\x84\x01\x82a\x1FkV[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`\x01\x80`\xA0\x1B\x03`@\x85\x01Q\x16``\x84\x01R`\x01\x80`\xA0\x1B\x03``\x85\x01Q\x16`\x80\x84\x01R`\x80\x84\x01Q`\xA0\x84\x01R`\xA0\x84\x01Q`\xC0\x84\x01R\x80\x91PP\x92\x91PPV[\x84\x81R\x83` \x82\x01R`\x01\x80`\xA0\x1B\x03\x83\x16`@\x82\x01R`\x80``\x82\x01R_a\x18\x0F`\x80\x83\x01\x84a\x1FkV[_`\x01`\xFF\x1B\x82\x01a JWa Ja\x1DKV[P_\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0F^Wa\x0F^a\x1DKV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0F^Wa\x0F^a\x1DKV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R_\x90a \xC8\x90\x83\x01\x84a\x1FkV[\x97\x96PPPPPPPV[__`@\x83\x85\x03\x12\x15a \xE4W__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[\x80\x82\x01\x80\x82\x11\x15a\x0F^Wa\x0F^a\x1DKV[\x81\x81\x03\x81\x81\x11\x15a\x0F^Wa\x0F^a\x1DKV[_______`\xE0\x88\x8A\x03\x12\x15a!1W__\xFD[\x87Q` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q\x92\x99P\x90\x97P\x95P\x93Pa!X`\x80\x89\x01a\x1D\xABV[\x92P`\xA0\x88\x01Qa!h\x81a\x1B\x16V[`\xC0\x89\x01Q\x90\x92Pa!y\x81a\x1B\x16V[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0F^Wa\x0F^a\x1DKV[_\x82a!\xBAWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x9Bd\xBF\xE53\xD2l\xB8nJU\xA8Z\x9C\xC9\xD0\xE89\x13\xDA\xA2u\">\xED\xCA\xCC\x1F\xBD\xF1\xBA5dsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static LIQUIDATOR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xA6W_5`\xE0\x1C\x80cW\x05\xAEC\x11a\0nW\x80cW\x05\xAEC\x14a\x01]W\x80c\x8D\xA5\xCB[\x14a\x01pW\x80c\x9A{\xFFy\x14a\x01\x82W\x80c\xEF\xB7D\0\x14a\x01\x95W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xA8W\x80c\xFAF\x1E3\x14a\x01\xBBW__\xFD[\x80c\x16\xF0\x11[\x14a\0\xAAW\x80c.C\xC9a\x14a\0\xE2W\x80cBL&[\x14a\x01\x03W\x80cLN\x7Fo\x14a\x01\x18W\x80cR\x9A5o\x14a\x01+W[__\xFD[a\0\xC5s2F{C\xBF\xA6rs\xFC}\xDD\xA0\x99\x9E\xE9\xA1/*\xAA\x08\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xF5a\0\xF06`\x04a\x1B}V[a\x01\xCEV[`@Q\x90\x81R` \x01a\0\xD9V[a\x01\x16a\x01\x116`\x04a\x1C,V[a\x05lV[\0[a\x01\x16a\x01&6`\x04a\x1C,V[a\x06\x1BV[a\x01Ma\x0196`\x04a\x1C,V[`\x02` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\xD9V[a\x01\x16a\x01k6`\x04a\x1CGV[a\x06gV[_Ta\0\xC5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\x16a\x01\x906`\x04a\x1CqV[a\x07;V[a\x01\x16a\x01\xA36`\x04a\x1C,V[a\t\x05V[a\x01\x16a\x01\xB66`\x04a\x1C,V[a\tNV[a\x01\x16a\x01\xC96`\x04a\x1C\xD6V[a\t\xC1V[_\x80T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x01\xF5WP3_\x90\x81R`\x02` R`@\x90 T`\xFF\x16[a\x02^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FOnly owner or liquidator can cal`D\x82\x01Rn6\x10:44\xB9\x903:\xB71\xBA4\xB7\xB7`\x89\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R_\x90`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xA2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xC6\x91\x90a\x1D%V[`@Qi\x06\xB6\x97GFV\xE77v\x17`\xB4\x1B` \x82\x01R\x90\x91P`*\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84\x84`@Q` \x01a\x03\x0B\x92\x91\x90a\x1D<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\x03\xBFWa\x03\xBA\x87`@Q\x80`\xC0\x01`@R\x80\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8A\x81R` \x01_\x81RPa\x0B\x16V[a\x04\xECV[`@Qh\x06\x87\x97\x06W'7v\x17`\xBC\x1B` \x82\x01R`)\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84\x84`@Q` \x01a\x04\0\x92\x91\x90a\x1D<V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x03a\x04\xAFWa\x03\xBA\x87`@Q\x80`\xC0\x01`@R\x80\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8A\x81R` \x01_\x81RPa\r'V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqInvalid swap venue`p\x1B`D\x82\x01R`d\x01a\x02UV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x81\x90`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x050W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05T\x91\x90a\x1D%V[a\x05^\x91\x90a\x1D_V[\x9A\x99PPPPPPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02U\x90a\x1D\x85V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81Rs2F{C\xBF\xA6rs\xFC}\xDD\xA0\x99\x9E\xE9\xA1/*\xAA\x08`\x04\x82\x01R_\x19`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x05\xF3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x17\x91\x90a\x1D\xBAV[PPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02U\x90a\x1D\x85V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x02` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02U\x90a\x1D\x85V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x06\xCDW`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x06\xC8W=__>=_\xFD[PPPV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07\x17W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC8\x91\x90a\x1D\xBAV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\x95W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7Fmsg.sender != activeKittenPair\0\0`D\x82\x01R`d\x01a\x02UV[_a\x07\xA2\x82\x84\x01\x84a\x1EAV[\x90P___a\x07\xB3\x84_\x01Qa\x0E\xB5V[` \x87\x01Q`@\x80\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q\x92Qb\xA7\x18\xA9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x91\x84\x16`$\x83\x01R\x92\x90\x92\x16`D\x83\x01R`d\x82\x01R_`\x84\x82\x01R\x92\x95P\x90\x93P\x91Ps2F{C\xBF\xA6rs\xFC}\xDD\xA0\x99\x9E\xE9\xA1/*\xAA\x08\x90b\xA7\x18\xA9\x90`\xA4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08<W__\xFD[PZ\xF1\x15\x80\x15a\x08NW=__>=_\xFD[PPPPa\x08^\x84_\x01Qa\x0E\xF4V[\x15a\x08\x7FW\x83Qa\x08n\x90a\x0F-V[\x84R`\xA0\x84\x01Qa\x08\x7F\x90\x85a\x0B\x16V[`\xA0\x84\x01Q`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x91\x92P\x82\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08\xD5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xF9\x91\x90a\x1D\xBAV[PPPPPPPPPPV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\t.W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02U\x90a\x1D\x85V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x02` R`@\x90 \x80T`\xFF\x19\x16\x90UV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\twW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x02U\x90a\x1D\x85V[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[_a\t\xCE\x82\x84\x01\x84a\x1EAV[\x90P___a\t\xDF\x84_\x01Qa\x0E\xB5V[\x92P\x92P\x92Pa\n\rs\"\xA9\xB8*l=+\xFBh\xF3$\xB2\xE86\x7F4m\xD6\xF3*a\n\x08\x85\x85\x85a\x0FdV[a\x0F\xCEV[` \x84\x01Q`@\x80\x86\x01Q``\x87\x01Q`\x80\x88\x01Q\x92Qb\xA7\x18\xA9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x91\x84\x16`$\x83\x01R\x92\x90\x92\x16`D\x83\x01R`d\x82\x01R_`\x84\x82\x01Rs2F{C\xBF\xA6rs\xFC}\xDD\xA0\x99\x9E\xE9\xA1/*\xAA\x08\x90b\xA7\x18\xA9\x90`\xA4\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n\x8EW__\xFD[PZ\xF1\x15\x80\x15a\n\xA0W=__>=_\xFD[PPPP__\x89\x13a\n\xB2W\x87a\n\xB4V[\x88[\x90Pa\n\xC2\x85_\x01Qa\x0E\xF4V[\x15a\n\xDEW\x84Qa\n\xD2\x90a\x0F-V[\x85Ra\n\xDE\x81\x86a\r'V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x92\x93P\x83\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01a\x08\xB9V[___a\x0B%\x84_\x01Qa\x0E\xB5V[`@Qc\x06\x80\x1C\xC3`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01\x81\x90R\x90\x85\x16`$\x83\x01\x81\x90R`\x01`D\x84\x01R\x94\x97P\x92\x95P\x90\x93P\x91\x11\x15\x90s\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B\x90ch\x01\xCC0\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x9CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xC0\x91\x90a\x1FPV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90Ua\x0C!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr$\xB7;0\xB64\xB2\x105\xB4\xBA:2\xB7\x1080\xB4\xB9`i\x1B`D\x82\x01R`d\x01a\x02UV[`@\x80Q`\x80\x80\x82\x01\x83R`\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83Rs\xDA\x12\xF4PX\nL\xC4\x85\xC3\xB5\x01\xBA\xB7\xB0\xB3\xCB\xC3\xB3\x1B` \x84\x01R\x90\x88\x01Q\x92\x82\x01\x92\x90\x92R\x90\x84\x16``\x82\x01Ra\x0Cr\x90a\x10\"V[`\xA0\x86\x01R`\x01T`\x01`\x01`\xA0\x1B\x03\x16c\x02,\r\x9F\x82a\x0C\x93W\x87a\x0C\x95V[_[\x83a\x0C\xA0W_a\x0C\xA2V[\x88[0\x89`@Q` \x01a\x0C\xB4\x91\x90a\x1F\x99V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xE2\x94\x93\x92\x91\x90a \nV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\xF9W__\xFD[PZ\xF1\x15\x80\x15a\r\x0BW=__>=_\xFD[PP`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UPPPPPPPPV[___a\r6\x84_\x01Qa\x0E\xB5V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x90\x83\x16\x10_a\rus\"\xA9\xB8*l=+\xFBh\xF3$\xB2\xE86\x7F4m\xD6\xF3*a\rp\x86\x88\x87a\x0FdV[a\x11\xA3V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xB3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xD7\x91\x90a\x1FPV[P`\x01`\x01`\xA0\x1B\x03\x81\x16c\x12\x8A\xCB\x080\x84a\r\xF2\x8Ba 6V[\x86a\x0E\x1BWa\x0E\x16`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a PV[a\x0E+V[a\x0E+d\x01\0\x02v\xA3`\x01a oV[\x8B`@Q` \x01a\x0E<\x91\x90a\x1F\x99V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Ek\x95\x94\x93\x92\x91\x90a \x8EV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E\x86W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xAA\x91\x90a \xD3V[PPPPPPPPPV[_\x80\x80a\x0E\xC2\x84\x82a\x12\x89V[\x92Pa\x0E\xCF\x84`\x14a\x12\xEDV[a\xFF\xFF\x16\x90Pa\x0E\xEBa\x0E\xE4`\x03`\x14a \xF5V[\x85\x90a\x12\x89V[\x91P\x91\x93\x90\x92PV[_a\x0F\x01`\x03`\x14a \xF5V[`\x14a\x0F\x0E`\x03\x82a \xF5V[a\x0F\x18\x91\x90a \xF5V[a\x0F\"\x91\x90a \xF5V[\x82Q\x10\x15\x90P\x91\x90PV[``a\x0F^a\x0F>`\x03`\x14a \xF5V[a\x0FJ`\x03`\x14a \xF5V[\x84Qa\x0FV\x91\x90a!\x08V[\x84\x91\x90a\x13\x97V[\x92\x91PPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x91\x90\x91R\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x11\x15a\x0F\x9EW\x91\x92\x91[P`@\x80Q``\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01Rb\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x90V[_a\x0F\xD9\x83\x83a\x11\xA3V[\x90P3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a\x06\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x1A[\x9D\x98[\x1AY\x08\x1C\x1B\xDB\xDB`\xA2\x1B`D\x82\x01R`d\x01a\x02UV[`@\x80Q`\xE0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R\x82_\x01Q`\x01`\x01`\xA0\x1B\x03\x16c9/7\xE9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x98W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xBC\x91\x90a!\x1BV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x89\x01R\x16`\xA0\x87\x01R\x15\x15`\x80\x86\x01R``\x85\x01R`@\x84\x01R` \x83\x01R\x81R_a\x10\xF5\x84\x83a\x14\xA3V[` \x85\x01Q\x85Q`\x80\x85\x01Q`@Qc\xCCV\xB2\xC5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x90\x15\x15`$\x82\x01R\x92\x93P_\x92\x91\x16\x90c\xCCV\xB2\xC5\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11RW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11v\x91\x90a\x1D%V[\x90Pa\x11\x84\x81a'\x10a!\x08V[a\x11\x90\x83a'\x10a!\x89V[a\x11\x9A\x91\x90a!\xA0V[\x95\x94PPPPPV[_\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x10a\x11\xC8W__\xFD[\x81Q` \x80\x84\x01Q`@\x80\x86\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81\x86\x01R\x94\x90\x92\x16\x84\x82\x01Rb\xFF\xFF\xFF\x90\x91\x16``\x80\x85\x01\x91\x90\x91R\x81Q\x80\x85\x03\x82\x01\x81R`\x80\x85\x01\x90\x92R\x81Q\x91\x90\x92\x01 `\x01`\x01`\xF8\x1B\x03\x19`\xA0\x84\x01R\x90\x85\x90\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\xA1\x83\x01R`\xB5\x82\x01R\x7F\x1E\xEDC\xDC\xAA.\xFD\xE0g.\xB5qd\x92\0\xA2\x927\xB7\x95\x8E{\x0F\xBDR\xF7_\xA3[~\xC5,`\xD5\x82\x01R`\xF5\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x93\x92PPPV[_a\x12\x95\x82`\x14a \xF5V[\x83Q\x10\x15a\x12\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RttoAddress_outOfBounds`X\x1B`D\x82\x01R`d\x01a\x02UV[P\x01` \x01Q`\x01``\x1B\x90\x04\x90V[_\x81a\x12\xFA\x81`\x03a \xF5V[\x10\x15a\x13<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01RptoUint24_overflow`x\x1B`D\x82\x01R`d\x01a\x02UV[a\x13G\x82`\x03a \xF5V[\x83Q\x10\x15a\x13\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RstoUint24_outOfBounds``\x1B`D\x82\x01R`d\x01a\x02UV[P\x01`\x03\x01Q\x90V[``\x81a\x13\xA5\x81`\x1Fa \xF5V[\x10\x15a\x13\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rmslice_overflow`\x90\x1B`D\x82\x01R`d\x01a\x02UV[a\x13\xEE\x82\x84a \xF5V[\x84Q\x10\x15a\x142W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rpslice_outOfBounds`x\x1B`D\x82\x01R`d\x01a\x02UV[``\x82\x15\x80\x15a\x14PW`@Q\x91P_\x82R` \x82\x01`@Ra\x14\x9AV[`@Q\x91P`\x1F\x84\x16\x80\x15` \x02\x81\x84\x01\x01\x85\x81\x01\x87\x83\x15` \x02\x84\x8B\x01\x01\x01[\x81\x83\x10\x15a\x14\x89W\x80Q\x83R` \x92\x83\x01\x92\x01a\x14qV[PP\x85\x84R`\x1F\x01`\x1F\x19\x16`@RP[P\x94\x93PPPPV[_\x81`\x80\x01Q\x15a\x16\xBAW_a\x14\xF3`@Q\x80`\xA0\x01`@R\x80\x85`@\x01Q\x81R` \x01\x85``\x01Q\x81R` \x01\x85`\x80\x01Q\x15\x15\x81R` \x01\x85_\x01Q\x81R` \x01\x85` \x01Q\x81RPa\x17.V[\x83Q`@\x85\x01Q\x91\x92P\x90a\x15\x10\x90g\r\xE0\xB6\xB3\xA7d\0\0a!\x89V[a\x15\x1A\x91\x90a!\xA0V[`@\x84\x01R` \x83\x01Q``\x84\x01Qa\x15;\x90g\r\xE0\xB6\xB3\xA7d\0\0a!\x89V[a\x15E\x91\x90a!\xA0V[\x83``\x01\x81\x81RPP__\x84`\xC0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15\x80W\x84``\x01Q\x85`@\x01Qa\x15\x8BV[\x84`@\x01Q\x85``\x01Q[\x91P\x91P\x84`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x86``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15\xDBW` \x85\x01Q`@\x87\x01Qa\x15\xCC\x90g\r\xE0\xB6\xB3\xA7d\0\0a!\x89V[a\x15\xD6\x91\x90a!\xA0V[a\x15\xFEV[\x84Q`@\x87\x01Qa\x15\xF4\x90g\r\xE0\xB6\xB3\xA7d\0\0a!\x89V[a\x15\xFE\x91\x90a!\xA0V[`@\x87\x01\x81\x90R_\x90a\x16\x11\x90\x83a!\x08V[\x90P_\x83a\x16W`@Q\x80`\xC0\x01`@R\x80\x85\x81R` \x01\x88\x81R` \x01\x87\x81R` \x01\x8A`\x80\x01Q\x15\x15\x81R` \x01\x8A_\x01Q\x81R` \x01\x8A` \x01Q\x81RPa\x18/V[a\x16a\x91\x90a!\x08V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x87`\xA0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x89``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\x94W\x87Qa\x16\x9AV[\x87` \x01Q[a\x16\xA4\x90\x83a!\x89V[a\x16\xAE\x91\x90a!\xA0V[\x95PPPPPPa\x0F^V[__\x83`\xC0\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x85``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x16\xECW\x83``\x01Q\x84`@\x01Qa\x16\xF7V[\x83`@\x01Q\x84``\x01Q[\x91P\x91P\x84`@\x01Q\x81a\x17\x0B\x91\x90a!\x08V[\x82\x86`@\x01Qa\x17\x1B\x91\x90a!\x89V[a\x17%\x91\x90a!\xA0V[\x92PPPa\x0F^V[_\x81`@\x01Q\x15a\x18\x19W``\x82\x01Q\x82Q_\x91\x90a\x17U\x90g\r\xE0\xB6\xB3\xA7d\0\0a!\x89V[a\x17_\x91\x90a!\xA0V[\x90P_\x83`\x80\x01Q\x84` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x17\x7F\x91\x90a!\x89V[a\x17\x89\x91\x90a!\xA0V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a\x17\x9F\x83\x85a!\x89V[a\x17\xA9\x91\x90a!\xA0V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a\x17\xBF\x84\x80a!\x89V[a\x17\xC9\x91\x90a!\xA0V[g\r\xE0\xB6\xB3\xA7d\0\0a\x17\xDC\x86\x80a!\x89V[a\x17\xE6\x91\x90a!\xA0V[a\x17\xF0\x91\x90a \xF5V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a\x18\x05\x82\x84a!\x89V[a\x18\x0F\x91\x90a!\xA0V[\x96\x95PPPPPPV[` \x82\x01Q\x82Qa\x0F^\x91\x90a!\x89V[\x91\x90PV[_\x80[`\xFF\x81\x10\x15a\x19\xEBW_a\x18M\x84`@\x01Q\x85_\x01Qa\x1A\x19V[\x90P\x83` \x01Q\x81\x10\x15a\x19BW_a\x18m\x85`@\x01Q\x86_\x01Qa\x1A\x94V[\x82\x86` \x01Qa\x18}\x91\x90a!\x08V[a\x18\x8F\x90g\r\xE0\xB6\xB3\xA7d\0\0a!\x89V[a\x18\x99\x91\x90a!\xA0V[\x90P\x80_\x03a\x19'W\x84` \x01Q\x82\x03a\x18\xB8WPPPP`@\x01Q\x90V[\x84` \x01Qa\x19\r`@Q\x80`\xA0\x01`@R\x80\x88`@\x01Q`\x01a\x18\xDC\x91\x90a \xF5V[\x81R` \x01\x88_\x01Q\x81R` \x01\x88``\x01Q\x15\x15\x81R` \x01\x88`\x80\x01Q\x81R` \x01\x88`\xA0\x01Q\x81RPa\x17.V[\x11\x15a\x19#W`@\x85\x01Qa\x11\x9A\x90`\x01a \xF5V[P`\x01[\x80\x85`@\x01Qa\x197\x91\x90a \xF5V[`@\x86\x01RPa\x19\xE2V[_a\x19T\x85`@\x01Q\x86_\x01Qa\x1A\x94V[` \x86\x01Qa\x19c\x90\x84a!\x08V[a\x19u\x90g\r\xE0\xB6\xB3\xA7d\0\0a!\x89V[a\x19\x7F\x91\x90a!\xA0V[\x90P\x80_\x03a\x19\xCBW\x84` \x01Q\x82\x14\x80a\x19\xB7WP\x84` \x01Qa\x19\xB5`\x01\x87`@\x01Qa\x19\xAE\x91\x90a!\x08V[\x87Qa\x1A\x19V[\x10[\x15a\x19\xC7WPPPP`@\x01Q\x90V[P`\x01[\x80\x85`@\x01Qa\x19\xDB\x91\x90a!\x08V[`@\x86\x01RP[P`\x01\x01a\x182V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x02`$\x82\x01Ra\x04/`\xF3\x1B`D\x82\x01R`d\x01a\x02UV[_\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x1A.\x84\x86a!\x89V[a\x1A8\x91\x90a!\xA0V[\x90P_g\r\xE0\xB6\xB3\xA7d\0\0a\x1AN\x85\x80a!\x89V[a\x1AX\x91\x90a!\xA0V[g\r\xE0\xB6\xB3\xA7d\0\0a\x1Ak\x87\x80a!\x89V[a\x1Au\x91\x90a!\xA0V[a\x1A\x7F\x91\x90a \xF5V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a\x11\x90\x82\x84a!\x89V[_g\r\xE0\xB6\xB3\xA7d\0\0\x83\x81a\x1A\xAA\x82\x80a!\x89V[a\x1A\xB4\x91\x90a!\xA0V[a\x1A\xBE\x91\x90a!\x89V[a\x1A\xC8\x91\x90a!\xA0V[g\r\xE0\xB6\xB3\xA7d\0\0\x80a\x1A\xDC\x85\x80a!\x89V[a\x1A\xE6\x91\x90a!\xA0V[a\x1A\xF1\x86`\x03a!\x89V[a\x1A\xFB\x91\x90a!\x89V[a\x1B\x05\x91\x90a!\xA0V[a\x1B\x0F\x91\x90a \xF5V[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B*W__\xFD[PV[\x805a\x18*\x81a\x1B\x16V[__\x83`\x1F\x84\x01\x12a\x1BHW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B_W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1BvW__\xFD[\x92P\x92\x90PV[________`\xC0\x89\x8B\x03\x12\x15a\x1B\x94W__\xFD[\x885a\x1B\x9F\x81a\x1B\x16V[\x97P` \x89\x015a\x1B\xAF\x81a\x1B\x16V[\x96P`@\x89\x015a\x1B\xBF\x81a\x1B\x16V[\x95P``\x89\x015\x94P`\x80\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\xE1W__\xFD[a\x1B\xED\x8B\x82\x8C\x01a\x1B8V[\x90\x95P\x93PP`\xA0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\x0CW__\xFD[a\x1C\x18\x8B\x82\x8C\x01a\x1B8V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[_` \x82\x84\x03\x12\x15a\x1C<W__\xFD[\x815a\x1B\x0F\x81a\x1B\x16V[__`@\x83\x85\x03\x12\x15a\x1CXW__\xFD[\x825a\x1Cc\x81a\x1B\x16V[\x94` \x93\x90\x93\x015\x93PPPV[_____`\x80\x86\x88\x03\x12\x15a\x1C\x85W__\xFD[\x855a\x1C\x90\x81a\x1B\x16V[\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C\xB9W__\xFD[a\x1C\xC5\x88\x82\x89\x01a\x1B8V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[____``\x85\x87\x03\x12\x15a\x1C\xE9W__\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\rW__\xFD[a\x1D\x19\x87\x82\x88\x01a\x1B8V[\x95\x98\x94\x97P\x95PPPPV[_` \x82\x84\x03\x12\x15a\x1D5W__\xFD[PQ\x91\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1D~Wa\x1D~a\x1DKV[P\x92\x91PPV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[\x80Q\x80\x15\x15\x81\x14a\x18*W__\xFD[_` \x82\x84\x03\x12\x15a\x1D\xCAW__\xFD[a\x1B\x0F\x82a\x1D\xABV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\xC0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E\nWa\x1E\na\x1D\xD3V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E9Wa\x1E9a\x1D\xD3V[`@R\x91\x90PV[_` \x82\x84\x03\x12\x15a\x1EQW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1EgW__\xFD[\x82\x01`\xC0\x81\x85\x03\x12\x15a\x1ExW__\xFD[a\x1E\x80a\x1D\xE7V[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\x96W__\xFD[\x82\x01`\x1F\x81\x01\x86\x13a\x1E\xA6W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1E\xC0Wa\x1E\xC0a\x1D\xD3V[a\x1E\xD3`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1E\x10V[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15a\x1E\xE7W__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x84RPPPa\x1F\x0B` \x83\x01a\x1B-V[` \x82\x01Ra\x1F\x1C`@\x83\x01a\x1B-V[`@\x82\x01Ra\x1F-``\x83\x01a\x1B-V[``\x82\x01R`\x80\x82\x81\x015\x90\x82\x01R`\xA0\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x1F`W__\xFD[\x81Qa\x1B\x0F\x81a\x1B\x16V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_\x82Q`\xC0` \x84\x01Ra\x1F\xB4`\xE0\x84\x01\x82a\x1FkV[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`\x01\x80`\xA0\x1B\x03`@\x85\x01Q\x16``\x84\x01R`\x01\x80`\xA0\x1B\x03``\x85\x01Q\x16`\x80\x84\x01R`\x80\x84\x01Q`\xA0\x84\x01R`\xA0\x84\x01Q`\xC0\x84\x01R\x80\x91PP\x92\x91PPV[\x84\x81R\x83` \x82\x01R`\x01\x80`\xA0\x1B\x03\x83\x16`@\x82\x01R`\x80``\x82\x01R_a\x18\x0F`\x80\x83\x01\x84a\x1FkV[_`\x01`\xFF\x1B\x82\x01a JWa Ja\x1DKV[P_\x03\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0F^Wa\x0F^a\x1DKV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0F^Wa\x0F^a\x1DKV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R_\x90a \xC8\x90\x83\x01\x84a\x1FkV[\x97\x96PPPPPPPV[__`@\x83\x85\x03\x12\x15a \xE4W__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[\x80\x82\x01\x80\x82\x11\x15a\x0F^Wa\x0F^a\x1DKV[\x81\x81\x03\x81\x81\x11\x15a\x0F^Wa\x0F^a\x1DKV[_______`\xE0\x88\x8A\x03\x12\x15a!1W__\xFD[\x87Q` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q\x92\x99P\x90\x97P\x95P\x93Pa!X`\x80\x89\x01a\x1D\xABV[\x92P`\xA0\x88\x01Qa!h\x81a\x1B\x16V[`\xC0\x89\x01Q\x90\x92Pa!y\x81a\x1B\x16V[\x80\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0F^Wa\x0F^a\x1DKV[_\x82a!\xBAWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x9Bd\xBF\xE53\xD2l\xB8nJU\xA8Z\x9C\xC9\xD0\xE89\x13\xDA\xA2u\">\xED\xCA\xCC\x1F\xBD\xF1\xBA5dsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static LIQUIDATOR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Liquidator<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Liquidator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Liquidator<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Liquidator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Liquidator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Liquidator)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Liquidator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LIQUIDATOR_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                LIQUIDATOR_ABI.clone(),
                LIQUIDATOR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addLiquidator` (0x4c4e7f6f) function
        pub fn add_liquidator(
            &self,
            liquidator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 78, 127, 111], liquidator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approvePool` (0x424c265b) function
        pub fn approve_pool(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 76, 38, 91], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hook` (0x9a7bff79) function
        pub fn hook(
            &self,
            sender: ::ethers::core::types::Address,
            amount_0_out: ::ethers::core::types::U256,
            amount_1_out: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [154, 123, 255, 121],
                    (sender, amount_0_out, amount_1_out, data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isLiquidator` (0x529a356f) function
        pub fn is_liquidator(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([82, 154, 53, 111], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidate` (0x2e43c961) function
        pub fn liquidate(
            &self,
            collateral_asset: ::ethers::core::types::Address,
            debt_asset: ::ethers::core::types::Address,
            user: ::ethers::core::types::Address,
            debt_to_cover: ::ethers::core::types::U256,
            swap_path: ::ethers::core::types::Bytes,
            swap_venue: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash(
                    [46, 67, 201, 97],
                    (
                        collateral_asset,
                        debt_asset,
                        user,
                        debt_to_cover,
                        swap_path,
                        swap_venue,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pool` (0x16f0115b) function
        pub fn pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([22, 240, 17, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recover` (0x5705ae43) function
        pub fn recover(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 5, 174, 67], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeLiquidator` (0xefb74400) function
        pub fn remove_liquidator(
            &self,
            liquidator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 183, 68, 0], liquidator)
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
        ///Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: ::ethers::core::types::I256,
            amount_1_delta: ::ethers::core::types::I256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Test` event
        pub fn test_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TestFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LiquidatorEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Liquidator<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Test", abi = "Test(uint256)")]
    pub struct TestFilter {
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LiquidatorEvents {
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        TestFilter(TestFilter),
    }
    impl ::ethers::contract::EthLogDecode for LiquidatorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(LiquidatorEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = TestFilter::decode_log(log) {
                return Ok(LiquidatorEvents::TestFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LiquidatorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TestFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for LiquidatorEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<TestFilter> for LiquidatorEvents {
        fn from(value: TestFilter) -> Self {
            Self::TestFilter(value)
        }
    }
    ///Container type for all input parameters for the `addLiquidator` function with signature `addLiquidator(address)` and selector `0x4c4e7f6f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "addLiquidator", abi = "addLiquidator(address)")]
    pub struct AddLiquidatorCall {
        pub liquidator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approvePool` function with signature `approvePool(address)` and selector `0x424c265b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approvePool", abi = "approvePool(address)")]
    pub struct ApprovePoolCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `hook` function with signature `hook(address,uint256,uint256,bytes)` and selector `0x9a7bff79`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "hook", abi = "hook(address,uint256,uint256,bytes)")]
    pub struct HookCall {
        pub sender: ::ethers::core::types::Address,
        pub amount_0_out: ::ethers::core::types::U256,
        pub amount_1_out: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `isLiquidator` function with signature `isLiquidator(address)` and selector `0x529a356f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isLiquidator", abi = "isLiquidator(address)")]
    pub struct IsLiquidatorCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `liquidate` function with signature `liquidate(address,address,address,uint256,bytes,string)` and selector `0x2e43c961`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "liquidate",
        abi = "liquidate(address,address,address,uint256,bytes,string)"
    )]
    pub struct LiquidateCall {
        pub collateral_asset: ::ethers::core::types::Address,
        pub debt_asset: ::ethers::core::types::Address,
        pub user: ::ethers::core::types::Address,
        pub debt_to_cover: ::ethers::core::types::U256,
        pub swap_path: ::ethers::core::types::Bytes,
        pub swap_venue: ::std::string::String,
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
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pool` function with signature `pool()` and selector `0x16f0115b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pool", abi = "pool()")]
    pub struct PoolCall;
    ///Container type for all input parameters for the `recover` function with signature `recover(address,uint256)` and selector `0x5705ae43`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "recover", abi = "recover(address,uint256)")]
    pub struct RecoverCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeLiquidator` function with signature `removeLiquidator(address)` and selector `0xefb74400`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "removeLiquidator", abi = "removeLiquidator(address)")]
    pub struct RemoveLiquidatorCall {
        pub liquidator: ::ethers::core::types::Address,
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
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
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
        Hash
    )]
    #[ethcall(
        name = "uniswapV3SwapCallback",
        abi = "uniswapV3SwapCallback(int256,int256,bytes)"
    )]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0_delta: ::ethers::core::types::I256,
        pub amount_1_delta: ::ethers::core::types::I256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LiquidatorCalls {
        AddLiquidator(AddLiquidatorCall),
        ApprovePool(ApprovePoolCall),
        Hook(HookCall),
        IsLiquidator(IsLiquidatorCall),
        Liquidate(LiquidateCall),
        Owner(OwnerCall),
        Pool(PoolCall),
        Recover(RecoverCall),
        RemoveLiquidator(RemoveLiquidatorCall),
        TransferOwnership(TransferOwnershipCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for LiquidatorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddLiquidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddLiquidator(decoded));
            }
            if let Ok(decoded) = <ApprovePoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApprovePool(decoded));
            }
            if let Ok(decoded) = <HookCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Hook(decoded));
            }
            if let Ok(decoded) = <IsLiquidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsLiquidator(decoded));
            }
            if let Ok(decoded) = <LiquidateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Liquidate(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pool(decoded));
            }
            if let Ok(decoded) = <RecoverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Recover(decoded));
            }
            if let Ok(decoded) = <RemoveLiquidatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveLiquidator(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UniswapV3SwapCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LiquidatorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddLiquidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApprovePool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Hook(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsLiquidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Liquidate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Recover(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveLiquidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LiquidatorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddLiquidator(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Hook(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsLiquidator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Liquidate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Recover(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveLiquidator(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AddLiquidatorCall> for LiquidatorCalls {
        fn from(value: AddLiquidatorCall) -> Self {
            Self::AddLiquidator(value)
        }
    }
    impl ::core::convert::From<ApprovePoolCall> for LiquidatorCalls {
        fn from(value: ApprovePoolCall) -> Self {
            Self::ApprovePool(value)
        }
    }
    impl ::core::convert::From<HookCall> for LiquidatorCalls {
        fn from(value: HookCall) -> Self {
            Self::Hook(value)
        }
    }
    impl ::core::convert::From<IsLiquidatorCall> for LiquidatorCalls {
        fn from(value: IsLiquidatorCall) -> Self {
            Self::IsLiquidator(value)
        }
    }
    impl ::core::convert::From<LiquidateCall> for LiquidatorCalls {
        fn from(value: LiquidateCall) -> Self {
            Self::Liquidate(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for LiquidatorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PoolCall> for LiquidatorCalls {
        fn from(value: PoolCall) -> Self {
            Self::Pool(value)
        }
    }
    impl ::core::convert::From<RecoverCall> for LiquidatorCalls {
        fn from(value: RecoverCall) -> Self {
            Self::Recover(value)
        }
    }
    impl ::core::convert::From<RemoveLiquidatorCall> for LiquidatorCalls {
        fn from(value: RemoveLiquidatorCall) -> Self {
            Self::RemoveLiquidator(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for LiquidatorCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for LiquidatorCalls {
        fn from(value: UniswapV3SwapCallbackCall) -> Self {
            Self::UniswapV3SwapCallback(value)
        }
    }
    ///Container type for all return fields from the `isLiquidator` function with signature `isLiquidator(address)` and selector `0x529a356f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsLiquidatorReturn(pub bool);
    ///Container type for all return fields from the `liquidate` function with signature `liquidate(address,address,address,uint256,bytes,string)` and selector `0x2e43c961`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LiquidateReturn {
        pub collateral_gain: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pool` function with signature `pool()` and selector `0x16f0115b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PoolReturn(pub ::ethers::core::types::Address);
}
