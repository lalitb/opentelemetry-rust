// DO NOT EDIT, this is an auto-generated file
//
// If you want to update the file:
// - Edit the template at scripts/templates/registry/rust/attributes.rs.j2
// - Run the script at scripts/generate-consts-from-spec.sh

//! # Trace Semantic Conventions
//!
//! The [trace semantic conventions] define a set of standardized attributes to
//! be used in `Span`s.
//!
//! [trace semantic conventions]: https://opentelemetry.io/docs/specs/semconv/general/trace/
//!
//! ## Usage
//!
//! ```rust
//! use opentelemetry::KeyValue;
//! use opentelemetry::{global, trace::Tracer as _};
//! use opentelemetry_semantic_conventions as semconv;
//!
//! let tracer = global::tracer("my-component");
//! let _span = tracer
//!     .span_builder("span-name")
//!     .with_attributes([
//!         KeyValue::new(semconv::trace::CLIENT_ADDRESS, "example.org"),
//!         KeyValue::new(semconv::trace::CLIENT_PORT, 80i64),
//!     ])
//!     .start(&tracer);
//! ```

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ANDROID_APP_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ANDROID_OS_API_LEVEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ANDROID_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::APP_BUILD_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::APP_INSTALLATION_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::APP_JANK_FRAME_COUNT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::APP_JANK_PERIOD;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::APP_JANK_THRESHOLD;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::APP_SCREEN_COORDINATE_X;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::APP_SCREEN_COORDINATE_Y;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::APP_WIDGET_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::APP_WIDGET_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ARTIFACT_ATTESTATION_FILENAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ARTIFACT_ATTESTATION_HASH;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ARTIFACT_ATTESTATION_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ARTIFACT_FILENAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ARTIFACT_HASH;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ARTIFACT_PURL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ARTIFACT_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_AUTHENTICATION_RESULT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_AUTHENTICATION_SCHEME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_AUTHORIZATION_POLICY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_AUTHORIZATION_RESULT;

#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_DIAGNOSTICS_EXCEPTION_RESULT;

#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_DIAGNOSTICS_HANDLER_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_IDENTITY_ERROR_CODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_IDENTITY_PASSWORD_CHECK_RESULT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_IDENTITY_RESULT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_IDENTITY_SIGN_IN_RESULT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_IDENTITY_SIGN_IN_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_IDENTITY_TOKEN_PURPOSE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_IDENTITY_TOKEN_VERIFIED;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_IDENTITY_USER_UPDATE_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_IDENTITY_USER_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_MEMORY_POOL_OWNER;

#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_RATE_LIMITING_POLICY;

#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_RATE_LIMITING_RESULT;

#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_REQUEST_IS_UNHANDLED;

#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_ROUTING_IS_FALLBACK;

#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_ROUTING_MATCH_STATUS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_SIGN_IN_IS_PERSISTENT;

#[allow(deprecated)]
pub use crate::attribute::ASPNETCORE_USER_IS_AUTHENTICATED;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_BEDROCK_GUARDRAIL_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_BEDROCK_KNOWLEDGE_BASE_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_ATTRIBUTE_DEFINITIONS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_ATTRIBUTES_TO_GET;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_CONSISTENT_READ;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_CONSUMED_CAPACITY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_COUNT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_EXCLUSIVE_START_TABLE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_GLOBAL_SECONDARY_INDEX_UPDATES;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_GLOBAL_SECONDARY_INDEXES;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_INDEX_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_ITEM_COLLECTION_METRICS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_LIMIT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_LOCAL_SECONDARY_INDEXES;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_PROJECTION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_PROVISIONED_READ_CAPACITY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_PROVISIONED_WRITE_CAPACITY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_SCAN_FORWARD;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_SCANNED_COUNT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_SEGMENT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_SELECT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_TABLE_COUNT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_TABLE_NAMES;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_DYNAMODB_TOTAL_SEGMENTS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_ECS_CLUSTER_ARN;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_ECS_CONTAINER_ARN;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_ECS_LAUNCHTYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_ECS_TASK_ARN;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_ECS_TASK_FAMILY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_ECS_TASK_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_ECS_TASK_REVISION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_EKS_CLUSTER_ARN;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_EXTENDED_REQUEST_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_KINESIS_STREAM_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_LAMBDA_INVOKED_ARN;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_LAMBDA_RESOURCE_MAPPING_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_LOG_GROUP_ARNS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_LOG_GROUP_NAMES;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_LOG_STREAM_ARNS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_LOG_STREAM_NAMES;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_REQUEST_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_S3_BUCKET;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_S3_COPY_SOURCE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_S3_DELETE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_S3_KEY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_S3_PART_NUMBER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_S3_UPLOAD_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_SECRETSMANAGER_SECRET_ARN;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_SNS_TOPIC_ARN;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_SQS_QUEUE_URL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_STEP_FUNCTIONS_ACTIVITY_ARN;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AWS_STEP_FUNCTIONS_STATE_MACHINE_ARN;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AZ_NAMESPACE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AZ_SERVICE_REQUEST_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AZURE_CLIENT_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AZURE_COSMOSDB_CONNECTION_MODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AZURE_COSMOSDB_CONSISTENCY_LEVEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AZURE_COSMOSDB_OPERATION_CONTACTED_REGIONS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AZURE_COSMOSDB_OPERATION_REQUEST_CHARGE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AZURE_COSMOSDB_REQUEST_BODY_SIZE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AZURE_COSMOSDB_RESPONSE_SUB_STATUS_CODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AZURE_RESOURCE_PROVIDER_NAMESPACE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::AZURE_SERVICE_REQUEST_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::BROWSER_BRANDS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::BROWSER_LANGUAGE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::BROWSER_MOBILE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::BROWSER_PLATFORM;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CASSANDRA_CONSISTENCY_LEVEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CASSANDRA_COORDINATOR_DC;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CASSANDRA_COORDINATOR_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CASSANDRA_PAGE_SIZE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CASSANDRA_QUERY_IDEMPOTENT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CASSANDRA_SPECULATIVE_EXECUTION_COUNT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CICD_PIPELINE_ACTION_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CICD_PIPELINE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CICD_PIPELINE_RESULT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CICD_PIPELINE_RUN_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CICD_PIPELINE_RUN_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CICD_PIPELINE_RUN_URL_FULL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CICD_PIPELINE_TASK_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CICD_PIPELINE_TASK_RUN_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CICD_PIPELINE_TASK_RUN_RESULT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CICD_PIPELINE_TASK_RUN_URL_FULL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CICD_PIPELINE_TASK_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CICD_SYSTEM_COMPONENT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CICD_WORKER_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CICD_WORKER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CICD_WORKER_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CICD_WORKER_URL_FULL;

#[allow(deprecated)]
pub use crate::attribute::CLIENT_ADDRESS;

#[allow(deprecated)]
pub use crate::attribute::CLIENT_PORT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUD_ACCOUNT_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUD_AVAILABILITY_ZONE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUD_PLATFORM;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUD_PROVIDER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUD_REGION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUD_RESOURCE_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUDEVENTS_EVENT_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUDEVENTS_EVENT_SOURCE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUDEVENTS_EVENT_SPEC_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUDEVENTS_EVENT_SUBJECT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUDEVENTS_EVENT_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUDFOUNDRY_APP_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUDFOUNDRY_APP_INSTANCE_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUDFOUNDRY_APP_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUDFOUNDRY_ORG_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUDFOUNDRY_ORG_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUDFOUNDRY_PROCESS_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUDFOUNDRY_PROCESS_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUDFOUNDRY_SPACE_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUDFOUNDRY_SPACE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUDFOUNDRY_SYSTEM_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CLOUDFOUNDRY_SYSTEM_INSTANCE_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CODE_COLUMN;

#[allow(deprecated)]
pub use crate::attribute::CODE_COLUMN_NUMBER;

#[allow(deprecated)]
pub use crate::attribute::CODE_FILE_PATH;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CODE_FILEPATH;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CODE_FUNCTION;

#[allow(deprecated)]
pub use crate::attribute::CODE_FUNCTION_NAME;

#[allow(deprecated)]
pub use crate::attribute::CODE_LINE_NUMBER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CODE_LINENO;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CODE_NAMESPACE;

#[allow(deprecated)]
pub use crate::attribute::CODE_STACKTRACE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_COMMAND;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_COMMAND_ARGS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_COMMAND_LINE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_CPU_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_CSI_PLUGIN_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_CSI_VOLUME_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_IMAGE_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_IMAGE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_IMAGE_REPO_DIGESTS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_IMAGE_TAGS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_LABEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_LABELS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_RUNTIME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_RUNTIME_DESCRIPTION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_RUNTIME_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CONTAINER_RUNTIME_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CPU_LOGICAL_NUMBER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CPU_MODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::CPYTHON_GC_GENERATION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_CASSANDRA_CONSISTENCY_LEVEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_CASSANDRA_COORDINATOR_DC;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_CASSANDRA_COORDINATOR_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_CASSANDRA_IDEMPOTENCE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_CASSANDRA_PAGE_SIZE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_CASSANDRA_SPECULATIVE_EXECUTION_COUNT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_CASSANDRA_TABLE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_CLIENT_CONNECTION_POOL_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_CLIENT_CONNECTION_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_CLIENT_CONNECTIONS_POOL_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_CLIENT_CONNECTIONS_STATE;

#[allow(deprecated)]
pub use crate::attribute::DB_COLLECTION_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_CONNECTION_STRING;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_COSMOSDB_CLIENT_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_COSMOSDB_CONNECTION_MODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_COSMOSDB_CONSISTENCY_LEVEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_COSMOSDB_CONTAINER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_COSMOSDB_OPERATION_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_COSMOSDB_REGIONS_CONTACTED;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_COSMOSDB_REQUEST_CHARGE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_COSMOSDB_REQUEST_CONTENT_LENGTH;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_COSMOSDB_STATUS_CODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_COSMOSDB_SUB_STATUS_CODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_ELASTICSEARCH_CLUSTER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_ELASTICSEARCH_NODE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_ELASTICSEARCH_PATH_PARTS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_INSTANCE_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_JDBC_DRIVER_CLASSNAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_MONGODB_COLLECTION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_MSSQL_INSTANCE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_NAME;

#[allow(deprecated)]
pub use crate::attribute::DB_NAMESPACE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_OPERATION;

#[allow(deprecated)]
pub use crate::attribute::DB_OPERATION_BATCH_SIZE;

#[allow(deprecated)]
pub use crate::attribute::DB_OPERATION_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_OPERATION_PARAMETER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_QUERY_PARAMETER;

#[allow(deprecated)]
pub use crate::attribute::DB_QUERY_SUMMARY;

#[allow(deprecated)]
pub use crate::attribute::DB_QUERY_TEXT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_REDIS_DATABASE_INDEX;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_RESPONSE_RETURNED_ROWS;

#[allow(deprecated)]
pub use crate::attribute::DB_RESPONSE_STATUS_CODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_SQL_TABLE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_STATEMENT;

#[allow(deprecated)]
pub use crate::attribute::DB_STORED_PROCEDURE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_SYSTEM;

#[allow(deprecated)]
pub use crate::attribute::DB_SYSTEM_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DB_USER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DEPLOYMENT_ENVIRONMENT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DEPLOYMENT_ENVIRONMENT_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DEPLOYMENT_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DEPLOYMENT_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DEPLOYMENT_STATUS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DESTINATION_ADDRESS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DESTINATION_PORT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DEVICE_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DEVICE_MANUFACTURER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DEVICE_MODEL_IDENTIFIER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DEVICE_MODEL_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DISK_IO_DIRECTION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DNS_ANSWERS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::DNS_QUESTION_NAME;

#[allow(deprecated)]
pub use crate::attribute::DOTNET_GC_HEAP_GENERATION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ELASTICSEARCH_NODE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ENDUSER_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ENDUSER_PSEUDO_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ENDUSER_ROLE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ENDUSER_SCOPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ERROR_MESSAGE;

#[allow(deprecated)]
pub use crate::attribute::ERROR_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::EVENT_NAME;

#[allow(deprecated)]
pub use crate::attribute::EXCEPTION_ESCAPED;

#[allow(deprecated)]
pub use crate::attribute::EXCEPTION_MESSAGE;

#[allow(deprecated)]
pub use crate::attribute::EXCEPTION_STACKTRACE;

#[allow(deprecated)]
pub use crate::attribute::EXCEPTION_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FAAS_COLDSTART;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FAAS_CRON;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FAAS_DOCUMENT_COLLECTION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FAAS_DOCUMENT_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FAAS_DOCUMENT_OPERATION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FAAS_DOCUMENT_TIME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FAAS_INSTANCE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FAAS_INVOCATION_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FAAS_INVOKED_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FAAS_INVOKED_PROVIDER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FAAS_INVOKED_REGION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FAAS_MAX_MEMORY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FAAS_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FAAS_TIME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FAAS_TRIGGER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FAAS_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FEATURE_FLAG_CONTEXT_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FEATURE_FLAG_EVALUATION_ERROR_MESSAGE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FEATURE_FLAG_EVALUATION_REASON;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FEATURE_FLAG_KEY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FEATURE_FLAG_PROVIDER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FEATURE_FLAG_RESULT_REASON;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FEATURE_FLAG_RESULT_VALUE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FEATURE_FLAG_RESULT_VARIANT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FEATURE_FLAG_SET_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FEATURE_FLAG_VARIANT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FEATURE_FLAG_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_ACCESSED;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_ATTRIBUTES;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_CHANGED;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_CREATED;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_DIRECTORY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_EXTENSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_FORK_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_GROUP_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_GROUP_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_INODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_MODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_MODIFIED;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_OWNER_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_OWNER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_PATH;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_SIZE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::FILE_SYMBOLIC_LINK_TARGET_PATH;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GCP_APPHUB_APPLICATION_CONTAINER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GCP_APPHUB_APPLICATION_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GCP_APPHUB_APPLICATION_LOCATION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GCP_APPHUB_SERVICE_CRITICALITY_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GCP_APPHUB_SERVICE_ENVIRONMENT_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GCP_APPHUB_SERVICE_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GCP_APPHUB_WORKLOAD_CRITICALITY_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GCP_APPHUB_WORKLOAD_ENVIRONMENT_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GCP_APPHUB_WORKLOAD_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GCP_CLIENT_SERVICE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GCP_CLOUD_RUN_JOB_EXECUTION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GCP_CLOUD_RUN_JOB_TASK_INDEX;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GCP_GCE_INSTANCE_HOSTNAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GCP_GCE_INSTANCE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_AGENT_DESCRIPTION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_AGENT_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_AGENT_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_COMPLETION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_CONVERSATION_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_DATA_SOURCE_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_INPUT_MESSAGES;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_OPENAI_REQUEST_RESPONSE_FORMAT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_OPENAI_REQUEST_SEED;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_OPENAI_REQUEST_SERVICE_TIER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_OPENAI_RESPONSE_SERVICE_TIER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_OPENAI_RESPONSE_SYSTEM_FINGERPRINT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_OPERATION_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_OUTPUT_MESSAGES;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_OUTPUT_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_PROMPT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_PROVIDER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_REQUEST_CHOICE_COUNT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_REQUEST_ENCODING_FORMATS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_REQUEST_FREQUENCY_PENALTY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_REQUEST_MAX_TOKENS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_REQUEST_MODEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_REQUEST_PRESENCE_PENALTY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_REQUEST_SEED;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_REQUEST_STOP_SEQUENCES;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_REQUEST_TEMPERATURE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_REQUEST_TOP_K;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_REQUEST_TOP_P;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_RESPONSE_FINISH_REASONS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_RESPONSE_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_RESPONSE_MODEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_SYSTEM;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_SYSTEM_INSTRUCTIONS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_TOKEN_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_TOOL_CALL_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_TOOL_DESCRIPTION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_TOOL_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_TOOL_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_USAGE_COMPLETION_TOKENS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_USAGE_INPUT_TOKENS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_USAGE_OUTPUT_TOKENS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEN_AI_USAGE_PROMPT_TOKENS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEO_CONTINENT_CODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEO_COUNTRY_ISO_CODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEO_LOCALITY_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEO_LOCATION_LAT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEO_LOCATION_LON;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEO_POSTAL_CODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GEO_REGION_ISO_CODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GO_MEMORY_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GRAPHQL_DOCUMENT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GRAPHQL_OPERATION_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::GRAPHQL_OPERATION_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HEROKU_APP_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HEROKU_RELEASE_COMMIT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HEROKU_RELEASE_CREATION_TIMESTAMP;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HOST_ARCH;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HOST_CPU_CACHE_L2_SIZE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HOST_CPU_FAMILY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HOST_CPU_MODEL_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HOST_CPU_MODEL_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HOST_CPU_STEPPING;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HOST_CPU_VENDOR_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HOST_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HOST_IMAGE_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HOST_IMAGE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HOST_IMAGE_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HOST_IP;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HOST_MAC;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HOST_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HOST_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_CLIENT_IP;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_CONNECTION_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_FLAVOR;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_HOST;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_METHOD;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_REQUEST_BODY_SIZE;

#[allow(deprecated)]
pub use crate::attribute::HTTP_REQUEST_HEADER;

#[allow(deprecated)]
pub use crate::attribute::HTTP_REQUEST_METHOD;

#[allow(deprecated)]
pub use crate::attribute::HTTP_REQUEST_METHOD_ORIGINAL;

#[allow(deprecated)]
pub use crate::attribute::HTTP_REQUEST_RESEND_COUNT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_REQUEST_SIZE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_REQUEST_CONTENT_LENGTH;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_REQUEST_CONTENT_LENGTH_UNCOMPRESSED;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_RESPONSE_BODY_SIZE;

#[allow(deprecated)]
pub use crate::attribute::HTTP_RESPONSE_HEADER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_RESPONSE_SIZE;

#[allow(deprecated)]
pub use crate::attribute::HTTP_RESPONSE_STATUS_CODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_RESPONSE_CONTENT_LENGTH;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_RESPONSE_CONTENT_LENGTH_UNCOMPRESSED;

#[allow(deprecated)]
pub use crate::attribute::HTTP_ROUTE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_SCHEME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_SERVER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_STATUS_CODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_TARGET;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_URL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HTTP_USER_AGENT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_BATTERY_CAPACITY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_BATTERY_CHEMISTRY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_BATTERY_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_BIOS_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_DRIVER_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_ENCLOSURE_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_FIRMWARE_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_GPU_TASK;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_LIMIT_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_LOGICAL_DISK_RAID_LEVEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_LOGICAL_DISK_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_MEMORY_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_MODEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_NETWORK_LOGICAL_ADDRESSES;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_NETWORK_PHYSICAL_ADDRESS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_PARENT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_PHYSICAL_DISK_SMART_ATTRIBUTE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_PHYSICAL_DISK_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_PHYSICAL_DISK_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_SENSOR_LOCATION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_SERIAL_NUMBER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_TAPE_DRIVE_OPERATION_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::HW_VENDOR;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::IOS_APP_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::IOS_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::JVM_BUFFER_POOL_NAME;

#[allow(deprecated)]
pub use crate::attribute::JVM_GC_ACTION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::JVM_GC_CAUSE;

#[allow(deprecated)]
pub use crate::attribute::JVM_GC_NAME;

#[allow(deprecated)]
pub use crate::attribute::JVM_MEMORY_POOL_NAME;

#[allow(deprecated)]
pub use crate::attribute::JVM_MEMORY_TYPE;

#[allow(deprecated)]
pub use crate::attribute::JVM_THREAD_DAEMON;

#[allow(deprecated)]
pub use crate::attribute::JVM_THREAD_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_CLUSTER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_CLUSTER_UID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_CONTAINER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_CONTAINER_RESTART_COUNT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_CONTAINER_STATUS_LAST_TERMINATED_REASON;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_CONTAINER_STATUS_REASON;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_CONTAINER_STATUS_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_CRONJOB_ANNOTATION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_CRONJOB_LABEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_CRONJOB_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_CRONJOB_UID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_DAEMONSET_ANNOTATION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_DAEMONSET_LABEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_DAEMONSET_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_DAEMONSET_UID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_DEPLOYMENT_ANNOTATION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_DEPLOYMENT_LABEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_DEPLOYMENT_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_DEPLOYMENT_UID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_HPA_METRIC_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_HPA_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_HPA_SCALETARGETREF_API_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_HPA_SCALETARGETREF_KIND;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_HPA_SCALETARGETREF_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_HPA_UID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_HUGEPAGE_SIZE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_JOB_ANNOTATION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_JOB_LABEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_JOB_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_JOB_UID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_NAMESPACE_ANNOTATION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_NAMESPACE_LABEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_NAMESPACE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_NAMESPACE_PHASE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_NODE_ANNOTATION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_NODE_CONDITION_STATUS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_NODE_CONDITION_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_NODE_LABEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_NODE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_NODE_UID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_POD_ANNOTATION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_POD_LABEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_POD_LABELS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_POD_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_POD_UID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_REPLICASET_ANNOTATION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_REPLICASET_LABEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_REPLICASET_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_REPLICASET_UID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_REPLICATIONCONTROLLER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_REPLICATIONCONTROLLER_UID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_RESOURCEQUOTA_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_RESOURCEQUOTA_RESOURCE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_RESOURCEQUOTA_UID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_STATEFULSET_ANNOTATION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_STATEFULSET_LABEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_STATEFULSET_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_STATEFULSET_UID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_STORAGECLASS_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_VOLUME_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::K8S_VOLUME_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::LINUX_MEMORY_SLAB_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::LOG_FILE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::LOG_FILE_NAME_RESOLVED;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::LOG_FILE_PATH;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::LOG_FILE_PATH_RESOLVED;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::LOG_IOSTREAM;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::LOG_RECORD_ORIGINAL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::LOG_RECORD_UID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MAINFRAME_LPAR_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGE_COMPRESSED_SIZE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGE_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGE_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGE_UNCOMPRESSED_SIZE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_BATCH_MESSAGE_COUNT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_CLIENT_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_CONSUMER_GROUP_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_DESTINATION_ANONYMOUS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_DESTINATION_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_DESTINATION_PARTITION_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_DESTINATION_SUBSCRIPTION_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_DESTINATION_TEMPLATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_DESTINATION_TEMPORARY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_DESTINATION_PUBLISH_ANONYMOUS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_DESTINATION_PUBLISH_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_EVENTHUBS_CONSUMER_GROUP;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_EVENTHUBS_MESSAGE_ENQUEUED_TIME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_GCP_PUBSUB_MESSAGE_ACK_DEADLINE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_GCP_PUBSUB_MESSAGE_ACK_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_GCP_PUBSUB_MESSAGE_DELIVERY_ATTEMPT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_GCP_PUBSUB_MESSAGE_ORDERING_KEY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_KAFKA_CONSUMER_GROUP;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_KAFKA_DESTINATION_PARTITION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_KAFKA_MESSAGE_KEY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_KAFKA_MESSAGE_OFFSET;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_KAFKA_MESSAGE_TOMBSTONE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_KAFKA_OFFSET;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_MESSAGE_BODY_SIZE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_MESSAGE_CONVERSATION_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_MESSAGE_ENVELOPE_SIZE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_MESSAGE_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_OPERATION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_OPERATION_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_OPERATION_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_RABBITMQ_DESTINATION_ROUTING_KEY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_RABBITMQ_MESSAGE_DELIVERY_TAG;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_ROCKETMQ_CLIENT_GROUP;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_ROCKETMQ_CONSUMPTION_MODEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_ROCKETMQ_MESSAGE_DELAY_TIME_LEVEL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_ROCKETMQ_MESSAGE_DELIVERY_TIMESTAMP;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_ROCKETMQ_MESSAGE_GROUP;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_ROCKETMQ_MESSAGE_KEYS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_ROCKETMQ_MESSAGE_TAG;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_ROCKETMQ_MESSAGE_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_ROCKETMQ_NAMESPACE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_SERVICEBUS_DESTINATION_SUBSCRIPTION_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_SERVICEBUS_DISPOSITION_STATUS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_SERVICEBUS_MESSAGE_DELIVERY_COUNT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_SERVICEBUS_MESSAGE_ENQUEUED_TIME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::MESSAGING_SYSTEM;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NET_HOST_IP;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NET_HOST_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NET_HOST_PORT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NET_PEER_IP;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NET_PEER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NET_PEER_PORT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NET_PROTOCOL_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NET_PROTOCOL_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NET_SOCK_FAMILY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NET_SOCK_HOST_ADDR;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NET_SOCK_HOST_PORT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NET_SOCK_PEER_ADDR;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NET_SOCK_PEER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NET_SOCK_PEER_PORT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NET_TRANSPORT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NETWORK_CARRIER_ICC;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NETWORK_CARRIER_MCC;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NETWORK_CARRIER_MNC;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NETWORK_CARRIER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NETWORK_CONNECTION_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NETWORK_CONNECTION_SUBTYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NETWORK_CONNECTION_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NETWORK_INTERFACE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NETWORK_IO_DIRECTION;

#[allow(deprecated)]
pub use crate::attribute::NETWORK_LOCAL_ADDRESS;

#[allow(deprecated)]
pub use crate::attribute::NETWORK_LOCAL_PORT;

#[allow(deprecated)]
pub use crate::attribute::NETWORK_PEER_ADDRESS;

#[allow(deprecated)]
pub use crate::attribute::NETWORK_PEER_PORT;

#[allow(deprecated)]
pub use crate::attribute::NETWORK_PROTOCOL_NAME;

#[allow(deprecated)]
pub use crate::attribute::NETWORK_PROTOCOL_VERSION;

#[allow(deprecated)]
pub use crate::attribute::NETWORK_TRANSPORT;

#[allow(deprecated)]
pub use crate::attribute::NETWORK_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::NODEJS_EVENTLOOP_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::OCI_MANIFEST_DIGEST;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::OPENAI_REQUEST_SERVICE_TIER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::OPENAI_RESPONSE_SERVICE_TIER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::OPENAI_RESPONSE_SYSTEM_FINGERPRINT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::OPENTRACING_REF_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::OS_BUILD_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::OS_DESCRIPTION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::OS_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::OS_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::OS_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::OTEL_COMPONENT_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::OTEL_COMPONENT_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::OTEL_LIBRARY_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::OTEL_LIBRARY_VERSION;

#[allow(deprecated)]
pub use crate::attribute::OTEL_SCOPE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::OTEL_SCOPE_SCHEMA_URL;

#[allow(deprecated)]
pub use crate::attribute::OTEL_SCOPE_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::OTEL_SPAN_PARENT_ORIGIN;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::OTEL_SPAN_SAMPLING_RESULT;

#[allow(deprecated)]
pub use crate::attribute::OTEL_STATUS_CODE;

#[allow(deprecated)]
pub use crate::attribute::OTEL_STATUS_DESCRIPTION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PEER_SERVICE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::POOL_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_ARGS_COUNT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_COMMAND;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_COMMAND_ARGS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_COMMAND_LINE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_CONTEXT_SWITCH_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_CPU_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_CREATION_TIME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_ENVIRONMENT_VARIABLE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_EXECUTABLE_BUILD_ID_GNU;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_EXECUTABLE_BUILD_ID_GO;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_EXECUTABLE_BUILD_ID_HTLHASH;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_EXECUTABLE_BUILD_ID_PROFILING;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_EXECUTABLE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_EXECUTABLE_PATH;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_EXIT_CODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_EXIT_TIME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_GROUP_LEADER_PID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_INTERACTIVE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_LINUX_CGROUP;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_OWNER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_PAGING_FAULT_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_PARENT_PID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_PID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_REAL_USER_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_REAL_USER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_RUNTIME_DESCRIPTION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_RUNTIME_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_RUNTIME_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_SAVED_USER_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_SAVED_USER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_SESSION_LEADER_PID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_TITLE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_USER_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_USER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_VPID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROCESS_WORKING_DIRECTORY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::PROFILE_FRAME_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::RPC_CONNECT_RPC_ERROR_CODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::RPC_CONNECT_RPC_REQUEST_METADATA;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::RPC_CONNECT_RPC_RESPONSE_METADATA;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::RPC_GRPC_REQUEST_METADATA;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::RPC_GRPC_RESPONSE_METADATA;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::RPC_GRPC_STATUS_CODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::RPC_JSONRPC_ERROR_CODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::RPC_JSONRPC_ERROR_MESSAGE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::RPC_JSONRPC_REQUEST_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::RPC_JSONRPC_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::RPC_MESSAGE_COMPRESSED_SIZE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::RPC_MESSAGE_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::RPC_MESSAGE_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::RPC_MESSAGE_UNCOMPRESSED_SIZE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::RPC_METHOD;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::RPC_SERVICE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::RPC_SYSTEM;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SECURITY_RULE_CATEGORY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SECURITY_RULE_DESCRIPTION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SECURITY_RULE_LICENSE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SECURITY_RULE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SECURITY_RULE_REFERENCE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SECURITY_RULE_RULESET_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SECURITY_RULE_UUID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SECURITY_RULE_VERSION;

#[allow(deprecated)]
pub use crate::attribute::SERVER_ADDRESS;

#[allow(deprecated)]
pub use crate::attribute::SERVER_PORT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SERVICE_INSTANCE_ID;

#[allow(deprecated)]
pub use crate::attribute::SERVICE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SERVICE_NAMESPACE;

#[allow(deprecated)]
pub use crate::attribute::SERVICE_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SESSION_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SESSION_PREVIOUS_ID;

#[allow(deprecated)]
pub use crate::attribute::SIGNALR_CONNECTION_STATUS;

#[allow(deprecated)]
pub use crate::attribute::SIGNALR_TRANSPORT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SOURCE_ADDRESS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SOURCE_PORT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SYSTEM_CPU_LOGICAL_NUMBER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SYSTEM_CPU_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SYSTEM_DEVICE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SYSTEM_FILESYSTEM_MODE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SYSTEM_FILESYSTEM_MOUNTPOINT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SYSTEM_FILESYSTEM_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SYSTEM_FILESYSTEM_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SYSTEM_MEMORY_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SYSTEM_NETWORK_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SYSTEM_PAGING_DIRECTION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SYSTEM_PAGING_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SYSTEM_PAGING_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SYSTEM_PROCESS_STATUS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::SYSTEM_PROCESSES_STATUS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TELEMETRY_DISTRO_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TELEMETRY_DISTRO_VERSION;

#[allow(deprecated)]
pub use crate::attribute::TELEMETRY_SDK_LANGUAGE;

#[allow(deprecated)]
pub use crate::attribute::TELEMETRY_SDK_NAME;

#[allow(deprecated)]
pub use crate::attribute::TELEMETRY_SDK_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TEST_CASE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TEST_CASE_RESULT_STATUS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TEST_SUITE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TEST_SUITE_RUN_STATUS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::THREAD_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::THREAD_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_CIPHER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_CLIENT_CERTIFICATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_CLIENT_CERTIFICATE_CHAIN;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_CLIENT_HASH_MD5;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_CLIENT_HASH_SHA1;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_CLIENT_HASH_SHA256;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_CLIENT_ISSUER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_CLIENT_JA3;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_CLIENT_NOT_AFTER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_CLIENT_NOT_BEFORE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_CLIENT_SERVER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_CLIENT_SUBJECT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_CLIENT_SUPPORTED_CIPHERS;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_CURVE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_ESTABLISHED;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_NEXT_PROTOCOL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_PROTOCOL_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_PROTOCOL_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_RESUMED;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_SERVER_CERTIFICATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_SERVER_CERTIFICATE_CHAIN;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_SERVER_HASH_MD5;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_SERVER_HASH_SHA1;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_SERVER_HASH_SHA256;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_SERVER_ISSUER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_SERVER_JA3S;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_SERVER_NOT_AFTER;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_SERVER_NOT_BEFORE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::TLS_SERVER_SUBJECT;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::URL_DOMAIN;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::URL_EXTENSION;

#[allow(deprecated)]
pub use crate::attribute::URL_FRAGMENT;

#[allow(deprecated)]
pub use crate::attribute::URL_FULL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::URL_ORIGINAL;

#[allow(deprecated)]
pub use crate::attribute::URL_PATH;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::URL_PORT;

#[allow(deprecated)]
pub use crate::attribute::URL_QUERY;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::URL_REGISTERED_DOMAIN;

#[allow(deprecated)]
pub use crate::attribute::URL_SCHEME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::URL_SUBDOMAIN;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::URL_TEMPLATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::URL_TOP_LEVEL_DOMAIN;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::USER_EMAIL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::USER_FULL_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::USER_HASH;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::USER_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::USER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::USER_ROLES;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::USER_AGENT_NAME;

#[allow(deprecated)]
pub use crate::attribute::USER_AGENT_ORIGINAL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::USER_AGENT_OS_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::USER_AGENT_OS_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::USER_AGENT_SYNTHETIC_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::USER_AGENT_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::V8JS_GC_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::V8JS_HEAP_SPACE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_CHANGE_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_CHANGE_STATE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_CHANGE_TITLE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_LINE_CHANGE_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_OWNER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_PROVIDER_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_REF_BASE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_REF_BASE_REVISION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_REF_BASE_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_REF_HEAD_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_REF_HEAD_REVISION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_REF_HEAD_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_REF_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_REPOSITORY_CHANGE_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_REPOSITORY_CHANGE_TITLE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_REPOSITORY_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_REPOSITORY_REF_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_REPOSITORY_REF_REVISION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_REPOSITORY_REF_TYPE;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_REPOSITORY_URL_FULL;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::VCS_REVISION_DELTA_DIRECTION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::WEBENGINE_DESCRIPTION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::WEBENGINE_NAME;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::WEBENGINE_VERSION;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ZOS_SMF_ID;

#[cfg(feature = "semconv_experimental")]
#[allow(deprecated)]
pub use crate::attribute::ZOS_SYSPLEX_NAME;
