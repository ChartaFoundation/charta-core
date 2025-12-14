/// Evidence type system for Charta
/// 
/// Evidence types enable deterministic collapse of probabilistic inputs.
/// Evidence objects wrap values with confidence, source, and metadata.

use serde::{Deserialize, Serialize};

/// Source of evidence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum EvidenceSource {
    LLM,
    OCR,
    API,
    User,
    Sensor,
}

/// Type of evidence
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceType {
    #[serde(rename = "numeric_estimate")]
    NumericEstimate,
    #[serde(rename = "categorical")]
    Categorical,
    #[serde(rename = "text_extraction")]
    TextExtraction,
    #[serde(rename = "boolean_assertion")]
    BooleanAssertion,
}

/// Evidence value wrapper
/// 
/// Evidence[T] wraps a value of type T with confidence, source, and metadata.
/// This enables deterministic evaluation of probabilistic inputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence<T> {
    /// The actual value
    pub value: T,
    /// Confidence level (0.0 to 1.0)
    #[serde(default = "default_confidence")]
    pub confidence: f64,
    /// Source of the evidence
    pub source: EvidenceSource,
    /// Type of evidence
    #[serde(rename = "evidence_type")]
    pub evidence_type: EvidenceType,
    /// Whether the evidence is disputed
    #[serde(default)]
    pub disputed: bool,
    /// Whether the evidence can be independently verified
    #[serde(default = "default_verifiable")]
    pub verifiable: bool,
    /// Permitted use classes
    #[serde(rename = "permitted_use", default)]
    pub permitted_use: Vec<String>,
}

fn default_confidence() -> f64 {
    1.0
}

fn default_verifiable() -> bool {
    true
}

impl<T> Evidence<T> {
    /// Create new evidence with default values
    pub fn new(
        value: T,
        source: EvidenceSource,
        evidence_type: EvidenceType,
    ) -> Self {
        Self {
            value,
            confidence: 1.0,
            source,
            evidence_type,
            disputed: false,
            verifiable: true,
            permitted_use: Vec::new(),
        }
    }

    /// Create evidence with confidence
    pub fn with_confidence(
        value: T,
        source: EvidenceSource,
        evidence_type: EvidenceType,
        confidence: f64,
    ) -> Self {
        Self {
            value,
            confidence: confidence.clamp(0.0, 1.0),
            source,
            evidence_type,
            disputed: false,
            verifiable: true,
            permitted_use: Vec::new(),
        }
    }

    /// Check if evidence meets confidence threshold
    pub fn meets_threshold(&self, threshold: f64) -> bool {
        self.confidence >= threshold && !self.disputed
    }

    /// Check if evidence is admissible for a use case
    pub fn is_admissible_for(&self, use_case: &str) -> bool {
        self.permitted_use.is_empty() || self.permitted_use.contains(&use_case.to_string())
    }
}

/// Evidence value for boolean signals
pub type EvidenceBool = Evidence<bool>;

/// Evidence value for numeric signals
pub type EvidenceNumeric = Evidence<f64>;

/// Evidence value for text signals
pub type EvidenceText = Evidence<String>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evidence_creation() {
        let evidence = Evidence::new(
            true,
            EvidenceSource::LLM,
            EvidenceType::BooleanAssertion,
        );
        
        assert_eq!(evidence.value, true);
        assert_eq!(evidence.confidence, 1.0);
        assert_eq!(evidence.source, EvidenceSource::LLM);
        assert!(!evidence.disputed);
    }

    #[test]
    fn test_evidence_threshold() {
        let evidence = Evidence::with_confidence(
            true,
            EvidenceSource::LLM,
            EvidenceType::BooleanAssertion,
            0.95,
        );
        
        assert!(evidence.meets_threshold(0.90));
        assert!(!evidence.meets_threshold(0.96));
    }

    #[test]
    fn test_evidence_disputed() {
        let mut evidence = Evidence::new(
            true,
            EvidenceSource::LLM,
            EvidenceType::BooleanAssertion,
        );
        
        assert!(evidence.meets_threshold(0.5));
        
        evidence.disputed = true;
        assert!(!evidence.meets_threshold(0.5));
    }
}
