extern crate hyper;

use std::io::copy;
use std::process::{Command, Stdio};

use hyper::header;
use hyper::net::{NetworkListener, HttpStream};

fn stream_ext_command(mut stream : HttpStream) {
    let cmdpath = "./target/debug/writechunks";
    let mut cmd = Command::new(cmdpath);
    cmd.stdout(Stdio::piped());
    let child = cmd.spawn().expect("writechunks failed to start");
    println!("child started.");
    let mut childstdout = child.stdout.unwrap();
    let mut headers = header::Headers::new();
    let resp = hyper::server::response::Response::new(&mut stream, &mut headers).start().unwrap();
    let (_,mut stream2, _, _) = resp.deconstruct();
    copy(&mut childstdout, &mut stream2).unwrap();
    match stream2.end() {
    Err(_) => {
        println!("Error ending stream");
    }
    _ => println!("done."),
    }
}

fn main() {
    let streamaddr = ("127.0.0.1", 50123);
    let mut streamlistener = hyper::net::HttpListener::new(streamaddr).unwrap();
    println!("streaming on {:?}", streamaddr);
    loop {
        for stream in streamlistener.accept() {
            println!("stream{:?}", stream);
            stream_ext_command(stream);
        }
    }
}
