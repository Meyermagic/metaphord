#![feature(phase)]

#[phase(plugin, link)]
extern crate log;
extern crate metaphor = "libmetaphor";
extern crate sync;

use std::sync::Arc;
use RwLock = std::sync::RWLock;

use metaphor::repository::find_repo_root;
use metaphor::tree::tree_diff;
use metaphor::{Database, Tag, Patch, TrivialDb};
use metaphor::diff::PatienceLinePatch;
use metaphor::{ToDisk, Object, ID, FlatTree, Tree};
use metaphor::rpc;

use metaphor::rpc::{CloneContext, ChangesContext, ChildrenContext,
	                  PushContext, TagContext, UntagContext, TaggedContext};
use RpcID = metaphor::rpc::ID;
use RpcCommit = metaphor::rpc::Commit;
use RpcChangeSeq = metaphor::rpc::ChangeSeq;
use RpcChange = metaphor::rpc::Change;
use RpcPatch = metaphor::rpc::Patch;

pub struct RepoToken;



pub struct RepoServerImpl;

impl RepoServerImpl {
	pub fn new() -> RepoServerImpl {
		RepoServerImpl
	}
}

impl rpc::Server for RepoServerImpl {
	fn clone(&mut self, mut context: CloneContext) {
		let (params, results) = context.get();
		unimplemented!();
		context.done();
	}

	fn changes(&mut self, mut context: ChangesContext) {
		unimplemented!();
	}

	fn children(&mut self, mut context: ChildrenContext) {
		unimplemented!();
	}

	fn push(&mut self, mut context: PushContext) {
		unimplemented!();
	}

	fn tag(&mut self, mut context: TagContext) {
		unimplemented!();
	}

	fn untag(&mut self, mut context: UntagContext) {
		unimplemented!();
	}

	fn tagged(&mut self, mut context: TaggedContext) {
		unimplemented!();
	}
}

fn main() {
	let mut repo_lock = Arc::new(RwLock::new(TrivialDb::new(&Path::new(".met"))));

	let repo_server = rpc::repository_server("127.0.0.1:8080", box RepoServerImpl::new()).unwrap();

	repo_server.serve();
}