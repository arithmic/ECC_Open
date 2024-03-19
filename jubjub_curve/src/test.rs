use crypto_bigint::U256;
use curve_traits::ProjectivePoint;
use bls381::scalar::Scalar as Fp;
use bls381::jubjub_scalar::JubScalar as Scalar;
use crate::jubjub::Jubjub;
    

pub const JUBARR1: [ProjectivePoint<Jubjub>; 50] = [ProjectivePoint{x: Fp(U256::from_be_hex("0e249db56d769a1d194466894ef3485e80181e6a0d68881f735c93fea7ef05df")), y: Fp(U256::from_be_hex("0e529c10e6c5ee960c394e09d108a8952e43600da83edd19fc136a95da58bf9c")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("1515dd67afa3105391302409f033c8ce9d709627f8dd3384eccc0e2d30807ed6")), y: Fp(U256::from_be_hex("529238137c1b84ae98ca391290b4cb14816963a3e2cd275859c11b7d1226dec1")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("5da0c3d94321d656b230a4035bac0441c08ee8a221b7c543df0ae63ecac5ffee")), y: Fp(U256::from_be_hex("08fd5881948010b63a6085eaebb84896e1d27cebb9a32642f0be7d2fbc72b607")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("50ce79c000b0c1790c2d8c2e4f899194407e1fd303d16657ed041b2d5423cc32")), y: Fp(U256::from_be_hex("206dac13ddf06ecfb19c4beb19dac272e45d80fd0b16e0c84d7f99b284ac1f89")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("34ea3b6bd43a7f2bd526577de905ad38230be92f96488cef1b180c839b1454a6")), y: Fp(U256::from_be_hex("19618935975de3dc3cb172ff6b4e4ac32e40c8ec91d45bb9168d52c9576bf3a0")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("23b72e94fe6ccf606a5a0ab21f9686dd0422f843ca86ccb2a77ce2fd1387d414")), y: Fp(U256::from_be_hex("596383b39b6e5c5568be66ec5bb365222fcd42bc340b5acab22c716a4ec8f230")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("12aaf7e997abe8cbba10d5497e51cc2141a4787de6aea8d16035d585b21a6198")), y: Fp(U256::from_be_hex("5b2f554db8277b573632edaf4a8105d32e8ec154c0ae2b12ebc2e6a50f645f69")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("36ce837695b9a573bc949178f045cadfe7889751cb293bb96974f976e742e241")), y: Fp(U256::from_be_hex("4b99a1467729ae6c755d3c5e3b8b2b3952dcbff89fdd707d3f39260cb243fa43")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("61fd8cc2232ab038ae95b3e6b968cbc9615cb1b36b1037609da5bedcbf64d1aa")), y: Fp(U256::from_be_hex("4c49c6e9e1a3dbce250e1bae68fa5710604220b49a924e40ccf89160c980f75f")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("373f4e157a5dc29418135cf279c911e4f06eef4481aa73f7b6baa87a469436e2")), y: Fp(U256::from_be_hex("005d2a571e03fef1d8af1196942b62376aca7b832ed052041640eb6229a3dbcf")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("44eee8d37e1f32888213b7537742762abb8d0d526af1120ffc85854e18f3dd8c")), y: Fp(U256::from_be_hex("5fd529145e42e22c11cfc3571202e2f930130aeab3c1462c158c54fe08528ccd")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("73e006569be29eee5b1ae08d39eab1b26516f764cbb748124a20d443930777ff")), y: Fp(U256::from_be_hex("467c62f2fda49d4cd66ea011b896e3cf6e941b3c573bf9f42a78ca3decfb6bdc")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("05320cf31499d3365889577486f31599441bc6336a1714102cd55829a1fc5ba1")), y: Fp(U256::from_be_hex("724ff0fffb6159efe434c5162916d34c9f55ef74975022e91138271a72580cab")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("71d1df061d53942cc244f24520c306558ec3a58695fd07ff320113fcc5aab76a")), y: Fp(U256::from_be_hex("7361f6584a5d1d71d9c7bc34d572614b1047a1e893db1eb05a0c04a9532c5135")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("5848eaf47314a6dc7813c91d000d7b2b4529e66c93e9c98e9d23f7428aaf835b")), y: Fp(U256::from_be_hex("2b5275f4f6a1e3838c9d7f3b839c13e21511306e17adf2b16e4a7f059a1a1b48")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("44572346c27eae7b2be8d6bdfc614e226afe3ca99263513d18cca0bade4b3b53")), y: Fp(U256::from_be_hex("45b8dccaf7d9207f56475c701d5fd70ba19609460bb349f57f9b1570951b335e")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("3d10da85b85debfaf72c66ab4891667b2806ba379e177897e5c99e6c6a464ad2")), y: Fp(U256::from_be_hex("46505e74f1d1ada50f6a43f0af72721936433bad5b54ae5c93671111991926d3")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("3a643f949b812ba4c81c77d91dcb0bf7341c16fa9ed780998ddf5faa66c9e151")), y: Fp(U256::from_be_hex("1452d2c4851bbee665f03361be47999e51ad578c432feed205bb37ea4e628a47")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("6b8dccf837abb02e715cc74b046cbbc3428049b3971653ad6b199b69c7e497c1")), y: Fp(U256::from_be_hex("1742d9075b70e76b339299e8e87d69dfef813eec23fecf510508f96c7fd5848c")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("3ca66f9f2c8afed4a05ee31004562bb13d07634c1fde1751b3d0186a4b1ec136")), y: Fp(U256::from_be_hex("022b074333751da26307fa0b31a4507a3f8e0fcf02598fba418ea1612d880fc5")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("383e9620d463e32666e37825a323b8ecf7a767a7ac166aafcdce1989705507c0")), y: Fp(U256::from_be_hex("49ae5eaa95d70e91f51c41f32c9cf97516d4517e4933a96577b082e0e3853e7a")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("48980fdc6d518a10ad910066ac63b34cb1d4498805d5d999b6bddecc17bddfa9")), y: Fp(U256::from_be_hex("37c1830fb6ba98725851183f931f05776ab666878039201db0b47601522e35f3")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("0107336d40c095d9a838c24543697eb5004081c737300e7eac6fd38d5fea21ab")), y: Fp(U256::from_be_hex("3b778f5e18282f2828bef83938c265c21adf494d746e0e6d9cb2add489b624d9")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("27d327b88fa529df46d0cc00e6d189ccb3599f3d56765907b546c408bba49da1")), y: Fp(U256::from_be_hex("19d98c4fdda6d8dbfa68531180252e071f5e25396750f0ea06e04e1dd44a0c27")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("1912ef2e90069fd5e2393943f2720b962f7c391218fc726c10da95772e14c102")), y: Fp(U256::from_be_hex("6740fd73c4890fd328a5bee7aa4edce101bb75000c5d7fec49b4cc7dd71d9538")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("094cd8038f54ebdb2d07a823d441c722d15429954b639abbe94b1e3c11d78172")), y: Fp(U256::from_be_hex("5242d76bf3529c8fec043461354753f142a5a1712acc048d226fd3e843f334bb")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("0fa4e62d3699c174a3cb9307ba307d041b49323c9bb43bec5c544fb624675ee2")), y: Fp(U256::from_be_hex("4113fcc521c4d2d0766297f9f06d2a47c0375f88226e1608fae1da0eea09b471")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("4eacedec534438b10de377dac94da3994b11ea83f4762ed8ddd5a431bcdcec9c")), y: Fp(U256::from_be_hex("52ea53fedf985fc4d98c890187014f7eedecab15dd78b9c6949b89c9940f796c")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("5ddd18018b62c7855da83c6cb10289ea7e994354e8c14e816e83f741abc0be07")), y: Fp(U256::from_be_hex("13b864ee866a1d841e4f85c3ac8ece8de9253ea51b9fb1a371ce0d32163929b8")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("24aaf9ae2cdc0467c8961ada7f773137ec96a7a80aebde8a022df43a89fec893")), y: Fp(U256::from_be_hex("26f1a192c57a3ad2eadbbe4c773478fb5f7672982c1da5fbe07f6210d628cd65")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("4ea7baa6a728c8019290b8b043d27531346e627cb862afe9b38b0bc85d3693df")), y: Fp(U256::from_be_hex("6dc4476750098b22d69ea8eb7e120ac0bd091851852a44de317e727655249ba9")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("706eb475eb18ec62ed9d29edc2dd70f5e559f1f134b7dbbae826caa1ef5eb754")), y: Fp(U256::from_be_hex("0cbb8ba6beb06046d3392e72f03ed04a2fb20b4879c80539851ca6d1827b5634")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("1befcb9d88b2b3ceef153c6c8d4a315595effd43bbbadbc00821f489004fe9c5")), y: Fp(U256::from_be_hex("01b80575d865863c4939cbbd557798370eeed657eded6974ff36514a70ba8956")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("37030e7d8902505ac8f088dcc7e8686000a4c56edc17ea0d968b756644bb21d5")), y: Fp(U256::from_be_hex("44a80544c1dffeaf2e2341d7b0a9ab6301e15168fa9c08650017f0b21b5c98fd")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("30c8bdf629aa9ea5ec8a2f6ab3bb3ceef1bf4bd8f77b39ebd9f765cf4fa9339b")), y: Fp(U256::from_be_hex("66169d20234cbe9303caee900bdd02b3ebc8b28267078ff069d3ce5144c24230")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("15ae25e6f2a31d882fbef705534ab5ab10ac5a794a714e25c5f1cd3a711f9914")), y: Fp(U256::from_be_hex("5faa66345f96f62c1c048bd9279052307061ffcf014459c9060cce80fc20c994")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("289fc9715e398d6eaa872e4ed1862f81d8c6ea43204be3ee8200d10f33a46293")), y: Fp(U256::from_be_hex("012625da436452a6f424c114a624f85e06ca6199146fb3da994d7881a2e814a9")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("1a77bf97f33ff95c6cc2be46a471874110edcf9e4f789ca98a5cf568d88fa5b9")), y: Fp(U256::from_be_hex("5cd3bdafa80517fb47cd58663c3b97dedf6d06c96fd571539398b3bbb74ea389")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("67dc86a58be55e96b11e5ed98441cf1aebd6d9c3e7b50361afda82863fbfa81e")), y: Fp(U256::from_be_hex("4baf060d5316fe128782544dce90105dbdb2ea72f56f9c5399a1f88448e2c677")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("36200822f0bfaa3dd2577d5c0ecff409024a876d1f6f0ec2661c2dfc34b41c49")), y: Fp(U256::from_be_hex("0b1944455bd76cf0e15d85a2ef3ee91977b56e085939905c6bbdee9ebf633541")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("43da9a9f3c78da9873d1ec7af2c9762c43a348abe078bc35bdcea96d606a2645")), y: Fp(U256::from_be_hex("395b332c4a3adb42201d22aa73ba87238b31f4ce4c586dfdce002345b2c97964")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("24f960da8625551cf41d8ccd4bee08134f43f82e463f9ccb1f0e59fe52ace2d6")), y: Fp(U256::from_be_hex("15c51723b6ba8ccbbd60654193bae66f5c2e1d400d1c8dd3ac7fa45e7189e63e")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("3762edea8c08440ed60957a2b0d58e2b9bfe5af842013f6839368ddf8b26ba13")), y: Fp(U256::from_be_hex("44af8f580368315b444ed401c569d9d9d1d3eb441a3be1e8b227b64093d870fa")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("32f6c7692a661f1d79923acc4d457bc1e0770b573c0f80fe35d2be0b0b234794")), y: Fp(U256::from_be_hex("66ca072f158db13f3503d53d8048d5c763a602799bfb33372aaeebacd64611f9")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("1b64fb74a32cdf356ec0f41fd35e3b38f1f2ac608fc8b6decc3f6b6e971034ef")), y: Fp(U256::from_be_hex("22b258262e7f86ee602d25a518cf60ab6533c73697794116e7f05101f49d4e73")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("583ae75e5f7524931d7d8e60931a2ec3a35363098703b5e1e7964207f0521d4f")), y: Fp(U256::from_be_hex("63413b54b204956742e28d7abe12b066631a3c054e18737b8763909c522b4d7a")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("21ef9ff864c90f09554fde26cee99d6ba591db5c16340bf5930361ba9d90a622")), y: Fp(U256::from_be_hex("323f21a7685b54dac9a56014af75b138ccac6826af49f2efeca301050b690e32")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("5a875c86ed1c16e93d053c59a55653ade937fcefad8150470ff78e7099b12f23")), y: Fp(U256::from_be_hex("4a494a10da281adcf6616912a0525fc7c81e406ee10ade54149db10c1eabd586")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("293b02edd5987a78660d59478fb0b9bcd5b1bec324717d5055403eff56e7d3a9")), y: Fp(U256::from_be_hex("6e9fa9adf4757b2e4c6e6d4fa49ac2f081df7f90605b229e105ec6391c07fe66")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("42ec21fd68108bcc377ad754d7b61215242b01e98ed265f087aafee734c629f4")), y: Fp(U256::from_be_hex("41515efd027eaac41679fb2cc7f70c3d6b99ed7be27ee838316c7a40a3217a64")), z: Fp::ONE }];

pub const JUBARR2: [ProjectivePoint<Jubjub>; 50] = [ProjectivePoint{x: Fp(U256::from_be_hex("01be89642861d60b8efd9fa071848a965b023615a38127bfe9e8e8aadb8981f8")), y: Fp(U256::from_be_hex("6ccd89f9ab0c19021f566ddf8b052bc40bbca13fb4b6ed5548f6518f124d3a57")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("0c7c820aee75a990bf3310984652149018efec1ec211b1467d14babf59f577dc")), y: Fp(U256::from_be_hex("514e27284883104cf0183ac213a14f0583f2b78464c53770e75b97d6035fa8cd")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("1029a5af1cb66d264b469399ef3ff92b8c33db1123092f805316961343344c0e")), y: Fp(U256::from_be_hex("2da4f29f4f27f0fe0b4a720ccf92dbcb9e33f59db7f7ae12311af44c6fe78c09")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("6b40dfa8b5b1bee9d13402c46ceb7c14010125f3388af4bdf1ba49172776582f")), y: Fp(U256::from_be_hex("4c55224de0c52c1373e49f333b30765d334c20595648f8d8958a53f9cab63f48")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("11e7d2ba93d184272a90cc40646269866bd4502f9346389b37157acf836e15f9")), y: Fp(U256::from_be_hex("593d2ec2e2a929f2255fba513e82fece38ea11c012e885613c6061371431b62a")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("0699a03b140e0eb682351b07d2ead02b13b711e4b8368a1c046f1f297f5c3f3c")), y: Fp(U256::from_be_hex("5a2d92b22255234155312c31441a9d1c870bb61a32dc883759114f044d666a5f")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("5c31c2050db54151eaaf8767e19910829cae255bc26d87a7ca1d4a66ccb365e1")), y: Fp(U256::from_be_hex("018a22438b1bc9e93e407008576f4076d1d15cb79abeaae25e66e25bac6abf6d")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("61dca61c7a61218795f175ba197ae8eba826e6ca8e4a231317ce07f61b29c3ff")), y: Fp(U256::from_be_hex("2597fa7ca92d3b46cf84dd7aa3a751f2bbdf8cb452080ccc4c3082432efa0a3d")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("0e898a481a4ffac27bd6d82715d0667a71e29ecd952a907c0ca0621331e94b55")), y: Fp(U256::from_be_hex("60175b200e921761d2c4cc0976d81454ddd016e1c40c0c28f3528b81c1218c09")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("0b1902784aff28d6409bb7313fef6d223745c6c183cd3df84732782962dc4277")), y: Fp(U256::from_be_hex("0b5e177b8ac06c8b9f8cf4fe86df2044638ab054e9965f3c9fe98c39dcf64e1a")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("2282ad7085f5599a9defcc79e547a970040b0e6f0ece534067f8c7b87533ac1d")), y: Fp(U256::from_be_hex("612a35f844bfe7a4111dacd6b0fba9bed6edb60149ddff6a8278d17549c1f9b3")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("01ddfb5b5a1d138b435baa73fc5630b6f84feba42e3d9a89d7e0759b448e0fa5")), y: Fp(U256::from_be_hex("00e11c96f49ce1f29e3d76626c9db3effb1cc39cdbf4148a127c781db36553c7")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("57e983eb49e9eacc699c1a2f891af5ed91661d7ac29e1fe4f47083b93229c85e")), y: Fp(U256::from_be_hex("3cf29d86b670ed72867d80b82146621a96332ddbec0a3968507076a05bb74e3d")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("39b7136b151cfa31307f707e0e1d1e5e7a37fa4c35fdb0807002696a1f0bf7ce")), y: Fp(U256::from_be_hex("4280b0d595103af2777d7873b4f95fd5be95115c4e33e678daa75f537fea54f6")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("36276b7f50ccf5c575963c6766958a6d51760af055a741560d2efb6c4636b9eb")), y: Fp(U256::from_be_hex("5804b17a9a578a01c5fa8497d2191d9e9b1bb440bcc27245ca6108a92811d798")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("547b00049e5c7bc1bd4a08a8722b0afd982787907f4a493c56f6b12de0df38a0")), y: Fp(U256::from_be_hex("382bc2ded8e32fe6567a5e5efffb1bb53ac91a493c56e064aef0bdcd6de986e3")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("5580fb0462707def6e13abddcb3fcc95579d2ccc1b99726e308ddc007e638ad7")), y: Fp(U256::from_be_hex("4b5f6d3486ca5d852f94f0d38666d0c32be74b58ddffaa90c6e83ab9afc5efc6")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("03fb2bf4d285762b80f7a502eee39beccd492c8ba5843e9dc57c2c1bee9fe84e")), y: Fp(U256::from_be_hex("6e728fb3d433b9e6a1836657f8d634193d8374c34b5267519a6ff23f185d2456")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("24174a473cb0434fd1d4be7b24cf252c2bcd71bd2e9b0e2e37c12a2a4fd9fdc6")), y: Fp(U256::from_be_hex("2af1d351c3435c759a0420c79ae203f23123aa7aff079549f8dd87e93a509496")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("5472bd6d87ebf1d9f1dc67e6df39126c1abfdbfa75d8ff6d59f0c06aa2356081")), y: Fp(U256::from_be_hex("6e77ca88fc20baff3cb376fd9fa0867b9a45531796773842e2b0b006d89d3755")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("34a23d881ce05f9ce677ae0ca0872c3b15d7be2932664f458e40acc03507c579")), y: Fp(U256::from_be_hex("2d47791aa063457245325f3355420f101ab47edecbd785ecdc39f937758d8291")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("32f908fa1a5f5d92f3db07f2d4fd7a6baf5c98a535df7b446bbefd17b2ef6495")), y: Fp(U256::from_be_hex("05a535cc4b60a796343e110250494cde7068df419eab29843d29bbb668dddd64")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("03b2459fd4cd011111b92329938eae917b80e1264f80972231535d1cbaee93fb")), y: Fp(U256::from_be_hex("634282c3d9508ea94801468be3e8e43587baf8c9c0579d92ee45c542c0218b00")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("27f7cc72f606a933c8154c638301f6da85d3d2c8de9450eed22a8c0defc73ec3")), y: Fp(U256::from_be_hex("2c98fb653d4eceb7b306fcc2cd205836d5a107a88eabb847fea09be36bedef86")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("4b22c5850aa164b65fc3053e2b85047d19245f99841f1174fd3b418f7e28e904")), y: Fp(U256::from_be_hex("22925edae65f5f0c9c2f3f3f747ffad1df1e0a5bbed48255b45f05ef6c79e62d")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("26e046dc9a4fc451056c050bfa0353966a85edb34acf360c782d43bb51dd3dcd")), y: Fp(U256::from_be_hex("288b4211de6229e98918da325755925d5029d505642edf63500fa1ef4b8b66c5")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("52987cf7123002b16ba3b21e35b56d057a4d2d47c6f8b040d15313cc7413af03")), y: Fp(U256::from_be_hex("4cc49691d579ae903427d2d98696595d433e6b023e255c534da5b731df004440")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("034e1609ab33f779e373e69f84d70bfc6a499f0477840ed9549c1e750aa5ea04")), y: Fp(U256::from_be_hex("62d6c6f01d145c8a5e8ab62e8be95eaea33384567488f7cf2b22da67fc814919")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("0f77054b33688637b9b6023a26c52e84cd43d6a3b2e6a6b663da18c6109c9df6")), y: Fp(U256::from_be_hex("44a2adb2aea3f1f9e2e2ac43933bffffdc94b8db941d21f90cba5e64ac1dadea")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("3a284c3cd48dc3117a57f504fbf49022f68c34cbd280b349416ef96be88f9b8d")), y: Fp(U256::from_be_hex("0d513df7dce380cfb28c3a127659889269dc92f76b1311383012cd06cfd8a9d4")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("18444a90fb23e6521819684cea27f24ecdc298411d81671d15150bb72621926f")), y: Fp(U256::from_be_hex("4bdfb6538b6ef39cb4560b2023f7f62e0c172faabe65d81572e5377226dbeab5")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("71fb4662642037127d6f6c3cc17cc74fb04afacbdbed283932e296042d41152e")), y: Fp(U256::from_be_hex("188c9577d691baf7690d3e27eea3b5c45966957bf1e88213818b9f3b78a57ddd")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("67a59d88da6cf46675be9be5b82cf44c48a8d8165909d73cb0018f6a10d239de")), y: Fp(U256::from_be_hex("2f381bede67de3562e989936d3927ac88cdfdac7ccf615963af3a2c17cd6d9b1")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("08233c20d9ccf278cef2b34e1a09bb48c2ae415b65bb355999329f78b34694cd")), y: Fp(U256::from_be_hex("2e50cd35312d9f8d2c2d48b15742a2a9129f8149cb0442630bbff84a7426fcfa")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("19e881f655b0946c688d45b648097fccd84bd4c5b91cd7dba4c8c59b9453fca7")), y: Fp(U256::from_be_hex("1038b4b429ff471e00e07b7a354c4e73dfe47befb991441d787ceae9b0417b2c")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("0281222cfb464e665001310887e840b1d05e0e1ef94a41a1cda169670d2361b8")), y: Fp(U256::from_be_hex("2fdb0d7d54a13c0620ee11af4dda6a4c807dd033248b8ebc3e5b4e38d9531096")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("32cb4920a8d19c6a63840abbe7cf1a7c190fa6eb62e43139967c9a740863cca7")), y: Fp(U256::from_be_hex("4b836e1c4c4ba02a155ebdbc2aa610f4cf38c388ff0dc25dd3897763bad384af")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("4d4cf7f58391e32f8c40859b38d4f3ff8e970f5f6fb808e7f02872cc8079bfb3")), y: Fp(U256::from_be_hex("00fb5a0d554fbaeb4d2b238921c7ce8d9c410cb4dbb5a08da841f31a85eb0c49")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("512a1472f920c76f1badbaae7f4113ffc5e162901376eaba71989c957be00efd")), y: Fp(U256::from_be_hex("0ec2a62c125af6f8eec0b0f5377915dfde39ab014ec840fba4f39561788488e1")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("3696c56d7e66a3063198d1b80cdbe57b1fd84bcd7e8fdd4543616301ff6b4f7a")), y: Fp(U256::from_be_hex("2f43978c3265eff9cb858c7688f0b8a2d651d1b45961aca86de20c141ea56b1b")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("130595340c40134526e3dd2cfd0c93bf33bcb716021adebff3092ae627022558")), y: Fp(U256::from_be_hex("1faff539fc0af6da10b551dcb8a6ded96dc2f029e1ca6af45db5509b1dea0bb5")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("341fc4fd0af86487a88aab0561f5082825a44669ddf90254004e501ed91b9758")), y: Fp(U256::from_be_hex("627095b0b4a7a2bd5cf53027cc954e9c9830a78b0822a40070f331f1d7edc5b2")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("02a965aae63e7028c4e1c757221108f0492374149fc3197b81fa4372e95188e0")), y: Fp(U256::from_be_hex("6d92d66ea7ed0057c75693e6aa9564462662f0c1c9345b18391b4296343d865e")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("08ecfbe31aba2b7a4da5a8d4a3841071ccdc27ef56d90b6d6483c690c30a0588")), y: Fp(U256::from_be_hex("628133cda9ff687cb878f702144204e06d5aaa0fbc2a3f0fa16b3b09b40a2cda")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("4c2efe2ce67ec5f5853469a33517325ff7a8377378c3159006a25af4683d6445")), y: Fp(U256::from_be_hex("4027266b52a4493d8de9818a9cf00e8d538a8483a9aa53cb88231206a2143c40")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("27e934685234d61caa17256a82919b46feba4d6154bb25d2f6cf90c8246076f8")), y: Fp(U256::from_be_hex("456922175e03b749945fe71e2edb3c3c157e7bcf31090210d6fb9f8e77ec6e5c")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("004088e2f687cf44cb80562d5004026af7c1e04a49685894c024ffe387e02c3c")), y: Fp(U256::from_be_hex("43ed7fc74e202dd51d0450dc73d9b901ecaced6dbf92beb206da1b32330e59f1")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("50d9c37023c50065c001bb2892dc7edc7d9a21bfe4f09006627f3fe36b926e3f")), y: Fp(U256::from_be_hex("5fe660d1d0085e721cbe1e58c18500ebe7b586bc13536d9eec17f1c28063ed99")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("734c646197f7d3a2c3fdbfdf1b987068a6fb6cb104b47c6623cf00f089356529")), y: Fp(U256::from_be_hex("695763ec258f0c6402e315b2699a4f987365e385af19eb286faa4c1c88a604d8")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("418eefd43ac5163c2794992bdaa5b05c317f46e64cd0a71595782033e631312c")), y: Fp(U256::from_be_hex("4276a11511291278277ba2939b9caebb983ea50ce46197ca98307639b577d6ee")), z: Fp::ONE }];

pub const JUBSUM: [ProjectivePoint<Jubjub>; 50] = [ProjectivePoint{x: Fp(U256::from_be_hex("72beef856184106f9624e6f9411277132f40e9f93df351387cc378a5460b0238")), y: Fp(U256::from_be_hex("32ba9c2ed899075413d49df54fa502824759cbb76d0280e36c7d88dea9505065")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("1e0f5ebcdf0940e0858dc547923cd3098c4723324b7f32d6375142d58d802d2c")), y: Fp(U256::from_be_hex("3c522ad2c7490d123920c97388303cf44ef2cdc988c1d22c102721268d290d4b")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("4e357cbb5422616ed94eec717b65846dbd25a41274e1ad8d47d0e640435456b4")), y: Fp(U256::from_be_hex("3413409be705df355a8cf3d5dd4efb7b282a90ec95e5bed326a56d93250317e7")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("3833da2644065841d68a5f1d4ecca8734c40d962c650078ef7ad8e7d3df09fbc")), y: Fp(U256::from_be_hex("338e5670eaaaf80ddf9a0aad7430ff84287568fddf930ee82343d8172b24e17c")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("34aedf460c05ed7a0224d881d6b365c83aa257d3a97b4c7554b75d7b0f23d7b9")), y: Fp(U256::from_be_hex("1091fd03e3cd8f4e82a99c90e7706531b9a795726ba8e771d7a1a1a578d0bb9d")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("1fe3bc3f3159362aa2af6132ad4b999ca658717fedcbd3784d5e3f2bd066906f")), y: Fp(U256::from_be_hex("51eb8c98d99a5acc1142dc15b57f0e7b150be68a5fd0e68c17efc6fa647f8a61")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("1ce0602a45997b5db61746bf938996087cf8394124616eae0c2cfd4d35d95f9b")), y: Fp(U256::from_be_hex("66aa98ff2cb8a67787b066e6d3af84afa476a7ea29252f7440f28c3478c11a3f")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("43368a2b14687c6c1c75ca33f35e1d45363fb7db7d6d3626c3a0e8b5d033bebd")), y: Fp(U256::from_be_hex("267147b1f68b2f6d8ab9d92e4f532bac7034ac5df29c25ee22f9a0d5f4ff67db")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("2382ac60d24f17efb6bbdb391c95bc35477664a55b930ddfa66df16e88fc8f9d")), y: Fp(U256::from_be_hex("0cdc3613717fd9d4e08871f2ceb0c40a350cfa51d6c936f90599e27be0c8a91c")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("23b9b0575de0cee67ffa0c31c16ae6c0e9800bc1ab69f0eb567f3220420b8264")), y: Fp(U256::from_be_hex("26750bd9bbdc0441fabd336c174844611cc2aeb3c9f81e997d7bd0fed38d6a66")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("57cfb08f8c435e730b883800be57f3047fd8c03e2a3d1791df7487dafd1580fb")), y: Fp(U256::from_be_hex("37b637819999b14426f3696b0842014bea4db818398093f3d4450f0aa1caf6b7")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("1f334d4498271c0aa00dea9ff0a480625c1d11e61dd9ce7673a19cc0e164d908")), y: Fp(U256::from_be_hex("52c12ef66564e6f089071deac065e6ea5b146cdecd0b6fd1e7ba6bcc92a8d3b4")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("04742d9a1e6c5479b92b235be793b7218601792b068c00d19d35a966919c86cf")), y: Fp(U256::from_be_hex("57793ea62d57b508f4f354680ef49a8a3f0bf0af1569dfd54c6d83813600c080")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("69684fbf14355a0c1c58e6fa98052c72dc4e206b49f0fcbc4ca4c961fc0796de")), y: Fp(U256::from_be_hex("7206104987b722c09733cadd3f9a2e60c7f0edc674f55c4b7c6b4f7c1be56413")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("67da91233150307f24c18372711aa2693842e0bb0b99f750aded803eb9b79d1c")), y: Fp(U256::from_be_hex("2fb99d02375838f0440a41a06b1402d4b633d2461607965861987f076704a2b4")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("568a9d2f5f8f7df203d74a1ec0cc26da8a86d5e7187a2375247b064dd39d6691")), y: Fp(U256::from_be_hex("50166ed65699ffed15c5d9130993eef8e77cfd4640c820ec9102965911a297ad")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("6f0996248fd0af5d5a8e139a753996d4983c78a6099135b5d686e30767bd7300")), y: Fp(U256::from_be_hex("4c9890b2433b3562bbc2238a473d0a8e10ec712848f2596247024a85abd1345c")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("3fdca747a9b2e9439f59d994e64d512f3d78ef116b9db6b38afb0beb936077b4")), y: Fp(U256::from_be_hex("11afb8d2af9732794cb382c632ad935df40f5683742e9a6329fa7e5aa615c99b")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("72603367fcf7ac1216e230a0e30207c218d9d9a30e61882d59e699b17f3b8686")), y: Fp(U256::from_be_hex("23396aca71ceef3e171ea3761189a6fa52fe7edeb15d16f68ce176e314bf817d")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("5e11919de36c98c369e6decb4cb3633945ffd34a9d4fb6e04821badd7e286b7b")), y: Fp(U256::from_be_hex("62bba40c01cb87070819b601ad3e636372614a5b8913a8e1667efbb830ba9ab9")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("3be7da3a0b98aceaff08840122a99daff2e22a5f6540698e4664dc6620ac29e0")), y: Fp(U256::from_be_hex("55e6b1298ec6ff2cb2d83547bf230386746fce151751808316ebe213e435c1f4")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("5ebd4a7452cc1c3035a337a481d0914a5ae40cc9d1f103073a8a69d69260d3de")), y: Fp(U256::from_be_hex("5542ca2d84d7b5a4dc03e7ccf16584ee2fb4f73ece341a3c64657e1ffedab6a7")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("11c20a1dbd29e9c3350a18973f48f050a10f6f8b137cb942f655c5475c93ca59")), y: Fp(U256::from_be_hex("401658bf1e86fca80aca6715064cc2961cf72a51745c227d0502d09d88e1b50d")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("4d5a98e15e5efd7241ea268468ae122be8406e9ed604da304b4bf5701770013b")), y: Fp(U256::from_be_hex("529b2f5971dfa21acd0f3342490f8fc724b7f564882d7669482da5c98db4fab9")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("2935a2f5b23931c8a4e86567e8a5fcec86db8e2d488bca713da2b8b71887c472")), y: Fp(U256::from_be_hex("565cc00fd3e52fa2c6137bde11aaf3def0243ef0f97f819dae8b4c14985b230d")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("7211792fd1d5099d870ea4c287a55c2cc1ffa4279398d7dae7e36c283fc6888c")), y: Fp(U256::from_be_hex("62ce10b2644271215601a3bc514f7fbefe20b3e2bbadb952f95a795e0653c867")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("14b849f8ddb274ef749c2d49e34c69c0ad85f156f41bcab5f2ac06505e58e584")), y: Fp(U256::from_be_hex("230ad945f90089c6013fec919c7e3702664060f388ec5c08209c248d85039268")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("4da8cfd784a670346a47cd6d82265b2152ca8ffc7ac71e87e762a7933e41a89d")), y: Fp(U256::from_be_hex("0997bcb7eeea0ed5beaff9da22928f48487a62b02f5ace4a0a31985064438be1")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("0cccc4df286eecb291962b6e46f59490d084896b39760ac90678d8c679f28f4f")), y: Fp(U256::from_be_hex("556313515e21f17d2d1370c8dc6a18773eb4763724fad911b5f509fdeb8f9f9b")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("5031a895098cfb4be5f0c6d77fdb7cc6958ece5b7389ada77afe5c70a80e0f66")), y: Fp(U256::from_be_hex("68f637ae76b5f64d310b6c981bbecf721a7a1c24604ff759d64cb80e8ed882ab")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("5b004935c28db44ce3cd151d8acb3304f69013ec529169fdd58cff2798478602")), y: Fp(U256::from_be_hex("33fb49028d148a64889256a780e8392934d13ba759c50a616b45c31e3c5697d0")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("4682b6d0f40f6d1bcdc8b115e0cb0e65237838d024c1b7092ed3974a1ad17c42")), y: Fp(U256::from_be_hex("5360c5623f796d793e288626c21e7ec7c54a3e6994fad05cd7af77c8b839c4bc")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("4aa2893c6d09e323da5a0eaabe2690f659669cf68396feb0d9442e7f43ead15b")), y: Fp(U256::from_be_hex("3b3ff3bd1503dc44cf28303bf1e2e4db25754b9f15d7d0f6c8ea604d9172cd34")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("3b26f22de2784ef526279f4db9e2cc6117801ff8d9e96562e7abf5fbc7c7edab")), y: Fp(U256::from_be_hex("4edcae09c1c3d277745314c33179a88c1158d744ee0077829bbd964b4d22df7f")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("45c598e6a789bf8ae350cf68b24778cb5e681589f6e8eff049a23a858d6b9785")), y: Fp(U256::from_be_hex("16df4a64deff7461f9446326ce7e0aab6af45a1ef695b1433b4333f781a993bd")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("700379efbe98638b7b4582323dbde1063008b831261f0e3e4bad72647380ac17")), y: Fp(U256::from_be_hex("1442b47b3120409f1025c3abc574dd586db6a119b13339b6eb73037211e0ac7a")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("1852084672e61718dd793adc3a090e57cdcd2be7a2187f9386c2225f990e0e49")), y: Fp(U256::from_be_hex("2ddd701fdd598cda0e262e85a48b431da8ecd850a94364c04b87c2a36da7422e")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("310df773fd53ceb7ab2277b92be7084e9582e2290a3fdc7029fac78537bef3bf")), y: Fp(U256::from_be_hex("0c81a1f888a03499e73ab7d9fbf47587192462203e1b085ad367cfd35826e651")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("0d9c9780849af7647f696e494379676173514af7408dba3eca49dbcc932c4250")), y: Fp(U256::from_be_hex("2003f4f053f6dfa12d8e10159dbf085eb6dd1327caceb1a695a60a39cb92a120")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("67d432273f87066024cb5fc703d29e8c5313ed0ad592bc5a6464039b39757a9c")), y: Fp(U256::from_be_hex("289f3ab17c7b8ec7cfce1210e97b81dce1b0a9bd842d5d9fa53f7b59e702ca50")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("2baba7229e2c84050a28d31811431b5dad0f44d1a2c9e006bd6bd75a917b789f")), y: Fp(U256::from_be_hex("5d1cdb4b11e797b9b34d6f3a4f2edffbbca3676838e3026edd9e93f624f76a40")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("4626436bbf09aa5bfee3ec1eca5bd8bab396a6c31667a0a2d9a2957426071975")), y: Fp(U256::from_be_hex("339b6026b0b759a723fe98b498c236f17f869db867ef363d1fd748b8aa543192")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("383e53321d840a4bae200ef29cd76c996cb8fc6fac9a5bff534aa74f03a33188")), y: Fp(U256::from_be_hex("52daac5d6249f15cfe0c916efb491be2ca90fb3379cac798fefbb7399f091e23")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("6cad6d1836914d2adf859f47b48c4adee0706eb20132098bd4e1646ba0211e4e")), y: Fp(U256::from_be_hex("376be2913e6c4ebde6584052e6f69b2db7ecd4649bcbf7911bc6756b82767f46")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("708ea5db6e1635814901989703905826a5c6d9bd8e478d1875b19913ce798b5c")), y: Fp(U256::from_be_hex("7143efbc6aa9c36688532f4b4c9e414122fa655be37725f318b89b66a0edf09b")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("108fc8c66430c99118dbad06fdf3bf370db15cb481cbf9084fb3d594bc4b14f8")), y: Fp(U256::from_be_hex("40b6194c1c23e497ca37aece4ab3399f46f4f193afc9c77a047f4f3fd406c52e")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("428c6e727cd19885160048c103038dff753a74111f4938b14e1bbe8aa07e0ad2")), y: Fp(U256::from_be_hex("0d537c973e21bc321a0077ff6838270d719c99836fb926a892df432427c4fcd1")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("6c5fc6f0d72c35db8eeb426e575f6f7f2ade4e23712a68b10c3eaca5fbf8fb43")), y: Fp(U256::from_be_hex("6e15601f8dd0f4f4f637af4269d3d44b37f24937d767794afce398b607fab526")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("577b3a7474c8f775a48235c83e387063f79e50746b096fbf2bddd3c9dfa77ba5")), y: Fp(U256::from_be_hex("7017d7494139750bd57c76cd0c1d706cc6e64a05286bcdd5812e013259aadaba")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("346ce6336514cdd398af6f8c2558c524fc71a60831ade352f39516aa74b46d3f")), y: Fp(U256::from_be_hex("71e03ec9618aaf4dfc5a9141b8034f90dd2d69d127b4e3423d821738c32aa3df")), z: Fp::ONE }];

pub const SCALAR_ARRAY: [Scalar; 50] = [Scalar(U256::from_be_hex("0a2bf0edcaede1bd3d2487c8298b03548b944ff0ce8c0d1aec70515fa9553d14")),
                Scalar(U256::from_be_hex("03aaa338a70d65754d6b8869bde3051446418b52a7d65c9cf8b3388ef2980274")),
                Scalar(U256::from_be_hex("059f6dfcafed4e40133907ca7040fd7c630130dbf7a52e6949a7332bde2dc7f1")),
                Scalar(U256::from_be_hex("027bb6cdc7eef713ff4e33625aa204b39813c8bb75773d8faf1776b91f97b982")),
                Scalar(U256::from_be_hex("08678b08e9ab9b42fbf6c7b07a38022a30341f1f50c380cd9d6ccc8bbf42e5dd")),
                Scalar(U256::from_be_hex("084cccfe9b6889d8a05ebec95c37ce13007c4cef227b712cd281cdb1eee50c70")),
                Scalar(U256::from_be_hex("0cf13353157fb43a438b720c14dd44d95219c24262c302c63d5ed47074c77b90")),
                Scalar(U256::from_be_hex("0e0db25768367f90176e959b3f2320c92bfc1883d7c588f91b7c866ccb8ddf61")),
                Scalar(U256::from_be_hex("004a58fe63d259bece445237f4c0c262c25081e9ea89c2e49a28956838568523")),
                Scalar(U256::from_be_hex("0949b74a4c09a658fb10fcf572b9f88bcf2d312a961c9b86c1603d48e75fde5d")),
                Scalar(U256::from_be_hex("0176be4df1e1ce4221ea5221c68d31be70e78167d9c00ef7b7f478e5a544ae47")),
                Scalar(U256::from_be_hex("01b2366817f1b4e05b38454dc0e384d724bd395fcddf979e499b69b16b7f7cf2")),
                Scalar(U256::from_be_hex("0d937110f2a73c34e7bb03cad0acf55566474155588878659a87721deb85a40c")),
                Scalar(U256::from_be_hex("0e6507cf7f42490ad7c335da9902bd77f22d6f47851601a2f5da293d6476494c")),
                Scalar(U256::from_be_hex("0d5f6fa22a6bcf45a10dc6fa8efe8036442cb192432e1868f988229797f6ce4a")),
                Scalar(U256::from_be_hex("0aac6fe419ca3dbc710de4ff8b8b1a434af5ba81bee14b71e8472982002028f4")),
                Scalar(U256::from_be_hex("01278f18b7a8cfd455a6e544bc17ebf6c91362bce06ba5b9da1e375537510930")),
                Scalar(U256::from_be_hex("086e985389669f96d465eb128c8b236b37eb3aedd5a9c744cc81a3ed7340d7f3")),
                Scalar(U256::from_be_hex("0c166ae91a8cb7aabdc16b14fd30f87b6eba386060347dc1cc6609eff5188be7")),
                Scalar(U256::from_be_hex("02e37a4b044b882627cc2b5a8f009c661fccaa0b619149a4fde17ea7dfa3fe4d")),
                Scalar(U256::from_be_hex("00e296480a4f1b8e1f0fe5f863376840c7a188e6c9eb1863b2dd25bad4fa7586")),
                Scalar(U256::from_be_hex("08ca098d0e3b70545747d38f8a49b2dab42bd075bcec91f2724c62dadfdb4537")),
                Scalar(U256::from_be_hex("03f015b0cfc1092df7cb8d43a219d3786df39d0d58dcd7a3e5ac5c57a24c96eb")),
                Scalar(U256::from_be_hex("059e5a289f0d0b3d10487859fe4801b6abcdee7a7f38bf21a2c2878cc9a794ac")),
                Scalar(U256::from_be_hex("0175afcd74e3d83856f4db66ad0950bbd9c19e9983d1f7278b2120c5d96ea62f")),
                Scalar(U256::from_be_hex("08fd3f68cf945fd8d2dd36197560a0d264ccb3dd822c8906eeb3bc434645e4d8")),
                Scalar(U256::from_be_hex("02295575887986beff82b5368fab7bcceb65cbbf2badc882a587d0f028d5480d")),
                Scalar(U256::from_be_hex("053fcc142c1a696051b46f51a93154c1879bd1541bc3ed6b591eb4dd639b153c")),
                Scalar(U256::from_be_hex("0c21def4f899bd0fd4e2ad7aaea880f526bba5ce5af09e4d4dc67ca9d3ca497c")),
                Scalar(U256::from_be_hex("04a30a5394f352da17848b16ce8a2a02bd523b735512706f19f8772de1736b90")),
                Scalar(U256::from_be_hex("05b9ceb0d5c85be2aa86bd48b0041c8ed12cb630515e4e87e064336312e82caa")),
                Scalar(U256::from_be_hex("050e33e26251a1ab4a244795fc4117a1070bed8df86addfedf401158546d34df")),
                Scalar(U256::from_be_hex("05cb18fd2c3a642c31a6aada0240e18b745b48670b541608ce9c7f1c03630138")),
                Scalar(U256::from_be_hex("0016411ee7ede0ceb11ac23f20cc2c328afc7c26b5602aeb6357eb89283e834f")),
                Scalar(U256::from_be_hex("0c88c6c414da16d9b13f5168ebf8fb39a1a4de1f9f8b8210aa9563c94b118124")),
                Scalar(U256::from_be_hex("0b1442ed178d0d20771bc6c6d6fcda29daa4a6d9b9e30de83177b78faaffaab2")),
                Scalar(U256::from_be_hex("0401ec702a1f2e62981325557a264490309c2f25bac8df10201750e750737390")),
                Scalar(U256::from_be_hex("057221f1ccdbbf045a85de73c1a437d1249e6b89d1d41dffba1c2a5136d3e872")),
                Scalar(U256::from_be_hex("00a52cf699e76d1e756f08ee80a289e3a94fd01e865e1e17615bc393a1d58b32")),
                Scalar(U256::from_be_hex("076eac545a416976dc60cfc793977cd98108b8b5bb81b49687f66249f814bd0f")),
                Scalar(U256::from_be_hex("00068801ddd9a14defb2a232d93ad37a1138d80c77f8f44f01b1aac4d5315e57")),
                Scalar(U256::from_be_hex("0d1e53dfcb1b4d4bf841d63ad51b53e167216973f8f7f5e097fd7d5571479d8c")),
                Scalar(U256::from_be_hex("01cfa17855ec36b745e7df1ee0f6e4bba2e329ce1f7ee8dda0591464b2ba9993")),
                Scalar(U256::from_be_hex("00b876349c3dee9b3355c16b7edfbaa8805d5408dd0a4544b99a56869e654015")),
                Scalar(U256::from_be_hex("001e86493b2181114eedccddb5bb50454e07b90050aa5160e4c8b927acac1744")),
                Scalar(U256::from_be_hex("0abceff7e28239329f51d57a606fb7157127100dd017f0fd4029ebfa9bd8a8cf")),
                Scalar(U256::from_be_hex("03e8695457ae85b0fbc43e72a70d10eca06eb69935809612826747cc2cb87422")),
                Scalar(U256::from_be_hex("0b3aacb71bce944e2c4ef36d9eb601c884b486b54f5f51d325fde7583be340fd")),
                Scalar(U256::from_be_hex("0b3d1b6e08332887d12afe8177ddc67e304e0cce651d5e7b4918f84bd7579bbb")),
                Scalar(U256::from_be_hex("09b19b7a5ed43311077ea4f0cbca5116bc4a589d24a9433e6d90a3f0337be9d0"))];

pub const JUBMUL: [ProjectivePoint<Jubjub>; 50] = [ProjectivePoint{x: Fp(U256::from_be_hex("1d9a84eaab5fe7cfcf0d8cd209581a401563c765203a4ddd8b33d05ed2350bbf")), y: Fp(U256::from_be_hex("1a10541178b5f141ef68ac5b5601c1967ce13d6f6e4a084cbf983d0ca0d623f9")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("182afe2627e43505b7631cb79a7701f4cc6daef138915170052259ffbe60807a")), y: Fp(U256::from_be_hex("15bd5090de7fa09ec4655445d42ba6662ed7d48be90906a605cf77709edb9ded")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("4b415ef5e9e86f84441462b81e709e954f56716dcd369e664d27fff171e47c3b")), y: Fp(U256::from_be_hex("19ac2bb24d83f65234f78cb21845c904a3737d33f1b8605b96b9255e680e366f")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("00f4ef5c7b6a06a7e28a450abeb59da16085ad85e4d8ed8c07dc9e554834a6dd")), y: Fp(U256::from_be_hex("5431ee067505a7fdd86122663f38768ba809fdddef65534ea0f893a84883a125")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("66f0986793f2d89fdc50482894e9c25a8bde9be20c08ec128da7bb6e38a858ef")), y: Fp(U256::from_be_hex("0d2160bf5db0bacf53719f9b7f7cc3c50fb2b0d045cbb88a6668a834a285ad3c")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("262db8d10c89770d6e3ca7ae0755224e8a5436d13147bebbdfa27b8ed570c492")), y: Fp(U256::from_be_hex("18d5e706965d81a353e4758de69ec3a4c424df81a709b554a30aaf5d8d01a63a")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("1858a5b447c8b01e8564ce1d7cf0fd9d490b3ebf2f92737ef9410de6e4dde9f8")), y: Fp(U256::from_be_hex("17212ef1d8c595f8d371856e47dbd8290b7e90bf72933dbd8728391f37b794a8")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("5d38fbecabcbbe8439b07b9e82881aed69714d4f3f20aeaeb52148711c469d8e")), y: Fp(U256::from_be_hex("15d8f8462452847238e35f16a3cfc9966b54e41e40371d15e716cffbf1e34b34")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("5765abfc6bdbe5b48ca951ea0a960d40fed39a6c892a76874691277ac5c8fcdf")), y: Fp(U256::from_be_hex("3cbe8cb466b9759567503181d409a9b19c412825f73c4e36f68d2ae7fd6af401")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("0806b37601034e283368cb086d33eca58716fad11261ff8d2f436e428177f1c3")), y: Fp(U256::from_be_hex("2aca318bf77f996b791751f430e0448f9d418ddf6fe4a08fa63856fb3b35b350")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("18e736acdd3374149f3434182fea103c96b10612266058825bb9a7d51032d031")), y: Fp(U256::from_be_hex("5041ee4b5961af5ba11b541b782fe0733da85be4f1f221e598d0846d75fe6db1")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("3e37e67c051d54cfbc110e98a1bebbb55825b5560a73cb76697cbabbc750947c")), y: Fp(U256::from_be_hex("364f58b1488e858f21eb9446a2bfd3aff2b69ec8f731fc163d54e8c41b0beee4")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("63e1e31473e35f3e465528b4c578604aeca6618ca60770d4287e4a1793dd7706")), y: Fp(U256::from_be_hex("56498bfb4ffb33b80e78428126f3326ee57c34684acf9c0e58b75b80681482aa")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("39df4f080994de2b6c26a4992e07ee0a0f8e39857c8eb46031a9e463611db809")), y: Fp(U256::from_be_hex("71971a890e1f462d118c72d9f1176dccd44ecbd7d1aa6969d7e448b6d88b424c")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("0159c8a2f816a14ce076a793a0d913e7faf29511eff602a0903cde39560b04ba")), y: Fp(U256::from_be_hex("5f3b4b197b922a81c713ff10c0d0bfeb21a0007d63215ccf4ab20c5c4dc3067c")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("335a5eb6fe421afd653976947c1a0f439da34f373729628b8323ca1250bcfa8f")), y: Fp(U256::from_be_hex("1d08b5905c53d5ebdb925730178a7c34d17bd5ffe18c1cf46ee871893b1a2f08")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("09c74742da598d1fd67a547b1ebfee73c896a57d85331f26ecc4be7c4844bee4")), y: Fp(U256::from_be_hex("225190863ce3d673278486898e537629d3d6f37547275018a2c216a88e74c2f9")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("44abf3eb7e59bff5109e752980de9a2f286a7e38329b95e0c1f742e00556d0ee")), y: Fp(U256::from_be_hex("3af93117d906ba47e20435abbd188e188af96e566b862643ae759c654c7f26da")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("520cd55e286eaf6b82ceac741bfc392b63150a36d9a2242c06cd438c912d8daf")), y: Fp(U256::from_be_hex("5f897cdf1db2ae99586d679abf7c83860595ceb95c10e81d1e1e4388fdbf333b")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("2248361bce086cd1b24fd1097b21320dacad14d46410625082f9b878c11c4139")), y: Fp(U256::from_be_hex("00685c7b61a162313a4226a6fce623509cd49cc9a517cecbfd98e93e3091163d")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("121b878714b7d0e4f06b42e8f1244c158fbf2a455f90e48b1f2ddaf7180c6983")), y: Fp(U256::from_be_hex("1f4f0b24e657e8db26d723e561246af523ad33c39a7cb84d7895dc0062c8f21c")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("2440768b513692729eab02c1dcbea551be0ff9163f4bb5b909e2ba5002b48c69")), y: Fp(U256::from_be_hex("494357f61a4271c4decdcdec3d9eba9adafe5ff79bda2d77896941f7b9cb214b")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("67a5b168d7085d5c567f8127b4d84b3cd3cf68d5fc58fdb8097c527f1dbb15b9")), y: Fp(U256::from_be_hex("080819798975d490814f6f2f17ffa2621c54a12d56a2a5ee2456f2617e015ea9")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("5c585174931cba6e27a17dc85ff1c3458efd349e76e26262fab4140b20adc2e8")), y: Fp(U256::from_be_hex("2be93a611df43b8606af4060368f6d3e19d2ca8c08566c2f1f4722d34f21db97")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("634a552803526bbc69f5073d0bc54594ab36176b340188d420b64d7a3cafc14f")), y: Fp(U256::from_be_hex("5c3cdef7803189adb8ec010ba511f03f12f885dc56300690baeaad00cbc42e71")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("15be4142ed250a7a9be49efd4221d2e24a77e1e6e0b3bd7e190ad3188c0c1abc")), y: Fp(U256::from_be_hex("30d6100790af8c8ad88cd312e61ab5dfebc696d884ed14e5172eecf8ac87661d")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("3de4903f91b98fe31a665a30cb7d2a1e6200f4b36d1c1bf30276aa85b1c6e3b3")), y: Fp(U256::from_be_hex("143c095aedeb645a400176968ab3f3e61e215696927275e4bc1aa8adc67dd1b2")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("36bca52a69940489b24872944cd8ddaf44d0ab166e488b3ed9211a9ab0f97f93")), y: Fp(U256::from_be_hex("0190e640e643564795920c4048ce6b2fdd526fd55406ab948f9797464957410f")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("4b6265599d17534550e8980acb16d3598044ba5dde34861e811193a7679506c8")), y: Fp(U256::from_be_hex("08a66a7ab6f1c180beec8298c3197220532da4d2e716d6b502899676e8253213")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("0399a8c0da988e48e3a44a55428a5b2768c88f42d78374b6a9fdcf320f9e5e3d")), y: Fp(U256::from_be_hex("63e5051ed60ba32534330a6c8d4876000e54963c137b2d27df4e7910af7fc4a4")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("1089bfcdb770c97f330896395f397aabc990f42f5bf22ceea5eb4e4ab0f0c737")), y: Fp(U256::from_be_hex("0606e0836466edc59c260bd9c377bd7ee49b54fed556eeecdc7c22caaeab4808")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("72b23be2bffd7c03b0a4d6a3d6503c90ad5637f86333f2132e817ec669d50a31")), y: Fp(U256::from_be_hex("19cdc3ddc56777afa4308db14c703acc74e8c9f6adb494af10ba846854b2fcd7")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("1d97e7cd34e5e9a3b96bc4ef26dc17386325576f280bc0ae426fc7e6366029ce")), y: Fp(U256::from_be_hex("2277d14fc1f47856be029424e381f6ddcbf228adb292786ef0c974987a89d817")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("288881534529381cdd3eb778123cd18815c5678d5d54cd52591cf62b9f176d95")), y: Fp(U256::from_be_hex("2f506fd3122d5d53810f2b2b6246ba8d3cc76efca0c591113a48c6e2dd942dea")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("4767541bdf3123e65c0aa4f8ca116669455d38762f2fc7de6fc10dad07d7322b")), y: Fp(U256::from_be_hex("711d290f4133489b9949b0f72e8d48462e4059824bffcc4d04a66d9550c3f81a")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("48cbef7835cc26fbb2694907b30d90212e1636b07f6300642fb05099eef066a3")), y: Fp(U256::from_be_hex("412fdbf14f8eda1650ec217b3724ee8dfa8f86f90069caaf5994392f725696af")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("4333fd9a3f791a030f867fe6e238e91b7d150defc6ac9d07820a6c55485f659e")), y: Fp(U256::from_be_hex("288325a5cdb5a9c4a566cecc433e9ef746671d5455ae56aa529e968dbe80d8b5")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("4ec3c02e5ab7fb55554c6cb37c94cf5f00089df8dac7dccaf393dca03f85d1f5")), y: Fp(U256::from_be_hex("67e653a53b48acb3ab7d1c0499e1877301fa1ee7a00df0baf73f2498ff6743d5")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("5817780c2ff5ce5cc4d5e6a90fb2d069689df19747d0a8c4eb5865a667061f08")), y: Fp(U256::from_be_hex("723cb89b035caba12df175359e71206370fbde94da08ded7167a49b396ec5698")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("6b46d406e5b564213c2c9fa76eb58739a2780b057b91608e0a031ed8250a24c2")), y: Fp(U256::from_be_hex("5f59bfd3330cd9c1316fc159a822997667683cf16b34aa289e923b207da2a7ca")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("0220aad4198f3cb0b1025635fc27884e1a176a8162476f0a86a1f082fb0d0ea3")), y: Fp(U256::from_be_hex("05a10b5f1914bd938064f78d39bb3a22f6150404cd69147c924c12c7c81ccebb")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("15e5a50662bc6a01d217ccf22be74bf4249a7641c7438c2b061f513047356113")), y: Fp(U256::from_be_hex("14df0c370a99334b20bd41e154059eb2198aaffb5f1e08aac5cbfb34daf40d5d")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("60b1c815da8709fc20aa19a7bf6e1fcb2d755a99334046492b894c1799776c5f")), y: Fp(U256::from_be_hex("54e18ca5eb29649e0f5b4845e96fcb244ce39f5d7cdaec2e0e6a13cb512ac3ac")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("5cc256709cce57ee3ce52abcf29e64236f61c796bdda6e81f9beb100d2d09f84")), y: Fp(U256::from_be_hex("0b87656b96d5c57e7cf47d05ebdf5358536ba2a5119f641ccaa68190f7b4d960")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("595133d2b93beaa62580524aab95352f3ca0e8c0d648797f3158cbf74e199de7")), y: Fp(U256::from_be_hex("4399b023849b631c95ebb2970349e7a5efacb7a7c5c47d2b0517cd69e7f43af8")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("2097e63727297c88e636cfa3a181a5adf3c13d44a7695ed7ee2f65c97a847ef2")), y: Fp(U256::from_be_hex("51e2cac8ce01cfca4c839161a3457fb748655224e3b02d911780a0a372172126")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("69bb8e63a95c0d08296ec6fd6f23eac74d134ae4fd17a5ce6424f481b2bc346e")), y: Fp(U256::from_be_hex("528b6f5a26c73c8ad8ef5f9455822ff0dbf9058d4c19556b54cfee7f14e4728d")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("57d40ae53bbeec07eda8110491d20460199dd7b487049ed5d5acdf6027a49427")), y: Fp(U256::from_be_hex("454c3d593aec7683b7ba0828ef374709cf7c62c7229e79de89e5554f8e09dc23")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("55f2574eeee5faabe575aecdfff339926cf07a5e23cb58d5206f61b46de6178f")), y: Fp(U256::from_be_hex("70f25a7d69a49eb70709b4ec31cfb88bd2d1d8fb5615dcd6d80dac2e13264e58")), z: Fp::ONE } ,
                ProjectivePoint{x: Fp(U256::from_be_hex("60d883a9c50d5f90385ae98ba6e38c479b333138c227b34493ee933e983b812c")), y: Fp(U256::from_be_hex("41b2e57d6ab9ae24da328525e11e6ac911254c909d44808ca5f2be7f112da414")), z: Fp::ONE }];

#[cfg(test)]
mod tests{
    use bls381::jubjub_scalar::JUB_SCALAR_MODULUS;
    use traits::traits::Field;
    use super::*;
    #[test]
    fn addcheck(){
        for i in 0..50{
            let a= JUBARR1[i];
            let b= JUBARR2[i];
            let c= JUBSUM[i];
            assert_eq!((a+b).to_affine(),c.to_affine());
        }
    }

    #[test]
    fn doublecheck(){
        for i in 0..50{
        let a= JUBARR1[i];
        assert_eq!((a+a).to_affine(),a.double().to_affine());
        }
    }

    #[test]
    fn add_mixcheck(){
        for i in 0..50{
            let a= JUBARR1[i];
            let b= JUBARR2[i].to_affine();
            let c= JUBSUM[i];
            let d = a.add_mixed(&b);
            assert_eq!(c.to_affine(),d.to_affine());
        }
    }

    #[test]
    fn checkmul(){
        for i in 0..50{
            let a= JUBARR1[i];
            let b= SCALAR_ARRAY[i];
            let c= a.mul(b);
            assert_eq!(c.to_affine(),JUBMUL[i].to_affine());
        }
    }
//................
//............Testing for multi exponentiation
//................................................
    // #[test]
    // fn checkmulexp(){
    //     let mut points: Vec<AffinePoint<Jubjub>> = Vec::new();
    //     let mut expo: Vec<Scalar> = Vec::new();
    //     let mut ans: ProjectivePoint<Jubjub> = ProjectivePoint::<Jubjub>::IDENTITY;
    //     for i in 0..50{
    //         points.push(JUBARR1[i].to_affine());
    //         expo.push(SCALAR_ARRAY[i]);
    //         ans+= JUBMUL[i];
    //     }
    //     let c = ProjectivePoint::multi_exponentiation(points, expo).to_affine();
    //     assert_eq!(ans.to_affine(),c);
    // }
//......................
//.............testing for Pippenger MSM
//..............................................
    #[test]
    fn check_pippengermsm(){
        let mut expo: Vec<Scalar> = Vec::new();
        let mut ans: ProjectivePoint<Jubjub> = ProjectivePoint::<Jubjub>::IDENTITY;
        for i in 0..50{
            expo.push(SCALAR_ARRAY[i]);
            ans+= JUBMUL[i];
        }
        let c = ProjectivePoint::pippenger_msm(&JUBARR1, &expo).to_affine();
        assert_eq!(ans.to_affine(),c);
    }
    #[test]
    fn checkconversion(){
        let a = JUBARR1[0].mul(SCALAR_ARRAY[0]);
        println!("{:?}\n",a);
        let b = a.to_affine();
        println!("{:?}\n",b);
        let c = b.to_projective();
        println!("{:?}\n",c);
        let d= c.to_affine();
        println!("{:?}\n",d);
        let e= d.to_projective();
        println!("{:?}\n",e);
        assert_eq!(b,d);
    }
    #[test]
    fn checkmul_1(){
            let a= Scalar::from_words(&JUB_SCALAR_MODULUS.to_words().to_vec()) ;
            let b= ProjectivePoint::<Jubjub>::GENERATOR;
            let c= b.mul(a);
            assert!(c.is_identity());  
    }
}

