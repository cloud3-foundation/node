use sp_core::sr25519;
// use sp_consensus_aura::sr25519::AuthorityId as AuraId;
// use sp_consensus_grandpa::AuthorityId as GrandpaId;

use node_cloud3::chain_spec::*;

#[test]
fn test_todo() {
    let seed = "Cloud3";
    let pare = get_from_seed::<sr25519::Public>(seed);
    let expected = "5DSDrF67gqHfnZpGNv4sHSsz16po2xnEbCp8D7Ez33oUaxsC";
    assert_eq!(pare.to_string(), expected);
}


