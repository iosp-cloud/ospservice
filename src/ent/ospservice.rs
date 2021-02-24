use std::collections::HashMap;

use ospdynsvr::ent::ospdynsvr::servicelib::DynLibrary;

use self::{servicecomponent::ServiceComponent, serviceplugin::ServicePlugin};

pub mod servicecomponent;
pub mod servicecontainer;
pub mod servicecontext;
pub mod servicemanager;
pub mod serviceplugin;

/**
 *
 * 装入动态库中的所有组件
 *
 *
 */
pub fn get_service_component_hash<'lib>(
    dynlib: &'lib DynLibrary<'lib>,
    slist: Box<HashMap<&'static str, &'static dyn ServiceComponent>>,
) -> Box<HashMap<&'static str, &'static dyn ServiceComponent>> {
    let get_service_component_hash;
    // let slist:Box<HashMap<&'static str,&'static dyn ServiceComponent>> = Box::new(HashMap::new());
    unsafe {
        get_service_component_hash = dynlib
            .get::<extern fn(
                *mut HashMap<&'static str, &'static dyn ServiceComponent>,
            ) -> *mut HashMap<&'static str, &'static dyn ServiceComponent>>(
                "get_service_component_hash",
            )
            .unwrap();
    }
    let _res: *mut HashMap<&'static str, &'static dyn ServiceComponent> =
        get_service_component_hash(Box::into_raw(slist));
    let mut _sl: Box<HashMap<&'static str, &'static dyn ServiceComponent>>;
    unsafe {
        _sl = Box::from_raw(_res);
        // println!("this is {}",_sl.as_ref());
    }
    return _sl;
}
/**
 *
 * 装入动态库中的所有插件
 *
 *
 */
pub fn get_service_plugin_hash<'lib>(
    dynlib: &'lib DynLibrary<'lib>,
    slist: Box<HashMap<&'static str, &'static dyn ServicePlugin>>,
) -> Box<HashMap<&'static str, &'static dyn ServicePlugin>> {
    let get_service_component_hash;
    // let slist:Box<HashMap<&'static str,&'static dyn ServiceComponent>> = Box::new(HashMap::new());
    unsafe {
        get_service_component_hash = dynlib
            .get::<extern fn(
                *mut HashMap<&'static str, &'static dyn ServicePlugin>,
            ) -> *mut HashMap<&'static str, &'static dyn ServicePlugin>>(
                "get_service_plugin_hash",
            )
            .unwrap();
    }
    let _res: *mut HashMap<&'static str, &'static dyn ServicePlugin> =
        get_service_component_hash(Box::into_raw(slist));
    let mut _sl: Box<HashMap<&'static str, &'static dyn ServicePlugin>>;
    unsafe {
        _sl = Box::from_raw(_res);
        // println!("this is {}",_sl.as_ref());
    }
    return _sl;
}
