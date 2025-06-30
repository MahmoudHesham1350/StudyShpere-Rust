// src/middleware/mod.rs
//! Middleware module for StudySphere
//! 
//! This module contains HTTP middleware for request processing,
//! authentication, logging, and other cross-cutting concerns.

pub mod auth;
pub mod authorization;
pub use auth::{auth_middleware, optional_auth_middleware, AuthenticatedUser, OptionalAuthenticatedUser};
pub use authorization::auth_middleware as group_admin_middleware;