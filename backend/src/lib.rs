pub mod app;
mod auth;
mod db;
mod errors;
mod model;
mod password_recovery;
mod repository;
mod routes;
mod service;
mod middleware;

pub use errors::Result;
