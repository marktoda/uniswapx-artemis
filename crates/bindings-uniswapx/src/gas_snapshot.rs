pub use gas_snapshot::*;
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
pub mod gas_snapshot {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CHECK_ENV_VAR"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("CHECK_ENV_VAR"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IS_SCRIPT"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("IS_SCRIPT"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SNAP_DIR"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("SNAP_DIR"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("GasMismatch"),
                ::std::vec![::ethers::core::abi::ethabi::AbiError {
                    name: ::std::borrow::ToOwned::to_owned("GasMismatch"),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("oldGas"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },
                        ::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newGas"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },
                    ],
                },],
            )]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static GASSNAPSHOT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\x04\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x0C\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15a\0-W`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x11\x81Rp.forge-snapshots/`x\x1B` \x82\x01Ra\0_\x90a\x011V[`@\x80Q\x80\x82\x01\x82R`\x14\x81R\x7FFORGE_SNAPSHOT_CHECK\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x90Qc~\xD1\xEC}`\xE0\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x91c~\xD1\xEC}\x91a\0\xC7\x91\x90`\x04\x01a\x02\xC7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x01\0WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\0\xFD\x91\x81\x01\x90a\x02\xE1V[`\x01[a\x01\x14W`\x0C\x80Ta\xFF\0\x19\x16\x90Ua\x04=V[`\x0C\x80T\x91\x15\x15a\x01\0\x02a\xFF\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\x04=V[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R`\0\x91\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01IW\x90PP\x90P`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d6\xB5\xB24\xB9`\xD9\x1B\x81RP\x81`\0\x81Q\x81\x10a\x01\x91Wa\x01\x91a\x03\x19V[` \x02` \x01\x01\x81\x90RP`@Q\x80`@\x01`@R\x80`\x02\x81R` \x01a\x02\xD7`\xF4\x1B\x81RP\x81`\x01\x81Q\x81\x10a\x01\xCAWa\x01\xCAa\x03\x19V[` \x02` \x01\x01\x81\x90RP\x81\x81`\x02\x81Q\x81\x10a\x01\xE9Wa\x01\xE9a\x03\x19V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc\x89\x16\x04g`\xE0\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x89\x16\x04g\x90a\x02+\x90\x84\x90`\x04\x01a\x03/V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x02JW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02r\x91\x90\x81\x01\x90a\x03\x91V[PPPV[`\0[\x83\x81\x10\x15a\x02\x92W\x81\x81\x01Q\x83\x82\x01R` \x01a\x02zV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x02\xB3\x81` \x86\x01` \x86\x01a\x02wV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x02\xDA` \x83\x01\x84a\x02\x9BV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x02\xF3W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02\xDAW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a\x03\x84W`?\x19\x88\x86\x03\x01\x84Ra\x03r\x85\x83Qa\x02\x9BV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a\x03VV[P\x92\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x03\xA3W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x03\xBAW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x03\xCEW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x03\xE0Wa\x03\xE0a\x03\x03V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x04\x08Wa\x04\x08a\x03\x03V[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\x04!W`\0\x80\xFD[a\x042\x83` \x83\x01` \x88\x01a\x02wV[\x97\x96PPPPPPPV[a\x01\x93\x80a\x04L`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c8\t,L\x14a\0FW\x80c\x9B'\xAA.\x14a\0\x98W\x80c\xF8\xCC\xBFG\x14a\0\xD4W[`\0\x80\xFD[a\0\x82`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01\x7FFORGE_SNAPSHOT_CHECK\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\0\x8F\x91\x90a\0\xF1V[`@Q\x80\x91\x03\x90\xF3[a\0\x82`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01\x7F.forge-snapshots/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`\x0CTa\0\xE1\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\x8FV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x01\x1EW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x01\x02V[P`\0`@\x82\x86\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x0C<\xF37\x18ES\x1Ah\xAD\xFD\xD6*e\x92\xD8\x19\x0B\xB0\x94&!\x97\x03\x13U\x11\xE6\x1AiiFdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static GASSNAPSHOT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c8\t,L\x14a\0FW\x80c\x9B'\xAA.\x14a\0\x98W\x80c\xF8\xCC\xBFG\x14a\0\xD4W[`\0\x80\xFD[a\0\x82`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01\x7FFORGE_SNAPSHOT_CHECK\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`@Qa\0\x8F\x91\x90a\0\xF1V[`@Q\x80\x91\x03\x90\xF3[a\0\x82`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01\x7F.forge-snapshots/\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[`\x0CTa\0\xE1\x90`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\0\x8FV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a\x01\x1EW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\x01\x02V[P`\0`@\x82\x86\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x0C<\xF37\x18ES\x1Ah\xAD\xFD\xD6*e\x92\xD8\x19\x0B\xB0\x94&!\x97\x03\x13U\x11\xE6\x1AiiFdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static GASSNAPSHOT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct GasSnapshot<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GasSnapshot<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GasSnapshot<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GasSnapshot<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GasSnapshot<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GasSnapshot))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GasSnapshot<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                GASSNAPSHOT_ABI.clone(),
                client,
            ))
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
                GASSNAPSHOT_ABI.clone(),
                GASSNAPSHOT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `CHECK_ENV_VAR` (0x38092c4c) function
        pub fn check_env_var(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([56, 9, 44, 76], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `IS_SCRIPT` (0xf8ccbf47) function
        pub fn is_script(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 204, 191, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SNAP_DIR` (0x9b27aa2e) function
        pub fn snap_dir(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([155, 39, 170, 46], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for GasSnapshot<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `GasMismatch` with signature `GasMismatch(uint256,uint256)` and selector `0x4354d0b2`
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
    #[etherror(name = "GasMismatch", abi = "GasMismatch(uint256,uint256)")]
    pub struct GasMismatch {
        pub old_gas: ::ethers::core::types::U256,
        pub new_gas: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `CHECK_ENV_VAR` function with signature `CHECK_ENV_VAR()` and selector `0x38092c4c`
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
    #[ethcall(name = "CHECK_ENV_VAR", abi = "CHECK_ENV_VAR()")]
    pub struct CheckEnvVarCall;
    ///Container type for all input parameters for the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
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
    #[ethcall(name = "IS_SCRIPT", abi = "IS_SCRIPT()")]
    pub struct IsScriptCall;
    ///Container type for all input parameters for the `SNAP_DIR` function with signature `SNAP_DIR()` and selector `0x9b27aa2e`
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
    #[ethcall(name = "SNAP_DIR", abi = "SNAP_DIR()")]
    pub struct SnapDirCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GasSnapshotCalls {
        CheckEnvVar(CheckEnvVarCall),
        IsScript(IsScriptCall),
        SnapDir(SnapDirCall),
    }
    impl ::ethers::core::abi::AbiDecode for GasSnapshotCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CheckEnvVarCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CheckEnvVar(decoded));
            }
            if let Ok(decoded) = <IsScriptCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsScript(decoded));
            }
            if let Ok(decoded) = <SnapDirCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SnapDir(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GasSnapshotCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckEnvVar(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsScript(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SnapDir(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for GasSnapshotCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckEnvVar(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsScript(element) => ::core::fmt::Display::fmt(element, f),
                Self::SnapDir(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckEnvVarCall> for GasSnapshotCalls {
        fn from(value: CheckEnvVarCall) -> Self {
            Self::CheckEnvVar(value)
        }
    }
    impl ::core::convert::From<IsScriptCall> for GasSnapshotCalls {
        fn from(value: IsScriptCall) -> Self {
            Self::IsScript(value)
        }
    }
    impl ::core::convert::From<SnapDirCall> for GasSnapshotCalls {
        fn from(value: SnapDirCall) -> Self {
            Self::SnapDir(value)
        }
    }
    ///Container type for all return fields from the `CHECK_ENV_VAR` function with signature `CHECK_ENV_VAR()` and selector `0x38092c4c`
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
    pub struct CheckEnvVarReturn(pub ::std::string::String);
    ///Container type for all return fields from the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
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
    pub struct IsScriptReturn(pub bool);
    ///Container type for all return fields from the `SNAP_DIR` function with signature `SNAP_DIR()` and selector `0x9b27aa2e`
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
    pub struct SnapDirReturn(pub ::std::string::String);
}
