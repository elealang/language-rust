//! Definitions
//! A fundamental Rust representation of Elea that is isomorphic to its 
//! formally defined structure. That is, no indexes, runtime additions, or 
//! changes to structure for ease-of-use.

use serde::{Deserialize, Serialize};

use create::elea::def::Program;


#[derive(Serialize, Deserialize)]
pub struct Spec {
    pub metadata: Metadata,
    pub programs: Vec<Program>,
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
}
