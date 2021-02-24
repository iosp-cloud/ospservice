use std::{collections::HashMap};

use ospbase::ent::ospbase::{request::Request, response::Response};

use super::{
    servicecomponent::ServiceComponent, servicecontainer::ServiceContainer,
    serviceplugin::ServicePlugin,
};

/**
*
*
*
*/
#[derive(Default)]
struct ServiceManager<'svr> {
    // 系统中加载的所有服务组件Map
    _servicecomponent_map: HashMap<&'svr str, &'svr dyn ServiceComponent>,
    // 系统中加载的所有服务插件Map
    _serviceplugin_map: HashMap<&'svr str, &'svr dyn ServicePlugin>,
    // 在当前所处的运行Namespace中已经加载的ServiceContainer
    _servicecontainer_map: HashMap<&'svr str, &'svr dyn ServiceContainer>,
}
/**
 *
 *
 *
 */
impl<'svr> ServiceManager<'svr> {
    /**
     *
     *
     *
     */
    pub fn _run_service(
        &self,
        _servicekey: &'svr str,
        _request: &'svr Request,
        _response: &'svr mut Response,
    ) -> &'svr Response {
        let sc = self._get_service_container(_servicekey,_request);
        match sc {
            Some(servicecontainer) => {
                return self._run_servicecontainer(servicecontainer,_request,_response);
            }
            None => {}
        }

        return _response;
    }

    pub fn _run_servicecontainer(
        &self,
        _sc: &'svr dyn ServiceContainer,
        _request: &'svr Request,
        _response: &'svr Response,
    ) -> &'svr Response {
        return _response;
    }
    /**
     *
     *
     *
     */
    pub fn _get_service_container(
        &self,
        _servicekey: &'svr str,
        _request: &Request,
    ) -> Option<&'svr dyn ServiceContainer> {
        //let mut _obc : Option<Box<ServiceContainer<'svr>>>;
        // 首先从内存中的hash
        // let _obc = self._get_sc_fromhash(_servicekey, _request);
        // match _obc {
        //     Option::Some(_) => return _obc,
        //     Option::None => return self._get_sc_fromdb(_servicekey, _request),
        // }
        return Option::None;
    }
    /**
     *
     *
     *
     */
    fn _get_sc_fromdb(
        &self,
        _servicekey: &str,
        _request: &Request,
    ) -> Option<&'svr dyn ServiceContainer> {
        //
        //if self._servicecontainer_map.contains_key(_servicekey) {
        //return self._load_fromdb.get(_servicekey,_request);
        //} else {
        return Option::None;
        //}
    }

    /**
     *
     *
     *
     */
    fn _get_sc_fromhash(
        &self,
        _servicekey: &str,
        _request: &Request,
    ) -> Option<&'svr dyn ServiceContainer>  {
        //
        //if self._servicecontainer_map.contains_key(_servicekey) {
            // self._servicecontainer_map.
        // return self._servicecontainer_map.get(_servicekey);
        //} else {
        return Option::None;
        //}
    }
}
