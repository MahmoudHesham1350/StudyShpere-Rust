// src/middleware/mod.rs
//! Middleware module for StudySphere
//! 
//! This module contains HTTP middleware for request processing,
//! authentication, logging, and other cross-cutting concerns.

pub mod auth;

pub use auth::{auth_middleware, optional_auth_middleware, AuthenticatedUser, OptionalAuthenticatedUser};