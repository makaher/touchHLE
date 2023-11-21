/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::dyld::{ConstantExports, HostConstant};
use crate::libc::mach::mach_port_t;
use crate::mem::{ConstVoidPtr, Mem};

pub type task_t = mach_port_t;

pub const TASK_SELF: task_t = 0x5441534B; // 'TASK'

pub const CONSTANTS: ConstantExports = &[(
    "_mach_task_self_",
    HostConstant::Custom(|mem: &mut Mem| -> ConstVoidPtr {
        mem.alloc_and_write(TASK_SELF).cast().cast_const()
    })
)];

