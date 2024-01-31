pub const NODE_URL: &str = "ws://127.0.0.1:9944";

pub const HUGGING_FACE_MODEL: &str = "OpenAssistant/oasst-sft-4-pythia-12b-epoch-3.5";

pub enum IPFSFetchProviderKind {
    CloudFlare,
    Web3Storage,
    IpfsIO,
}

pub struct IPFSFetchProvider<'a> {
    pub kind: IPFSFetchProviderKind,
    pub address: &'a str,
}

pub const IPFS_WEB3: &str = "https://w3s.link/ipfs/";
pub const IPFS_CLOUDFLARE: &str = "https://cloudflare-ipfs.com/ipfs/";
pub const IPFS_IO: &str = "https://gateway.ipfs.io/ipfs/";

pub const EVERLAND_ENDPOINT: &str = "https://endpoint.4everland.co";

pub const DEFAULT_IPFS_FETCH_PROVIDER: IPFSFetchProvider = IPFSFetchProvider {
    kind: IPFSFetchProviderKind::Web3Storage,
    address: IPFS_WEB3,
};
