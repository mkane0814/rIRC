#![feature(use_extern_macros)]
#[macro_use]
extern crate failure;
extern crate futures;
extern crate irc;
extern crate tokio_core;

extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
#[macro_use]
extern crate relm_attributes;



pub mod input;
pub mod ui;