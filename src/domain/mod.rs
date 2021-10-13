

pub mod libs;
pub mod exprocess;
pub mod repository;

pub type Runner = libs::exprocess::Runner<exprocess::AppCore,repository::AppRepository>;