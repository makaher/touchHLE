/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSDate`.

use crate::environment::Environment;
use std::time;
use std::time::{Duration, SystemTime};
use chrono::{DateTime, TimeZone, Utc};

use super::NSTimeInterval;
use crate::objc::{autorelease, id, msg, objc_classes, ClassExports, HostObject, NSZonePtr};

struct NSDateHostObject {
    instant: SystemTime,
}
impl HostObject for NSDateHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSDate: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(NSDateHostObject {
        instant: SystemTime::now()
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

+ (id)date {
    let new = msg![env; this alloc];

    log_dbg!("[(NSDate*){:?} date]: New date {:?}", this, new);

    autorelease(env, new)
}

+ (NSTimeInterval)timeIntervalSinceReferenceDate {
    let reference = Utc.with_ymd_and_hms(2001, 1, 1, 0, 0, 0).unwrap();
    let diff = Utc::now() - reference;
    let secs = diff.num_seconds();
    let nanos = (diff - chrono::Duration::seconds(secs)).num_nanoseconds().unwrap();
    secs as f64 + nanos as f64 / 1_000_000_000.0
}

- (id)init {
    this
}

- (id)initWithTimeIntervalSinceNow:(NSTimeInterval)secs {
    let host_object = env.objc.borrow_mut::<NSDateHostObject>(this);
    host_object.instant = SystemTime::now() + Duration::from_secs_f64(secs);
    this
}

- (NSTimeInterval)timeIntervalSinceDate:(id)anotherDate {
    assert!(!anotherDate.is_null());
    let host_object = env.objc.borrow::<NSDateHostObject>(this);
    let another_date_host_object = env.objc.borrow::<NSDateHostObject>(anotherDate);
    let result = another_date_host_object.instant.duration_since(host_object.instant).unwrap().as_secs_f64();
    log_dbg!("[(NSDate*){:?} timeIntervalSinceDate:{:?}]: result {} seconds", this, anotherDate, result);
    result
}

- (NSTimeInterval)timeIntervalSince1970 {
    let host_object = env.objc.borrow::<NSDateHostObject>(this);
    host_object.instant.duration_since(time::UNIX_EPOCH).unwrap().as_secs_f64()
}

@end

};

pub fn to_date(env: &mut Environment, date: id) -> SystemTime {
    env.objc.borrow::<NSDateHostObject>(date).instant
}
