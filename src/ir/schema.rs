use serde::{Deserialize, Serialize};
use crate::error::{Result, ValidationError};

/// Charta Intermediate Representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IR {
    pub version: String,
    pub module: Module,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<Intent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signals: Option<Vec<SignalDecl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coils: Option<Vec<CoilDecl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rungs: Option<Vec<RungDecl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<BlockDecl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<NetworkDecl>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_privacy: Option<DataPrivacy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<Quality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<Cost>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPrivacy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii_handling: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quality {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_precision: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_recall: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cost_per_submission: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalDecl {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoilDecl {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latching: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RungDecl {
    pub name: String,
    pub guard: GuardExpr,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GuardExpr {
    #[serde(rename = "contact")]
    Contact {
        name: String,
        contact_type: String, // "NO" or "NC"
        #[serde(skip_serializing_if = "Option::is_none")]
        arguments: Option<Vec<Expr>>,
    },
    #[serde(rename = "and")]
    And {
        left: Box<GuardExpr>,
        right: Box<GuardExpr>,
    },
    #[serde(rename = "or")]
    Or {
        left: Box<GuardExpr>,
        right: Box<GuardExpr>,
    },
    #[serde(rename = "not")]
    Not {
        expr: Box<GuardExpr>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Expr {
    String(String),
    Number(f64),
    Boolean(bool),
    Identifier(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    #[serde(rename = "type")]
    pub action_type: String, // "energise" or "de_energise"
    pub coil: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<Expr>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDecl {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<PortDecl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<PortDecl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortDecl {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDecl {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wires: Option<Vec<Wire>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wire {
    pub source: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    pub name: String,
    pub source: String,
}

/// Load IR schema from file
pub fn load_schema(schema_path: &str) -> Result<jsonschema::JSONSchema> {
    let schema_content = std::fs::read_to_string(schema_path)?;
    let schema_json: serde_json::Value = serde_json::from_str(&schema_content)?;
    let schema = jsonschema::JSONSchema::compile(&schema_json)
        .map_err(|e| ValidationError::SchemaLoad(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to compile schema: {}", e)
        )))?;
    Ok(schema)
}
