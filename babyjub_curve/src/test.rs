#[cfg(test)]
mod tests {
    use crate::babyjubjub::BabyJubjub;
    use bn254::babyjub_scalar::BABYJUB_SCALAR_MODULUS;
    use bn254::{babyjub_scalar::BabyjubScalar as Scalar, scalar::Scalar as Fp};
    use crypto_bigint::U256;
    use curve_traits::ProjectivePoint;
    use curve_traits::{affinepoint, projectivepoint::is_on_curve};
    use traits::traits::Field;
    const BABYJUBARR1: [ProjectivePoint<BabyJubjub>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1c423919997e08d28d5e9f4396587dac723c70fc42aedeafc99a5795d48590d8",
            )),
            y: Fp(U256::from_be_hex(
                "1e5effc0cf4e8f5e1bf893b081cac419f1677929fe920b1635c5321dd38c60be",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1277d7dc48cfb9e6744db3af21fe85fded2916611d5fdad0244e594dbdbf55ac",
            )),
            y: Fp(U256::from_be_hex(
                "1ee58752ad988d729306c049d52303c29bcc63430c87eb341eee3e0850f7421e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "17ad431dad8450f80860686786a764ff9e0eb4a363ec48e746d6bf33a09272df",
            )),
            y: Fp(U256::from_be_hex(
                "16c8bcfda0f2363a4a53f0baa9da107b0473fdd4b1bab28c56321458c0dcd85c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "15e6addfa3614fdf7608d42e97aaf710e40b5216e1f73cf17fa9f9f7319b14e3",
            )),
            y: Fp(U256::from_be_hex(
                "00be01a6696211de3538530d30d5baa1251a0dba37c977836a9827aee33cc0d9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "148958ceba902e93abfcdd52f13c0a85f4150399f01dee60f69f7264119dce84",
            )),
            y: Fp(U256::from_be_hex(
                "26976691b1739ef28fff0f12338f12944f361f590eceff601c1f433aa7689435",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "187b0cdd8cc88f329fe51247976aac6658bfbc5862b7f0431cc0d683bdbcb188",
            )),
            y: Fp(U256::from_be_hex(
                "02b078910465a023f256945f0277f9086a2aa9a7855bce09284f041ac983792e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "12f32a82f018fd28c70f9c9017f3dbef409d05891a557e8844a20ada5bdeb8ca",
            )),
            y: Fp(U256::from_be_hex(
                "2837ac080a26fdc2ad503a8f29e9d82486286f5f054851562225c0aab82def7d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1b6144c671b2a806f7d5d67374e7a0ea521cea0f88839b7160df5621a90a79b1",
            )),
            y: Fp(U256::from_be_hex(
                "0ebc5fd5ea2cf0945687ef29c5e9c2f48c2a3693633923ddeebe45fde7d6ff00",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2934760e49ca18f51316c99dc1fae8cbffaa3283fab7ca5728349ebbe0e007b4",
            )),
            y: Fp(U256::from_be_hex(
                "1606fdd16bdb4bd745663b0a319138e6b22f01d3c925f4e957ce0b74549e2ac5",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "12938c568217bc866abae15e38b9b3dfc21f4281d5966a1994369914e5d35469",
            )),
            y: Fp(U256::from_be_hex(
                "1140b0eabba5dfe9e8b0c2d9cc7e029ff0a398871a49fdc76b53211de1545810",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "301828365fbf83ad753b4821555ab94c2fbf14c74a0dfd367ae8c1aacdc0968d",
            )),
            y: Fp(U256::from_be_hex(
                "254480bdd943718e3f076e8a748599ab39792bcec8783c159fee9dbda60accbb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2ac3a8d93a34fd0cf96a232c3d42e9a7b60a0bda7a10e6f9ef9874c081187878",
            )),
            y: Fp(U256::from_be_hex(
                "01cfc8420a360b092a1a9ce832d272e856f1b3572be9f950d92bb3d2755c4f21",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1ced76c6956e884f1ae4e46fea3c340ca107ec758eab1e5a4719ae1347cd17fc",
            )),
            y: Fp(U256::from_be_hex(
                "25cc14ce43b0dbe2fa524ec7cdd29616f05ab53a6161c9b29d0af45999cd95f0",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "186cf213b56f73db858e8baf77670114881c6058cf56ab0d6fb5b5534a317d44",
            )),
            y: Fp(U256::from_be_hex(
                "2f32e621e08294b8376273bfaeedbaa6606a4b885c7010f00d0ba2f5cf0a463a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "212404a047c0ac2b7431508d5866ed37d1c86bf59a4572d2ca2dfb348883a45a",
            )),
            y: Fp(U256::from_be_hex(
                "2b7d3be0b4dfc4ecdf842d7e6b306b5e4ce4b62bca1ee79ee76808cbeefee371",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "10534876f098668c4374d3adee516de8eb0dc1671c14da8dfbab62007faa929c",
            )),
            y: Fp(U256::from_be_hex(
                "0b047932a68fafe41d50a150b78a4b174f14a2bf8abe3a2a76af4b437a4d10de",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0ce6a90620fe28b6000f50d60c7db653c0902a992b6cdeb9f318751ec9c9e689",
            )),
            y: Fp(U256::from_be_hex(
                "2c3e5988d7f4a044219e47f2e6d86ec269da74a01c3b2c4dc34dfc3c31bd27f6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "192be97c34b8d46b998a7b3a00c36142b662c7ccba3860327b5ea585b7cf7b01",
            )),
            y: Fp(U256::from_be_hex(
                "2ddd0539fb687a94c0caee589de06f70ddafbb9a475ea6c3f6e64a9d173c3994",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1391ce88f905600132a471bc297cd859f99520f138064300957489b08b1f280b",
            )),
            y: Fp(U256::from_be_hex(
                "13f45c7e7d1970ef387aa9c49e988275d05c5cad27f18cbee33744835c1735e8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2aae95705db4cd2e0c69f87588942c5f372b981bfcb3c3b0cd075549d1840b90",
            )),
            y: Fp(U256::from_be_hex(
                "119402b7b451bab72a2892c16dceb28fae4726a1de78d2bb675d947dcd957271",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2762ff2a755c44b99336504d6b678e484b2113fa33cbbffe51be89662ddeefcd",
            )),
            y: Fp(U256::from_be_hex(
                "07d510371e7e810b6d2665095048c1db93853a6c26a87c6d86c41abcd3f57e44",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1d71a064de2854176bdbf096eba4fef1a7960ef7906152a1e6383595ed0c094b",
            )),
            y: Fp(U256::from_be_hex(
                "020dff86246badb732a3dbb53bb435ecaafcf1547c50d38c397a83f30496870c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2d76c203fe74108b9d6f9a6b65da1e1c4ede212c1a7c70b03c539a3a9a57ffd0",
            )),
            y: Fp(U256::from_be_hex(
                "0825502e478f2e00d8ef791c9171593ec93243daec8447db80bd76d8eda7ac20",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "156a426de57e29c3abb444f56d77c87755607aa3a5d7b1053a8dc056093784a4",
            )),
            y: Fp(U256::from_be_hex(
                "16b0f4171b14dfd31876d290a95377c9d54465efe7da0e12e6dd7fb8e484246d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1874297360edb43d04fd2e1e8e6109bc5d17792ab9efdca76e1c792ac069ed45",
            )),
            y: Fp(U256::from_be_hex(
                "288a7168c051f01923df6de30c6f369f2f0cd00ea1e499dd9720a57655984cc8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0b6fa2b5353ab3386c4ff1bfa7586862421f3b42f592868a88b335eb31d46ac7",
            )),
            y: Fp(U256::from_be_hex(
                "02928f21fe25e20ec16be608cd32c81ae6928605315130ce19cd5d83b7716c99",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "249d06b98849c47fbec753eceb8e17017a899f4525919994454fd028da11e3b4",
            )),
            y: Fp(U256::from_be_hex(
                "17b7d75524516ecd012bc404827c8ee165e25183db1baf1b8f17dcf2d5ff6433",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "206599125f74c28de0934436d9d8b8d9cc4250f788323b995c182505ff7ad3a0",
            )),
            y: Fp(U256::from_be_hex(
                "274ce6aeeb61fe977ba172b9506a17fdf0e50bb12849c6398c03794f4f48650c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "24ebc79f6af2e20090e076deaf2dfe63b659dd2fa222b83865542893a13c1ee6",
            )),
            y: Fp(U256::from_be_hex(
                "1d7a191d413c8aebab3c00348ed2850f3945a78135106ce6bf175ac1badd9560",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "17f60fa6f5c136b8660eaf99c2c5a3662330c4e723677a4a0616463eb174801c",
            )),
            y: Fp(U256::from_be_hex(
                "0148f1c9d6f8950100ddf3e03c4ea3cf2e819e77d4137efc21bece17227b27f1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "205b80ec3f7fa896c6bb81b2e174e0c2c7357ca4102157c8a7c06f870f929f83",
            )),
            y: Fp(U256::from_be_hex(
                "0025dea3499bf55e058d19986a9f54df5b0d8e1150227aac58038fe9aca7bb43",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "136f04ca67712e3e4678a418d87dc3fb4cdad10b5a52494cecec18a46dfd39d7",
            )),
            y: Fp(U256::from_be_hex(
                "13802829732695ffbf84c1ddbe4fe3bc30445b5c378cfeabcdde9a7be1abe91d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1d83e39f8cce51f9960ade1453e985b380b345fc4ce11d0a8cd1feae904767bb",
            )),
            y: Fp(U256::from_be_hex(
                "237eeb1afa5957287e03ec62214421df30fab58c204fd6469dc95965edd7ec6b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2ca4224999fcd19b98b295d5922b75b90112c883609d48f773bc1c5a2ab23244",
            )),
            y: Fp(U256::from_be_hex(
                "1e44bb25dd31101cbf497551944c5747e5c61ed9fe4b7f597bf4b7a01eb14dc6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "134cff61de69f641a8ec93dc8636429fee99f0163782fdac7c90342f39321cbb",
            )),
            y: Fp(U256::from_be_hex(
                "1015de3c3aab76b1a21b0d1d65371e6693a5c8edd4f74667a8d40ad2958d78b9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "068aa9a0205d6ea642464673cb47131c977db219686c3f4c9ef631e528585c81",
            )),
            y: Fp(U256::from_be_hex(
                "12a11885c7744b16e055b16edb2a19dc5eaeefb59a0b22c350cb03b3b53990f2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "13531001c6b389341dde3e2b1fc7d50a79b76ce50458cde4c99c77d6e8afd5d4",
            )),
            y: Fp(U256::from_be_hex(
                "03d3797e9916344a2f23b3225db5109bdd4d194396995a7bad6ea7169133385e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "13f64cc2d5c659888e0798061454d3b7980357b74afc8309a6ddc310e32923c3",
            )),
            y: Fp(U256::from_be_hex(
                "125fd16bd842f08a2a02f8f57c7947769957a65e557eb629ff6a2f631fef2c1d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2e42a0d12f0231a9e235e79b76e1b5050f22aef941a791a765294331148a98c8",
            )),
            y: Fp(U256::from_be_hex(
                "2eb9411c6292ecb9ce1bbac6a5476a509583b02dbe592cef7ffda27afbe2f91b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "11e9849897c2b42167e37975ed86f9e1b5dbda841165a63c9e3c63f3cf6e292c",
            )),
            y: Fp(U256::from_be_hex(
                "01f50235152cba3d6aa4e2bba35792824aa35770ccc9649ea50e4ad2c07540d4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "189c97f168a2a1f18547c04f7541c40d8d117c1b9e6d2850f57d8c7b910bd88d",
            )),
            y: Fp(U256::from_be_hex(
                "21605967836178222addf099914adbdecfb56850d57c4e76769aeede00005761",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0acc12497c28c87f20e61d6ecde7efcca8fba93c1b8e9fdb24bb905a68818401",
            )),
            y: Fp(U256::from_be_hex(
                "235507c3cf7d5a5ad29c20283a84749a65db2f936a9327b9f0512d60dfec0424",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0c66302517cc79829d0ca7d2e9299caadfeff2619c9fa4105961ca76d4799242",
            )),
            y: Fp(U256::from_be_hex(
                "1c94fb0146763da1d02f62ce6582f636f6eef4e1a3537f5c5be09d66968faed5",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0f87efe683a87e2ec3f013e94a60b8cfdf7a2bd4ce6c11742c6f1c839957b0d8",
            )),
            y: Fp(U256::from_be_hex(
                "22ebdb37ac7d150f8981dc91d300544b893eb617ca360f059f2397a500193fe0",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2b46c79fca6cec31108229bae20953d827561d6a9ff64bec3a8e25dd4b2b6aec",
            )),
            y: Fp(U256::from_be_hex(
                "289e39c0e2ed07b6e84af995bb06ff27b911731c70839aeaf6cc4b6ef5405140",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "09bf73c86a8ff0da4fc8ca1b9f50a27a6efc14e8fac894c35eaf55dbb70aebd6",
            )),
            y: Fp(U256::from_be_hex(
                "144abce7844404721beb15b17e36bbff37c4ecc3be43af656bbc0ab57578a0e6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1175312391d5f02bdba348ddc898955bca3ee42fdf5754bbc63be7e91b961d22",
            )),
            y: Fp(U256::from_be_hex(
                "1f17520e5708b7b5edeb8dfc468bfe004b1132fe1ace2cdfc33959beb5c27a9b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2bda2c80b13ab384cb81f15d90cc5c0c169b90d39e58bc0401ccaa3c0c896c20",
            )),
            y: Fp(U256::from_be_hex(
                "1092c844def5df70b8fd5cb30387aa83a930067a76ea76cc2c2ec9a37c76585e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "260f635495cc9ade2bba5ca365da699408b903d602fe46764f4c006683885b7e",
            )),
            y: Fp(U256::from_be_hex(
                "0c5b56fbe24c48c591416aac46f5da01d578334319cead163a324490dc15c038",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "01e5c2f5b96b01b5fda6a102a584f3f6c4e7ec3e474ece9deaa619cbe7a9070f",
            )),
            y: Fp(U256::from_be_hex(
                "24e6269e92c16ff6f30859d38c92db3cd6b86f52ab5e249fe0ad7a25186dfbd7",
            )),
            z: Fp::ONE,
        },
    ];
    const BABYJUBARR2: [ProjectivePoint<BabyJubjub>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2d2cb97453645db13542bc74a5b9f956eb10a924016a91a2c19738cc81faf464",
            )),
            y: Fp(U256::from_be_hex(
                "29f7547d5505c74668ae85c4423aeba77c7d3d18abcadd1e2bd1d69337293ba7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "301763e743bacda459669d7c6760b70c3cdb151690c656a6a07c43e0b496f8ca",
            )),
            y: Fp(U256::from_be_hex(
                "16fb611d85f17606cf68f45ceccff305926c5f466e449d656d3b87fc84a6911c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1baeba197aa9b6bea3975a3cb7b3db58402ec0bd4451528ecb00ccd25ea18a56",
            )),
            y: Fp(U256::from_be_hex(
                "014ed2d14ec857e1a95301efa61f11dd72429d967eafcb7220bfb002134aa25c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1abdc2deeb9e2cbaee11a223592ed066b79aa6886e0a6f89cd22fb42d87cb33a",
            )),
            y: Fp(U256::from_be_hex(
                "1ccf69a130fd7045a653f4769c27cf17b2f0242f5de0a0876ce7a2826017fa1b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0fa0a6bbff3f4ddcdd273fe3b9df6d58d5122fe24ee34d0a6efcc8a75024afd1",
            )),
            y: Fp(U256::from_be_hex(
                "182ec7ae51d99640c5869e115479383d3e90e5129890d9ad73fbfa7230f7c381",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0b409ea025d581270d0058aff3b12b676502759a2bdf4939e8ef59797de42bc5",
            )),
            y: Fp(U256::from_be_hex(
                "0c9bcb63bb3712178cf8e829388a822295d88228349058274d1a89304ac41345",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2595987178e89464c85db4e1b4c92223fc004110f58bd6af56cbc7b1cb2f01aa",
            )),
            y: Fp(U256::from_be_hex(
                "232a8b93624f4d73c083f11ca0049bb32cf8b17db4b580dad23865f721a78664",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "222f6bb165a8fcb5e1cb5278f8e10b166e82b91d6b26cefb83cbc7e06e3b62d2",
            )),
            y: Fp(U256::from_be_hex(
                "305553e7dab2fe277990a7be00082105cf0174b4e5b6ba55b4095a6e075d465c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0a4fefa395fcfec438a76135d3150bae59971c21d4860f32d528404ce0b3f49f",
            )),
            y: Fp(U256::from_be_hex(
                "10dfe1c628408d4c85e893bd9e406b9ec394f32002146f915c220e27053febb2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "13c0ccd233965813b8f3ac79a20c307a8c1bb69ec9fc9137098ccf6cb5c9a0bd",
            )),
            y: Fp(U256::from_be_hex(
                "0d05e6147afeda7666b51e011d42d533ec96c65ce15a550b74ac920600dea926",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1ab307ceaf75207ce2d250b95706bdfcec42e807c40215e394de42a0874feace",
            )),
            y: Fp(U256::from_be_hex(
                "188188d0fc8824b7e750ad2a8f6c5d8ccaf6373fee724c8903da9fdd08bdd5e0",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "09ec7cdbfcd68561b73410de0f69a08e9ee6c702f758f7ba478ba91950b40039",
            )),
            y: Fp(U256::from_be_hex(
                "2be379813abc4f7a7947e13773b0bd2ff6bd7538ff73b851b01ac7f9a750d2bc",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "24e4844fc863793cbd392632ed96adedf2d64a1c1828b6a404b89250dc54d853",
            )),
            y: Fp(U256::from_be_hex(
                "107cd377c441e9fa3d45536a0c3fe2ab3f57bca794fd0679a6fad24fac3f29e9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1e37dd19047e8395965d65276588be5d6a97b9020a4a32bf06866dc616750df5",
            )),
            y: Fp(U256::from_be_hex(
                "2eebf8ff52f8c152a24898c64d6a238bbd5c4282b8ec85a4ab711af51bd4ae8b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "27d4348cc918e5c3280b9dbb45c34d35453418ea279a02f51ae94d83e44eaa6f",
            )),
            y: Fp(U256::from_be_hex(
                "05b6dafdd0209a54b5e478ad908a820898b684a4e3b899b505cdb7445c87a582",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2e1df6418d40149d181212a9d619c7a0b66653c52b1ab562958f55ea99c1c276",
            )),
            y: Fp(U256::from_be_hex(
                "17058228094418a16ecef7919836435b06a8019f17676c0f5604dc57589cb1d6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1b5bf0ec5de61ca650e13846baa38014c19b69ce9eb22ff11b15032b0fafeb85",
            )),
            y: Fp(U256::from_be_hex(
                "296a99a01a60c79723fc29338d8f67d67f8db5be5a20be7f1fdfda22bf3031e4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "3042bd4df486d12d3ecf2a3989117f12d0819148ae495262618bd8e6aefebd26",
            )),
            y: Fp(U256::from_be_hex(
                "092a12e66f31e7854ad6829d2fd9a6c7cbb94f444249d607912710bbc689926f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1eeb76f43fd9b11f5e64c08ff36fbf2f9b26658da79b17a9423fed8fd2e061b4",
            )),
            y: Fp(U256::from_be_hex(
                "0249cd9e2e6a9b9a2511bfd8659d11eb6fd3b3d66e6aee8c24bcda2b545d061d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1f1818c07530ddb141e045e54aa02a4a77c028f12f150b3ea44c9fde08bc10ab",
            )),
            y: Fp(U256::from_be_hex(
                "162e34544ea2a136a0df021a93b1035375595afa226ed6e39ebf2540de0c6ef2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "16a65b13286b15e3b77b768570aed8df653eb27c423bd5e40abcc5037c3d6c5d",
            )),
            y: Fp(U256::from_be_hex(
                "1337bf58c2250c174bb2eadb9bfe1626cd99b1820d14c49feb1d3a844dacb147",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0d1b9119609b8614cc419aa35245d1c5aa56588462fdf5696eae89e168f6b114",
            )),
            y: Fp(U256::from_be_hex(
                "15f44aafd4d45270b134b9f901b1ce50ef6ecda579585e40101ebe7b3258ee71",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2abbf682da895a250bfe0b20aa25ed19a4d8de3872c64d723d8bd11667b73513",
            )),
            y: Fp(U256::from_be_hex(
                "097e3d8213b21cfa8ab1422f92e2f15c8c89fb596367315c9e9a67cc56f7a9f3",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "027a61502c186cb1e6acd056f969a83b3e395b9519ef1572f68df4a60f98c3fa",
            )),
            y: Fp(U256::from_be_hex(
                "23537f84202a57a8745e1e9d5b243a56255295b4cc645e90b931c0d57db45f60",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "137b1aa1ab8182137ed797c0f575a962210c1482912f743defbbccfd4b928ea8",
            )),
            y: Fp(U256::from_be_hex(
                "2101bf7f4e121d0c43223ab6fbec98b1f17f973bcb3c17d3ca0c36358c142c09",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "226e4fc2992b3ba4379f195e136638fe543131d5d71464610b4bc96f9a29f955",
            )),
            y: Fp(U256::from_be_hex(
                "24b5de98b65248ae9bd65e8a2a3361b7e222a8c6fda5126f665fcdbc9e718218",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0bc571c73874f5c5bf7f851a5f1de0c2ec55822dc9ddb5beaf2f9ce62ce830de",
            )),
            y: Fp(U256::from_be_hex(
                "2ebb26b0559e0ef8d17dd0f2e9a9d4eaca9786f9717a11aa4f78450ec97d29ef",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1de1851c14197f3ad6e6aa859c353142b20ae1255e70d51d8db8148f98b89f36",
            )),
            y: Fp(U256::from_be_hex(
                "1bdbcc0716829455e77f3d6a88f8f74c2a96118728cea4ac6d9299803a13c022",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2fa68ca143f2012fa4b0eeeb5a58e7041c823e8ae6693035fc2d575c266ee864",
            )),
            y: Fp(U256::from_be_hex(
                "0365340dcb8bf5f98210ab7171310b12546305ff4afad90e011f58750f074804",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "07b988ca6d73a619ca0e74162556654f9148765df008865b4b03e51f39e393a8",
            )),
            y: Fp(U256::from_be_hex(
                "123f9b45d51587be61a84bc44654abfe19e034466ed0bfe86f50b82c861df624",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "139bf9baf1fc2c039ad0e1f156a71fe6c5d5b85edf5106aefb37b4bc804cc5e5",
            )),
            y: Fp(U256::from_be_hex(
                "1508f1e9f32e17190c885759dc1bc59d0ba1ea479434d5e6d20ebdfd8c33e50a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0363b215829699c570658b1d7528798cd509449bf133e27e83f00a2b990227bc",
            )),
            y: Fp(U256::from_be_hex(
                "1f3d4e449d2247321349e60977926f33dbcaa278feee6f8ce9234ee39768d1ab",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "15301faa6426796e0af814d356876c43e8b12859fa5ad2181e895f26a6786f5b",
            )),
            y: Fp(U256::from_be_hex(
                "066adca40b7d6dfc19e74bd9a2c8d63ae3cc9d55c9df6843a3cc545a2c5d75b5",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "22f9e0f83a2e636bdaf3ca7dfb46f936a71d7d5f426736893b6cf001137b1872",
            )),
            y: Fp(U256::from_be_hex(
                "0b51966bc581f3982d07af9581967ddb6cbd996051ca0c02e8f1a31259c04ffc",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "218d359333a0fe24849b98f8922327ff7bd589bb82100397592f311e89d0a609",
            )),
            y: Fp(U256::from_be_hex(
                "1d66c18ace6e64df58b40fbe23c06c4a98f5977e45cfaefbdb2a9605107908bc",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0b3ed5e720e5267216ee96df95e32556be2f2238977eda25836ad6e1503a276a",
            )),
            y: Fp(U256::from_be_hex(
                "1b06372b710040b4dd0bf3651b914ebc1466e133328f451c33516b479a593dbd",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2ccab022b25df2bcbc227a8868f46965cc741cf03939d1b14e8f42892643269b",
            )),
            y: Fp(U256::from_be_hex(
                "2a4a06566b8b3e842765dfe8e3e1a7ca13a945a426ea3a2a442555281a892860",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2ba2c4024ed15c0987237a410fec662b33be04a42380510576052666e3408dd1",
            )),
            y: Fp(U256::from_be_hex(
                "0231afee45bca2c1e3e7980a0a33175803b8121f69e25f58d196683d7c36a8e9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "206ca0c3af643b210713545d0487ffc8d79fa360fa533287cc79cdb43ebdd07e",
            )),
            y: Fp(U256::from_be_hex(
                "159ccb2a2746bb31b9f47569c7247c99185e94850ae69a150ccc3c301481bbbb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "25017b75e2db2bf79f9888d466d4c3c80174afcccb1b06b3198b3c6a19869629",
            )),
            y: Fp(U256::from_be_hex(
                "0788f7d1a1743e2699cb6bf1727246ade09cbaeb8a6b29159bab0b373fc9403f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "07f43d78b71d2a4065d0fae894e3351e90607a75f5cab0c2dae9f4abedb2eaaa",
            )),
            y: Fp(U256::from_be_hex(
                "0aa86eed1a500cb880e534807d279943b74428187aff304c0ce15cfc0acff90d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "074921e24289320f6785247e6c120161b85724b33f36091e53d41e759dd83103",
            )),
            y: Fp(U256::from_be_hex(
                "275dac13222ed50fdd6f79cddd554eea82b43780c2ce8af758116e9520de50fd",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "014ab221372b3f6ad39ed8d0848d70d6a318e29227a1cd7f4f3f5e52322e6ce2",
            )),
            y: Fp(U256::from_be_hex(
                "17c37d4c88a7d91019151ba1c83e5b3fd2884bb6c86188b27c74c4cc432bfe0c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "28f66ea59d672be82a2e75e144eb2329ea3399dff058712644efaf12b675a602",
            )),
            y: Fp(U256::from_be_hex(
                "1ba67e504d9e270c3616066be47da98b35d590cae35d870286e689c9337ed2fd",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "13685fac108047af3bb9b51a383d0359b3607f7dfe4124774d6ab239f8313b17",
            )),
            y: Fp(U256::from_be_hex(
                "2007452bac69e005a1e3b52f8e41146e71f4ddaa37f4c27c41fb4aa38fc2bef1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1a6a80465acc65b2ff13f5d7ffa843456e665777a5f6bfe85046e1aa826c5f36",
            )),
            y: Fp(U256::from_be_hex(
                "05c8af362c83f37af02f99d9d1036795d025d2225978a72d82e06a77e08e81fb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0efc43397baa1c76c34b835a59ce90356d65c2a6577d795a7d028ccfd8baf29b",
            )),
            y: Fp(U256::from_be_hex(
                "1e80fec6150d8f425ed1adbd2dc75bd5e6e48f4ba239c9a127a3c4d959b4a82f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "10c7c7fb9e5f64651294084894e91b70100d7867822e22e007321f579736fc79",
            )),
            y: Fp(U256::from_be_hex(
                "1049420b93e378bcb23e98950e44e6bf583155dfbc629960e826ca5b1aa2a4b3",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2efc62b635c0511aa36f10fbee078c3e04665f0e173fca7dfdf173b66cc7832b",
            )),
            y: Fp(U256::from_be_hex(
                "0c7eeaa66548a6b3449a6893f92ea48ccdbf561a6075fdaddd7bdda37affc97e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0a7c1d4703c98e6247c3f0a5ac0fd84d8af753a180ed83c878c230c492c663ad",
            )),
            y: Fp(U256::from_be_hex(
                "1e6ca73e901f4b39a1383c27d9a237431ff848d34fd258f4490118ee8c709e7c",
            )),
            z: Fp::ONE,
        },
    ];

    const BABYJUBADD: [ProjectivePoint<BabyJubjub>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0a09b0ecef2ab564a3b793adc9a4143535fac0c19fb44b76f2fc7e47a5b95777",
            )),
            y: Fp(U256::from_be_hex(
                "1d2d637d6fa0041a1da45b6d158354dbc211bb85112fff08985a17ae5cc1fa8a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "18f8ca31ff297755f43d2fa21c1c7dfc29b2da21d9dde996ff61ac611229fe16",
            )),
            y: Fp(U256::from_be_hex(
                "083e9fe89caa2820a63dabeaa8727263cf5241b2c165a42f7ddf81cd01bb23a6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0f1aa1a0759a8238e66036cb3500e75af2ca722e0712448ec416bc8b775fd7ea",
            )),
            y: Fp(U256::from_be_hex(
                "05d5eace4eab189945f2a1eb6b75645c90bf059934533aa71f912416ba32ecac",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0c04ec38a7e6647dc040c166db8f9bfc71f288588c4e270b6210d1772d48ac44",
            )),
            y: Fp(U256::from_be_hex(
                "0242692380e7eb0bafc5289d4d422c88e5b6b94f698175e0a94a27cb2c7903ef",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "024fda113c9de8dc2af0f336913c1442fd4cb9e080892ec9eaf1c0514c0ebc2c",
            )),
            y: Fp(U256::from_be_hex(
                "0b921795834688463721498832a5bc06bb769aebde7169c4775906c5b7dfde1b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "14efb053e40f1dcbd6b4f68be2e07eaaad2333ebe60ac5758559542daf255c38",
            )),
            y: Fp(U256::from_be_hex(
                "036a38b1535144a7b532de0ff0f08e7265cdf3c69499b94cf4d4b1e837de4acb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0c05324df905ff832311e3bf577c699155a390d4fed16a31b59eb496ee886e52",
            )),
            y: Fp(U256::from_be_hex(
                "06c41723351a217b7ea4d15767d5af9b3d85705226a9511f962cf12b3e989146",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "26d86eac52885481700b0b90f2a5a728ca388d5642356dba99cfa3b3a7c108d3",
            )),
            y: Fp(U256::from_be_hex(
                "0be33540f27dabb7294a0a8a6a36184cc8b9bef0dd2245d0301f748033c635b7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0e818513474cfcd94b69701fc23bb54c8971aca83d128b5385783f3dc4faea8e",
            )),
            y: Fp(U256::from_be_hex(
                "01bbe663263738fe017c1be2bd88e1c930e0039e487f7e4447cdf4bb7f87ac8f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "214e637964332e429bdc998a8c594e1de46c6d9074faa4b790dc54625be7b624",
            )),
            y: Fp(U256::from_be_hex(
                "19b4fe66accbd76593094099c89075aadfe198d5772a068137b8a112f0d9e29f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "111e49edd112fc05ab323295ce668e81dbd916b5af9993377bdf2462e4cb8aab",
            )),
            y: Fp(U256::from_be_hex(
                "24e81eda1f8ecfc9a2ea55b523fcedb0b017a4e5ceccb87bb1fa331259bc3f84",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "23fd53aa13b7be102b41de0fd042128ad2971da032b15d94c9c85cc9ebb2aebf",
            )),
            y: Fp(U256::from_be_hex(
                "158675b6d66525bd86f329484792d5f29f92f3d156a21bd7f821c9cb4201fa97",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "092907ed233b3397cf618bcf9ad4dd0b95b915fbb334dd8285914dbb1c37438e",
            )),
            y: Fp(U256::from_be_hex(
                "111e6c4c0a9c08f3a3c9553c143fdeafe0534d6268221d8f9347e1fbe1f0425c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "21a8579e33494ff0b52035e24f943884b60d65b4eae664ea77bf6817f6715c76",
            )),
            y: Fp(U256::from_be_hex(
                "178b2339dd8ab86b64fc0aee3d2875946cb5efbf1fc02cc08a57d57e4ea788cd",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "07954d3275e77314790dd851c89902058a226bf689bbbdd5c3ee1460868cffe4",
            )),
            y: Fp(U256::from_be_hex(
                "0416bde235d8d18d2f8d381591243cccc1e2ebdf3bea3a9c0cf3fc0c0ebff6dd",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "17adfd8678f0f3896f0d76ca19985171ef5c7ff413b8dffb2a60793200b7223c",
            )),
            y: Fp(U256::from_be_hex(
                "042c10376bcc6508faf0868b8513b1fdc79250538f0804073dd36f5cd12fce14",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1cda1f8405cff46618a8e774a01dc4e40f8cf005991a0bea890d864fc042107b",
            )),
            y: Fp(U256::from_be_hex(
                "1ee809f800556b136fd0fede12db9342808f3c06fb1fe2ebcf92c403debe3c0a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2f7c5afb4827740881e7c0f2d8a1908467f3a99da6a2e33228c1b69c4d60b29b",
            )),
            y: Fp(U256::from_be_hex(
                "0aac31255e7542b3b9c866fe10db7390d089cace66a54028028bebd84437654f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2fb85cece214523e179c45bd50de901a67b618cef1b142e9937bc7abe961f54d",
            )),
            y: Fp(U256::from_be_hex(
                "1ca1fb66ade9012d34e819e4b73238aabdc184870e586cedb00ed0c6924a2060",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1d302a3b235745c6e81b3be3c9455423887d9fb4a8d1e00e54bf54887766dba4",
            )),
            y: Fp(U256::from_be_hex(
                "15c31ec2bd7f6b2bd61c7bd5b51ab17b68ebb2d59eb10ef82b7d1286883a6f5e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1e9c109c9f5553a0306fa2e4e60d4455e1f134d8ca9683a1d96d0a066b261cfb",
            )),
            y: Fp(U256::from_be_hex(
                "03e93de152993bb0851968b46a9978d3acf8673feed3da526b74d997c260a205",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "112a5e6b0afc5f1c20839bf85e8cd64898b416330035976a9ff818cf0a7025dc",
            )),
            y: Fp(U256::from_be_hex(
                "08a72645a373a85443ee8c0c56734fa2073a350a347964651fa5609e2ce91ab5",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1216db039c02a3a1e5dfb9a63ed0a490603afc99b8d5ebc26a3dad2691fc1823",
            )),
            y: Fp(U256::from_be_hex(
                "2b3eca14b68cde4c62b849cc640cad65f9eefd36445555506517a1f6f2abb1aa",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "039818b123bf8ffc38a97e0af14741d18d4090fd88cdd4635ac946a4e6c70f13",
            )),
            y: Fp(U256::from_be_hex(
                "2dd348e9c23cc6e2a09c8782e36bd80a35f547d8cbb6efeccc24e347f1c99c9c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2180b62b06f001f7b141f0748c15ca1ef957ecab0cddada4d2148f3484136856",
            )),
            y: Fp(U256::from_be_hex(
                "1482adc53f4fa97ee9cd24340c42c2377a69df1e146eb535085da6bf3e16de1b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1d76513176b6f23c1b7ad8341cee45128da3c36ef39f61e0615d6499855e9189",
            )),
            y: Fp(U256::from_be_hex(
                "25b43eb268379bc44437c1b30deea446ce5cae5452a230352fb9c4a133ac534a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1f21c7bcee4fae34cbe9f40d9b8cc3167fbb3b52fa308097c059f1efa253302f",
            )),
            y: Fp(U256::from_be_hex(
                "0af66b2676980c4267c050c8cc57be09cda3a1143d3b2ad27bfc5a779797a9cc",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2d8c414cd1ba88a8a0fff3d844b3eb16fe02a5319c7c98cac11ad0d8056a31ab",
            )),
            y: Fp(U256::from_be_hex(
                "0427b04cc14b26b37454a1ee555fae83fc2ccb381f79da3956669b8779794fd8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "279e55f74bd09e1435c9077bd29887657bd44b777d87a317a379c914438c278e",
            )),
            y: Fp(U256::from_be_hex(
                "2227cbc0c75d49063fb60e6e15c7cebb28937695a542b0d14b06ad5d095e66d6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "24f8f905a5dfca70c30919de1141e3891807e5c84a76518f4f3d8ceeeb5b6437",
            )),
            y: Fp(U256::from_be_hex(
                "1de23aa257d42de543cf0e185cb422526d6a6266300e78c5168a193e8c8037eb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2d2e401543e4c6852d4eed78587aa233c0fdaa9cd1ef1587b51768fbd5cb4461",
            )),
            y: Fp(U256::from_be_hex(
                "0a873e6b832d7497e7f027ff7138b08d178bc6c8c0455870f8f3012ca1dddaf9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1ce68fa34c16d3a7ac14fcbaa82bab45a549bd1604916a2f21b0c7aa7e77a798",
            )),
            y: Fp(U256::from_be_hex(
                "12dc783e3009efbed1ba9152d6aafb3529ce19e13ccae383f324ca1d501a4c01",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2972150df95323322902e50ed0402ec180767afad79c337d2dfd8702bd18c51d",
            )),
            y: Fp(U256::from_be_hex(
                "063b2856aea6ac16be6f9c567401acfa6f6e0e3b932deea5760aa8acc4b1121b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2ccb4862e60ff54ee99e83d8e4cd6f2bab5e088da20582544f5deb16901cf61a",
            )),
            y: Fp(U256::from_be_hex(
                "19b2b6d4660838f2316ee6443d8dbc3732ad73cd1ff61cfe889031ade731ad51",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0210e5d7e4fd36bb8c2073c6dcdb65a12c0cbe3f31433aac3bd31b370d4949e5",
            )),
            y: Fp(U256::from_be_hex(
                "1dea795d940d46a42ee121ba7ca47a0c6c9fb97dee620515917a2726b41f9475",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05b67c07fa7f984baf25e88376ef340be66b2cb4d007e452f77501d6fddc8841",
            )),
            y: Fp(U256::from_be_hex(
                "1813aba252513e3777f4d6ec8a34424552cd6911096fb29383a7729598020f2b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03dd6a930d6453eaad043aea589bac41e8a8f9e337b6445231c19388ac600e77",
            )),
            y: Fp(U256::from_be_hex(
                "07b2d9eeaebbd4c229b70227899edae7acecd07a264147945dcc9edd18c0db99",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "18a9a7a500a5150857dca98dd7dd743d13740323f3cbb1b7518e339ba2722eae",
            )),
            y: Fp(U256::from_be_hex(
                "12f58d00f97e4474e151c71b49136939e4736c68a3036df765c92bef770cec50",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "005b80e71779988adf2c675f4e2da5b5ce70ba3319aa76ac0b9f2ce12105f86b",
            )),
            y: Fp(U256::from_be_hex(
                "1d41488e5d91e50bf27fac0b5ef2ff7ea01d2eed842d0052a10dc42f6af8dcce",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0c9e8a08783588a31e1e5559863e39418d5cf44ddb0942829ab4484fac62ba87",
            )),
            y: Fp(U256::from_be_hex(
                "03673764b451166d5c9b50d4447d71e6cbcaf7ea214a295c76b5457253c75b82",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1297488214e610934c798ab7c7cd43149f2050a1abab836e7d1521b960445715",
            )),
            y: Fp(U256::from_be_hex(
                "21d9c051a099339413bbf69a6871743b6a130fa1c761d4bab32ea702d856136c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2b30c69c01b1d2716f436fbdd596ca58389488dc89636952ef001443456d9944",
            )),
            y: Fp(U256::from_be_hex(
                "2e356c66671af69ec8ebdb86026d5fe6be370d007da8a3eae566eac0ab6696cf",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0426dc14cf43d20b4c15e0e9cdf97ebfb4bc2f9e73737333aca69449def0ec25",
            )),
            y: Fp(U256::from_be_hex(
                "0c1d101aff4fa443f318554dbf84cae2bbf9d03156ae168f665ce9c25f7c50ae",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2e0e99e494a4588430344827482b04244a38b45f95c6da6c4adcb7bcd7844ed5",
            )),
            y: Fp(U256::from_be_hex(
                "0666bb7a95e2280bcdf44cbacb138d9594e11e38301b9e571f8d9cc55b1587d5",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1d2c8e596f87dd29d0611eae9cae818540d6bce0741bdfc32f1f76fb9da35104",
            )),
            y: Fp(U256::from_be_hex(
                "19ea0ba32a6ae660ad1d590868ac1d3ce8ace77b770fec2f4384e606055c3252",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1ac43f8d67d7b0f8acfac32cd8e3a31952f68be89154d695c1372b516520c07e",
            )),
            y: Fp(U256::from_be_hex(
                "0f0324193fb6fd593ec57eefc2a27a882e4f25af65d51cec942b5d680a3c67c1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "20e79a9daca3ce2e1ed0ba94630e6fa50e9b87b673380bb7de178724de00d98f",
            )),
            y: Fp(U256::from_be_hex(
                "2fa4413be2e2025cd1e1b5b421fb293127f9688404a0ba839f4ec7698782cb07",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0431bd7242234cd75e8901172c9ef3cec6f4a713935ecd05fca11beb6f4e2f8e",
            )),
            y: Fp(U256::from_be_hex(
                "08be9fcd7cbdef284420e45b74d3fac00b282d92e579d04a364aa5c4b4e69987",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02546d2da9a7e7a27a0ae6784198825b3e29725e37727f3144b20749058a268f",
            )),
            y: Fp(U256::from_be_hex(
                "047581a5cd7f582aa38da64f52f0040fbbf186519b7bd512b18dcd41d38b5c3b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1512c18d840cf13756c38e3526735013906042b871a45ceb33c6ff036315fe24",
            )),
            y: Fp(U256::from_be_hex(
                "03ac26f8913cdf59a52f9a60ac77d5d1d6377a81f4a03db7be8783f69d481dcd",
            )),
            z: Fp::ONE,
        },
    ];
    pub const BABYJUBSCAL: [Scalar; 50] = [
        Scalar(U256::from_be_hex(
            "0e6ea0d1c583a9ab861ead1bb6d1f1868f50929d8a42460b048ce189532cd2b0",
        )),
        Scalar(U256::from_be_hex(
            "07a4bca3785d1cfae1c88526d431a440e036f3ba5484aae4bdc711e44097a2f6",
        )),
        Scalar(U256::from_be_hex(
            "0d1c03219f27aa774819b21935384adb1f8496f5defb89691f66bad018983932",
        )),
        Scalar(U256::from_be_hex(
            "008830f9a2e6c014ede5a04131489971eeba3494d02f71182240a4e6cc469003",
        )),
        Scalar(U256::from_be_hex(
            "0d2d18e91c9125ee226919ce4773e11425d785d2ae1cef93efd7cd237a6331ad",
        )),
        Scalar(U256::from_be_hex(
            "0e69f04fdf502826369cb231cd45cea666d614c116e2fd8a81921d1623891aa4",
        )),
        Scalar(U256::from_be_hex(
            "0d0286840e3e881f1e0d8599e059467a65a6f39e1a37db3e6dcf946eed35baa0",
        )),
        Scalar(U256::from_be_hex(
            "0a41d87ec5f60f3793128b668cc8def892e0b8906b99a30cebe4388c191540f9",
        )),
        Scalar(U256::from_be_hex(
            "0793a247c8b05d0aee00d9fb7a4b0a9c86f1b586afdaa941e40b906242e298ed",
        )),
        Scalar(U256::from_be_hex(
            "0c6f45eb0e3933a588bb59c3bc869f6669fa28faab0c372ec6f04ab22dc7c71d",
        )),
        Scalar(U256::from_be_hex(
            "0a5680816a0fd66fe2697ebf929d09c42c164f25c88ea9f5b8357d1260c52c6e",
        )),
        Scalar(U256::from_be_hex(
            "0b980b230aebd54de184965942c019f432cecaa7f307a16c2271a13f945e1b7e",
        )),
        Scalar(U256::from_be_hex(
            "08dc79fa43c484b9af0eaec0e3018684837a3f5ab2cea87e52ebd59bc088bc7d",
        )),
        Scalar(U256::from_be_hex(
            "06d50c294a8231bad66c5d9233cf6bbd4d7aa0f19d43d7c5ebc6b2648c5610bd",
        )),
        Scalar(U256::from_be_hex(
            "07b1a61194f53c349aa8d2261a680d7d5b0891ddcaf5123ff8db303e0f26b6fa",
        )),
        Scalar(U256::from_be_hex(
            "06f175bb4213d8ae73335c469abeeb5c2d844618c31f7c019485b92fb0fb5a63",
        )),
        Scalar(U256::from_be_hex(
            "008e5fef9bdbfc9a7f6c443b0f8125d0cefddac327d366d317f924256d546ccc",
        )),
        Scalar(U256::from_be_hex(
            "084aa35c4a853ab2ee37652ec31e47b3783ff33e3da56927493fdd2b14fbe688",
        )),
        Scalar(U256::from_be_hex(
            "023176c53cc30c40e819c2e253ff4ab668cf63e4e8a98f05b366e598b58c4e81",
        )),
        Scalar(U256::from_be_hex(
            "0c7e990498d95c2cfdc96c5c0b423b471a0f414633ebfc9cbe2a7b9d34ff4016",
        )),
        Scalar(U256::from_be_hex(
            "031d69558ccdd9031e12df406b197fbb9f0914f93c1b3e2042c101fc4a2cf7ae",
        )),
        Scalar(U256::from_be_hex(
            "0502841d4fd78057f650d0ee520ccea6f30d9374cf813f283cd64ee1e4171e9a",
        )),
        Scalar(U256::from_be_hex(
            "0401af92bb95888463c028e0a0f17770125adf2e3de541b2fafc2d0c98d6291b",
        )),
        Scalar(U256::from_be_hex(
            "0ac1c1a93fca84643048f90e8859ba913cc9341c82c5a487cb96a4e23471160a",
        )),
        Scalar(U256::from_be_hex(
            "0d6807de5802bc8569666a16180ec2219e018733cc97951b8c617d928f953d39",
        )),
        Scalar(U256::from_be_hex(
            "0e3d77a0cd633ae000cb15b76716b0ffc9e4d1778a187041cf8c945ced7b0fad",
        )),
        Scalar(U256::from_be_hex(
            "06a3c0b769138ce95de9af44d9b812e67b1c973760f4f7c333b0c491c07ab6d4",
        )),
        Scalar(U256::from_be_hex(
            "02a4db57ca2ca3f20b1e0be786b77b36646244fdee8ccbf755d2a5edd6bd9057",
        )),
        Scalar(U256::from_be_hex(
            "0cfbc2ce56257ce263add4981c4199451f2eb890237b7ae7887062b5a2480966",
        )),
        Scalar(U256::from_be_hex(
            "0c88bad486d1d669a6cc4de7e8385593e3a84624081e1e63d55d78771418da11",
        )),
        Scalar(U256::from_be_hex(
            "0be26dd0e65102292d7789c4a52032303afc2e745b34a42962a3530e2635b6f6",
        )),
        Scalar(U256::from_be_hex(
            "02ffadd21df29efb005b1dd2cc8eaedc4bbd27e50d904720b92320ce43b95600",
        )),
        Scalar(U256::from_be_hex(
            "00e9bff05c47766bf4e7552a5a684b249b77b6cbd99086859bce9bb31c9a8597",
        )),
        Scalar(U256::from_be_hex(
            "009d9f11435670bb37de2dda6fcceb0a4f96bf53fa7cce0042a4582c46357c4f",
        )),
        Scalar(U256::from_be_hex(
            "04917d017dac5c637041dd475c9d5e5f626e108639e8557415c9e8dae46f5622",
        )),
        Scalar(U256::from_be_hex(
            "0da52ca01711f5a93133d96a06d6bc8adc20ec2d29c93e760e1971ec78d4fb7c",
        )),
        Scalar(U256::from_be_hex(
            "0c170f0793e9d2da4fbc36a3e1fd73445e287b9676b0bc17bd5a92339e469ab9",
        )),
        Scalar(U256::from_be_hex(
            "0395329a293092c27d0572fbdf5c162d930d3e59b4dbde84520f20cbdc938184",
        )),
        Scalar(U256::from_be_hex(
            "01a3619d1a87272fb1427e7ef29af755ec81d2cd7e3ed91a0a64f7b71e3d385b",
        )),
        Scalar(U256::from_be_hex(
            "02cb91577d3eb77078e6ef0ec336bcfeb826b98b09ddf678f64c022897d20e71",
        )),
        Scalar(U256::from_be_hex(
            "0ae35f7df9e47793ffeedb4ef307741d896f9feb634da704ab976ebe81617fd3",
        )),
        Scalar(U256::from_be_hex(
            "0967270e071b2f73a049d75dbf7b2631f5037eaed979793276eb7821f4737323",
        )),
        Scalar(U256::from_be_hex(
            "03d8829e5c37dd324c7ab4ad35485b9a1f7ddcd881b37a30c4d97e53fba8805c",
        )),
        Scalar(U256::from_be_hex(
            "000da03697e981af01fdfe9e329e1e4d9bc5e756c7d7e6208ec36d66b4886483",
        )),
        Scalar(U256::from_be_hex(
            "04e4f06b4c7920f8af44f0699d750edc78febdf2d5e4a02f1eb5dd4c483f505b",
        )),
        Scalar(U256::from_be_hex(
            "087cd32b9d32a6887225e2fd31b9818fbf596468556f088171b9d67585aa220e",
        )),
        Scalar(U256::from_be_hex(
            "0c7dac67f25d585ca61cbb4ba7ff3147f559fedfd3ce79498e67296a0c3a05e5",
        )),
        Scalar(U256::from_be_hex(
            "0a592d70653db58dff7f223a1c9614309bd73490e9d74218e4633889b70b3fed",
        )),
        Scalar(U256::from_be_hex(
            "03a4dfcf3bdadc5f6d905763f48c587a5179e40d6a57c037f0ea4c606460d6e3",
        )),
        Scalar(U256::from_be_hex(
            "07f6d206e044b35f4a52cc372c52a81f462441bc2687c1aed28b2a3de9fb6285",
        )),
    ];
    pub const BABYJUBMUL: [ProjectivePoint<BabyJubjub>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2e71380e4333b97738d96efcb73cc3e2a83efc6052a2eef2cdb8a9cf0126a106",
            )),
            y: Fp(U256::from_be_hex(
                "2cce4ca29fd59d7727b9c876488b00eb991a5f54627532d526de220784d9a2d9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "13f64935aadb67d4675aa8aef4bf9dc8414270e38dad53193f354f8aa5157dc6",
            )),
            y: Fp(U256::from_be_hex(
                "25b3fc46060294a711dd75ef8764d592742686ebabdb35efd6f07ad9a78c78f1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "117b8543297f3463b83259d0e2d99ad8f07e281e475f229002126794c6bd719b",
            )),
            y: Fp(U256::from_be_hex(
                "21f5470d28d92389ba7609e998ec46f4d36b4c7a4be05474db53a60a86183d31",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1ec0ee15f5260a17308260eb58d8b235cbf8670bc5225a7f214b16092ac3adf3",
            )),
            y: Fp(U256::from_be_hex(
                "1f38099ae6c5ca359c3e6426e2a737e84515e80e0e8bb65e49131071de3c66f8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2ef740be82e22f7a7ec079e4dca1052bf890bdca9a661d59f0a43bbd13a10486",
            )),
            y: Fp(U256::from_be_hex(
                "0084528bbca95a38ea9ae65289d3b305eec9b1c7075a5f79a3eda4405976d1e3",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1da49e5c34b929ddb9e7274b35fd05b7d6f142b9f3c2b0a058a6bd6c3c2ef138",
            )),
            y: Fp(U256::from_be_hex(
                "0d62920d7b31efb21bd651aa6da5a8f3af64f8e5f9b7a6542bdd780d3d11744a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1dbeda643301d8c586a1d17d173a2fb46dbf70d1c8701ccb567c00ffcb79d736",
            )),
            y: Fp(U256::from_be_hex(
                "2fbb465e00c281a275fc6b5013bd02f3f58fc3d0275a06cd0a1665880fbb6ac2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2e5433ad28913207d8689759bd4003c403b01bb4670f27179e36b410d10b16b5",
            )),
            y: Fp(U256::from_be_hex(
                "084d22250119f2bd56b187a020baf306025c02805f1ba7b81ff7aa10303ee309",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "21dbf1f3a9e222bb3fd2d269b119c5cec638b754ed4c97cdfe68a8df83f44b4b",
            )),
            y: Fp(U256::from_be_hex(
                "1c146fa7d315aeb87bb7940732a4aa425e7a58a0ece04b3f731b99f2689f6df8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "076804ef8d2a9220cc07a849069a8f51bc75253791fad48b8b5afde572aef069",
            )),
            y: Fp(U256::from_be_hex(
                "200fae27b86451edf421ab58dd894ac6faec88f5352ca1637a9c4737147ffaa1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0680ba99ac4d8b49bd9de9cce5d41c6bd9cbdcb834ff7bb93584b23110b6994d",
            )),
            y: Fp(U256::from_be_hex(
                "24b6ec094e10c91b303d108a86b6b5f099e42a45d2aa02c55be018c4e6ef4072",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1c885c211fe9e76d967880dc082609d00b0e89cf2de2185f86b199e5e3036850",
            )),
            y: Fp(U256::from_be_hex(
                "161885d277e305fd1a3ffc9e2c71752fb1994a3f8c0f27016998d40a535b2ec7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2e4eae9bee77a604e255376a7d13f41c6ebad76f91fa8890c12c138dab398af9",
            )),
            y: Fp(U256::from_be_hex(
                "19ab34e94076c55e8d9d9078f1c1f1bdfdf31539040962ae11554a295c1bb82b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2789b723d30d2cf724866293ebbcc0f62b48a34634c8f3f9ac67fd3402af5cb2",
            )),
            y: Fp(U256::from_be_hex(
                "25aa4f2c77c4e1db5c6b22c8c141f13b29f52efac1fb69cc4a6481e9d06f60b2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "143b02c5817958d1fd906806683fa04cb9210bb0e1ed7d9dbcbfde87c3b62699",
            )),
            y: Fp(U256::from_be_hex(
                "0ce9893f56c1ed887a9cfba6c44ba054b7c79b0daedf43baca4c882f45f454eb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2691a59e57dbcdc517fab9a920887d92f611a6df046d0db3c907e7fb473d9519",
            )),
            y: Fp(U256::from_be_hex(
                "0b596c34b6806a8f6a5f791530d3ce89793fa518d5e93d0c7a14550c97eb660a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "11f1d7b735d54f2a9bb2d2629776766496e9eaa9aafe6efe2fb139ff2124f563",
            )),
            y: Fp(U256::from_be_hex(
                "24008ab35c395190c7aca18a0986b6195c7764f56410ebeacd5ad16e19881b08",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "103fc8009ea1d42a6000798346d06569e004adad0ef2329618c9e4afba315240",
            )),
            y: Fp(U256::from_be_hex(
                "09dadfad9a01a313170d58166d3c23ced6260a17d19086e208084a20930cc07c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2dda6e76e1c7bc0dc83863fde768b1a2a98363f3b471fc5c2fcc7700b06d94f6",
            )),
            y: Fp(U256::from_be_hex(
                "05ec9ab3879c2052d54d2801e41180923592d30055c88fdbe609fbe62dbd1441",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1a3b2526cf34812b2cbc45d2ea50e62e6cfe8abd534cdf79246377678a253fae",
            )),
            y: Fp(U256::from_be_hex(
                "1910ff971f66f3cea187e304c25ec17a67da43a2aee9e37e17a88a6591b1b3d6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "18af7fe07273b21557fd8339547a7c6a24adacf0deaef56b01bc5780970cf81b",
            )),
            y: Fp(U256::from_be_hex(
                "0bf32145f6d980fecf8b008302b5da4aaa768a97723cd0195e9331ba2d8a96b8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "09baa2a1ac1b687b9fb7d8eb844eb5823a3342a49f9283cb6dd5c56b8c9b1557",
            )),
            y: Fp(U256::from_be_hex(
                "1460fa6c36447588777d8a6942386947830ae87c2b0144cc6bf2d3b085428f12",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "09b2bef4818e0d99735b437c4a98a1eba54ad7b43334eb6c1c90dbe50cab3531",
            )),
            y: Fp(U256::from_be_hex(
                "1a7d454a32663d71024bf53fb7e486b2a95d2dc137f6bce77a4bbba5a45b3de4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2c74a33439c2b002604597a0ca6ef76ec795c171789d84636ce50dd8b538e6de",
            )),
            y: Fp(U256::from_be_hex(
                "23a2f8a553f450444d9ed956fa3f768f5a6efe9614ab07bc5e0dd50042f6428f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0925845f3e279246c8b55117e753b4cf3963166d8e4e991a0d883a8a4921e3c2",
            )),
            y: Fp(U256::from_be_hex(
                "093d34d3c345fbf1248b00f41f2fbace292c632492b72a4d3f1a898b216b4711",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "25d552307eb2d91ca28010bda6e18d0226e3255a5013817f14f3e5c994f89382",
            )),
            y: Fp(U256::from_be_hex(
                "00bb3dafee12a595e49f114779ecbbd04f5a3e917e48899cfd9e5087ee71b74f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2e2b6ecbf794d04f8bf9f122f976f449d15ad8d1262f4776d9ca47deb33a9706",
            )),
            y: Fp(U256::from_be_hex(
                "0e08ab5b8a2733bd6a9f84db228bd43636ea362f7b229e0e468e9378ccb71559",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1848cc1361402376f52d121e99ac99797dab7f07f7fde4db64b054be51b68c5b",
            )),
            y: Fp(U256::from_be_hex(
                "14076d28485528335f40d7b1b7e003c9cda2389064af6ee26d233dcc032d12a1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2989214e857d8774b2401acb8bbbae844f61f3c81a87db68cf59f6aa80f353a7",
            )),
            y: Fp(U256::from_be_hex(
                "1489577a976cc9ba7a2edcc4055e76183bd5bca565e654e5b072b41bafdd1c47",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "025fc5b337b2c702c6d79ac5cd4522e3372b357e7d2c84ab3873f4cca14e79d4",
            )),
            y: Fp(U256::from_be_hex(
                "0de44bfe21ba6830d2d714a744c033119bb594be8f38995ec1ec21498125b919",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2d5f9af587b32f16cbc8870a79f8618d2537d15633d6ef594db3f9a0b2526ceb",
            )),
            y: Fp(U256::from_be_hex(
                "18d1b16d463547f47a81d88c9738e102ceb4852f188605faa5c86faa6c63b426",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1cbfec71c2d92712fcc776f53498b46943a16b3c1453ef4e2d55eb59ee21e1ce",
            )),
            y: Fp(U256::from_be_hex(
                "2e56032a24630f21abb05b6971973c768b5b22952ddd8b03fbbae1160268f65c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1a2b615cb382c9d2959cdcc470a0e030dd5b3f5295aa28f5bb649c3d5df8e2c5",
            )),
            y: Fp(U256::from_be_hex(
                "18bea9734ac9fa4f2563465bddfabaeec2441ea847d6cd109ac5e4853f98a635",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0e795bb9d8bb68ea7b7dff5504dd920e0e7c833235a075cc96a9ac73a169eeac",
            )),
            y: Fp(U256::from_be_hex(
                "256b1baa9e20b265d6f405097b2ad441fd8cc8ac0427e0e67402119321110ad5",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0029b77c22dbb3346d4f0505d0e53995b88c5bd15b6dade15221c16f5b613217",
            )),
            y: Fp(U256::from_be_hex(
                "0475dd30631d178848147853c51e0d6ccb51ec2b92a2af778efe23b5f16aec7d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "10e35477e24eb7b9e7519eee9dda1340d41cbfdd110ef733791f89ed83a0d8fe",
            )),
            y: Fp(U256::from_be_hex(
                "07d0a85e254e2e4d02749066cd2ccd2868be3184365cbd313488348ed77d0fd4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0c6ee76e0729eccd3553b3bfa078ce5677ce9bfe280d3658a8a1f4edbde9c5cc",
            )),
            y: Fp(U256::from_be_hex(
                "22b7b1fa3ca885dd70c8735d2327f717af5c0854f1be96288874400e83e8e3a4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2611e441d0dd28e19218a5baa905e00675d4c5398533771400a773871d490ada",
            )),
            y: Fp(U256::from_be_hex(
                "077ca63db24f2f86c25f07d2ba9951bf16ce82fd6ae87d9a350d790dc04fab6d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "09635030d4a54813e7d4744d84150e5a65ba29ad648ebf6e9370e6e8d9ba9a1f",
            )),
            y: Fp(U256::from_be_hex(
                "189d21642156c82f0639abb1b69eadfc9e593d4fc5f32bfa8dc0b7e93ba6e475",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "11802f5bbee74e640885325dd0d6fa240e4a80fc69c03231eb1966e1b6fc0936",
            )),
            y: Fp(U256::from_be_hex(
                "1d3b7832d80467e4b1813857773a3946b18840e8c73f5a82c925517806b954af",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "25b142ff422aa5ed21fbd2610d83ef38748478d4557ae2d60f1d725f205bb018",
            )),
            y: Fp(U256::from_be_hex(
                "11cf0df20786db2fb1239687c805750adbad2d045479fff8591318f2a501bbc8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "22a838e1b0c6438211a4682c7ea78456cc38d063b1dbae20da5d741bb341a315",
            )),
            y: Fp(U256::from_be_hex(
                "27badfff0e98a323905b962660030905d98db6841205810d576bc68e80fdc7a5",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2a89aaaf20651aeb5bc9e38ec7a2460f41be6b9d0f0d1f432f3212af696bed65",
            )),
            y: Fp(U256::from_be_hex(
                "21b875fc116717ab84019327c02f6bba072f6bce13c707782af386f603e0aecd",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1810345e2c766111f6902ecad30b8fc91a1f980cdc21d55c58d41f9f9a65ea93",
            )),
            y: Fp(U256::from_be_hex(
                "2ec761c7231e8620f2150585746c70cf6ceb04dd943a4ea76f1ff806c57e0672",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "15c4171535f59bd5e2babe42c59f175aca4603d5de04ded34881a30c900676bb",
            )),
            y: Fp(U256::from_be_hex(
                "1fb22e09e6e3832de51cea72223e9525ce3990fd6f61707309ee89a97e01c163",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "28787ae6530165a4e77f842fa8556c72bb0bd7bf80e3484445cdcc066d84c5e0",
            )),
            y: Fp(U256::from_be_hex(
                "02e79d3b9f4d4a4f86801ca58e0e5a4cfcfb13bf393aa11f182fcd1c3ac70b81",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2a31a3852408d95224a8db8278fdd646e9fee5f5649d93df448e3a6bb3ace080",
            )),
            y: Fp(U256::from_be_hex(
                "1c370d8defbb48b4a11f832de4f843208da2ea7a4c28574d61f03b7a1f5ccbd1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1a1c028ce3cbad3908f01413d82e0e9e0b16d4f9d9c3ba85105c268b65ac743f",
            )),
            y: Fp(U256::from_be_hex(
                "0c185e3a701587275cf5c5922671d26bcfeb0f73400d186eb55417238bc31e7b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2a6ec9da70bcea20764cf2a63718dcc5ff88d9736b0feeb71080f066d2001fcd",
            )),
            y: Fp(U256::from_be_hex(
                "25abff401213d31b8a343e6e1cb1f6e256ceb0fb6961dc78147a291e3fd45154",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2a8a5b660323ee4c1610860f15d1be5a14198bf9434127b067aeb9d392040078",
            )),
            y: Fp(U256::from_be_hex(
                "216f1ed3987da6f77d783e788b4c2a438755b2396172d5a033e8fdaa1a28e51e",
            )),
            z: Fp::ONE,
        },
    ];

    #[test]
     fn checkoncurve() {
         for i in 0..50 {
             let a = BABYJUBARR1[i];
             let b = BABYJUBARR2[i];
             let c = BABYJUBADD[i];
             let d = (BABYJUBARR1[i]).mul(BABYJUBSCAL[i]);
             assert_eq!(true, is_on_curve(a));
             assert_eq!(true, is_on_curve(b));
             assert_eq!(true, is_on_curve(c));
             assert_eq!(true, affinepoint::is_on_curve(d.to_affine()));
             assert_eq!(true, is_on_curve(ProjectivePoint::<BabyJubjub>::GENERATOR));
         }
     }
    #[test]
    fn babyjubadd() {
        for i in 0..50 {
            let a = BABYJUBARR1[i];
            let b = BABYJUBARR2[i];
            let id: ProjectivePoint<BabyJubjub> = ProjectivePoint::IDENTITY;
            assert_eq!((a + b).to_affine(), BABYJUBADD[i].to_affine());
            assert_eq!((a + id).to_affine(), a.to_affine())
        }
    }
    #[test]
    fn babyjubadddoub() {
        for i in 0..50 {
            let a = (BABYJUBARR1[i]).mul(BABYJUBSCAL[i]);
            assert_eq!((a + a).to_affine(), a.double().to_affine());
        }
        let a1: ProjectivePoint<BabyJubjub> = ProjectivePoint::GENERATOR;
        assert_eq!((a1 + a1).to_affine(), a1.double().to_affine());
    }

    #[test]
    fn add_mix() {
        for i in 0..50 {
            let a = BABYJUBARR1[i];
            let b = BABYJUBARR2[i];
            let b1 = BABYJUBARR2[i].to_affine();
            assert_eq!(a.add_mixed(&b1).to_affine(), (a + b).to_affine());
        }
    }
    #[test]
    fn checkmul() {
        for i in 0..50 {
            let a = BABYJUBARR1[i];
            let b = BABYJUBSCAL[i];
            let c = a.mul(b);
            assert_eq!(c.to_affine(), BABYJUBMUL[i].to_affine());
        }
    }
    #[test]
    fn babyjubconversion() {
        let a = BABYJUBARR1[0].mul(BABYJUBSCAL[0]);
        let b = a.to_affine();
        let c = b.to_projective();
        let d = c.to_affine();
        assert_eq!(b, d);
    }
    #[test]
    fn checkmul_1() {
        let a = Scalar::from_words(&BABYJUB_SCALAR_MODULUS.to_words().to_vec());
        let b = ProjectivePoint::<BabyJubjub>::GENERATOR;
        let c = b.mul(a);
        let d = ProjectivePoint::<BabyJubjub>::IDENTITY;
        assert!(d.is_identity());
        assert!(c.is_identity());
    }
}
