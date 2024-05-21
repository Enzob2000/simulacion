use serde::{Deserialize,Serialize};

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Traza{
    pub nombre:String,
    pub traza:String
 
 }