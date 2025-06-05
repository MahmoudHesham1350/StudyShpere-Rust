use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
    pub token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    Access,
    Refresh,
}

impl Claims {
    pub fn new(user_id: Uuid, token_type: TokenType, expires_in_minutes: i64) -> Self {
        let now = Utc::now();
        let exp = now + Duration::minutes(expires_in_minutes);
        
        Self {
            sub: user_id.to_string(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
            token_type,
        }
    }
    
    pub fn user_id(&self) -> Result<Uuid> {
        Uuid::parse_str(&self.sub).map_err(Into::into)
    }
}

pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtManager {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
        }
    }
    
    pub fn generate_access_token(&self, user_id: Uuid) -> Result<String> {
        let claims = Claims::new(user_id, TokenType::Access, 15); // 15 minutes
        self.encode_token(&claims)
    }
    
    pub fn generate_refresh_token(&self, user_id: Uuid) -> Result<String> {
        let claims = Claims::new(user_id, TokenType::Refresh, 10080); // 7 days
        self.encode_token(&claims)
    }
    
    pub fn verify_token(&self, token: &str) -> Result<TokenData<Claims>> {
        let mut validation = Validation::default();
        validation.validate_exp = true; // Enable expiration validation
        
        decode::<Claims>(token, &self.decoding_key, &validation)
            .map_err(Into::into)
    }
    
    pub fn verify_access_token(&self, token: &str) -> Result<Claims> {
        let token_data = self.verify_token(token)?;
        
        if token_data.claims.token_type != TokenType::Access {
            return Err(anyhow::anyhow!("Invalid token type"));
        }
        
        Ok(token_data.claims)
    }
    
    pub fn verify_refresh_token(&self, token: &str) -> Result<Claims> {
        let token_data = self.verify_token(token)?;
        
        if token_data.claims.token_type != TokenType::Refresh {
            return Err(anyhow::anyhow!("Invalid token type"));
        }
        
        Ok(token_data.claims)
    }
    
    fn encode_token(&self, claims: &Claims) -> Result<String> {
        encode(&Header::default(), claims, &self.encoding_key)
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claims_creation() {
        let user_id = Uuid::new_v4();
        let claims = Claims::new(user_id, TokenType::Access, 15);
        
        assert_eq!(claims.user_id().unwrap(), user_id);
        assert_eq!(claims.token_type, TokenType::Access);
        assert!(claims.exp > claims.iat);
    }
    
    #[test]
    fn test_jwt_manager_access_token() {
        let manager = JwtManager::new("test_secret");
        let user_id = Uuid::new_v4();
        
        let token = manager.generate_access_token(user_id).unwrap();
        let claims = manager.verify_access_token(&token).unwrap();
        
        assert_eq!(claims.user_id().unwrap(), user_id);
        assert_eq!(claims.token_type, TokenType::Access);
    }
    
    #[test]
    fn test_jwt_manager_refresh_token() {
        let manager = JwtManager::new("test_secret");
        let user_id = Uuid::new_v4();
        
        let token = manager.generate_refresh_token(user_id).unwrap();
        let claims = manager.verify_refresh_token(&token).unwrap();
        
        assert_eq!(claims.user_id().unwrap(), user_id);
        assert_eq!(claims.token_type, TokenType::Refresh);
    }
    
    #[test]
    fn test_invalid_token_type() {
        let manager = JwtManager::new("test_secret");
        let user_id = Uuid::new_v4();
        
        let access_token = manager.generate_access_token(user_id).unwrap();
        let refresh_token = manager.generate_refresh_token(user_id).unwrap();
        
        // Try to verify access token as refresh token
        assert!(manager.verify_refresh_token(&access_token).is_err());
        
        // Try to verify refresh token as access token
        assert!(manager.verify_access_token(&refresh_token).is_err());
    }
    
    #[test]
    fn test_invalid_secret() {
        let manager1 = JwtManager::new("secret1");
        let manager2 = JwtManager::new("secret2");
        let user_id = Uuid::new_v4();
        
        let token = manager1.generate_access_token(user_id).unwrap();
        
        // Token signed with different secret should fail verification
        assert!(manager2.verify_access_token(&token).is_err());
    }
    
    #[test]
    fn test_expired_token() {
        let user_id = Uuid::new_v4();
        let expired_claims = Claims::new(user_id, TokenType::Access, -3); // Already expired
        
        let manager = JwtManager::new("test_secret");
        let token = manager.encode_token(&expired_claims).unwrap();
        
        // Expired token should fail verification
        assert!(manager.verify_access_token(&token).is_err());
    }
    
    #[test]
    fn test_malformed_token() {
        let manager = JwtManager::new("test_secret");
        
        assert!(manager.verify_access_token("invalid.token.here").is_err());
        assert!(manager.verify_access_token("").is_err());
        assert!(manager.verify_access_token("not.a.jwt").is_err());
    }
}
