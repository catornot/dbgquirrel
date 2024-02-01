use dbgview::init_window;
use debug::DebugState;
use parking_lot::Mutex;
use rrplug::prelude::*;
use stacktrace::StackTrace;
use std::sync::mpsc::{self, Sender};

use crate::hooks::init_hooks;

mod dbgview;
mod debug;
mod hooks;
mod stacktrace;

pub struct VmSpecific<T> {
    server: T,
    client: T,
    ui: T,
}

impl<T: Default> VmSpecific<T> {
    pub fn new() -> Self {
        Self {
            server: T::default(),
            client: T::default(),
            ui: T::default(),
        }
    }

    pub fn get(&self, context: ScriptContext) -> &T {
        match context {
            ScriptContext::SERVER => &self.server,
            ScriptContext::CLIENT => &self.client,
            ScriptContext::UI => &self.ui,
        }
    }

    pub fn get_mut(&mut self, context: ScriptContext) -> &mut T {
        match context {
            ScriptContext::SERVER => &mut self.server,
            ScriptContext::CLIENT => &mut self.client,
            ScriptContext::UI => &mut self.ui,
        }
    }
}

impl<T: Default> Default for VmSpecific<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DebugPlugin {
    pub(crate) send_stack_info: Mutex<Sender<(ScriptContext, StackTrace)>>,
    pub(crate) debug_info: VmSpecific<DebugState>,
}

impl Plugin for DebugPlugin {
    const PLUGIN_INFO: PluginInfo = PluginInfo::new(
        "dbgQuirrel\0",       // name
        "_QUIRREL_\0", // used for the label in the log should be 9 chars long to be consitent
        "DBGQUIRREL\0", // dependency string that mods can use
        PluginContext::all(), // context -> if it has only client it will not load on dedicated servers
    );

    fn new(_reloaded: bool) -> Self {
        let (send, recv) = mpsc::channel();

        std::thread::spawn(move || init_window(recv));

        Self {
            send_stack_info: send.into(),
            debug_info: VmSpecific::new(),
        }
    }

    fn on_dll_load(
        &self,
        _engine_data: Option<&EngineData>,
        dll_ptr: &DLLPointer,
        engine_token: EngineToken,
    ) {
        init_hooks(dll_ptr.which_dll(), engine_token)
    }
}

entry!(DebugPlugin);

/// .
///
/// # Panics
///
/// Panics if .
///
/// # Safety
///
/// .
pub unsafe fn sqvm_to_context(sqvm: *mut HSquirrelVM) -> ScriptContext {
    *(&sqvm
        .as_ref()
        .unwrap()
        .sharedState
        .as_ref()
        .unwrap()
        .cSquirrelVM
        .as_ref()
        .unwrap()
        .vmContext as *const i32)
        .cast::<ScriptContext>()
        .as_ref()
        .unwrap()
}
