use winapi::um::winnt::{
    DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
};

#[no_mangle]
extern "C" fn DllMain(_: *const u8, fdw_reason: u32, _: *const u8) -> u32 {
    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            println!("DLL_PROCESS_ATTACH");
            println!("Hello from the DLL!");
            for i in 0..10 {
                std::thread::sleep(std::time::Duration::from_millis(1000));
                println!("{:?} DLL_MAIN running", i);
            }
        }
        DLL_PROCESS_DETACH => {
            println!("DLL_PROCESS_DETACH");
        }
        DLL_THREAD_ATTACH => {
            println!("DLL_THREAD_ATTACH");
            println!("Hello from the DLL!");
            for i in 0..10 {
                std::thread::sleep(std::time::Duration::from_millis(1000));
                println!("{:?} DLL_MAIN running", i);
            }
        }
        DLL_THREAD_DETACH => {
            println!("DLL_THREAD_DETACH");
        }
        _ => {
            println!("Unknown");
        }
    }
    1
}
