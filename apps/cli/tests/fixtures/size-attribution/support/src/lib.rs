// SPDX-License-Identifier: MPL-2.0

#[inline(never)]
pub fn fixture_alpha(n: u64) -> u64 {
    n.wrapping_mul(3).wrapping_add(1)
}
