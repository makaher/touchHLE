/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#![allow(non_camel_case_types)]

use crate::dyld::{export_c_func, FunctionExports};
use crate::mem::{guest_size_of, MutPtr, SafeRead};
use crate::Environment;

type mach_port_t = u32;
type host_t = mach_port_t;
type integer_t = i32;
type host_flavor_t = integer_t;
type host_info_t = MutPtr<integer_t>;
type natural_t = u32;
type mach_msg_type_number_t = natural_t;
type kern_return_t = i32;

const HOST_SELF: host_t = 0x484F5354; // HOST
const HOST_VM_INFO: host_flavor_t = 2;
const KERN_SUCCESS: kern_return_t = 0;

#[repr(C, packed)]
struct vm_statistics {
    free_count: integer_t,
    active_count: integer_t,
    inactive_count: integer_t,
    wire_count: integer_t,
    zero_fill_count: integer_t,
    reactivations: integer_t,
    pageins: integer_t,
    pageouts: integer_t,
    faults: integer_t,
    cow_faults: integer_t,
    lookups: integer_t,
    hits: integer_t,
}
unsafe impl SafeRead for vm_statistics {}

/// Returns the host port of the current task
/// Since we do not have real mach port management
/// let's just hope that the fake value is good enough
fn mach_host_self(_: &mut Environment) -> host_t {
    HOST_SELF
}

/// Returns various kinds of host statistics
fn host_statistics(
    env: &mut Environment,
    host_priv: host_t,
    flavor: host_flavor_t,
    host_info_out: host_info_t,
    host_info_out_count: MutPtr<mach_msg_type_number_t>,
) -> kern_return_t {
    if host_priv != HOST_SELF {
        unimplemented!("Attempt to get statistics for non-self host")
    }

    let out_size_available = env.mem.read(host_info_out_count);
    match flavor {
        HOST_VM_INFO => {
            let out_size_expected = guest_size_of::<vm_statistics>() / guest_size_of::<integer_t>();
            assert!(out_size_expected <= out_size_available);
            env.mem.memset(host_info_out.cast(), 0, out_size_available);
            env.mem.write(
                host_info_out.cast(),
                vm_statistics {
                    free_count: 65536,  // 256MB
                    active_count: 8192, // 32MB
                    inactive_count: 57344,
                    wire_count: 0,
                    zero_fill_count: 57344,
                    reactivations: 0,
                    pageins: 0,
                    pageouts: 0,
                    faults: 0,
                    cow_faults: 0,
                    lookups: 0,
                    hits: 0,
                },
            );
        }
        _ => unimplemented!("TODO: flavor {:?}", flavor),
    }
    KERN_SUCCESS
}

#[repr(C, packed)]
struct utsname {
    sysname: [u8; 256],
    nodename: [u8; 256],
    release: [u8; 256],
    version: [u8; 256],
    machine: [u8; 256],
}

unsafe impl SafeRead for utsname {}

fn uname(env: &mut Environment, ptr: MutPtr<utsname>) -> i32 {
    let version = b"9.4.0";
    let nodename = b"iPhone";
    let release = b"9.4.0";
    let sysname = b"Darwin";
    let machine = b"iPhone1,1";
    let mut res = utsname {
        sysname: [0; 256],
        nodename: [0; 256],
        release: [0; 256],
        version: [0; 256],
        machine: [0; 256],
    };
    res.version[..version.len()].copy_from_slice(version);
    res.nodename[..nodename.len()].copy_from_slice(nodename);
    res.release[..release.len()].copy_from_slice(release);
    res.sysname[..sysname.len()].copy_from_slice(sysname);
    res.machine[..machine.len()].copy_from_slice(machine);
    env.mem.write(ptr, res);
    0
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(mach_host_self()),
    export_c_func!(host_statistics(_, _, _, _)),
    export_c_func!(uname(_))
];
