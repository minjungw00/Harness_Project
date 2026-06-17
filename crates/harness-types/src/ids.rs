use std::fmt;

use serde::{Deserialize, Serialize};

macro_rules! opaque_string_type {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Creates an opaque string identifier wrapper.
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            /// Returns the identifier as a string slice.
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consumes the wrapper and returns the underlying string.
            pub fn into_inner(self) -> String {
                self.0
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }
    };
}

opaque_string_type!(ProjectId, "Opaque project identifier.");
opaque_string_type!(TaskId, "Opaque Task identifier.");
opaque_string_type!(SurfaceId, "Opaque local surface identifier.");
opaque_string_type!(
    SurfaceInstanceId,
    "Opaque local surface-instance identifier."
);
opaque_string_type!(RequestId, "Opaque request identifier.");
opaque_string_type!(IdempotencyKey, "Opaque idempotency-key identifier.");
opaque_string_type!(EventId, "Opaque event identifier.");
opaque_string_type!(RecordId, "Opaque state-record identifier.");
opaque_string_type!(BaselineRef, "Opaque baseline identifier.");
opaque_string_type!(ChangeUnitId, "Opaque Change Unit identifier.");
opaque_string_type!(
    WriteAuthorizationId,
    "Opaque Write Authorization identifier."
);
opaque_string_type!(RunId, "Opaque Run identifier.");
opaque_string_type!(ArtifactId, "Opaque artifact identifier.");
opaque_string_type!(
    ArtifactInputId,
    "Opaque request-local artifact input identifier."
);
opaque_string_type!(
    StagedArtifactHandleId,
    "Opaque staged-artifact handle identifier."
);
opaque_string_type!(UserJudgmentId, "Opaque user-judgment identifier.");
opaque_string_type!(
    UserJudgmentOptionId,
    "Opaque judgment-local option identifier."
);
opaque_string_type!(RiskId, "Opaque residual-risk identifier.");
opaque_string_type!(StorageRef, "Opaque artifact storage reference.");
opaque_string_type!(RequestHash, "Deterministic canonical request hash string.");
