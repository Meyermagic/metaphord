#![feature(phase)]

#[phase(plugin, link)]
extern crate log;
extern crate metaphor = "libmetaphor";
extern crate sync;

use sync::Arc;
use sync::raw::RwLock;

use std::io::fs;
use std::io;

use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};

use metaphor::repository::find_repo_root;
use metaphor::tree::tree_diff;
use metaphor::{Database, Tag, Patch, TrivialDb};
use metaphor::diff::PatienceLinePatch;
use metaphor::{ToDisk, Object, ID, FlatTree, Tree};

pub struct RepoToken;



fn handle_client(mut stream: TcpStream, db_lock: Arc<RwLock<TrivialDb>>) {
	debug!("got client");
	let call = stream.read_le_u64().unwrap();
	match call {
		1 => {
			// Clone
			let path_byte_length = stream.read_le_u64().unwrap();
			let path_bytes = stream.read_exact(path_byte_length).unwrap();

		},
		2 => {
			// Push
		},
		3 => {
			// Pull
		},
		_ => {
			error!("Didn't understand message");
		}
	}
}


fn main() {
	let mut repo_lock = Arc::new(RwLock::new(TrivialDb::new(&Path::new(".met"))));


	debug!("starting server");
	let listener = TcpListener::bind("127.0.0.1", 8080);
	let mut acceptor = listener.listen();

	for stream in acceptor.incoming() {
		match stream {
			Err(e) => { error!("failed to connect to client"); },
			Ok(stream) => spawn(proc() {
				handle_client(stream, repo_lock.clone());
			}),
		}
	}

	drop(acceptor);
}