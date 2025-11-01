use std::sync::{Arc, LazyLock};

use alloy_chains::Chain;
use alloy_consensus::constants::MAINNET_GENESIS_HASH;
use alloy_eips::{eip1559::BaseFeeParams, BlobScheduleBlobParams};
use alloy_primitives::{address, b256, U256};
use reth_ethereum_forks::{hardfork, EthereumHardfork, ForkCondition, Hardfork};
use reth_primitives_traits::SealedHeader;

use crate::{
    make_genesis_header, ChainSpec, DepositContract, MAINNET_DEPOSIT_CONTRACT,
    MAINNET_PRUNE_DELETE_LIMIT,
};

hardfork!(
    #[derive(Default)]
    PulsechainHardfork {
        /// Frontier: <https://blog.ethereum.org/2015/03/03/ethereum-launch-process>.
        Frontier,
        /// Homestead: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/homestead.md>.
        Homestead,
        /// The DAO fork: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/dao-fork.md>.
        Dao,
        /// Tangerine: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/tangerine-whistle.md>.
        Tangerine,
        /// Spurious Dragon: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/spurious-dragon.md>.
        SpuriousDragon,
        /// Byzantium: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/byzantium.md>.
        Byzantium,
        /// Constantinople: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/constantinople.md>.
        Constantinople,
        /// Petersburg: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/petersburg.md>.
        Petersburg,
        /// Istanbul: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/istanbul.md>.
        Istanbul,
        /// Muir Glacier: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/muir-glacier.md>.
        MuirGlacier,
        /// Berlin: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/berlin.md>.
        Berlin,
        /// London: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/london.md>.
        London,
        /// Arrow Glacier: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/arrow-glacier.md>.
        ArrowGlacier,
        /// Gray Glacier: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/gray-glacier.md>.
        GrayGlacier,
        /// Paris: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/paris.md>.
        Paris,
        /// Shanghai: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/shanghai.md>.
        Shanghai,
        /// Cancun: <https://github.com/ethereum/execution-specs/blob/master/network-upgrades/mainnet-upgrades/cancun.md>
        Cancun,
        /// Prague.
        #[default]
        Prague,
        /// Osaka: <https://eips.ethereum.org/EIPS/eip-7607>
        Osaka,
        // BPOs: <https://eips.ethereum.org/EIPS/eip-7892>
        /// BPO 1
        Bpo1,
        /// BPO 2
        Bpo2,
        /// BPO 3
        Bpo3,
        /// BPO 4
        Bpo4,
        /// BPO 5
        Bpo5,
        /// Amsterdam: <https://eips.ethereum.org/EIPS/eip-7773>
        Amsterdam,

        // PulseChain modifications
        PrimordialPulseBlock,
        Treasury,
    }
);

impl PulsechainHardfork {
    /// Ethereum mainnet list of hardforks.
    pub(crate) const fn mainnet() -> [(Self, ForkCondition); 16] {
        [
            (Self::Homestead, ForkCondition::Block(1_150_000)),
            (Self::Dao, ForkCondition::Block(1_920_000)),
            (Self::Tangerine, ForkCondition::Block(2_463_000)),
            (Self::SpuriousDragon, ForkCondition::Block(2_675_000)),
            (Self::Byzantium, ForkCondition::Block(4_370_000)),
            (Self::Constantinople, ForkCondition::Block(7_280_000)),
            (Self::Petersburg, ForkCondition::Block(7_280_000)),
            (Self::Istanbul, ForkCondition::Block(9_069_000)),
            (Self::MuirGlacier, ForkCondition::Block(9_200_000)),
            (Self::Berlin, ForkCondition::Block(12_244_000)),
            (Self::London, ForkCondition::Block(12_965_000)),
            (Self::ArrowGlacier, ForkCondition::Block(13_773_000)),
            (Self::GrayGlacier, ForkCondition::Block(15_050_000)),
            (
                Self::Paris,
                ForkCondition::TTD {
                    activation_block_number: 15_537_394,
                    fork_block: None,
                    total_difficulty: U256::from_limbs([
                        58_750_003_716_598_352_947_541_u128 as u64,
                        (58_750_003_716_598_352_947_541_u128 >> 64) as u64,
                        0,
                        0,
                    ]),
                },
            ),
            (Self::PrimordialPulseBlock, ForkCondition::Block(17_233_000)),
            (Self::Shanghai, ForkCondition::Timestamp(1683786515)),
        ]
    }
}

/// The Pulsechain mainnet spec
pub static PULSECHAIN_MAINNET: LazyLock<Arc<ChainSpec>> = LazyLock::new(|| {
    let genesis = serde_json::from_str(include_str!("../res/genesis/pulsechain_mainnet.json"))
        .expect("Can't deserialize Mainnet genesis json");
    let hardforks = PulsechainHardfork::mainnet().into();
    let mut spec = ChainSpec {
        chain: Chain::from_named(alloy_chains::NamedChain::Pulsechain),
        genesis_header: SealedHeader::new(
            make_genesis_header(&genesis, &hardforks),
            MAINNET_GENESIS_HASH,
        ),
        genesis,
        // <https://etherscan.io/block/15537394>
        paris_block_and_final_difficulty: Some((
            15537394,
            U256::from(58_750_003_716_598_352_947_541_u128),
        )),
        hardforks,
        // https://etherscan.io/tx/0xe75fb554e433e03763a1560646ee22dcb74e5274b34c5ad644e7c0f619a7e1d0
        deposit_contract: Some(MAINNET_DEPOSIT_CONTRACT),
        base_fee_params: crate::BaseFeeParamsKind::Constant(BaseFeeParams::ethereum()),
        prune_delete_limit: MAINNET_PRUNE_DELETE_LIMIT,
        blob_params: BlobScheduleBlobParams::default(),
    };
    spec.genesis.config.dao_fork_support = true;
    spec.into()
});

// /// Deposit contract address: `0x00000000219ab540356cbb839cbe05303d7705fa`
// const MAINNET_DEPOSIT_CONTRACT: DepositContract = DepositContract::new(
//     address!("0x3693693693693693693693693693693693693693"),
//     11052984,
//     b256!("0x649bbc62d0e31342afea4e5cd82d4049e7e1ee912fc0889aa790803be39038c5"),
// );
