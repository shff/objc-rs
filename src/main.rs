#[macro_use]
extern crate objc;

#[link(name = "CoreGraphics", kind = "framework")]
extern {}

fn main() {
    send![@NSAutoreleasePool new];
    send![send![@NSString alloc], autorelease];
    // send![send![@NSString alloc], autorelease:1];
}
