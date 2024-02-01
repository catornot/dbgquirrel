use std::{ffi::CStr, fmt::Display};

use rrplug::{bindings::squirreldatatypes::SQClosure, high::squirrel::SQHandle, prelude::*};

pub enum StackTrace {
    Call(String),
    Pushed(String),
    Aquired(String),
    Misc(String),
    DebugBegin(i32),
}

impl StackTrace {
    pub fn value_pushed<T>(value: T) -> Self
    where
        T: ToString,
    {
        Self::Pushed(value.to_string())
    }

    pub fn value_gotten<T>(value: T) -> Self
    where
        T: ToString,
    {
        Self::Pushed(value.to_string())
    }

    pub fn func_called(sqvm: *mut HSquirrelVM) -> Self {
        // SQFUNCTIONS.from_sqvm(sqvm).

        unsafe {
            let sqvm = sqvm.as_mut().unwrap();
            // let info = sqvm._callstack.as_mut().unwrap();
            let function_name = (0..20)
                .filter_map(|i| sqvm._stack.add(i).as_ref())
                .find_map(|object| SQHandle::<SQClosure>::new(*object).ok())
                .map(|closure| {
                    CStr::from_ptr(
                        closure
                            .take()
                            ._VAL
                            .asClosure
                            .as_mut()
                            .unwrap()
                            ._function
                            ._VAL
                            .asString
                            .as_ref()
                            .unwrap()
                            ._val
                            .as_ptr(),
                    )
                    .to_string_lossy()
                    .to_string()
                })
                .unwrap_or_else(|| "UNK".to_string());

            Self::Call(function_name)
        }
    }
}

impl Display for StackTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackTrace::Call(func) => f.write_fmt(format_args!("Called {func}")),
            StackTrace::Pushed(pushed) => f.write_fmt(format_args!("Pushed {pushed}")),
            StackTrace::Aquired(got) => f.write_fmt(format_args!("Feteched {got}")),
            StackTrace::Misc(misc) => f.write_fmt(format_args!("{misc}")),
            StackTrace::DebugBegin(i) => f.write_fmt(format_args!("debug {i}")),
        }
    }
}
