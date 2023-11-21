/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#![allow(non_camel_case_types)]

pub mod mach_host;
pub mod mach_task;
pub mod mach_thread_info;
pub mod mach_time;
pub mod semaphore;

type mach_port_t = u32;
type integer_t = i32;
type natural_t = u32;
type mach_msg_type_number_t = natural_t;
type kern_return_t = i32;
const KERN_SUCCESS: kern_return_t = 0;
type boolean_t = i32;

enum MachObjectContent {
    Semaphore,
}

struct MachObject {
    refcount: u32,
    content: MachObjectContent
}

#[derive(Default)]
pub struct State {
    ports: Vec<Option<MachObject>>,
}
