use akropolisos_runtime::types::Token;
use akropolisos_runtime::{
    constants::currency::*, AccountId, AuthorityDiscoveryConfig, BabeConfig, Balance,
    BalancesConfig, Block, BridgeConfig, ContractsConfig, CouncilConfig, DemocracyConfig,
    GenesisConfig, GrandpaConfig, ImOnlineConfig, IndicesConfig, SessionConfig, SessionKeys,
    Signature, SocietyConfig, StakerStatus, StakingConfig, SudoConfig, SystemConfig,
    TechnicalCommitteeConfig, TokenConfig, WASM_BINARY,
};
use grandpa_primitives::AuthorityId as GrandpaId;
use hex_literal::hex;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};
use telemetry::TelemetryEndpoints;

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const AKROPOLIS_TELEMETRY_URL: &str = "wss://167.99.142.212:1024";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client::ForkBlocks<Block>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client::BadBlocks<Block>,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

fn session_keys(
    grandpa: GrandpaId,
    babe: BabeId,
    im_online: ImOnlineId,
    authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
    }
}

// generate genesis from file
#[allow(unused)]
pub fn akropolisos_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/akropolisos.json")[..])
}

pub fn syracuse_testnet_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/akropolisos_syracuse.json")[..])
}

fn akropolisos_staging_genesis() -> GenesisConfig {
        // stash, controller, session-key
        // generated with secret:
        // for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
        // and
        // for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done
        let initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)> = vec![(
            // 5Fbsd6WXDGiLTxunqeK5BATNiocfCqu9bS1yArVjCgeBLkVy
            hex!["9c7a2ee14e565db0c69f78c7b4cd839fbf52b607d867e9e9c5a79042898a0d12"].into(),
            // 5EnCiV7wSHeNhjW3FSUwiJNkcc2SBkPLn5Nj93FmbLtBjQUq
            hex!["781ead1e2fa9ccb74b44c19d29cb2a7a4b5be3972927ae98cd3877523976a276"].into(),
            // 5Fb9ayurnxnaXj56CjmyQLBiadfRCqUbL2VWNbbe1nZU6wiC
            hex!["9becad03e6dcac03cee07edebca5475314861492cdfc96a2144a67bbe9699332"].unchecked_into(),
            // 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
            hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"].unchecked_into(),
            // 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
            hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"].unchecked_into(),
            // 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
            hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"].unchecked_into(),
        ),(
            // 5ERawXCzCWkjVq3xz1W5KGNtVx2VdefvZ62Bw1FEuZW4Vny2
            hex!["68655684472b743e456907b398d3a44c113f189e56d1bbfd55e889e295dfde78"].into(),
            // 5Gc4vr42hH1uDZc93Nayk5G7i687bAQdHHc9unLuyeawHipF
            hex!["c8dc79e36b29395413399edaec3e20fcca7205fb19776ed8ddb25d6f427ec40e"].into(),
            // 5EockCXN6YkiNCDjpqqnbcqd4ad35nU4RmA1ikM4YeRN4WcE
            hex!["7932cff431e748892fa48e10c63c17d30f80ca42e4de3921e641249cd7fa3c2f"].unchecked_into(),
            // 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
            hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"].unchecked_into(),
            // 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
            hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"].unchecked_into(),
            // 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
            hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"].unchecked_into(),
        ),(
            // 5DyVtKWPidondEu8iHZgi6Ffv9yrJJ1NDNLom3X9cTDi98qp
            hex!["547ff0ab649283a7ae01dbc2eb73932eba2fb09075e9485ff369082a2ff38d65"].into(),
            // 5FeD54vGVNpFX3PndHPXJ2MDakc462vBCD5mgtWRnWYCpZU9
            hex!["9e42241d7cd91d001773b0b616d523dd80e13c6c2cab860b1234ef1b9ffc1526"].into(),
            // 5E1jLYfLdUQKrFrtqoKgFrRvxM3oQPMbf6DfcsrugZZ5Bn8d
            hex!["5633b70b80a6c8bb16270f82cca6d56b27ed7b76c8fd5af2986a25a4788ce440"].unchecked_into(),
            // 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
            hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"].unchecked_into(),
            // 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
            hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"].unchecked_into(),
            // 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
            hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"].unchecked_into(),
        ),(
            // 5HYZnKWe5FVZQ33ZRJK1rG3WaLMztxWrrNDb1JRwaHHVWyP9
            hex!["f26cdb14b5aec7b2789fd5ca80f979cef3761897ae1f37ffb3e154cbcc1c2663"].into(),
            // 5EPQdAQ39WQNLCRjWsCk5jErsCitHiY5ZmjfWzzbXDoAoYbn
            hex!["66bc1e5d275da50b72b15de072a2468a5ad414919ca9054d2695767cf650012f"].into(),
            // 5DMa31Hd5u1dwoRKgC4uvqyrdK45RHv3CpwvpUC1EzuwDit4
            hex!["3919132b851ef0fd2dae42a7e734fe547af5a6b809006100f48944d7fae8e8ef"].unchecked_into(),
            // 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
            hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"].unchecked_into(),
            // 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
            hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"].unchecked_into(),
            // 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
            hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"].unchecked_into(),
        )];

    // generated with secret: subkey inspect "$secret"/fir
    let root_key: AccountId = hex![
        // 5Ff3iXP75ruzroPWRP2FYBHWnmGGBSb63857BgnzCoXNxfPo
        "9ee5e5bdc0ec239eb164f865ecc345ce4c88e76ee002e0f7e318097347471809"
    ]
    .into();

    let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

    testnet_genesis(initial_authorities, root_key, Some(endowed_accounts), false)
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
    // let boot_nodes = vec![
    //     "/ip4/178.128.225.241/tcp/30353/p2p/QmYdDmRbpyjjM1M4aLS1btAMq4ouopsQLnHjp8imodomZa".to_string(),
    //     "/ip4/157.230.35.215/tcp/30353/p2p/QmdRRSjFmwQxrzDTih6c6di3W1oCf8BjELYF783hji4ZsA".to_string()
    // ];
    ChainSpec::from_genesis(
        "Akropolis OS Staging Testnet",
        "akropolisos_staging_testnet",
        akropolisos_staging_genesis,
        vec![],
        Some(TelemetryEndpoints::new(vec![(
            STAGING_TELEMETRY_URL.to_string(),
            0,
        ),
        (AKROPOLIS_TELEMETRY_URL.to_string(), 0)
        ])),
        None,
        None,
        Default::default(),
    )
}


/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(
    seed: &str,
) -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
    )
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )>,
    root_key: AccountId,
    endowed_accounts: Option<Vec<AccountId>>,
    enable_println: bool,
) -> GenesisConfig {
    let bridge_validators: Vec<AccountId> = vec![
        hex!("0d96d3dbdb55964e521a2f1dc1428ae55336063fd8f0e07bebbcb1becf79a67b").into(),
        // 5CtXvt2othnZpkneuTg6xENMwXbmwV3da1YeNAeYx5wMaCvz
        hex!("80133ea92f48aa928119aaaf524bc75e436a5c9eb24878a9e28ac7b0b37aa81a").into(), 
        // 5CqXmy44eTwGQCX8GaLrUfTAyEswGSd4PgSKMgUdLfDLBhZZ
        hex!("3c7f612cdda6d0a3aad9da0fb6cb624721b04067f00bd0034062e6e2db2cd23e").into(), 
        // 5DnUF5fQ6KNYPWRAcHYpMu32pUtdLv6ksRcSLeuofrxmPsTU
    ];
    let tokens = vec![
        Token {
            id: 0,
            decimals: 18,
            symbol: Vec::from("DAI"),
        },
        Token {
            id: 1,
            decimals: 18,
            symbol: Vec::from("cDAI"),
        },
        Token {
            id: 2,
            decimals: 18,
            symbol: Vec::from("USDT"),
        },
        Token {
            id: 3,
            decimals: 18,
            symbol: Vec::from("USDC"),
        },
    ];
    let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ]
    });
    let num_endowed_accounts = endowed_accounts.len();

    const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
    const STASH: Balance = 100 * DOLLARS;

    GenesisConfig {
        system: Some(SystemConfig {
            code: WASM_BINARY.to_vec(),
            changes_trie_config: Default::default(),
        }),
        balances: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, ENDOWMENT))
                .chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
                .collect(),
        }),
        pallet_indices: Some(IndicesConfig { indices: vec![] }),
        pallet_session: Some(SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        }),
        pallet_staking: Some(StakingConfig {
            validator_count: initial_authorities.len() as u32 * 2,
            minimum_validator_count: initial_authorities.len() as u32,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        }),
        pallet_democracy: Some(DemocracyConfig::default()),
        pallet_collective_Instance1: Some(CouncilConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            phantom: Default::default(),
        }),
        pallet_collective_Instance2: Some(TechnicalCommitteeConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            phantom: Default::default(),
        }),
        pallet_contracts: Some(ContractsConfig {
            current_schedule: pallet_contracts::Schedule {
                enable_println, // this should only be enabled on development chains
                ..Default::default()
            },
            gas_price: 1 * MILLICENTS,
        }),
        sudo: Some(SudoConfig { key: root_key }),
        pallet_babe: Some(BabeConfig {
            authorities: vec![],
        }),
        pallet_im_online: Some(ImOnlineConfig { keys: vec![] }),
        pallet_authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
        grandpa: Some(GrandpaConfig {
            authorities: vec![],
        }),
        pallet_membership_Instance1: Some(Default::default()),
        pallet_treasury: Some(Default::default()),
        pallet_society: Some(SocietyConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            pot: 0,
            max_members: 999,
        }),
        pallet_vesting: Some(Default::default()),
        bridge: Some(BridgeConfig {
            validator_accounts: bridge_validators,
            validators_count: 3u32,
            current_limits: vec![
                100 * 10u128.pow(18),
                200 * 10u128.pow(18),
                50 * 10u128.pow(18),
                400 * 10u128.pow(18),
                10 * 10u128.pow(18),
            ],
        }),
        dao: None,
        token: Some(TokenConfig { tokens }),
    }
}

fn development_config_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![get_authority_keys_from_seed("Alice")],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        None,
        true,
    )
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Development",
        "akropolisos_dev",
        development_config_genesis,
        vec![],
        None,
        None,
        None,
        Default::default(),
    )
}

fn local_testnet_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            get_authority_keys_from_seed("Alice"),
            get_authority_keys_from_seed("Bob"),
        ],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        None,
        false,
    )
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "akropolisos_local_testnet",
        local_testnet_genesis,
        vec![],
        None,
        None,
        None,
        Default::default(),
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::service::{new_full, new_light};
    use sc_service_test;
    use sp_runtime::BuildStorage;

    fn local_testnet_genesis_instant_single() -> GenesisConfig {
        testnet_genesis(
            vec![get_authority_keys_from_seed("Alice")],
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            None,
            false,
        )
    }

    /// Local testnet config (single validator - Alice)
    pub fn integration_test_config_with_single_authority() -> ChainSpec {
        ChainSpec::from_genesis(
            "Integration Test",
            "test",
            local_testnet_genesis_instant_single,
            vec![],
            None,
            None,
            None,
            Default::default(),
        )
    }

    /// Local testnet config (multivalidator Alice + Bob)
    pub fn integration_test_config_with_two_authorities() -> ChainSpec {
        ChainSpec::from_genesis(
            "Integration Test",
            "test",
            local_testnet_genesis,
            vec![],
            None,
            None,
            None,
            Default::default(),
        )
    }

    #[test]
    #[ignore]
    fn test_connectivity() {
        sc_service_test::connectivity(
            integration_test_config_with_two_authorities(),
            |config| new_full(config),
            |config| new_light(config),
        );
    }

    #[test]
    fn test_create_development_chain_spec() {
        development_config().build_storage().unwrap();
    }

    #[test]
    fn test_create_local_testnet_chain_spec() {
        local_testnet_config().build_storage().unwrap();
    }

    #[test]
    fn test_staging_test_net_chain_spec() {
        staging_testnet_config().build_storage().unwrap();
    }
}

// // fn akropolis_genesis() -> Result<ChainSpec, String> {
// // 	ChainSpec::from_embedded(include_bytes!("../res/akropolis.json"))
// // }

// fn akropolis_staging_genesis() -> GenesisConfig {
//     let endowed_accounts = vec![
//         get_account_id_from_seed::<sr25519::Public>(
//             "ac093ae2c4b5cc62aca5ceca961ed3bd3ad65d0fdcc3cbd206109d5ab970e171",
//         ), // 5FxGqPvuyvKaGvwaHAiTjvVpQMoZcgd1tLbWWWyPH4QNyc6Q
//     ];

//     let initial_authorities = vec![
//         get_authority_keys_from_seed(
//             "927b39cac18dabc394d7c744fad4467d51310bf299330f9810427f8508d6ee09",
//         ), // 5FNmTHadRw12fPwkrSdoKNznX5HpTcvcwvmKu5PF1suGiwP8
//         get_authority_keys_from_seed(
//             "6cd3b2029a60d1e8a415de9aeed40b76ed3815678f75557b12db2b57559f8d43",
//         ), // 5EXPv3Y6obajCD9PTCa4u6ZdZgQ2wowFh8yZA7DiKibirpDW
//         get_authority_keys_from_seed(
//             "c763486fcc0753cfde644da6d193d092d10015384cb5ef6cca7597bbb9a900b3",
//         ), // 5Ga8sxc52JGnb31zhizJpS9ixVzMneDjse8XLNAi4Gvp2mhB
//         get_authority_keys_from_seed(
//             "10fffd9162e7950a449eff6024ac326321228df2659c2a1f9d5c084c56fcc112",
//         ), // 5CSzfigG2EGM3MmCcjKSAJMdtgbh4eNKc54kVU9BJBPVxju3
//         get_authority_keys_from_seed(
//             "4e18e46d6e8c086a81a9162fa72d95bb3a0712f0ab73ea872cc88b810bdd2575",
//         ), // 5Dq6zBbF78utVLB16oAc3b7bCJKRksuoomoWeBF7LsbKVcjx
//         get_authority_keys_from_seed(
//             "a17221f222c706dea7adfb7e6ec3dbba9a7febc8eed6ff3aa5428db31a16c875",
//         ), // 5FiPUGuYULQhcxkdUhAakHprBFQWj37ac5YwaSo5Kph9Vypz
//     ];
//     let bridge_validators = vec![
//         get_account_id_from_seed::<sr25519::Public>(
//             "3a495ac93eca02fa4f64bcc99b2f950b7df8d866b4b107596a0ea7a547b48753",
//         ), // 5DP8Rd8jUQD9oukZduPSMxdrH8g3r4mzS1zXLZCS6qDissTm
//         get_account_id_from_seed::<sr25519::Public>(
//             "1450cad95384831a1b267f2d18273b83b77aaee8555a23b7f1abbb48b5af8e77",
//         ), // 5CXLpEbkeqp475Y8p7uMeiimgKXX6haZ1fCT4jzyry26CPxp
//         get_account_id_from_seed::<sr25519::Public>(
//             "2452305cbdb33a55de1bc46f6897fd96d724d8bccc5ca4783f6f654af8582d58",
//         ), // 5CtKzjXcWrD8GRQqorFiwHF9oUbx2wHpf43erxB2u7dpfCq9
//     ];
