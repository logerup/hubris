// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]
#![no_main]

use userlib::*;

task_slot!(USER_LEDS, user_leds);

#[export_name = "main"]
pub fn main() -> ! {

    let user_leds = drv_user_leds_api::UserLeds::from(USER_LEDS.get_task_id());

    loop {
        // Toggle the green LED
        user_leds.led_toggle(0).unwrap();
    }
}

// Needed for notifcations to work
include!(concat!(env!("OUT_DIR"), "/notifications.rs"));
