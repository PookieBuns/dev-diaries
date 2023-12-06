pub mod app;
mod auth;
mod db;
mod errors;
mod middleware;
mod model;
mod password_recovery;
mod repository;
mod routes;
mod service;

pub use errors::Result;
