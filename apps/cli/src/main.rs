// SPDX-License-Identifier: MPL-2.0
//! Driving adapter: argv → `ConvertRoster` → reported path/exit code.
//! Wires the filesystem adapter (`fs`) and the logger (`log`).

mod fs;
mod log;

fn main() {
    // The conversion behaviour is implemented by the change's scenario tasks.
    std::process::exit(1);
}
