use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Pagination utilities for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: u32,
    pub page_size: u32,
    pub offset: u32,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
            offset: 0,
        }
    }
}

impl PaginationParams {
    pub fn new(page: Option<u32>, page_size: Option<u32>) -> Self {
        let page = page.unwrap_or(1).max(1);
        let page_size = page_size.unwrap_or(20).clamp(1, 100);
        let offset = (page - 1) * page_size;
        
        Self {
            page,
            page_size,
            offset,
        }
    }
    
    pub fn calculate_total_pages(total_items: u64, page_size: u32) -> u32 {
        ((total_items as f64) / (page_size as f64)).ceil() as u32
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub current_page: u32,
    pub page_size: u32,
    pub total_items: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_previous: bool,
}

/// Query parameter utilities
pub struct QueryUtils;

impl QueryUtils {
    /// Parse search query with optional filters
    pub fn parse_search_params(
        query: Option<String>,
        filters: HashMap<String, String>,
    ) -> SearchParams {
        SearchParams {
            query: query.map(|q| q.trim().to_string()).filter(|q| !q.is_empty()),
            filters,
        }
    }
    
    /// Parse sorting parameters
    pub fn parse_sort_params(sort_by: Option<String>) -> SortParams {
        let sort_by = sort_by.unwrap_or_else(|| "created_at".to_string());
        
        let (field, direction) = if sort_by.starts_with('-') {
            (sort_by[1..].to_string(), SortDirection::Desc)
        } else {
            (sort_by, SortDirection::Asc)
        };
        
        SortParams { field, direction }
    }
}

#[derive(Debug, Clone)]
pub struct SearchParams {
    pub query: Option<String>,
    pub filters: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct SortParams {
    pub field: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    pub fn as_sql(&self) -> &'static str {
        match self {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        }
    }
}

/// Date and time utilities
pub struct DateUtils;

impl DateUtils {
    /// Get current UTC timestamp
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }
    
    /// Parse ISO 8601 date string
    pub fn parse_iso_date(date_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
        DateTime::parse_from_rfc3339(date_str)
            .map(|dt| dt.with_timezone(&Utc))
    }
    
    /// Format date for API responses
    pub fn format_api_date(date: DateTime<Utc>) -> String {
        date.to_rfc3339()
    }
    
    /// Check if date is within a range
    pub fn is_date_in_range(
        date: DateTime<Utc>,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> bool {
        if let Some(start) = start {
            if date < start {
                return false;
            }
        }
        
        if let Some(end) = end {
            if date > end {
                return false;
            }
        }
        
        true
    }
}

/// UUID utilities
pub struct UuidUtils;

impl UuidUtils {
    /// Generate a new v4 UUID
    pub fn generate() -> Uuid {
        Uuid::new_v4()
    }
    
    /// Parse UUID from string with better error handling
    pub fn parse(uuid_str: &str) -> Result<Uuid, String> {
        Uuid::parse_str(uuid_str)
            .map_err(|_| format!("Invalid UUID format: {}", uuid_str))
    }
    
    /// Check if string is a valid UUID
    pub fn is_valid(uuid_str: &str) -> bool {
        Uuid::parse_str(uuid_str).is_ok()
    }
}

/// Error formatting utilities
pub struct ErrorUtils;

impl ErrorUtils {
    /// Format validation errors for API responses
    pub fn format_validation_errors(errors: Vec<String>) -> HashMap<String, Vec<String>> {
        let mut formatted = HashMap::new();
        formatted.insert("validation_errors".to_string(), errors);
        formatted
    }
    
    /// Create a standard error response
    pub fn create_error_response(error_type: &str, message: &str) -> HashMap<String, String> {
        let mut response = HashMap::new();
        response.insert("error".to_string(), error_type.to_string());
        response.insert("message".to_string(), message.to_string());
        response
    }
}

/// String utilities
pub struct StringUtils;

impl StringUtils {
    /// Generate a slug from a title
    pub fn slugify(input: &str) -> String {
        input
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("-")
    }
    
    /// Truncate string with ellipsis
    pub fn truncate(input: &str, max_length: usize) -> String {
        if input.len() <= max_length {
            input.to_string()
        } else {
            format!("{}...", &input[..max_length.saturating_sub(3)])
        }
    }
    
    /// Clean whitespace (normalize to single spaces)
    pub fn clean_whitespace(input: &str) -> String {
        input
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_params() {
        let params = PaginationParams::new(Some(2), Some(15));
        assert_eq!(params.page, 2);
        assert_eq!(params.page_size, 15);
        assert_eq!(params.offset, 15);
        
        // Test defaults
        let default_params = PaginationParams::new(None, None);
        assert_eq!(default_params.page, 1);
        assert_eq!(default_params.page_size, 20);
        assert_eq!(default_params.offset, 0);
        
        // Test clamping
        let clamped = PaginationParams::new(Some(0), Some(500));
        assert_eq!(clamped.page, 1);
        assert_eq!(clamped.page_size, 100);
    }
    
    #[test]
    fn test_total_pages_calculation() {
        assert_eq!(PaginationParams::calculate_total_pages(0, 20), 0);
        assert_eq!(PaginationParams::calculate_total_pages(19, 20), 1);
        assert_eq!(PaginationParams::calculate_total_pages(20, 20), 1);
        assert_eq!(PaginationParams::calculate_total_pages(21, 20), 2);
        assert_eq!(PaginationParams::calculate_total_pages(100, 25), 4);
    }
    
    #[test]
    fn test_query_utils_search_params() {
        let mut filters = HashMap::new();
        filters.insert("category".to_string(), "test".to_string());
        
        let params = QueryUtils::parse_search_params(
            Some("  search term  ".to_string()),
            filters.clone(),
        );
        
        assert_eq!(params.query, Some("search term".to_string()));
        assert_eq!(params.filters, filters);
        
        // Test empty query
        let empty_params = QueryUtils::parse_search_params(
            Some("   ".to_string()),
            HashMap::new(),
        );
        assert_eq!(empty_params.query, None);
    }
    
    #[test]
    fn test_sort_params() {
        let asc_sort = QueryUtils::parse_sort_params(Some("name".to_string()));
        assert_eq!(asc_sort.field, "name");
        assert_eq!(asc_sort.direction, SortDirection::Asc);
        
        let desc_sort = QueryUtils::parse_sort_params(Some("-created_at".to_string()));
        assert_eq!(desc_sort.field, "created_at");
        assert_eq!(desc_sort.direction, SortDirection::Desc);
        
        let default_sort = QueryUtils::parse_sort_params(None);
        assert_eq!(default_sort.field, "created_at");
        assert_eq!(default_sort.direction, SortDirection::Asc);
    }
    
    #[test]
    fn test_sort_direction_sql() {
        assert_eq!(SortDirection::Asc.as_sql(), "ASC");
        assert_eq!(SortDirection::Desc.as_sql(), "DESC");
    }
    
    #[test]
    fn test_date_utils() {
        let now = DateUtils::now();
        assert!(now <= Utc::now());
        
        let iso_date = "2023-12-25T10:30:00Z";
        let parsed = DateUtils::parse_iso_date(iso_date).unwrap();
        assert_eq!(DateUtils::format_api_date(parsed), "2023-12-25T10:30:00+00:00");
        
        // Test date range
        let test_date = Utc::now();
        let start = test_date - chrono::Duration::hours(1);
        let end = test_date + chrono::Duration::hours(1);
        
        assert!(DateUtils::is_date_in_range(test_date, Some(start), Some(end)));
        assert!(!DateUtils::is_date_in_range(test_date, Some(end), None));
        assert!(!DateUtils::is_date_in_range(test_date, None, Some(start)));
    }
    
    #[test]
    fn test_uuid_utils() {
        let uuid = UuidUtils::generate();
        assert!(UuidUtils::is_valid(&uuid.to_string()));
        
        let parsed = UuidUtils::parse(&uuid.to_string()).unwrap();
        assert_eq!(parsed, uuid);
        
        assert!(UuidUtils::parse("invalid-uuid").is_err());
        assert!(!UuidUtils::is_valid("invalid-uuid"));
    }
    
    #[test]
    fn test_error_utils() {
        let validation_errors = vec![
            "Field is required".to_string(),
            "Invalid format".to_string(),
        ];
        
        let formatted = ErrorUtils::format_validation_errors(validation_errors.clone());
        assert_eq!(formatted.get("validation_errors"), Some(&validation_errors));
        
        let error_response = ErrorUtils::create_error_response("validation_error", "Invalid input");
        assert_eq!(error_response.get("error"), Some(&"validation_error".to_string()));
        assert_eq!(error_response.get("message"), Some(&"Invalid input".to_string()));
    }
    
    #[test]
    fn test_string_utils() {
        assert_eq!(StringUtils::slugify("Hello World!"), "hello-world");
        assert_eq!(StringUtils::slugify("Test@123#Title"), "test-123-title");
        assert_eq!(StringUtils::slugify("Multiple---Dashes"), "multiple-dashes");
        
        assert_eq!(StringUtils::truncate("Short", 10), "Short");
        assert_eq!(StringUtils::truncate("This is a long string", 10), "This is...");
        assert_eq!(StringUtils::truncate("ABC", 3), "ABC");
        
        assert_eq!(StringUtils::clean_whitespace("  multiple   spaces  "), "multiple spaces");
        assert_eq!(StringUtils::clean_whitespace("normal text"), "normal text");
        assert_eq!(StringUtils::clean_whitespace(""), "");
    }
}
