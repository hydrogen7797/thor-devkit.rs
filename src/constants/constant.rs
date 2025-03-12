pub const NODE_URL: &str = "ws://127.0.0.1:9944";

pub const HUGGING_FACE_MODEL: &str = "OpenAssistant/oasst-sft-4-pythia-12b-epoch-3.5";

pub const IROH_UPLOAD_SERVER: &str = "http://localhost:3000/upload"; // Change if needed

pub enum IPFSFetchProviderKind {
    CloudFlare,
    Web3Storage,
    IpfsIO,
    ForEverDedicated,
    Iroh,
}

pub struct IPFSFetchProvider<'a> {
    pub kind: IPFSFetchProviderKind,
    pub address: &'a str,
}

pub const IPFS_WEB3: &str = "https://w3s.link/ipfs/";
pub const IPFS_CLOUDFLARE: &str = "https://cloudflare-ipfs.com/ipfs/";
pub const IPFS_IO: &str = "https://gateway.ipfs.io/ipfs/";
pub const IPFS_4EVER_DEDICATED: &str =
    "https://42683ff2b1a2ac5ad2fef0ee01995d78.ipfs.4everland.link/ipfs/";

pub const EVERLAND_ENDPOINT: &str = "https://endpoint.4everland.co";

pub const IROH_GATEWAY: &str = "http://localhost:4000/blob/";

pub const DEFAULT_IPFS_FETCH_PROVIDER: IPFSFetchProvider = IPFSFetchProvider {
    kind: IPFSFetchProviderKind::Iroh,
    address: IROH_GATEWAY,
};
