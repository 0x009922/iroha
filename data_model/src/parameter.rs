//! Parameters are responsible for chain-wide configuration

use core::{num::NonZeroU32, time::Duration};

use derive_more::Display;
use iroha_data_model_derive::model_single;
use iroha_schema::IntoSchema;
use parity_scale_codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

use crate::{
    metadata::Limits as MetadataLimits, transaction::TransactionLimits, JsonString, LengthLimits,
};

/// Macro utility to generate code related to core parameters with less boilerplate
macro_rules! map_core_params {
    ($callback:ident) => {
        $callback!(
            (
                MaxTransactionsInBlock,
                max_transactions_in_block,
                NonZeroU32,
            ),
            (BlockTimeMs, block_time_ms, u64),
            (CommitTimeMs, commit_time_ms, u64),
            (TransactionLimits, transaction_limits, TransactionLimits),
            (DomainMetadataLimits, domain_metadata_limits, MetadataLimits),
            (
                AssetDefinitionMetadataLimits,
                asset_definition_metadata_limits,
                MetadataLimits,
            ),
            (
                AccountMetadataLimits,
                account_metadata_limits,
                MetadataLimits,
            ),
            (AssetMetadataLimits, asset_metadata_limits, MetadataLimits),
            (
                TriggerMetadataLimits,
                trigger_metadata_limits,
                MetadataLimits,
            ),
            (IdentLengthLimits, ident_length_limits, LengthLimits),
            (ExecutorFuelLimit, executor_fuel_limit, u64),
            (ExecutorMaxMemoryBytes, executor_max_memory_bytes, u32),
            (WasmFuelLimit, wasm_fuel_limit, u64),
            (WasmMaxMemoryBytes, wasm_max_memory_bytes, u32),
        );
    };
}

macro_rules! declare_parameter_box {
    ($(($variant:ident, $field:ident, $ty:ty $(,)?)),+ $(,)?) => {
        model_single! {
            /// A single configuration parameter.
            #[derive(
                Debug,
                Clone,
                PartialEq,
                Eq,
                PartialOrd,
                Ord,
                Decode,
                Encode,
                Deserialize,
                Serialize,
                IntoSchema,
            )]
            #[ffi_type(opaque)]
            #[allow(missing_docs)]
            pub enum ParameterBox {
                $($variant($ty),)+
                /// Custom executor-defined parameter
                Custom(JsonString)
            }
        }
    };
}

map_core_params!(declare_parameter_box);

macro_rules! declare_parameters_state {
    ($(($variant:ident, $field:ident, $ty:ty $(,)?)),+ $(,)?) => {
        model_single! {
            /// Complete state of on-chain configuration
            #[derive(
                Debug,
                Display,
                Clone,
                Decode,
                Encode,
                Deserialize,
                Serialize,
                IntoSchema,
                PartialEq,
                Eq,
                Ord,
                PartialOrd
            )]
            #[display(fmt = "{self:?}")]
            #[ffi_type(opaque)]
            #[allow(missing_docs)]
            pub struct Parameters {
                $(pub $field: $ty,)+
                pub custom: Option<JsonString>
            }
        }
    };
}

map_core_params!(declare_parameters_state);

macro_rules! impl_with_mapping {
    ($(($variant:ident, $field:ident, $ty:ty $(,)?)),+ $(,)?) => {
        impl core::fmt::Display for ParameterBox {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    $(Self::$variant(value) => write!(f, "PARAMETER `{}` = `{value}`", stringify!($variant)),)+
                    Self::Custom(value) => write!(f, "PARAMETER `Custom` = `{value:?}`")
                }
            }
        }

        impl Parameters {
            /// Patch with a single parameter
            pub fn patch(&mut self, parameter: ParameterBox) {
                match parameter {
                    $(ParameterBox::$variant(value) => {
                        self.$field = value;
                    })+
                    ParameterBox::Custom(value) => {
                        self.custom = Some(value)
                    }
                }
            }
        }
    };
}

map_core_params!(impl_with_mapping);

impl Parameters {
    /// Getter with conversion from plain milliseconds to [`Duration`]
    pub fn block_time(&self) -> Duration {
        Duration::from_millis(self.block_time_ms)
    }

    /// Getter with conversion from plain milliseconds to [`Duration`]
    pub fn commit_time(&self) -> Duration {
        Duration::from_millis(self.commit_time_ms)
    }

    /// Computes consensus estimation as `block_time + commit_time / 2`. Returns [`None`]
    /// if [`Duration::checked_add`] overflows.
    pub fn consensus_estimation(&self) -> Option<Duration> {
        self.commit_time()
            .checked_div(2)
            .expect("2 is not 0")
            .checked_add(self.block_time())
    }
}

/// TODO
#[derive(Copy, Clone)]
pub struct CoreSequenceError;

/// Commonly used items
pub mod prelude {
    pub use super::{ParameterBox, Parameters};
}
