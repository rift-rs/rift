// (c) Copyright 2022 Christian Saide
// SPDX-License-Identifier: GPL-3.0-or-later

use librift::riftd;

fn main() {
    let code = riftd::run();
    std::process::exit(code)
}
