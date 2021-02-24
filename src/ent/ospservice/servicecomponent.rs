use std::{any::Any, sync::Arc, rc::Weak};

use ospbase::ent::ospbase::datatypes::OSPResult;
//use ospdynsvr::ent::ospdynsvr::servicelib::DynLibrary;

use super::{
    servicecontainer::ServiceContainer, servicecontext::ServiceContext, serviceplugin::PluginStub,
};

pub struct ServiceData {}

#[derive(Clone)]
pub struct ServiceStub<'svr> {
    pub serviceid: &'svr str,
    pub servicename: &'svr str,
    pub isenable: bool,
//    pub dynlibrary: Weak<DynLibrary<'svr>>,
    pub servicecomp: &'svr dyn ServiceComponent,
    pub servicedata: *mut ServiceData,
    pub pluginlist: Vec<&'svr PluginStub<'svr>>,
    pub prioservice: Option<Weak<ServiceStub<'svr>>>,
    pub nextservice: Option<Box<ServiceStub<'svr>>>,
}

/**
*
*
*
*/
pub trait ServiceComponent: Any + Send + Sync + 'static {
    // 组件ID
    fn compid(&self) -> &str;
    /// 组件名称
    fn compname(&self) -> &str;
    // 初始化组件
    fn init_component<'svr>(
        &self,
        servicestub: &'svr mut ServiceStub,
        servicecontainer: &'svr mut dyn ServiceContainer,
        servicecontext: &'svr mut dyn ServiceContext,
    ) -> OSPResult<u32>;
    // 准备组件
    fn pre_component<'svr>(
        &self,
        servicestub: Arc<&'svr mut ServiceStub>,
        servicecontainer: Arc<&'svr mut dyn ServiceContainer>,
        servicecontext: Arc<&'svr mut dyn ServiceContext>,
    ) -> OSPResult<u32>;
    /// 执行组件
    fn run_component<'svr>(
        &self,
        servicestub: Arc<&'svr mut ServiceStub>,
        servicecontainer: Arc<&'svr mut dyn ServiceContainer>,
        servicecontext: Arc<&'svr mut dyn ServiceContext>,
    ) -> OSPResult<u32>;
    /// 结束组件
    fn end_component<'svr>(
        &self,
        servicestub: Arc<&'svr mut ServiceStub>,
        servicecontainer: Arc<&'svr mut dyn ServiceContainer>,
        servicecontext: Arc<&'svr mut dyn ServiceContext>,
    ) -> OSPResult<u32>;
    // 是否可执行
    fn is_compenable(&self) -> bool {
        true
    }
}
