use std::{any::Any, sync::Arc, rc::Weak};

use ospbase::ent::ospbase::datatypes::OSPResult;
use ospdynsvr::ent::ospdynsvr::servicelib::DynLibrary;

use super::{servicecomponent::ServiceComponent, servicecontainer::ServiceContainer, servicecontext::ServiceContext};

pub struct PluginData {}

#[derive(Clone)]
pub struct PluginStub<'svr> {
    pub pluginid: &'svr str,
    pub pluginname: &'svr str,
    pub isenable: bool,
    pub dynlibrary: &'svr DynLibrary<'svr>,
    pub plugincomp: &'svr dyn ServicePlugin,
    pub plugindata: *mut PluginData,
    pub prioplugin: Option<Weak<PluginStub<'svr>>>,
    pub nextplugin: Option<Box<PluginStub<'svr>>>,
}

/**
*
*
*
*/
pub trait ServicePlugin: Any + Send + Sync + 'static {
    fn pluginid(&self) -> &str;
    /// 插件名称
    fn pluginname(&self) -> &str;
    // 初始化插件
    fn init_plugin(
        &self,
        pluginstub: Arc<PluginStub>,
        servicecontainer: Arc<dyn ServiceContainer>,
        servicecontext: Arc<dyn ServiceContext>,
        servicecomponent: Arc<dyn ServiceComponent>
    ) -> OSPResult<u32>;
    // 初始化插件
    fn pre_plugin(
        &self,
        pluginstub: Arc<PluginStub>,
        servicecontainer: Arc<dyn ServiceContainer>,
        servicecontext: Arc<dyn ServiceContext>,
        servicecomponent: Arc<dyn ServiceComponent>
    ) -> OSPResult<u32>;
    /// 执行插件
    fn run_plugin(
        &self,
        pluginstub: Arc<PluginStub>,
        servicecontainer: Arc<dyn ServiceContainer>,
        servicecontext: Arc<dyn ServiceContext>,
        servicecomponent: Arc<dyn ServiceComponent>
    ) -> OSPResult<u32>;
    /// 结束插件
    fn end_plugin(
        &self,
        pluginstub: Arc<PluginStub>,
        servicecontainer: Arc<dyn ServiceContainer>,
        servicecontext: Arc<dyn ServiceContext>,
        servicecomponent: Arc<dyn ServiceComponent>
    ) -> OSPResult<u32>;
    // 是否可执行
    fn is_pluginenable(&self) -> bool {
        true
    }
}
