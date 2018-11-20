//! Test that the generated code uses the right ready and error types.
#![feature(futures_api, pin, arbitary_self_types)]

extern crate futures;
#[macro_use]
extern crate state_machine_future;

use futures::Poll;
use state_machine_future::RentToOwn;

pub struct MyReady;
pub struct MyError;

#[derive(StateMachineFuture)]
pub enum Fsm {
    #[state_machine_future(start)]
    #[state_machine_future(transitions(Ready))]
    Start,

    #[state_machine_future(ready)]
    Ready(MyReady),

    #[state_machine_future(error)]
    Error(MyError),
}

impl PollFsm for Fsm {
    fn poll_start<'a>(_: &'a mut RentToOwn<'a, Start>) -> Poll<Result<AfterStart, MyError>> {
        Ok(Async::Ready(AfterStart::Ready(Ready(MyReady))))
    }
}
