use kernel_sync::{LockAction, rwlock::RwLock, spin::SpinMutex, ticket::TicketMutex};
pub struct NoIrqLockAction;
impl LockAction for NoIrqLockAction {
    fn before_lock() {
        // push_off(); //disable interrupt
    }
    fn after_lock() {
        // pop_off(); //enable interrupt
    }
}

fn main() {
    let x = SpinMutex::<_,NoIrqLockAction>::new(0);
    *x.lock() = 19;
    assert_eq!(*x.lock(), 19);
    let y = TicketMutex::<_,NoIrqLockAction>::new(0);
    *y.lock() = 19;
    assert_eq!(*y.lock(), 19);
    let z = RwLock::<_,NoIrqLockAction>::new(0);
    *z.write() = 19;
    assert_eq!(*z.read(), 19);
}
