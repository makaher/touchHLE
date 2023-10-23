/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::abi::GuestArg;
use crate::dyld::{ConstantExports, FunctionExports, HostConstant};
use crate::environment::Environment;
use crate::frameworks::core_graphics::cg_affine_transform::CGAffineTransform;
use crate::frameworks::core_graphics::{CGFloat, CGPoint, CGRect, CGSize};
use crate::mem::SafeRead;
use crate::{export_c_func, impl_GuestRet_for_large_struct};

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C, packed)]
pub struct CATransform3D {
    pub m11: CGFloat,
    pub m12: CGFloat,
    pub m13: CGFloat,
    pub m14: CGFloat,
    pub m21: CGFloat,
    pub m22: CGFloat,
    pub m23: CGFloat,
    pub m24: CGFloat,
    pub m31: CGFloat,
    pub m32: CGFloat,
    pub m33: CGFloat,
    pub m34: CGFloat,
    pub m41: CGFloat,
    pub m42: CGFloat,
    pub m43: CGFloat,
    pub m44: CGFloat,
}
unsafe impl SafeRead for CATransform3D {}
impl GuestArg for CATransform3D {
    const REG_COUNT: usize = 16;

    fn from_regs(regs: &[u32]) -> Self {
        CATransform3D {
            m11: GuestArg::from_regs(&regs[0..1]),
            m12: GuestArg::from_regs(&regs[1..2]),
            m13: GuestArg::from_regs(&regs[2..3]),
            m14: GuestArg::from_regs(&regs[3..4]),
            m21: GuestArg::from_regs(&regs[4..5]),
            m22: GuestArg::from_regs(&regs[5..6]),
            m23: GuestArg::from_regs(&regs[6..7]),
            m24: GuestArg::from_regs(&regs[7..8]),
            m31: GuestArg::from_regs(&regs[8..9]),
            m32: GuestArg::from_regs(&regs[9..10]),
            m33: GuestArg::from_regs(&regs[10..11]),
            m34: GuestArg::from_regs(&regs[11..12]),
            m41: GuestArg::from_regs(&regs[12..13]),
            m42: GuestArg::from_regs(&regs[13..14]),
            m43: GuestArg::from_regs(&regs[14..15]),
            m44: GuestArg::from_regs(&regs[15..16]),
        }
    }
    fn to_regs(self, regs: &mut [u32]) {
        self.m11.to_regs(&mut regs[0..1]);
        self.m12.to_regs(&mut regs[1..2]);
        self.m13.to_regs(&mut regs[2..3]);
        self.m14.to_regs(&mut regs[3..4]);
        self.m21.to_regs(&mut regs[4..5]);
        self.m22.to_regs(&mut regs[5..6]);
        self.m23.to_regs(&mut regs[6..7]);
        self.m24.to_regs(&mut regs[7..8]);
        self.m31.to_regs(&mut regs[8..9]);
        self.m32.to_regs(&mut regs[9..10]);
        self.m33.to_regs(&mut regs[10..11]);
        self.m34.to_regs(&mut regs[11..12]);
        self.m41.to_regs(&mut regs[12..13]);
        self.m42.to_regs(&mut regs[13..14]);
        self.m43.to_regs(&mut regs[14..15]);
        self.m44.to_regs(&mut regs[15..16]);
    }
}
impl_GuestRet_for_large_struct!(CATransform3D);

#[rustfmt::skip]
pub const CATransform3DIdentity: CATransform3D = CATransform3D {
    m11: 1.0, m12: 0.0, m13: 0.0, m14: 0.0,
    m21: 0.0, m22: 1.0, m23: 0.0, m24: 0.0,
    m31: 0.0, m32: 0.0, m33: 1.0, m34: 0.0,
    m41: 0.0, m42: 0.0, m43: 0.0, m44: 1.0,
};

impl Default for CATransform3D {
    fn default() -> Self {
        CATransform3DIdentity
    }
}

pub const CONSTANTS: ConstantExports = &[(
    "_CATransform3DIdentity",
    HostConstant::Custom(|mem| {
        mem.alloc_and_write(CATransform3DIdentity)
            .cast()
            .cast_const()
    }),
)];

#[rustfmt::skip]
pub fn CATransform3DMakeAffineTransform(_: &mut Environment, t: CGAffineTransform) -> CATransform3D {
    CATransform3D {
        m11: t.a, m12: t.b, m13: 0.0, m14: 0.0,
        m21: t.c, m22: t.d, m23: 0.0, m24: 0.0,
        m31: 0.0, m32: 0.0, m33: 1.0, m34: 0.0,
        m41: t.tx, m42: t.ty, m43: 0.0, m44: 1.0,
    }
}

#[rustfmt::skip]
pub fn CATransform3DGetAffineTransform(_: &mut Environment, t: CATransform3D) -> CGAffineTransform {
    CGAffineTransform {
        a: t.m11, b: t.m12,
        c: t.m21, d: t.m22,
        tx: t.m41, ty: t.m42
    }
}

#[rustfmt::skip]
pub fn CATransform3DInvert(_: &mut Environment, t: CATransform3D) -> CATransform3D {
    let d = (t.m11 * t.m22 * t.m33 * t.m44) + (t.m11 * t.m23 * t.m34 * t.m42) + (t.m11 * t.m24 * t.m32 * t.m43)
        - (t.m11 * t.m24 * t.m33 * t.m42) - (t.m11 * t.m23 * t.m32 * t.m44) - (t.m11 * t.m22 * t.m34 * t.m43)
        - (t.m12 * t.m21 * t.m33 * t.m44) - (t.m13 * t.m21 * t.m34 * t.m42) - (t.m14 * t.m21 * t.m32 * t.m43)
        + (t.m14 * t.m21 * t.m33 * t.m42) + (t.m13 * t.m21 * t.m32 * t.m44) + (t.m12 * t.m21 * t.m34 * t.m43)
        + (t.m12 * t.m23 * t.m31 * t.m44) + (t.m13 * t.m24 * t.m31 * t.m42) + (t.m14 * t.m22 * t.m31 * t.m43)
        - (t.m14 * t.m23 * t.m31 * t.m42) - (t.m13 * t.m22 * t.m31 * t.m44) - (t.m12 * t.m24 * t.m31 * t.m43)
        - (t.m12 * t.m23 * t.m34 * t.m41) - (t.m13 * t.m24 * t.m32 * t.m41) - (t.m14 * t.m22 * t.m33 * t.m41)
        + (t.m14 * t.m23 * t.m32 * t.m41) + (t.m13 * t.m22 * t.m34 * t.m41) + (t.m12 * t.m24 * t.m33 * t.m41);
    CATransform3D {
        m11: (t.m22 * t.m33 * t.m44 + t.m23 * t.m34 * t.m42 + t.m24 * t.m32 * t.m43 - t.m24 * t.m33 * t.m42 - t.m23 * t.m32 * t.m44 - t.m22 * t.m34 * t.m43) / d,
        m12: (-t.m12 * t.m33 * t.m44 - t.m13 * t.m34 * t.m42 - t.m14 * t.m32 * t.m43 + t.m14 * t.m33 * t.m42 + t.m13 * t.m32 * t.m44 + t.m12 * t.m34 * t.m43) / d,
        m13: (t.m12 * t.m23 * t.m44 + t.m13 * t.m24 * t.m42 + t.m14 * t.m22 * t.m43 - t.m14 * t.m23 * t.m42 - t.m13 * t.m22 * t.m44 - t.m12 * t.m24 * t.m43) / d,
        m14: (-t.m12 * t.m23 * t.m34 - t.m13 * t.m24 * t.m32 - t.m14 * t.m22 * t.m33 + t.m14 * t.m23 * t.m32 + t.m13 * t.m22 * t.m34 + t.m12 * t.m24 * t.m33) / d,
        m21: (-t.m21 * t.m33 * t.m44 - t.m23 * t.m34 * t.m41 - t.m24 * t.m31 * t.m43 + t.m24 * t.m33 * t.m41 + t.m23 * t.m31 * t.m44 + t.m21 * t.m34 * t.m43) / d,
        m22: (t.m11 * t.m33 * t.m44 + t.m13 * t.m34 * t.m41 + t.m14 * t.m31 * t.m43 - t.m14 * t.m33 * t.m41 - t.m13 * t.m31 * t.m44 - t.m11 * t.m34 * t.m43) / d,
        m23: (-t.m11 * t.m23 * t.m44 - t.m13 * t.m24 * t.m41 - t.m14 * t.m21 * t.m43 + t.m14 * t.m23 * t.m41 + t.m13 * t.m21 * t.m44 + t.m11 * t.m24 * t.m43) / d,
        m24: (t.m11 * t.m23 * t.m34 + t.m13 * t.m24 * t.m31 + t.m14 * t.m21 * t.m33 - t.m14 * t.m23 * t.m31 - t.m13 * t.m21 * t.m34 - t.m11 * t.m24 * t.m33) / d,
        m31: (t.m21 * t.m32 * t.m44 + t.m22 * t.m34 * t.m41 + t.m24 * t.m31 * t.m42 - t.m24 * t.m32 * t.m41 - t.m22 * t.m31 * t.m44 - t.m21 * t.m34 * t.m42) / d,
        m32: (-t.m11 * t.m32 * t.m44 - t.m12 * t.m34 * t.m41 - t.m14 * t.m31 * t.m42 + t.m14 * t.m32 * t.m41 + t.m12 * t.m31 * t.m44 + t.m11 * t.m34 * t.m42) / d,
        m33: (t.m11 * t.m22 * t.m44 + t.m12 * t.m24 * t.m41 + t.m14 * t.m21 * t.m42 - t.m14 * t.m22 * t.m41 - t.m12 * t.m21 * t.m44 - t.m11 * t.m24 * t.m42) / d,
        m34: (-t.m11 * t.m22 * t.m34 - t.m12 * t.m24 * t.m31 - t.m14 * t.m21 * t.m32 + t.m14 * t.m22 * t.m31 + t.m12 * t.m21 * t.m34 + t.m11 * t.m24 * t.m32) / d,
        m41: (-t.m21 * t.m32 * t.m43 - t.m22 * t.m33 * t.m41 - t.m23 * t.m31 * t.m42 + t.m23 * t.m32 * t.m41 + t.m22 * t.m31 * t.m43 + t.m21 * t.m33 * t.m42) / d,
        m42: (t.m11 * t.m32 * t.m43 + t.m12 * t.m33 * t.m41 + t.m13 * t.m31 * t.m42 - t.m13 * t.m32 * t.m41 - t.m12 * t.m31 * t.m43 - t.m11 * t.m33 * t.m42) / d,
        m43: (-t.m11 * t.m22 * t.m43 - t.m12 * t.m23 * t.m41 - t.m13 * t.m21 * t.m42 + t.m13 * t.m22 * t.m41 + t.m12 * t.m21 * t.m43 + t.m11 * t.m23 * t.m42) / d,
        m44: (t.m11 * t.m22 * t.m33 + t.m12 * t.m23 * t.m31 + t.m13 * t.m21 * t.m32 - t.m13 * t.m22 * t.m31 - t.m12 * t.m21 * t.m33 - t.m11 * t.m23 * t.m32) / d,
    }
}

fn min_partial<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

fn min_4f(a: CGFloat, b: CGFloat, c: CGFloat, d: CGFloat) -> CGFloat {
    min_partial(min_partial(a, b), min_partial(c, d))
}

fn max_partial<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn max_4f(a: CGFloat, b: CGFloat, c: CGFloat, d: CGFloat) -> CGFloat {
    max_partial(max_partial(a, b), max_partial(c, d))
}

impl CATransform3D {
    pub fn applyToPoint(&self, x: CGFloat, y: CGFloat) -> CGPoint {
        let newX = self.m11 * x + self.m21 * y + self.m41;
        let newY = self.m12 * x + self.m22 * y + self.m42;
        let newW = self.m14 * x + self.m24 * y + self.m44;

        CGPoint {
            x: newX / newW,
            y: newY / newW,
        }
    }
    pub fn applyToRect(&self, r: CGRect) -> CGRect {
        let tl = self.applyToPoint(r.origin.x, r.origin.y);
        let tr = self.applyToPoint(r.origin.x + r.size.width, r.origin.y);
        let bl = self.applyToPoint(r.origin.x, r.origin.y + r.size.height);
        let br = self.applyToPoint(r.origin.x + r.size.width, r.origin.y + r.size.height);

        let nix = min_4f(tl.x, tr.x, bl.x, br.x);
        let nxx = max_4f(tl.x, tr.x, bl.x, br.x);
        let niy = min_4f(tl.y, tr.y, bl.y, br.y);
        let nxy = max_4f(tl.y, tr.y, bl.y, br.y);
        CGRect {
            origin: CGPoint { x: nix, y: niy },
            size: CGSize {
                width: nxx - nix,
                height: nxy - niy,
            },
        }
    }
    #[rustfmt::skip]
    pub fn as_matrix(&self) -> [CGFloat; 16] {
        [
            self.m11, self.m12, self.m13, self.m14,
            self.m21, self.m22, self.m23, self.m24,
            self.m31, self.m32, self.m33, self.m34,
            self.m41, self.m42, self.m43, self.m44,
        ]
    }
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CATransform3DGetAffineTransform(_)),
    export_c_func!(CATransform3DMakeAffineTransform(_)),
    export_c_func!(CATransform3DInvert(_)),
];
