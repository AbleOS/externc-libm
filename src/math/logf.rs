// SPDX-License-Identifier: Apache-2.0
/*
 * File: src/math/logf.rs
 *
 * The logf function.
 *
 * Author: HTG-YT
 * Copyright (c) 2021 The LibM Team of the HaruxOS Project
 */

#[no_mangle]
pub extern "C" fn logf(x: f32) -> f32 {
    libm::logf(x)
}