use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use anyhow::Result;
use validator::{Validate, ValidationError};
use regex::Regex;

/// Password utility functions for hashing and verification
pub struct PasswordUtils;

impl PasswordUtils {
    /// Hash a password using Argon2
    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?;
        
        Ok(password_hash.to_string())
    }
    
    /// Verify a password against a hash
    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| anyhow::anyhow!("Invalid hash format: {}", e))?;
        
        let argon2 = Argon2::default();
        
        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    
    /// Check password strength
    pub fn validate_password_strength(password: &str) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if password.len() < 8 {
            errors.push("Password must be at least 8 characters long".to_string());
        }
        
        if password.len() > 128 {
            errors.push("Password must be no more than 128 characters long".to_string());
        }
        
        if !password.chars().any(|c| c.is_ascii_lowercase()) {
            errors.push("Password must contain at least one lowercase letter".to_string());
        }
        
        if !password.chars().any(|c| c.is_ascii_uppercase()) {
            errors.push("Password must contain at least one uppercase letter".to_string());
        }
        
        if !password.chars().any(|c| c.is_ascii_digit()) {
            errors.push("Password must contain at least one digit".to_string());
        }
        
        if !password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)) {
            errors.push("Password must contain at least one special character".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Email validation utilities
pub struct EmailUtils;

impl EmailUtils {
    /// Basic email validation using regex
    pub fn is_valid_email(email: &str) -> bool {
        let email_regex = Regex::new(
            r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
        ).unwrap();
        
        email_regex.is_match(email) && email.len() <= 254
    }
    
    /// Normalize email (lowercase, trim)
    pub fn normalize_email(email: &str) -> String {
        email.trim().to_lowercase()
    }
}

/// Username validation utilities
pub struct UsernameUtils;

impl UsernameUtils {
    /// Validate username format
    pub fn is_valid_username(username: &str) -> bool {
        let username_regex = Regex::new(r"^[a-zA-Z0-9_-]{3,30}$").unwrap();
        username_regex.is_match(username)
    }
    
    /// Check if username contains only allowed characters
    pub fn validate_username_characters(username: &str) -> Result<(), String> {
        if username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }
        
        if username.len() < 3 {
            return Err("Username must be at least 3 characters long".to_string());
        }
        
        if username.len() > 30 {
            return Err("Username must be no more than 30 characters long".to_string());
        }
        
        if !username.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
            return Err("Username can only contain letters, numbers, underscores, and hyphens".to_string());
        }
        
        Ok(())
    }
}

/// Text sanitization utilities
pub struct TextUtils;

impl TextUtils {
    /// Sanitize text input (trim whitespace, limit length)
    pub fn sanitize_text(text: &str, max_length: usize) -> String {
        text.trim()
            .chars()
            .take(max_length)
            .collect()
    }
    
    /// Remove dangerous HTML characters
    pub fn escape_html(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }
    
    /// Check if text contains only printable ASCII characters
    pub fn is_safe_text(text: &str) -> bool {
        text.chars().all(|c| c.is_ascii_graphic() || c.is_ascii_whitespace())
    }
}

/// Custom validator for password confirmation
pub fn validate_password_match(password: &str, confirm_password: &str) -> Result<(), ValidationError> {
    if password != confirm_password {
        return Err(ValidationError::new("password_mismatch"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "TestPassword123!";
        let hash = PasswordUtils::hash_password(password).unwrap();
        
        assert_ne!(hash, password);
        assert!(hash.len() > 0);
        
        // Verify the password
        assert!(PasswordUtils::verify_password(password, &hash).unwrap());
        assert!(!PasswordUtils::verify_password("wrong_password", &hash).unwrap());
    }
    
    #[test]
    fn test_password_strength_validation() {
        // Valid password
        assert!(PasswordUtils::validate_password_strength("StrongPass123!").is_ok());
        
        // Too short
        assert!(PasswordUtils::validate_password_strength("short").is_err());
        
        // No uppercase
        assert!(PasswordUtils::validate_password_strength("lowercase123!").is_err());
        
        // No lowercase
        assert!(PasswordUtils::validate_password_strength("UPPERCASE123!").is_err());
        
        // No digits
        assert!(PasswordUtils::validate_password_strength("NoDigitsHere!").is_err());
        
        // No special characters
        assert!(PasswordUtils::validate_password_strength("NoSpecialChars123").is_err());
        
        // Too long
        let long_password = "a".repeat(129);
        assert!(PasswordUtils::validate_password_strength(&long_password).is_err());
    }
    
    #[test]
    fn test_email_validation() {
        // Valid emails
        assert!(EmailUtils::is_valid_email("user@example.com"));
        assert!(EmailUtils::is_valid_email("test.email+tag@domain.co.uk"));
        assert!(EmailUtils::is_valid_email("user123@test-domain.org"));
        
        // Invalid emails
        assert!(!EmailUtils::is_valid_email("invalid.email"));
        assert!(!EmailUtils::is_valid_email("@domain.com"));
        assert!(!EmailUtils::is_valid_email("user@"));
        assert!(!EmailUtils::is_valid_email("user@domain"));
        assert!(!EmailUtils::is_valid_email(""));
        
        // Too long email
        let long_email = format!("{}@example.com", "a".repeat(250));
        assert!(!EmailUtils::is_valid_email(&long_email));
    }
    
    #[test]
    fn test_email_normalization() {
        assert_eq!(EmailUtils::normalize_email("  User@Example.COM  "), "user@example.com");
        assert_eq!(EmailUtils::normalize_email("TEST@DOMAIN.ORG"), "test@domain.org");
    }
    
    #[test]
    fn test_username_validation() {
        // Valid usernames
        assert!(UsernameUtils::is_valid_username("user123"));
        assert!(UsernameUtils::is_valid_username("test_user"));
        assert!(UsernameUtils::is_valid_username("my-username"));
        assert!(UsernameUtils::is_valid_username("abc"));
        
        // Invalid usernames
        assert!(!UsernameUtils::is_valid_username("ab")); // Too short
        assert!(!UsernameUtils::is_valid_username("user@domain")); // Invalid characters
        assert!(!UsernameUtils::is_valid_username("user name")); // Space
        assert!(!UsernameUtils::is_valid_username("")); // Empty
    }
    
    #[test]
    fn test_username_character_validation() {
        assert!(UsernameUtils::validate_username_characters("valid_user").is_ok());
        assert!(UsernameUtils::validate_username_characters("user-123").is_ok());
        
        assert!(UsernameUtils::validate_username_characters("").is_err());
        assert!(UsernameUtils::validate_username_characters("ab").is_err());
        assert!(UsernameUtils::validate_username_characters("user@domain").is_err());
    }
    
    #[test]
    fn test_text_sanitization() {
        assert_eq!(TextUtils::sanitize_text("  hello world  ", 100), "hello world");
        assert_eq!(TextUtils::sanitize_text("very long text", 5), "very ");
        assert_eq!(TextUtils::sanitize_text("", 10), "");
    }
    
    #[test]
    fn test_html_escaping() {
        assert_eq!(TextUtils::escape_html("<script>alert('xss')</script>"), 
                   "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;");
        assert_eq!(TextUtils::escape_html("\"quoted\" & 'apostrophe'"), 
                   "&quot;quoted&quot; &amp; &#39;apostrophe&#39;");
    }
    
    #[test]
    fn test_safe_text() {
        assert!(TextUtils::is_safe_text("Hello World 123!"));
        assert!(TextUtils::is_safe_text("Test with spaces and punctuation."));
        assert!(!TextUtils::is_safe_text("Text with \x00 null byte"));
    }
    
    #[test]
    fn test_password_match_validation() {
        assert!(validate_password_match("password123", "password123").is_ok());
        assert!(validate_password_match("password123", "different").is_err());
    }
}
