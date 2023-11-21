/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::objc::{ClassExports, HostObject, id};
use crate::{msg_class, objc_classes, msg};
use crate::frameworks::foundation::ns_array::to_vec;

#[derive(Default)]
pub struct State {
    queue: Option<id>,
}

#[derive(Default)]
struct QueueHostObject {
    observers: id
}

impl HostObject for QueueHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation SKPaymentQueue: NSObject

+ (id)defaultQueue {
    if let Some(queue) = env.framework_state.store_kit.sk_payment_queue.queue {
        queue
    } else {
        let observers = msg_class![env; NSMutableArray new];
        let host = QueueHostObject {
            observers
        };
        let new = env.objc.alloc_static_object(
            this,
            Box::new(host),
            &mut env.mem
        );
        env.framework_state.store_kit.sk_payment_queue.queue = Some(new);
        new
   }
}

+ (bool)canMakePayments {
    true
}

- (())addTransactionObserver:(id)observer {
    let observers = env.objc.borrow::<QueueHostObject>(this).observers;
    msg![env; observers addObject: observer]
}

- (())addPayment:(id)payment {
    let observers = env.objc.borrow::<QueueHostObject>(this).observers;
    let observers = to_vec(env, observers);
    for observer in observers {
        let tx: id = msg_class![env; SKPaymentTransaction transactionWithPayment:payment];
        let txa: id = msg_class![env; NSArray arrayWithObject:tx];
        () = msg![env; observer paymentQueue:this updatedTransactions:txa];
    }
}

- (())finishTransaction:(id)_tx {

}

- (id)retain { this }
- (())release {}
- (id)autorelease { this }

@end

};