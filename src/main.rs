/* ZMQ scriptpubkey checker
 * Written in 2014 by
 *   Andrew Poelstra <apoelstra@wpsoftware.net>
 *
 * To the extent possible under law, the author(s) have dedicated all
 * copyright and related and neighboring rights to this software to
 * the public domain worldwide. This software is distributed without
 * any warranty.
 *
 * You should have received a copy of the CC0 Public Domain Dedication
 * along with this software.
 * If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
 */

//! # scriptpubkey-checker
//!
//! Send it scriptpubkeys over ZMQ, it replies true/false
//! where true means "provably unspendable"
//!

#![crate_name = "scriptpubkey-checker"]
#![license = "CC0"]

// Experimental features we need
#![feature(globs)]
#![feature(phase)]
#![feature(macro_rules)]

// Coding conventions
#![warn(non_uppercase_statics)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![warn(missing_doc)]

extern crate bitcoin;
extern crate zmq;

use bitcoin::blockdata::script::Script;

fn main() {
    let mut ctx = zmq::Context::new();
    let mut sock = ctx.socket(zmq::REP).unwrap();
    assert!(sock.bind("tcp://*:8003").is_ok());

    loop {
        // Crash on decode failure
        let scriptpubkey = Script::from_vec(sock.recv_bytes(0).unwrap());

        if scriptpubkey.is_provably_unspendable() {
            sock.send([1], 0).unwrap();
        } else {
            sock.send([0], 0).unwrap();
        }
    }
}

