use retour::{Error as RetourError, GenericDetour};
use rrplug::{
    bindings::{
        class_types::cplayer::CPlayer,
        squirrelclasstypes::*,
        squirreldatatypes::*,
        squirrelfunctions::{SQUIRREL_CLIENT_FUNCS, SQUIRREL_SERVER_FUNCS},
    },
    mid::utils::from_char_ptr,
    prelude::*,
};
use std::cell::{Ref, RefCell};

use crate::{exports::PLUGIN, sqvm_to_context, stacktrace::StackTrace};

pub static CLIENT_DETOURS: EngineGlobal<RefCell<Option<DetouredSquirrelFunctions>>> =
    EngineGlobal::new(RefCell::new(None));
pub static SERVER_DETOURS: EngineGlobal<RefCell<Option<DetouredSquirrelFunctions>>> =
    EngineGlobal::new(RefCell::new(None));

pub struct DetouredSquirrelFunctions {
    // pub register_squirrel_func: RegisterSquirrelFuncType, // hooked by northstar :(
    pub sq_defconst: GenericDetour<sq_defconstType>,
    pub sq_compilebuffer: GenericDetour<sq_compilebufferType>,
    pub sq_call: GenericDetour<sq_callType>,
    pub sq_raiseerror: GenericDetour<sq_raiseerrorType>,
    pub sq_compilefile: GenericDetour<sq_compilefileType>,
    pub sq_newarray: GenericDetour<sq_newarrayType>,
    pub sq_arrayappend: GenericDetour<sq_arrayappendType>,
    pub sq_newtable: GenericDetour<sq_newtableType>,
    pub sq_newslot: GenericDetour<sq_newslotType>,
    pub sq_pushroottable: GenericDetour<sq_pushroottableType>,
    pub sq_pushstring: GenericDetour<sq_pushstringType>,
    pub sq_pushinteger: GenericDetour<sq_pushintegerType>,
    pub sq_pushfloat: GenericDetour<sq_pushfloatType>,
    pub sq_pushbool: GenericDetour<sq_pushboolType>,
    pub sq_pushasset: GenericDetour<sq_pushassetType>,
    pub sq_pushvector: GenericDetour<sq_pushvectorType>,
    pub sq_pushobject: GenericDetour<sq_pushobjectType>,
    pub sq_getstring: GenericDetour<sq_getstringType>,
    pub sq_getinteger: GenericDetour<sq_getintegerType>,
    pub sq_getfloat: GenericDetour<sq_getfloatType>,
    pub sq_getbool: GenericDetour<sq_getboolType>,
    pub sq_get: GenericDetour<sq_getType>,
    pub sq_getasset: GenericDetour<sq_getassetType>,
    pub sq_getuserdata: GenericDetour<sq_getuserdataType>,
    pub sq_getvector: GenericDetour<sq_getvectorType>,
    pub sq_getthisentity: GenericDetour<sq_getthisentityType>,
    pub sq_getobject: GenericDetour<sq_getobjectType>,
    pub sq_stackinfos: GenericDetour<sq_stackinfosType>,
    pub sq_createuserdata: GenericDetour<sq_createuserdataType>,
    pub sq_setuserdatatypeid: GenericDetour<sq_setuserdatatypeidType>,
    pub sq_getfunction: GenericDetour<sq_getfunctionType>,
    pub sq_getentityfrominstance: GenericDetour<sq_getentityfrominstanceType>,
    pub sq_pushnewstructinstance: GenericDetour<sq_pushnewstructinstanceType>,
    pub sq_sealstructslot: GenericDetour<sq_sealstructslotType>,
}

impl DetouredSquirrelFunctions {
    pub fn try_new(funcs: &SquirrelFunctions) -> Result<Self, RetourError> {
        unsafe {
            let detours = Self {
                sq_defconst: GenericDetour::new(funcs.sq_defconst, hook_sq_defconst)?,
                sq_compilebuffer: GenericDetour::new(
                    funcs.sq_compilebuffer,
                    hook_sq_compilebuffer,
                )?,
                sq_call: GenericDetour::new(funcs.sq_call, hook_sq_call)?,
                sq_raiseerror: GenericDetour::new(funcs.sq_raiseerror, hook_sq_raiseerror)?,
                sq_compilefile: GenericDetour::new(funcs.sq_compilefile, hook_sq_compilefile)?,
                sq_newarray: GenericDetour::new(funcs.sq_newarray, hook_sq_newarray)?,
                sq_arrayappend: GenericDetour::new(funcs.sq_arrayappend, hook_sq_arrayappend)?,
                sq_newtable: GenericDetour::new(funcs.sq_newtable, hook_sq_newtable)?,
                sq_newslot: GenericDetour::new(funcs.sq_newslot, hook_sq_newslot)?,
                sq_pushroottable: GenericDetour::new(
                    funcs.sq_pushroottable,
                    hook_sq_pushroottable,
                )?,
                sq_pushstring: GenericDetour::new(funcs.sq_pushstring, hook_sq_pushstring)?,
                sq_pushinteger: GenericDetour::new(funcs.sq_pushinteger, hook_sq_pushinteger)?,
                sq_pushfloat: GenericDetour::new(funcs.sq_pushfloat, hook_sq_pushfloat)?,
                sq_pushbool: GenericDetour::new(funcs.sq_pushbool, hook_sq_pushbool)?,
                sq_pushasset: GenericDetour::new(funcs.sq_pushasset, hook_sq_pushasset)?,
                sq_pushvector: GenericDetour::new(funcs.sq_pushvector, hook_sq_pushvector)?,
                sq_pushobject: GenericDetour::new(funcs.sq_pushobject, hook_sq_pushobject)?,
                sq_getstring: GenericDetour::new(funcs.sq_getstring, hook_sq_getstring)?,
                sq_getinteger: GenericDetour::new(funcs.sq_getinteger, hook_sq_getinteger)?,
                sq_getfloat: GenericDetour::new(funcs.sq_getfloat, hook_sq_getfloat)?,
                sq_getbool: GenericDetour::new(funcs.sq_getbool, hook_sq_getbool)?,
                sq_get: GenericDetour::new(funcs.sq_get, hook_sq_get)?,
                sq_getasset: GenericDetour::new(funcs.sq_getasset, hook_sq_getasset)?,
                sq_getuserdata: GenericDetour::new(funcs.sq_getuserdata, hook_sq_getuserdata)?,
                sq_getvector: GenericDetour::new(funcs.sq_getvector, hook_sq_getvector)?,
                sq_getthisentity: GenericDetour::new(
                    funcs.sq_getthisentity,
                    hook_sq_getthisentity,
                )?,
                sq_getobject: GenericDetour::new(funcs.sq_getobject, hook_sq_getobject)?,
                sq_stackinfos: GenericDetour::new(funcs.sq_stackinfos, hook_sq_stackinfos)?,
                sq_createuserdata: GenericDetour::new(
                    funcs.sq_createuserdata,
                    hook_sq_createuserdata,
                )?,
                sq_setuserdatatypeid: GenericDetour::new(
                    funcs.sq_setuserdatatypeid,
                    hook_sq_setuserdatatypeid,
                )?,
                sq_getfunction: GenericDetour::new(funcs.sq_getfunction, hook_sq_getfunction)?,
                sq_getentityfrominstance: GenericDetour::new(
                    funcs.sq_getentityfrominstance,
                    hook_sq_getentityfrominstance,
                )?,

                sq_pushnewstructinstance: GenericDetour::new(
                    funcs.sq_pushnewstructinstance,
                    hook_sq_pushnewstructinstance,
                )?,
                sq_sealstructslot: GenericDetour::new(
                    funcs.sq_sealstructslot,
                    hook_sq_sealstructslot,
                )?,
            };
            Ok(detours)
        }
    }

    fn enable(self) -> Result<Self, RetourError> {
        unsafe {
            self.sq_defconst.enable()?;
            self.sq_compilebuffer.enable()?;
            self.sq_call.enable()?;
            self.sq_raiseerror.enable()?;
            self.sq_compilefile.enable()?;
            self.sq_newarray.enable()?;
            self.sq_arrayappend.enable()?;
            self.sq_newtable.enable()?;
            self.sq_newslot.enable()?;
            self.sq_pushroottable.enable()?;
            self.sq_pushstring.enable()?;
            self.sq_pushinteger.enable()?;
            self.sq_pushfloat.enable()?;
            self.sq_pushbool.enable()?;
            self.sq_pushasset.enable()?;
            self.sq_pushvector.enable()?;
            self.sq_pushobject.enable()?;
            self.sq_getstring.enable()?;
            self.sq_getinteger.enable()?;
            self.sq_getfloat.enable()?;
            self.sq_getbool.enable()?;
            self.sq_get.enable()?;
            self.sq_getasset.enable()?;
            self.sq_getuserdata.enable()?;
            self.sq_getvector.enable()?;
            self.sq_getthisentity.enable()?;
            self.sq_getobject.enable()?;
            self.sq_stackinfos.enable()?;
            self.sq_createuserdata.enable()?;
            self.sq_setuserdatatypeid.enable()?;
            self.sq_getfunction.enable()?;
            self.sq_getentityfrominstance.enable()?;
            self.sq_pushnewstructinstance.enable()?;
            self.sq_sealstructslot.enable()?;
        }
        Ok(self)
    }
}

pub fn init_hooks(dll: &WhichDll, engine_token: EngineToken) {
    match dll {
        WhichDll::Client => {
            _ = CLIENT_DETOURS.get(engine_token).borrow_mut().replace(
                DetouredSquirrelFunctions::try_new(
                    &SQUIRREL_CLIENT_FUNCS
                        .get()
                        .expect("client functions where not init at this time, weird!")
                        .into(),
                )
                .expect("failed to init client hooks")
                .enable()
                .expect("failed to enable hooks"),
            )
        }
        WhichDll::Server => {
            _ = SERVER_DETOURS.get(engine_token).borrow_mut().replace(
                DetouredSquirrelFunctions::try_new(
                    &SQUIRREL_SERVER_FUNCS
                        .get()
                        .expect("server functions where not init at this time, weird!")
                        .into(),
                )
                .expect("failed to init client hooks")
                .enable()
                .expect("failed to enable hooks"),
            )
        }
        _ => {}
    };
}

fn hooks_from_sqvm(sqvm: *mut HSquirrelVM) -> Ref<'static, Option<DetouredSquirrelFunctions>> {
    let engine_token = unsafe { EngineToken::new_unchecked() }; // will only be called in sq function hooks so it's sound
    match unsafe { sqvm_to_context(sqvm) } {
        ScriptContext::SERVER => SERVER_DETOURS.get(engine_token).borrow(),
        _ => CLIENT_DETOURS.get(engine_token).borrow(),
    }
}

fn try_debug(sqvm: *mut HSquirrelVM) {
    let plugin = PLUGIN.wait();
    let context = unsafe { sqvm_to_context(sqvm) };

    _ = plugin
        .send_stack_info
        .lock()
        .send((context, StackTrace::DebugBegin(8)));

    match context {
        ScriptContext::SERVER if *plugin.debug_info.server.paused.lock() => {
            _ = plugin.debug_info.server.unpause_waiter.lock().recv()
        }
        ScriptContext::CLIENT if *plugin.debug_info.client.paused.lock() => {
            _ = plugin.debug_info.client.unpause_waiter.lock().recv()
        }
        ScriptContext::UI if *plugin.debug_info.ui.paused.lock() => {
            _ = plugin.debug_info.ui.unpause_waiter.lock().recv()
        }
        _ => {}
    }
}

fn push_log(sqvm: *mut HSquirrelVM, log: StackTrace) {
    _ = PLUGIN
        .wait()
        .send_stack_info
        .lock()
        .send((unsafe { sqvm_to_context(sqvm) }, log));
}

pub unsafe extern "C" fn hook_sq_defconst(
    sqvm: *mut CSquirrelVM,
    name: *const SQChar,
    value: ::std::os::raw::c_int,
) {
    let cssqvm = unsafe { (*sqvm).sqvm };
    try_debug(cssqvm);
    hooks_from_sqvm(cssqvm)
        .as_ref()
        .unwrap()
        .sq_defconst
        .call(sqvm, name, value);
}
pub extern "C" fn hook_sq_compilebuffer(
    sqvm: *mut HSquirrelVM,
    compile_buffer: *mut CompileBufferState,
    file: *const ::std::os::raw::c_char,
    a1: ::std::os::raw::c_int,
    should_throw_error: SQBool,
) -> SQRESULT {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_compilebuffer
            .call(sqvm, compile_buffer, file, a1, should_throw_error)
    }
}
pub unsafe extern "C" fn hook_sq_call(
    sqvm: *mut HSquirrelVM,
    args: SQInteger,
    should_return: SQBool,
    throw_error: SQBool,
) -> SQRESULT {
    push_log(sqvm, StackTrace::func_called(sqvm));
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_call
            .call(sqvm, args, should_return, throw_error)
    }
}
pub unsafe extern "C" fn hook_sq_raiseerror(
    sqvm: *mut HSquirrelVM,
    error: *const SQChar,
) -> SQInteger {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_raiseerror
            .call(sqvm, error)
    }
}
pub unsafe extern "C" fn hook_sq_compilefile(
    sqvm: *mut CSquirrelVM,
    path: *const ::std::os::raw::c_char,
    name: *const ::std::os::raw::c_char,
    a4: ::std::os::raw::c_int,
) -> bool {
    let cssqvm = unsafe { (*sqvm).sqvm };
    try_debug(cssqvm);
    unsafe {
        hooks_from_sqvm(cssqvm)
            .as_ref()
            .unwrap()
            .sq_compilefile
            .call(sqvm, path, name, a4)
    }
}
pub unsafe extern "C" fn hook_sq_newarray(sqvm: *mut HSquirrelVM, stackpos: SQInteger) {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_newarray
            .call(sqvm, stackpos)
    }
}
pub unsafe extern "C" fn hook_sq_arrayappend(
    sqvm: *mut HSquirrelVM,
    stackpos: SQInteger,
) -> SQRESULT {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_arrayappend
            .call(sqvm, stackpos)
    }
}
pub unsafe extern "C" fn hook_sq_newtable(sqvm: *mut HSquirrelVM) -> SQRESULT {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_newtable
            .call(sqvm)
    }
}
pub unsafe extern "C" fn hook_sq_newslot(
    sqvm: *mut HSquirrelVM,
    idx: SQInteger,
    _static: SQBool,
) -> SQRESULT {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_newslot
            .call(sqvm, idx, _static)
    }
}
pub unsafe extern "C" fn hook_sq_pushroottable(sqvm: *mut HSquirrelVM) {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_pushroottable
            .call(sqvm)
    }
}
pub unsafe extern "C" fn hook_sq_pushstring(
    sqvm: *mut HSquirrelVM,
    str: *const SQChar,
    length: SQInteger,
) {
    push_log(sqvm, StackTrace::value_pushed(from_char_ptr::<String>(str)));
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_pushstring
            .call(sqvm, str, length)
    }
}
pub unsafe extern "C" fn hook_sq_pushinteger(sqvm: *mut HSquirrelVM, i: SQInteger) {
    push_log(sqvm, StackTrace::value_pushed(i));
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_pushinteger
            .call(sqvm, i)
    }
}
pub unsafe extern "C" fn hook_sq_pushfloat(sqvm: *mut HSquirrelVM, f: SQFloat) {
    push_log(sqvm, StackTrace::value_pushed(f));
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_pushfloat
            .call(sqvm, f)
    }
}
pub unsafe extern "C" fn hook_sq_pushbool(sqvm: *mut HSquirrelVM, b: SQBool) {
    push_log(sqvm, StackTrace::value_pushed(b));
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_pushbool
            .call(sqvm, b)
    }
}
pub unsafe extern "C" fn hook_sq_pushasset(
    sqvm: *mut HSquirrelVM,
    str: *const SQChar,
    length: SQInteger,
) {
    push_log(sqvm, StackTrace::value_pushed(from_char_ptr::<String>(str)));
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_pushasset
            .call(sqvm, str, length)
    }
}
pub unsafe extern "C" fn hook_sq_pushvector(sqvm: *mut HSquirrelVM, vec: *const SQFloat) {
    push_log(
        sqvm,
        StackTrace::value_pushed(format!("{:?}", Vector3::from(vec.cast()))),
    );
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_pushvector
            .call(sqvm, vec)
    }
}
pub unsafe extern "C" fn hook_sq_pushobject(sqvm: *mut HSquirrelVM, obj: *mut SQObject) {
    push_log(sqvm, StackTrace::value_pushed("A Object"));
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_pushobject
            .call(sqvm, obj)
    }
}
pub unsafe extern "C" fn hook_sq_getstring(
    sqvm: *mut HSquirrelVM,
    stackpos: SQInteger,
) -> *const SQChar {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_getstring
            .call(sqvm, stackpos)
    }
}
pub unsafe extern "C" fn hook_sq_getinteger(
    sqvm: *mut HSquirrelVM,
    stackpos: SQInteger,
) -> SQInteger {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_getinteger
            .call(sqvm, stackpos)
    }
}
pub unsafe extern "C" fn hook_sq_getfloat(sqvm: *mut HSquirrelVM, stackpos: SQInteger) -> SQFloat {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_getfloat
            .call(sqvm, stackpos)
    }
}
pub unsafe extern "C" fn hook_sq_getbool(sqvm: *mut HSquirrelVM, stackpos: SQInteger) -> SQBool {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_getbool
            .call(sqvm, stackpos)
    }
}
pub unsafe extern "C" fn hook_sq_get(sqvm: *mut HSquirrelVM, stackpos: SQInteger) -> SQRESULT {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_get
            .call(sqvm, stackpos)
    }
}
pub unsafe extern "C" fn hook_sq_getasset(
    sqvm: *mut HSquirrelVM,
    stackpos: SQInteger,
    result: *mut *const ::std::os::raw::c_char,
) -> SQRESULT {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_getasset
            .call(sqvm, stackpos, result)
    }
}
pub unsafe extern "C" fn hook_sq_getuserdata(
    sqvm: *mut HSquirrelVM,
    stackpos: SQInteger,
    data: *mut *mut ::std::os::raw::c_void,
    type_id: *mut u64,
) -> SQRESULT {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_getuserdata
            .call(sqvm, stackpos, data, type_id)
    }
}
pub unsafe extern "C" fn hook_sq_getvector(
    sqvm: *mut HSquirrelVM,
    stackpos: SQInteger,
) -> *mut SQFloat {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_getvector
            .call(sqvm, stackpos)
    }
}
pub unsafe extern "C" fn hook_sq_getthisentity(
    sqvm: *mut HSquirrelVM,
    entity: *mut *mut ::std::os::raw::c_void,
) -> SQBool {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_getthisentity
            .call(sqvm, entity)
    }
}
pub unsafe extern "C" fn hook_sq_getobject(
    sqvm: *mut HSquirrelVM,
    stack_pos: SQInteger,
    out_obj: *mut SQObject,
) {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_getobject
            .call(sqvm, stack_pos, out_obj)
    }
}
pub unsafe extern "C" fn hook_sq_stackinfos(
    sqvm: *mut HSquirrelVM,
    level: ::std::os::raw::c_int,
    out_obj: *mut SQStackInfos,
    call_stack_size: ::std::os::raw::c_int,
) -> ::std::os::raw::c_longlong {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm).as_ref().unwrap().sq_stackinfos.call(
            sqvm,
            level,
            out_obj,
            call_stack_size,
        )
    }
}
pub unsafe extern "C" fn hook_sq_createuserdata(
    sqvm: *mut HSquirrelVM,
    size: SQInteger,
) -> *mut ::std::os::raw::c_void {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_createuserdata
            .call(sqvm, size)
    }
}
pub unsafe extern "C" fn hook_sq_setuserdatatypeid(
    sqvm: *mut HSquirrelVM,
    stackpos: SQInteger,
    type_id: u64,
) -> SQRESULT {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_setuserdatatypeid
            .call(sqvm, stackpos, type_id)
    }
}
pub unsafe extern "C" fn hook_sq_getentityfrominstance(
    sqvm: *mut CSquirrelVM,
    instance: *mut SQObject,
    entity_constant: *mut *mut ::std::os::raw::c_char,
) -> *mut CPlayer {
    let hssqvm = unsafe { (*sqvm).sqvm };
    try_debug(hssqvm);
    unsafe {
        hooks_from_sqvm(hssqvm)
            .as_ref()
            .unwrap()
            .sq_getentityfrominstance
            .call(sqvm, instance, entity_constant)
    }
}
pub unsafe extern "C" fn hook_sq_getfunction(
    sqvm: *mut HSquirrelVM,
    name: *const ::std::os::raw::c_char,
    return_obj: *mut SQObject,
    signature: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    push_log(
        sqvm,
        StackTrace::value_gotten(from_char_ptr::<String>(name)),
    );
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_getfunction
            .call(sqvm, name, return_obj, signature)
    }
}
pub unsafe extern "C" fn hook_sq_pushnewstructinstance(
    sqvm: *mut HSquirrelVM,
    field_count: ::std::os::raw::c_int,
) -> SQRESULT {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_pushnewstructinstance
            .call(sqvm, field_count)
    }
}
pub unsafe extern "C" fn hook_sq_sealstructslot(
    sqvm: *mut HSquirrelVM,
    slot_index: ::std::os::raw::c_int,
) -> SQRESULT {
    try_debug(sqvm);
    unsafe {
        hooks_from_sqvm(sqvm)
            .as_ref()
            .unwrap()
            .sq_sealstructslot
            .call(sqvm, slot_index)
    }
}
