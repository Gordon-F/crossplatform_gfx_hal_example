use std::{
    panic::{self, UnwindSafe},
    process,
};

#[no_mangle]
pub extern "C" fn main_rs() -> std::os::raw::c_int {
    stop_unwind(game_lib::main_rs)
}

fn stop_unwind<F: FnOnce() -> T + UnwindSafe, T>(f: F) -> T {
    match panic::catch_unwind(f) {
        Ok(t) => t,
        Err(_) => {
            eprintln!("Attempt to Unwind out of rust code");
            process::abort()
        }
    }
}
