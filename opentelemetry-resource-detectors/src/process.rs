//! Process resource detector
//!
//! Detect process related information like pid, executable name.

use opentelemetry::{KeyValue, StringValue, Value};
use opentelemetry_sdk::resource::ResourceDetector;
use opentelemetry_sdk::Resource;
use std::env::args_os;
use std::process::id;

/// Detect process information.
///
/// This resource detector returns the following information:
///
/// - process command line arguments(`process.command_args`), the full command arguments of this
///   application.
/// - OS assigned process id(`process.pid`).
pub struct ProcessResourceDetector;

impl ResourceDetector for ProcessResourceDetector {
    fn detect(&self) -> Resource {
        let arguments = args_os();
        let cmd_arg_val = arguments
            .into_iter()
            .map(|arg| arg.to_string_lossy().into_owned().into())
            .collect::<Vec<StringValue>>();
        Resource::builder_empty()
            .with_attributes(vec![
                KeyValue::new(
                    opentelemetry_semantic_conventions::attribute::PROCESS_COMMAND_ARGS,
                    Value::Array(cmd_arg_val.into()),
                ),
                KeyValue::new(
                    opentelemetry_semantic_conventions::attribute::PROCESS_PID,
                    id() as i64,
                ),
            ])
            .build()
    }
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod tests {
    use super::ProcessResourceDetector;
    use opentelemetry_sdk::resource::ResourceDetector;

    #[test]
    fn test_processor_resource_detector() {
        let resource = ProcessResourceDetector.detect();
        assert_eq!(resource.len(), 2); // we cannot assert on the values because it changes along with runtime.
    }
}
