use std::any::Any;


/**
*
*
*
*/
pub trait ServiceContext : Any + Send + Sync {
     
}

/**
*
*
*
*/
#[derive(Debug)]
struct ServiceContextStruct<'svr> {
    doname     : &'svr String,
}

