/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod sk_payment_queue;
pub mod sk_request;

#[derive(Default)]
pub struct State {
    sk_payment_queue: sk_payment_queue::State
}
