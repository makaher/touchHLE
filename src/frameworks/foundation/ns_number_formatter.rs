/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::frameworks::foundation::ns_string::from_rust_string;
use crate::frameworks::foundation::NSUInteger;
use crate::objc::{autorelease, ClassExports, HostObject, id, nil, NSZonePtr, release, retain, msg};
use crate::objc_classes;

type NSNumberFormatterBehavior = NSUInteger;
type NSNumberFormatterStyle = NSUInteger;

struct NumberFormatterHostObject {
    behavior: NSNumberFormatterBehavior,
    style: NSNumberFormatterStyle,
    locale: id,
}
impl HostObject for NumberFormatterHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSNumberFormatter: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(NumberFormatterHostObject {
        behavior: 0,
        style: 0,
        locale: nil
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (())setFormatterBehavior:(NSNumberFormatterBehavior)behavior {
    env.objc.borrow_mut::<NumberFormatterHostObject>(this).behavior = dbg!(behavior);
}

- (())setNumberStyle:(NSNumberFormatterStyle)style {
    env.objc.borrow_mut::<NumberFormatterHostObject>(this).style = dbg!(style);
}

- (())setLocale:(id)locale {
    retain(env, locale);
    let obj = env.objc.borrow_mut::<NumberFormatterHostObject>(this);
    let old = obj.locale;
    obj.locale = locale;
    release(env, old);
}

- (id)stringFromNumber:(id)number {
    let val: i32 = msg![env; number intValue];
    let st = from_rust_string(env, format!("{}", val));
    autorelease(env, st)
}

- (id)currencyCode {
    let st = from_rust_string(env, "XTH".to_string());
    autorelease(env, st)
}

@end

};