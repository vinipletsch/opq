use std::thread;
use std::sync::{Arc, Mutex};
use std::boxed::Box;


use pub_sub::Subscription;
use libloading::{Library, Symbol};
use zmq;
use serde_json;

use event_requester::{EventRequester, SyncEventRequester};
use opqapi::MakaiPlugin;
use opqapi::protocol::TriggerMessage;
use config::Settings;

pub struct PluginManager {
    loaded_libraries: Vec<Library>,
    plugin_threads: Vec<thread::JoinHandle<()>>,
    trigger: SyncEventRequester,
}

impl PluginManager {
    pub fn new(ctx: &zmq::Context, settings: &Settings) -> PluginManager {
        PluginManager {
            //plugins : Vec::new(),
            loaded_libraries: Vec::new(),
            plugin_threads: Vec::new(),
            trigger: Arc::new(Mutex::new(EventRequester::new(ctx, settings))),
        }
    }

    /// Load a plugin from a DLL or shared library.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because there are no guarantees that the
    /// plugin loaded will be correct. In particular, all plugins must be
    /// compiled against the *exact* same version of the library that will be
    /// loading them!
    ///
    /// Failure to ensure ABI compatibility will most probably result in UB
    /// because the vtable we expect to get (from `Box<Plugin>`) and the vtable
    /// we actually get may be completely different.
    pub unsafe fn load_plugin(
        &mut self,
        mut document: serde_json::Value,
        subscription: Subscription<Arc<TriggerMessage>>,

    ) -> Result<(), String> {
        let filename = document.get("path").unwrap().as_str().unwrap().to_string();
        type PluginCreate = unsafe fn() -> *mut MakaiPlugin;

        let lib = Library::new(filename).or(Err("No such file or directory."))?;

        // We need to keep the library around otherwise our plugin's vtable will
        // point to garbage. We do this little dance to make sure the library
        // doesn't end up getting moved.
        self.loaded_libraries.push(lib);

        let lib = self.loaded_libraries.last().unwrap();

        let constructor: Symbol<PluginCreate> = lib.get(b"_plugin_create")
            .or(Err("Could not load symbol."))?;
        let boxed_raw = constructor();

        let mut plugin = Box::from_raw(boxed_raw);

        let trigger = self.trigger.clone();
        self.plugin_threads.push(thread::spawn(move || {
            plugin.on_plugin_load(document.to_string());
            loop {
                let msg = subscription.recv().unwrap();
                let res = plugin.process_measurement(msg);
                match res {
                    Some(x) => trigger.lock().unwrap().trigger(x),
                    None => (),
                };
            }
        }));

        Ok(())
    }
}
