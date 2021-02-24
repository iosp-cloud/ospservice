use std::{any::Any, sync::Arc};

use super::servicecomponent::ServiceStub;


pub struct ContainerData{}

pub trait ServiceContainer: Any + Send + Sync {

}

#[derive(Clone)]
pub struct ServiceContainerStruct<'svr> {
    pub serviceid   : &'svr str,
    pub servicename : &'svr str,
    pub pre_svr     : Option<Arc<ServiceStub<'svr>>>,
    pub run_svr     : Option<Arc<ServiceStub<'svr>>>,
    pub end_svr     : Option<Arc<ServiceStub<'svr>>>,
    pub tran_type   : u32,
    pub db_type     : u32,
    pub mem_type    : u32,
    pub mq_type     : u32,
    pub ms_type     : u32,
    pub sidecar_type: u32,
}
/**
*
*
**/
impl <'svr>ServiceContainerStruct<'svr> {

}

// pub trait ServiceContainer: Any + Send + Sync {



// }