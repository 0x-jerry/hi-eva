use std::ffi::CStr;

use block::ConcreteBlock;
use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use objc::rc::autoreleasepool;
use objc::{class, msg_send, sel, sel_impl};

pub fn listen_focused_app() {
    _listen_focused_app();
}
fn _listen_focused_app() {
    unsafe {
        let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        let center: id = msg_send![workspace, notificationCenter];

        let block = ConcreteBlock::new(move |notification: id| {
            let info: id = msg_send![notification, userInfo];

            let ns_name = NSString::alloc(nil).init_str("NSWorkspaceApplicationKey");

            let app: id = msg_send![info, objectForKey: ns_name];

            let name: id = msg_send![app, localizedName];
            let name = NSString::UTF8String(name);
            let name = CStr::from_ptr(name).to_string_lossy().to_string();

            println!("Received notification: {:?}", name);
        })
        .copy();

        // Convert the block to an Objective-C block
        let name = NSString::alloc(nil).init_str("NSWorkspaceDidActivateApplicationNotification");

        let _: () = msg_send![center,
            addObserverForName: name
            object: nil
            queue: nil
            usingBlock: block
        ];

        println!("listen started");
    }
}

/// This should run in main thread
pub fn run_loop() {
    unsafe {
        autoreleasepool(|| {
            let run_loop: id = msg_send![class!(NSRunLoop), mainRunLoop];
            let _: () = msg_send![run_loop, run];
        });
    }
}
