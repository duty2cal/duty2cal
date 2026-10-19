// SPDX-License-Identifier: MPL-2.0

fn main() {
    let value = std::hint::black_box(1u64);
    println!("{}", fixture_support::fixture_alpha(value));
}
