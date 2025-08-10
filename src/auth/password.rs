use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

pub fn verify_password(password: &str, hashed: &str) -> anyhow::Result<bool> {
    let is_valid = verify(password, hashed)?;
    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::{assert_err, assert_ok};

    #[test]
    fn test_hash_password_success() {
        let password = "test_password123";
        let result = hash_password(password);
        
        let hashed = result.unwrap();
        
        // Hash should not be empty and should not equal the original password
        assert!(!hashed.is_empty());
        assert_ne!(hashed, password);
        
        // Should start with bcrypt prefix
        assert!(hashed.starts_with("$2b$"));
    }

    #[test]
    fn test_hash_password_empty() {
        let result = hash_password("");
        let hashed = result.unwrap();
        assert!(!hashed.is_empty());
        assert!(hashed.starts_with("$2b$"));
    }

    #[test]
    fn test_hash_password_long() {
        let long_password = "a".repeat(1000);
        let result = hash_password(&long_password);
        let hashed = result.unwrap();
        assert!(!hashed.is_empty());
        assert!(hashed.starts_with("$2b$"));
    }

    #[test]
    fn test_hash_password_special_chars() {
        let special_password = "!@#$%^&*()_+-=[]{}|;:,.<>?";
        let result = hash_password(special_password);
        let hashed = result.unwrap();
        assert!(!hashed.is_empty());
        assert!(hashed.starts_with("$2b$"));
    }

    #[test]
    fn test_hash_password_unicode() {
        let unicode_password = "пароль密码🔒";
        let result = hash_password(unicode_password);
        let hashed = result.unwrap();
        assert!(!hashed.is_empty());
        assert!(hashed.starts_with("$2b$"));
    }

    #[test]
    fn test_different_passwords_produce_different_hashes() {
        let password1 = "password1";
        let password2 = "password2";
        
        let hash1 = hash_password(password1).unwrap();
        let hash2 = hash_password(password2).unwrap();
        
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_same_password_produces_different_hashes() {
        let password = "same_password";
        
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();
        
        // Due to salt, same password should produce different hashes
        assert_ne!(hash1, hash2);
        
        // But both should verify successfully
        assert!(verify_password(password, &hash1).unwrap());
        assert!(verify_password(password, &hash2).unwrap());
    }

    #[test]
    fn test_verify_password_success() {
        let password = "correct_password";
        let hashed = hash_password(password).unwrap();
        
        let result = verify_password(password, &hashed);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_verify_password_failure() {
        let correct_password = "correct_password";
        let wrong_password = "wrong_password";
        let hashed = hash_password(correct_password).unwrap();
        
        let result = verify_password(wrong_password, &hashed);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_verify_password_empty_password() {
        let password = "";
        let hashed = hash_password(password).unwrap();
        
        let result = verify_password(password, &hashed);
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        // Wrong password should fail
        let wrong_result = verify_password("not_empty", &hashed);
        assert_ok!(wrong_result);
        assert!(!wrong_result.unwrap());
    }

    #[test]
    fn test_verify_password_invalid_hash() {
        let password = "test_password";
        let invalid_hash = "invalid_hash_format";
        
        let result = verify_password(password, invalid_hash);
        assert_err!(result);
    }

    #[test]
    fn test_verify_password_empty_hash() {
        let password = "test_password";
        
        let result = verify_password(password, "");
        assert_err!(result);
    }

    #[test]
    fn test_verify_password_malformed_hash() {
        let password = "test_password";
        let malformed_hash = "$2b$invalid";
        
        let result = verify_password(password, malformed_hash);
        assert_err!(result);
    }

    #[test]
    fn test_case_sensitive_password() {
        let password = "CaseSensitive";
        let hashed = hash_password(password).unwrap();
        
        // Correct case should work
        assert!(verify_password(password, &hashed).unwrap());
        
        // Wrong case should fail
        assert!(!verify_password("casesensitive", &hashed).unwrap());
        assert!(!verify_password("CASESENSITIVE", &hashed).unwrap());
    }

    #[test]
    fn test_whitespace_sensitive_password() {
        let password = "password with spaces";
        let hashed = hash_password(password).unwrap();
        
        // Exact match should work
        assert!(verify_password(password, &hashed).unwrap());
        
        // Different whitespace should fail
        assert!(!verify_password("passwordwithspaces", &hashed).unwrap());
        assert!(!verify_password(" password with spaces ", &hashed).unwrap());
        assert!(!verify_password("password  with  spaces", &hashed).unwrap());
    }

    #[test]
    fn test_long_password_verification() {
        let long_password = "a".repeat(500);
        let hashed = hash_password(&long_password).unwrap();
        
        assert!(verify_password(&long_password, &hashed).unwrap());
        
        // bcrypt truncates passwords at 72 bytes, so test a difference within that limit
        let different_password = "b".repeat(70) + "aa";
        assert!(!verify_password(&different_password, &hashed).unwrap());
    }

    #[test]
    fn test_unicode_password_verification() {
        let unicode_password = "密码测试🔐مرور";
        let hashed = hash_password(unicode_password).unwrap();
        
        assert!(verify_password(unicode_password, &hashed).unwrap());
        
        let different_unicode = "密码测试🔐مرو"; // Last character different
        assert!(!verify_password(different_unicode, &hashed).unwrap());
    }

    #[test]
    fn test_hash_consistency() {
        let password = "consistency_test";
        
        // Generate multiple hashes
        let hashes: Vec<String> = (0..5)
            .map(|_| hash_password(password).unwrap())
            .collect();
        
        // All should be different due to random salt
        for i in 0..hashes.len() {
            for j in i + 1..hashes.len() {
                assert_ne!(hashes[i], hashes[j]);
            }
        }
        
        // But all should verify correctly
        for hash in &hashes {
            assert!(verify_password(password, hash).unwrap());
        }
    }

    #[test]
    fn test_bcrypt_format_validation() {
        let password = "test_password";
        let hashed = hash_password(password).unwrap();
        
        // Should have correct bcrypt format: $2b$cost$salthash
        let parts: Vec<&str> = hashed.split('$').collect();
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0], ""); // Empty before first $
        assert_eq!(parts[1], "2b");
        
        // Cost should be a number
        let cost: u32 = parts[2].parse().expect("Cost should be a number");
        assert_eq!(cost, DEFAULT_COST);
        
        // Salt and hash should be 53 characters (22 salt + 31 hash)
        assert_eq!(parts[3].len(), 53);
    }
}