/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::frameworks::foundation::NSUInteger;
use crate::libc::posix_io::{close, FileDescriptor, lseek, O_RDONLY, O_WRONLY, off_t, open_direct, read, SEEK_SET, write};
use crate::mem::ConstVoidPtr;
use crate::objc::{ClassExports, HostObject, NSZonePtr, id, msg, msg_class};
use crate::objc_classes;

#[derive(Default)]
struct HandleHostObject {
    fd: FileDescriptor,
    close: bool
}

impl HostObject for HandleHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSFileHandle: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<HandleHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

+ (id)fileHandleForReadingAtPath:(id)path {
    let path = msg![env; path UTF8String];
    let fd = open_direct(env, path, O_RDONLY);
    let new = msg![env; this alloc];
    msg![env; new initWithFileDescriptor:fd closeOnDealloc:true]
}

+ (id)fileHandleForWritingAtPath:(id)path {
    let path = msg![env; path UTF8String];
    let fd = open_direct(env, path, O_WRONLY);
    let new = msg![env; this alloc];
    msg![env; new initWithFileDescriptor:fd closeOnDealloc:true]
}

- (id)initWithFileDescriptor:(FileDescriptor)fd
              closeOnDealloc:(bool)close {
    let obj = env.objc.borrow_mut::<HandleHostObject>(this);
    obj.fd = fd;
    obj.close = close;
    this
}

- (())seekToFileOffset:(u64)offset {
    let obj = env.objc.borrow::<HandleHostObject>(this);
    lseek(env, obj.fd, offset as off_t, SEEK_SET);
}

- (id)readDataOfLength:(NSUInteger)length {
    let buf = env.mem.alloc(length);
    let obj = env.objc.borrow::<HandleHostObject>(this);
    let rsize = read(env, obj.fd, buf, length) as NSUInteger;
    msg_class![env; NSData dataWithBytesNoCopy:buf length: rsize]
}

- (())writeData:(id)data {
    let bytes: ConstVoidPtr = msg![env; data bytes];
    let length: NSUInteger = msg![env; data length];
    let obj = env.objc.borrow::<HandleHostObject>(this);
    write(env, obj.fd, bytes, length);
}

- (())closeFile {
    let fd = env.objc.borrow::<HandleHostObject>(this).fd;
    close(env, fd);
    env.objc.borrow_mut::<HandleHostObject>(this).close = false;
}

- (())dealloc {
    let obj = env.objc.borrow::<HandleHostObject>(this);
    if obj.close {
        close(env, obj.fd);
    }
    env.objc.dealloc_object(this, &mut env.mem);
}

@end
};