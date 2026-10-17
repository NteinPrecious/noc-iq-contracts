use soroban_sdk::Env;

pub const PROTOCOL_VERSION: u32 = 1;
pub const STORAGE_VERSION: u32 = 1;

/// Returns `(protocol_version, storage_version)` for off-chain integrators.
pub fn get_version(_env: &Env) -> (u32, u32) {
    (PROTOCOL_VERSION, STORAGE_VERSION)
}

/// Returns the contract's protocol version (the first element of
/// [`get_version`]) as a single `u32`.
pub fn get_contract_version(env: &Env) -> u32 {
    get_version(env).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_version_matches_protocol_version() {
        let env = Env::default();
        assert_eq!(get_contract_version(&env), PROTOCOL_VERSION);
    }

    #[test]
    fn test_get_version_reports_protocol_and_storage_versions() {
        let env = Env::default();
        assert_eq!(get_version(&env), (PROTOCOL_VERSION, STORAGE_VERSION));
    }

    #[test]
    fn test_version_discovery_is_stable_across_calls() {
        let env = Env::default();
        assert_eq!(get_contract_version(&env), get_contract_version(&env));
    }
}
