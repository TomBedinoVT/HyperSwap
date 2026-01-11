use serde_json::Value;

/// Validates that the encrypted data has the correct structure
/// without attempting to decrypt it
pub fn validate_encrypted_format(encrypted_data: &str) -> Result<(), String> {
    let json: Value = serde_json::from_str(encrypted_data)
        .map_err(|e| format!("Invalid JSON format: {}", e))?;

    // Check required fields
    if !json.get("v").is_some() {
        return Err("Missing 'v' (version) field".to_string());
    }

    if !json.get("iv").is_some() {
        return Err("Missing 'iv' (initialization vector) field".to_string());
    }

    if !json.get("ct").is_some() {
        return Err("Missing 'ct' (ciphertext) field".to_string());
    }

    if !json.get("tag").is_some() {
        return Err("Missing 'tag' (authentication tag) field".to_string());
    }

    // Validate field types
    if let Some(iv) = json.get("iv").and_then(|v| v.as_str()) {
        if iv.len() < 16 {
            return Err("IV too short".to_string());
        }
    } else {
        return Err("'iv' must be a string".to_string());
    }

    if let Some(ct) = json.get("ct").and_then(|v| v.as_str()) {
        if ct.is_empty() {
            return Err("Ciphertext cannot be empty".to_string());
        }
    } else {
        return Err("'ct' must be a string".to_string());
    }

    if let Some(tag) = json.get("tag").and_then(|v| v.as_str()) {
        if tag.len() < 16 {
            return Err("Tag too short".to_string());
        }
    } else {
        return Err("'tag' must be a string".to_string());
    }

    Ok(())
}

