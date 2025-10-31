//! Ethereum bootnodes come from <https://github.com/ledgerwatch/erigon/blob/devel/params/bootnodes.go>

/// Ethereum Foundation Go Bootnodes
pub static MAINNET_BOOTNODES: [&str; 4] = [
    "enode://d860a01f9722d78051619d1e2351aba3f43f943f6f00718d1b9baa4101932a1f5011f16bb2b1bb35db20d6fe28fa0bf09636d26a87d31de9ec6203eeedb1f666@18.138.108.67:30303",   // bootnode-aws-ap-southeast-1-001
    "enode://22a8232c3abc76a16ae9d6c3b164f98775fe226f0917b0ca871128a74a8e9630b458460865bab457221f1d448dd9791d24c4e5d88786180ac185df813a68d4de@3.209.45.79:30303",     // bootnode-aws-us-east-1-001
    "enode://2b252ab6a1d0f971d9722cb839a42cb81db019ba44c08754628ab4a823487071b5695317c8ccd085219c3a03af063495b2f1da8d18218da2d6a82981b45e6ffc@65.108.70.101:30303",   // bootnode-hetzner-hel
    "enode://4aeb4ab6c14b23e2c4cfdce879c04b0748a20d8e9b59e25ded2a08143e265c6c25936e74cbc8e641e3312ca288673d91f2f93f8e277de3cfa444ecdaaf982052@157.90.35.166:30303",   // bootnode-hetzner-fsn
];

/// Ethereum Foundation Sepolia Bootnodes
pub static SEPOLIA_BOOTNODES: [&str; 5] = [
    "enode://4e5e92199ee224a01932a377160aa432f31d0b351f84ab413a8e0a42f4f36476f8fb1cbe914af0d9aef0d51665c214cf653c651c4bbd9d5550a934f241f1682b@138.197.51.181:30303", // sepolia-bootnode-1-nyc3
    "enode://143e11fb766781d22d92a2e33f8f104cddae4411a122295ed1fdb6638de96a6ce65f5b7c964ba3763bba27961738fef7d3ecc739268f3e5e771fb4c87b6234ba@146.190.1.103:30303",  // sepolia-bootnode-1-sfo3
    "enode://8b61dc2d06c3f96fddcbebb0efb29d60d3598650275dc469c22229d3e5620369b0d3dedafd929835fe7f489618f19f456fe7c0df572bf2d914a9f4e006f783a9@170.64.250.88:30303",  // sepolia-bootnode-1-syd1
    "enode://10d62eff032205fcef19497f35ca8477bea0eadfff6d769a147e895d8b2b8f8ae6341630c645c30f5df6e67547c03494ced3d9c5764e8622a26587b083b028e8@139.59.49.206:30303",  // sepolia-bootnode-1-blr1
    "enode://9e9492e2e8836114cc75f5b929784f4f46c324ad01daf87d956f98b3b6c5fcba95524d6e5cf9861dc96a2c8a171ea7105bb554a197455058de185fa870970c7c@138.68.123.152:30303", // sepolia-bootnode-1-ams3
];

/// Ethereum Foundation Holesky Bootnodes
pub static HOLESKY_BOOTNODES: [&str; 2] = [
    "enode://ac906289e4b7f12df423d654c5a962b6ebe5b3a74cc9e06292a85221f9a64a6f1cfdd6b714ed6dacef51578f92b34c60ee91e9ede9c7f8fadc4d347326d95e2b@146.190.13.128:30303",
    "enode://a3435a0155a3e837c02f5e7f5662a2f1fbc25b48e4dc232016e1c51b544cb5b4510ef633ea3278c0e970fa8ad8141e2d4d0f9f95456c537ff05fdf9b31c15072@178.128.136.233:30303",
];

/// Ethereum Foundation Hoodi Bootnodes
/// From: <https://github.com/eth-clients/hoodi/blob/main/metadata/enodes.yaml>
pub static HOODI_BOOTNODES: [&str; 3] = [
    "enode://2112dd3839dd752813d4df7f40936f06829fc54c0e051a93967c26e5f5d27d99d886b57b4ffcc3c475e930ec9e79c56ef1dbb7d86ca5ee83a9d2ccf36e5c240c@134.209.138.84:30303",
    "enode://60203fcb3524e07c5df60a14ae1c9c5b24023ea5d47463dfae051d2c9f3219f309657537576090ca0ae641f73d419f53d8e8000d7a464319d4784acd7d2abc41@209.38.124.160:30303",
    "enode://8ae4a48101b2299597341263da0deb47cc38aa4d3ef4b7430b897d49bfa10eb1ccfe1655679b1ed46928ef177fbf21b86837bd724400196c508427a6f41602cd@134.199.184.23:30303",
];

/// Pulsechain Bootnodes
/// From: https://gitlab.com/pulsechaincom/go-pulse/-/blob/master/params/bootnodes.go?ref_type=heads#L33
pub static PULSECHAIN_BOOTNODES: [&str; 10] = [
    "enode://bdb96e7ff6607414a4be8cdc8458861e9c22a25a0c254c7bb9c9c8423912e998b59e7ba012801538480eb78cec4d6766ab0b379d0b60356de84a7cdaec988c0b@5.9.124.244:30303",     // bootnode-001-hetzner-fsn
	"enode://d69f8d28804ab34f7d5e20ac8bd4940412602787e2c37fc3600adc60dcd5d0a52e1fe1baccbefb6e278e1ee59fcb099c45db242edeb5e0a4547ff971218a0592@148.251.54.222:30303",  // bootnode-002-hetzner-fsn
	"enode://1c9e030aa44b95b8239e1c97926787e12770c015b9dbf7a89b1178a5f4fab02462fde3489662119872dad5998e23440f78daae753d7a8f800900d871f08650a4@65.108.236.231:30303",  // bootnode-003-hetzner-hel
	"enode://95097eaeda4118297ad0ccb6160e1c9188af7560d25b4724052e0f004a33aaddb0e468103d622c77539b692fd1d9f3c156cb76c9ea402a86e3170d6ae60092e7@135.181.212.228:30303", // bootnode-004-hetzner-hel
	"enode://da30ab2475cda64c2454b659a3ef045884c7d02b97d524d710020fdc2f37192b0aac7992bca8b7afd57474eb477e95567c8e0fe98003b779834f265304376c3c@135.181.229.180:30303", // bootnode-001 maintained by www.g4mm4.io
	"enode://01d93871155cbe270bc60acfebc1aa859aacce002acaac39d633aa8e7c186ee26d19a41a50d8bc094c025a546ae5e1a38dc21ead75b4e7ddf4e917988d2f7c74@46.4.224.159:30303",    // bootnode-002 maintained by www.g4mm4.io
	"enode://96367e5e533cde68b6d3e7cc5308901fb1e4b1df51d2a0442df365fcfb8ba27a6e8bcde44b3629579da9e13d819f6059386a1e81ea4c5fd10d14599639c16214@46.4.224.160:30303",    // bootnode-003 maintained by www.g4mm4.io
	"enode://aece632270d66ff6bf9e9528e766b5829fb3b7812d48e4934c2768c45976b5f98559ce6d5763dc16d4351b15e776b55e2b983a0c367bdbe6279cfb3242f2587e@95.217.148.233:30303",  // bootnode-004 maintained by www.g4mm4.io
	"enode://95e1761e526d77fc732416a31c9c1795863b557ea02880101c01d14d13fdabb9312ce45c4f3037ad88002815f6826a36d86e42a1a7122f9188c64f53c4b68b1e@148.251.185.52:30303",  // bootnode-005 maintained by www.g4mm4.io
	"enode://0ad3bc059105b0cbc1d30a330f79b4fd4ef40f37782194daa6d3412a29a69e0190dd246fc019be9157a4bf095b584ab7874beba4c71c02156f602f32ff389f00@138.201.220.52:30303",  // bootnode-006 maintained by www.g4mm4.io
];

/// Pulsechain Bootnodes
/// From: https://gitlab.com/pulsechaincom/go-pulse/-/blob/master/params/bootnodes.go?ref_type=heads#L48
pub static PULSECHAIN_TESTNET_BOOTNODES: [&str; 8] = [
    "enode://3edb6b2b76ef50af30d3b02e098f00546f1a460ff1c82adad2639a57f6742c69516d24d760c0dd4555334adb01e6f3327f1a61056b3d89db4de10060248e8dea@65.21.204.190:30303",   // bootnode-001-hetzner-hel
	"enode://2b9af9cc9d09e2d2ef8cb3203f859e69b0175c1d7c41e14acf5162b239a773a966eea98a71999af9424ddb5b27a44759318869f8a4ba954483889aafdd6ea921@157.90.129.118:30303",  // bootnode-002-hetzner-fsn
	"enode://2181f1b061713260eb806a7824d880088bbf3b47cf60fa7bc610439aedd20c213479df83a6eeaf42b41ad6f3eac6973ddc1d8d903a00094603ad667d5d87161f@37.27.57.158:30303",    // bootnode-001 maintained by www.g4mm4.io
	"enode://c1a8bc7b4a7fa66e3eed6732d966f98de6b4e4243353e9c2f4d632126b8da73022b3becf1582e940d3feeaf3243f63304356856053c76a7ea6cc5c50ad21d483@213.133.100.132:30303", // bootnode-002 maintained by www.g4mm4.io
	"enode://7dce6f27d102ae4fac47042b0ed8fadfce0037a5384ae171017b8b6684efe57bb850359e00582a6f8099ac60b41e16efe46afb8772270e5e1cad3f7ed79d0e41@85.10.193.180:30303",   // bootnode-003 maintained by www.g4mm4.io
	"enode://94eedc89cebf735374bbae8078fff23744d7b118af6c0f33804d1ccf6cc8fdb9db7f55ccf81455034bc34b43f00fdc7ea5693b86d6c6098fc9603f689d0d1fca@95.217.150.118:30303",  // bootnode-004 maintained by www.g4mm4.io
	"enode://5999295986a65151d416dc09635da46896e8cd5e2f0dda0823ed3a0981dc50885407e5a990aa34e165c345e7bebaa837fcf9afaaa5e62d5add1fed6d4c9edbcc@95.217.148.234:30303",  // bootnode-005 maintained by www.g4mm4.io
	"enode://86831392545cec45fa30b578717684c4ffcf2e2bf050d4ecfdd5b9a6b2136e10d58f8606bacdd137e6ce68c1081442e39347ed391f166366f4951ab031156e93@138.201.193.233:30303", // bootnode-006 maintained by www.g4mm4.io
];
