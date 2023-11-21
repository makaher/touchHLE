/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#![allow(non_camel_case_types)]

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use crate::dyld::FunctionExports;
use crate::environment::Environment;
use crate::export_c_func;
use crate::libc::mach::{kern_return_t, mach_port_t};
use crate::libc::mach::mach_task::{TASK_SELF, task_t};
use crate::libc::semaphore::SemaphoreHostObject;
use crate::mem::MutPtr;

type semaphore_t = mach_port_t;

fn semaphore_create(
    env: &mut Environment,
    task: task_t,
    semaphore: MutPtr<semaphore_t>,
    _policy: i32,
    value: i32
) -> kern_return_t {
    if task != TASK_SELF {
        unimplemented!("Attempt to create a semaphore for non-self task");
    }
    let host_sem_rc = Rc::new(RefCell::new(SemaphoreHostObject {
        value,
        waiting: HashSet::new(),
        guest_sem: None,
    }));
    let sem = env.mem.alloc_and_write(0);
    (*host_sem_rc).borrow_mut().guest_sem = Some(sem);
    env.libc_state.semaphore.open_semaphores.insert(sem, host_sem_rc);
    env.mem.write(semaphore, sem.to_bits());
    0
}

fn semaphore_wait(env: &mut Environment, semaphore: semaphore_t) -> kern_return_t {
    env.sem_decrement(MutPtr::from_bits(semaphore), true);
    0
}

fn semaphore_signal(env: &mut Environment, semaphore: semaphore_t) -> kern_return_t {
    env.sem_increment(MutPtr::from_bits(semaphore));
    0
}

fn semaphore_destroy(env: &mut Environment, task: task_t, semaphore: semaphore_t) -> kern_return_t {
    if task != TASK_SELF {
        unimplemented!("Attempt to destroy a semaphore for non-self task");
    }
    let sem = MutPtr::from_bits(semaphore);
    let host_sem_rc = env
        .libc_state
        .semaphore
        .open_semaphores
        .remove(&sem)
        .unwrap();
    let mut host_sem = (*host_sem_rc).borrow_mut();
    env.mem.free(host_sem.guest_sem.unwrap().cast());
    host_sem.guest_sem = None;
    0 // success
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(semaphore_create(_, _, _, _)),
    export_c_func!(semaphore_wait(_)),
    export_c_func!(semaphore_signal(_)),
    export_c_func!(semaphore_destroy(_, _)),
];


