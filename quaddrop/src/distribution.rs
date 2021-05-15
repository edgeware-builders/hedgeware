use hex_literal::*;
use rococo_parachain_primitives::AccountId;
use rococo_parachain_primitives::Balance;

/// Split endowment amount for Commonwealth
pub const COMMONWEALTH_ENDOWMENT: Balance = 50_000_000_000_000_000_000_000;
/// Split endowment amount for stash
pub const STASH_ENDOWMENT: Balance = 10_000_000_000_000_000;
/// Split endowment amount for controllers
pub const CONTROLLER_ENDOWMENT: Balance = 10_000_000_000_000_000;
/// Genesis allocation that will fit into the "balances" module for Commonwealth/Founders
pub fn get_commonwealth_allocation() -> Vec<(AccountId, Balance)> {
	return vec![(
		hex!["14ad3d151938d63a4e02454f034a3158c719ed9de2e233dd0843c2d81ddba53d"].into(),
		5_500_000_000_000_000_000_000_000,
	), (
		hex!["12d490251399a081935bf731184d2bf37d228bc38d3d68a8e3822933bcf23a09"].into(),
		5_500_000_000_000_000_000_000_000,
	), (
		hex!["a87d1f2e04d8e95499f8a6f18214355bcb2fd2c9370ab5c19f379dd9d3167075"].into(),
		5_500_000_000_000_000_000_000_000,
	), (
		hex!["4cb0922d0fb5217553da0da70bd4076812ad2a5cce860524ff7b5e6e3629f962"].into(),
		3_000_000_000_000_000_000_000_000,
	), (
		hex!["78040adec849fff1c66c16ab8ac1534ed27e37a8a1da8aa3239267a883369566"].into(),
		1_500_000_000_000_000_000_000_000,
	), (
		hex!["cc3d67fe87c81b5895ed89cfb1c44cc29c3798bac93368487dfc11364d6e3068"].into(),
		COMMONWEALTH_ENDOWMENT,
	), (
		hex!["eeb7482b9cce124538b1aeea1a7935d313b9f01cc6192fb4cc6bdf1b0f6b4430"].into(),
		COMMONWEALTH_ENDOWMENT,
	), (
		hex!["765e19400b3f7d44e5677d24d9914ae8cabb1bf3ef81ebc1ca72ad99d312af46"].into(),
		COMMONWEALTH_ENDOWMENT,
	), (
		hex!["ca91588bb9258ade926d0c0631798d7e3f17c4581fae56283287d54883244a55"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["1ec5e3d9a77ac81d6da0290c04d003bbcb04af8c4902bd59dbf9be4dfa47234f"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["d6fb4a2f0d5dfc62c37a09e6aac5c3ea4ce2ba021f553c940e63d59dadf0cd24"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["720967cda4c9097924d705695b62dfb6dc6dbeade65b5575abf5c4ca38e50503"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["38a58e82baf9df6ec1f9a7064a337f872778649f3dd9002e3fe48df94b475232"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["de90c8b070c0a63fbf52655af7492dc8e7d985334a4c60c02bc2f59424ff1430"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["0e33e22cd22b272f388bcd41f13942d803089106ec350b8754e1d290ee6ff52b"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["9665bd715c72b686c2557fe11e6cd54924adef62dc1f52cf43a503f363cf843c"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["6aa8f0dd6b6221788d68bf2486126fb14bb65ea710028c11f7ca131e0df10707"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["464c96a206e310511a27acc92b2e410a14bd83cb8788b522d0cee03f0d285862"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["ae5bfe517affa6f7456ad6b9f7465520059e6d7b2a8928673460461abb741c18"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["34c71b1e42e910b94b8cbb2c960873bd4bf0db6e80afdf41cdc52acd91d6393f"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["6a782c02fd24ed538224f3d0bda56146bc6bacd34f9a784c1b5367e19cda456e"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["d02002915139ac3e4552c5006f92cccfbf8b02cb4d4ca1993a69d63368cc1f0c"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["4864744ab79cd62cbc1094da8e6f54aba8cba7ed6d616bdd8df10572d146c15c"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["143f9f9019fa62919ed6da39b8f147cb52501438e1e9e7a82d22d7b49df18e59"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["a01bef9c8591ae4155f9483eee09db092fb39bdd580e3715e927333e4aa89d6d"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["4e7de9c8f3564fe5cc5057de51c41b40a7da801d22c6ee5aa57f8bb2838ae857"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["00e5a14e08930f94148569274ca1e9355938fabf65fffef3b7cb3c3e3edabb23"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["ce64070e4dffe61183241dca3e922b65ecd509430a3e283fab5c143532f79d3e"].into(),
		STASH_ENDOWMENT,
	), (
		hex!["b0492fa7ac84ecb20f9f69e1c328b521fce8f472af2cc13784286d2240e4924c"].into(),
		CONTROLLER_ENDOWMENT,
	), (
		hex!["58e8d8750021d11f5bf1106966235ed293a4288511016af7f3b2e81a84ead342"].into(),
		92_500_000_000_000_000_000_000_000,
	), (
		hex!["688421b084a363cb394c5e3a7c79f44482bf2f15f6d86ea37ae110a3af238f07"].into(),
		10_000_000_000_000_000_000_000_000,
	), (
		hex!["765169492c492ee29f2af9af46f9e1b117aa0283b73a4361ae12ace1c41a6c72"].into(),
		150_000_000_000_000_000_000_000_000,
	), (
		hex!["6490626e3bde470c449e90b83df92ddb8514f02067a0ddd66f1080b5033dec2d"].into(),
		1474_790_000_000_000_000_002_374,
	), (
		hex!["ec80b8b78a2b283f0a48712c8446241cf5f36d2f480559cdc73253981963f402"].into(),
		25_000_000_000_000_000_000_000,
	)]
}