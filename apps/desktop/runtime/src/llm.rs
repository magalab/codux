use crate::{
    runtime_trace::runtime_trace,
    settings::{AIProviderSettings, AISettings},
};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Route the shared `codux-llm` crate's provider-call tracing back through the
/// desktop's `runtime_trace`. Installed once; cheap to call repeatedly.
pub(crate) fn install_llm_trace_hook() {
    codux_llm::set_trace_hook(runtime_trace);
}

include!("llm/types.rs");
include!("llm/provider.rs");
include!("llm/completion.rs");
