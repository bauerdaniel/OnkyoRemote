#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::State;
use std::sync::Mutex;
use std::time::Duration;

struct SharedData {
    remote: Mutex<iscp::Remote>,
}

#[get("/")]
fn index() -> &'static str {
    "
    ISCP REST
    
    Control your Onkyo receiver over a REST interface

    /dicover
        discovers all available devices in the network

    /devices
        lists the discovered devices
    
    /device/<id>/power/<on/off>
        powers on/off the device with given id

    /device/<id>/mute/<on/off>
        mutes or unmutes the device with given id
    "
}

#[get("/discover")]
fn discover(shared: State<SharedData>) -> String {
    let shared_data: &SharedData = shared.inner();
    let mut locked_remote = shared_data.remote.lock().expect("Cannot access remote state");
    locked_remote.discover(Duration::from_secs(5));
    locked_remote.serialize()
}

#[get("/devices")]
fn devices(shared: State<SharedData>) -> String {
    let shared_data: &SharedData = shared.inner();
    let locked_remote = shared_data.remote.lock().expect("Cannot access remote state");
    locked_remote.serialize()
}

#[get("/device/<id>/power/off")]
fn power_off(shared: State<SharedData>, id: usize) {
    let shared_data: &SharedData = shared.inner();
    let locked_remote = shared_data.remote.lock().expect("Cannot access remote state");
    let _res = locked_remote.device(id).commands().main().power_off();
}

#[get("/device/<id>/power/on")]
fn power_on(shared: State<SharedData>, id: usize) {
    let shared_data: &SharedData = shared.inner();
    let locked_remote = shared_data.remote.lock().expect("Cannot access remote state");
    let _res = locked_remote.device(id).commands().main().power_on();
}

#[get("/device/<id>/mute/off")]
fn mute_off(shared: State<SharedData>, id: usize) {
    let shared_data: &SharedData = shared.inner();
    let locked_remote = shared_data.remote.lock().expect("Cannot access remote state");
    let _res = locked_remote.device(id).commands().main().mute_off();
}

#[get("/device/<id>/mute/on")]
fn mute_on(shared: State<SharedData>, id: usize) {
    let shared_data: &SharedData = shared.inner();
    let locked_remote = shared_data.remote.lock().expect("Cannot access remote state");
    let _res = locked_remote.device(id).commands().main().mute_on();
}

fn main() {
    rocket::ignite()
        .manage(SharedData {
            remote: Mutex::new(iscp::Remote::load())
        })
        .mount("/", routes![index, discover, devices, power_off, power_on, mute_off, mute_on])
        .launch();
}