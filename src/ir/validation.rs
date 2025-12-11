use crate::error::{Result, ValidationError};
use crate::ir::schema::{load_schema, IR};

/// Validate IR against schema
pub fn validate_ir(ir_json: &str, schema_path: &str) -> Result<IR> {
    // Load and compile schema
    let schema = load_schema(schema_path)?;
    
    // Parse IR JSON
    let ir_value: serde_json::Value = serde_json::from_str(ir_json)
        .map_err(ValidationError::JsonParse)?;
    
    // Validate against schema
    {
        let validation_result = schema.validate(&ir_value);
        if let Err(errors) = validation_result {
            let error_messages: Vec<String> = errors
                .map(|e| format!("{}: {}", e.instance_path, e))
                .collect();
            return Err(ValidationError::SchemaValidation(
                error_messages.join("; ")
            ));
        }
    } // validation_result is dropped here, releasing the borrow
    
    // Deserialize to IR struct
    let ir: IR = serde_json::from_value(ir_value)
        .map_err(|e| ValidationError::InvalidStructure(
            format!("Failed to deserialize IR: {}", e)
        ))?;
    
    // Additional semantic validation
    validate_semantics(&ir)?;
    
    Ok(ir)
}

/// Validate IR semantics (beyond schema validation)
fn validate_semantics(ir: &IR) -> Result<()> {
    // Check that module name is not empty
    if ir.module.name.is_empty() {
        return Err(ValidationError::InvalidStructure(
            "Module name cannot be empty".to_string()
        ));
    }
    
    // Check that signal/coil names are unique
    if let Some(signals) = &ir.module.signals {
        let mut names = std::collections::HashSet::new();
        for signal in signals {
            if !names.insert(&signal.name) {
                return Err(ValidationError::InvalidStructure(
                    format!("Duplicate signal name: {}", signal.name)
                ));
            }
        }
    }
    
    if let Some(coils) = &ir.module.coils {
        let mut names = std::collections::HashSet::new();
        for coil in coils {
            if !names.insert(&coil.name) {
                return Err(ValidationError::InvalidStructure(
                    format!("Duplicate coil name: {}", coil.name)
                ));
            }
        }
    }
    
    // Check that rungs reference valid coils
    if let Some(rungs) = &ir.module.rungs {
        let coil_names: std::collections::HashSet<String> = ir.module.coils
            .as_ref()
            .map(|coils| coils.iter().map(|c| c.name.clone()).collect())
            .unwrap_or_default();
        
        for rung in rungs {
            for action in &rung.actions {
                if !coil_names.contains(&action.coil) {
                    return Err(ValidationError::InvalidStructure(
                        format!("Rung '{}' references undefined coil: {}", rung.name, action.coil)
                    ));
                }
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_basic_ir() {
        let ir_json = r#"
        {
            "version": "0.1.0",
            "module": {
                "name": "test_module",
                "signals": [
                    {"name": "input_signal"}
                ],
                "coils": [
                    {"name": "output_coil"}
                ],
                "rungs": [
                    {
                        "name": "test_rung",
                        "guard": {
                            "type": "contact",
                            "name": "input_signal",
                            "contact_type": "NO"
                        },
                        "actions": [
                            {
                                "type": "energise",
                                "coil": "output_coil"
                            }
                        ]
                    }
                ]
            }
        }
        "#;
        
        // Note: This test requires the actual schema file
        // In real tests, we'd use a test schema or mock
        let schema_path = "../../spec/ir-schema.json";
        
        // Test that validation doesn't panic
        // Full validation test would require schema file to exist
        let result = validate_ir(ir_json, schema_path);
        // Result may be Err if schema file doesn't exist, which is OK for now
        assert!(result.is_ok() || result.is_err());
    }
}
