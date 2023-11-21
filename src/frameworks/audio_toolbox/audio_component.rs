/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::dyld::FunctionExports;
use crate::environment::Environment;
use crate::export_c_func;
use crate::frameworks::carbon_core::OSStatus;
use crate::frameworks::core_audio_types::fourcc;
use crate::mem::{ConstPtr, ConstVoidPtr, GuestUSize, MutPtr, SafeRead};

type AudioComponent = GuestUSize;
type OSType = u32;

#[repr(C, packed)]
#[derive(Debug)]
struct AudioComponentDescription {
    component_type: OSType,
    component_sub_type: OSType,
    component_manufacturer: OSType,
    component_flags: u32,
    component_flags_mask: u32
}

unsafe impl SafeRead for AudioComponentDescription {}

#[repr(C, packed)]
struct OpaqueAudioComponentInstance {
    _filler: u8,
}
unsafe impl SafeRead for OpaqueAudioComponentInstance {}

type AudioComponentInstance = MutPtr<OpaqueAudioComponentInstance>;
type AudioUnit = AudioComponentInstance;
type AudioUnitPropertyID = u32;
type AudioUnitScope = u32;
type AudioUnitElement = u32;

fn AudioComponentFindNext(
    env: &mut Environment,
    in_component: AudioComponent,
    in_desc: ConstPtr<AudioComponentDescription>
) -> AudioComponent {
    assert_eq!(in_component, 0); //TODO
    let desc = env.mem.read(in_desc);
    assert!(desc.component_type == fourcc(b"auou"));
    assert!(desc.component_sub_type == fourcc(b"rioc"));
    assert!(desc.component_manufacturer == fourcc(b"appl"));
    fourcc(b"rioc") // Should be sufficiently pointer-shaped
}

fn AudioComponentInstanceNew(
    env: &mut Environment,
    in_component: AudioComponent,
    out_instance: MutPtr<AudioComponentInstance>
) -> OSStatus {
    assert_eq!(in_component, fourcc(b"rioc")); //TODO
    let guest_audio_comp_instance = env.mem.alloc_and_write(OpaqueAudioComponentInstance {
        _filler: 0
    });
    env.mem.write(out_instance, guest_audio_comp_instance);
    0
}

fn AudioUnitSetProperty(
    env: &mut Environment,
    in_unit: AudioUnit,
    in_id: AudioUnitPropertyID,
    in_scope: AudioUnitScope,
    in_element: AudioUnitElement,
    in_data: ConstVoidPtr,
    in_data_size: u32
) -> OSStatus {
    dbg!(in_id, in_scope, in_element);
    0
}

fn AudioUnitInitialize(
    env: &mut Environment,
    in_unit: AudioUnit,
) -> OSStatus {
    dbg!();
    0
}

fn AudioOutputUnitStart(
    env: &mut Environment,
    in_unit: AudioUnit,
) -> OSStatus {
    dbg!();
    0
}

fn AudioOutputUnitStop(
    env: &mut Environment,
    in_unit: AudioUnit,
) -> OSStatus {
    dbg!();
    0
}

fn AudioUnitUninitialize(
    env: &mut Environment,
    in_unit: AudioUnit,
) -> OSStatus {
    dbg!();
    0
}

fn AudioComponentInstanceDispose(
    env: &mut Environment,
    in_unit: AudioUnit,
) -> OSStatus {
    dbg!();
    0
}


pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(AudioComponentFindNext(_, _)),
    export_c_func!(AudioComponentInstanceNew(_, _)),
    export_c_func!(AudioUnitSetProperty(_, _, _, _, _, _)),
    export_c_func!(AudioUnitInitialize(_)),
    export_c_func!(AudioOutputUnitStart(_)),
    export_c_func!(AudioOutputUnitStop(_)),
    export_c_func!(AudioUnitUninitialize(_)),
    export_c_func!(AudioComponentInstanceDispose(_)),
];