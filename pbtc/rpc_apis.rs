use std::str::FromStr;
use std::collections::HashSet;
use rpc::Dependencies;
use ethcore_rpc::Extendable;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Api {
	/// Raw
	Raw,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ApiSet {
	List(HashSet<Api>),
}

impl Default for ApiSet {
	fn default() -> Self {
		ApiSet::List(vec![Api::Raw].into_iter().collect())
	}
}

impl FromStr for Api {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"raw" => Ok(Api::Raw),
			api => Err(format!("Unknown api: {}", api)),
		}
	}
}

impl ApiSet {
	pub fn list_apis(&self) -> HashSet<Api> {
		match *self {
			ApiSet::List(ref apis) => apis.clone(),
		}
	}
}

pub fn setup_rpc<T: Extendable>(server: T, apis: ApiSet, deps: Dependencies) -> T {
	use ethcore_rpc::v1::*;

	for api in apis.list_apis() {
		match api {
			Api::Raw => server.add_delegate(RawClient::new(RawClientCore::new(deps.local_sync_node.clone())).to_delegate()),
		}
	}
	server
}