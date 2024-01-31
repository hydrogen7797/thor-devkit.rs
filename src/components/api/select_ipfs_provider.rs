pub enum IPFSProvider {
    Crust,
    Web3Storage,
    Everland,
}

// https://github.com/crustio/crust-apps/blob/master/packages/apps-config/src/ipfs-gateway-endpoints/index.ts

pub const CRUST_GATEWAY: &str = "https://gw.crustfiles.app";

pub const WEB3_STORAGE_API: &str = "https://api.web3.storage";

pub const WEB3_STORAGE_API_UPLOAD: &str = "https://api.web3.storage/upload";

pub const EVERLAND_UPLOAD: &str = "https://api.4everland.dev";

pub const DEFAULT_IPFS_PROVIDER: IPFSProvider = IPFSProvider::Everland;
