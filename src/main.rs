#![no_std]
#![no_main]
#![windows_subsystem = "windows"]
use core::panic::PanicInfo;
use windows_sys::{
    Win32::Foundation::*, Win32::System::Threading::*, Win32::UI::WindowsAndMessaging::*,
};

#[no_mangle]
pub extern "system" fn mainCRTStartup() {
    unsafe {
        let event = CreateEventW(core::ptr::null(), 1, 0, core::ptr::null());
        SetEvent(event);
        WaitForSingleObject(event, 0);
        CloseHandle(event);

        MessageBoxA(0, b"Text\0".as_ptr(), b"Caption\0".as_ptr(), MB_OK);
    }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}