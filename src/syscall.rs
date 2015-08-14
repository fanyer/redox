use core::mem::size_of;
use core::ptr;

use alloc::boxed::*;

use common::memory::*;
use common::resource::*;
use common::string::*;
use common::url::*;

use programs::session::*;

pub fn request(url: &URL, callback: Box<FnBox(&mut SessionItem, String)>){
    unsafe{
        let url_ptr: *const URL = url;
        let callback_ptr: *mut Box<FnBox(&mut SessionItem, String)> = alloc(size_of::<Box<FnBox(&mut SessionItem, String)>>()) as *mut Box<FnBox(&mut SessionItem, String)>;
        ptr::write(callback_ptr, callback);
        asm!("int 0x80"
            :
            : "{eax}"(1), "{ebx}"(url_ptr as u32), "{ecx}"(callback_ptr as u32)
            :
            : "intel");
    }
}

pub fn open(url: &URL) -> Box<Resource> {
    unsafe{
        let url_ptr: *const URL = url;
        let resource_ptr: *mut Box<Resource> = alloc(size_of::<Box<Resource>>()) as *mut Box<Resource>;
        asm!("int 0x80"
            :
            : "{eax}"(2), "{ebx}"(url_ptr as u32), "{ecx}"(resource_ptr as u32)
            :
            : "intel");
        let resource = ptr::read(resource_ptr);
        unalloc(resource_ptr as usize);
        return resource;
    }
}
