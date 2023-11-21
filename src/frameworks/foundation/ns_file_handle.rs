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
    let fd = env.objc.borrow::<HandleHostObject>(this).fd;
    let mut read_off = 0;
    while read_off < length {
        let rsize = read(env, fd, buf + read_off, length - read_off);
        if rsize == 0 || rsize == -1 {
            break;
        }
        read_off += rsize as u32;
    }

    msg_class![env; NSData dataWithBytesNoCopy:buf length: read_off]
}

- (())writeData:(id)data {
    let bytes: ConstVoidPtr = msg![env; data bytes];
    let length: NSUInteger = msg![env; data length];
    let fd = env.objc.borrow::<HandleHostObject>(this).fd;
    let mut write_off = 0;
    while write_off < length {
        let wsize = write(env, fd, bytes + write_off, length - write_off);
        if wsize == 0 || wsize == -1 {
            break;
        }
        write_off += wsize as u32;
    }
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