#[cfg(test)]
mod tests {
    use bn254::fp::Fp as Scalar;
    use bn254::scalar::Scalar as Fp;
    use crypto_bigint::U256;
    use curve_traits::ProjectivePoint;
    use curve_traits::{affinepoint, projectivepoint::is_on_curve};
    use traits::traits::PrimeField;

    use crate::grumpkin::Grumpkin;

    const GRUMPKIN1: [ProjectivePoint<Grumpkin>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1b32807c87a06c83d5422e49156a8fddd2e88ac682ed0e184fc9baea5429ec6b",
            )),
            y: Fp(U256::from_be_hex(
                "1134314e30be47b2ee06527fc092039cb9841f87cdf9fe0f66465795219b8216",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2113e7c607d2a71e5a41c2a5b70f1f64f12ddef724e37cfa322d195049f877c2",
            )),
            y: Fp(U256::from_be_hex(
                "073239b4424590aa72d4408a7bf8ee872a837a0ed06b6c13d2cebf02d172cc5c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1af775092abce479008e1b33bfa9447318727cf5e6aee2d27b72fbd9cee0e2be",
            )),
            y: Fp(U256::from_be_hex(
                "0a559c9accc06589f1426d0fa725b431ef1de165d8fdc6fdda0a7d8fd5fed8c8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0bde0c38a9094aa11360ead0a285dd6f326c24e8f98eddfd361398825298acb6",
            )),
            y: Fp(U256::from_be_hex(
                "196d501c2dd36182902bb5742a3353b35d48ba4dbf5a5ffef169be67ce48282c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2c716e26eaf9998486ea4af197d72dfc6620177831f65b3412bbe7a91f6a75fa",
            )),
            y: Fp(U256::from_be_hex(
                "056b2ac825078bfdfcdf986bd0a6e81f3364e4d1d24bb6ef43efaad0ee13b0c3",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "21b120dd90c108edececcf1cc60597d7a5b023a003e212e4ffbb40a10afd46c4",
            )),
            y: Fp(U256::from_be_hex(
                "0fe3e963537ca3df2ca6805c64a7896e1434f181960b6887549c21bb5a6dab03",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02e7e61144996b65158cc1002cf5839f4925baacb0dd7367aa60434fedf8ce4b",
            )),
            y: Fp(U256::from_be_hex(
                "0c26cd69badca38556cb4271a6f7e5642f0a582a46ea3c98d68729e20280584d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2504cf8ffdc5093f24819092b87fca70fd0eff159dee0fe0e5c86eeca60ba674",
            )),
            y: Fp(U256::from_be_hex(
                "22dd7be2aafaaddc937e6ca0d052cb6d2296e352e9f9fe1b82b6f959e10e2456",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2c1c9389839e9d61bf0798aef8e4389ae7a0d5c8c5a118ddf7e0b82607ed43c3",
            )),
            y: Fp(U256::from_be_hex(
                "2b34073abe4fbcfdc0cc750cea7fa70828c9622ac7c739d25bea98ebe25ca3f5",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "264e16be23206dcb5b338404e4386f2b0c7f4f98455a2e3a7479444ed5f7f337",
            )),
            y: Fp(U256::from_be_hex(
                "295a80da37612f8ac47c44874b3c469e455e5e5061d3f257d359cd2cf602f821",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "101ca2289ad0e7624a5034d9af8d720922d586b7972bd5a3bb90f641db5472de",
            )),
            y: Fp(U256::from_be_hex(
                "2415e1dd2e813de9abede000703f0b2c816ae576cd3c4b73f40f202c549c6887",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0a23a9babc99fec6a20d81e930c26fb6218db8b0ac96885b4fd88e085382e08c",
            )),
            y: Fp(U256::from_be_hex(
                "24063c2ece59cffd4c5623ab9d199c5eb61b9e0924e8156be7fcc97e4d484733",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0137737b706359ebedda434e4d7cf040fe48be7b594636b2044909648776a9ca",
            )),
            y: Fp(U256::from_be_hex(
                "1cf8b0d0bc6a35775f29d55af349ce623aa72d40a446efc65d4409327d472229",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1aefcfc2580f8f9b9f939d81520a453768de9e63ff34334f8c053f44c632f0b5",
            )),
            y: Fp(U256::from_be_hex(
                "282ef580aa5b08a29721d4eb0d045514a47302b78b04dba060fd28524be3114a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2c0025f558e8dfa53fe984eb1a3628f3198cf53723126e6527633e1176e5e825",
            )),
            y: Fp(U256::from_be_hex(
                "0243d8799dfa56c3b4f1bc09f0e153b0fc4736b23e6416bf7e0a441651394a67",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0d30d4102c0fd5b7e5866acc40f6cc9b16905a013bfda2b81220bc802b524358",
            )),
            y: Fp(U256::from_be_hex(
                "0cdbf0376f5ea18aeaf2fbc9ecc8c6e0b98c12ddfe20ae7ab2c072d119231804",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2aa4da19ab8f8d6f2964b8a18bc0a5270a0ca0e0696d397415453f0f1cf16219",
            )),
            y: Fp(U256::from_be_hex(
                "22902da9edee11839cf6d30d6f3b946bb37045388388a1ef726ab1c2e1b1053d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "126689e05b64ba6b552e7888a22483a9920185c275cadbd8efefd023d259ab8b",
            )),
            y: Fp(U256::from_be_hex(
                "159d70afd1b008ebd61df048ade937e69bc64bd87ff57145a925004c405e8bcb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1b4aaa7ad7e5cc012315b294b1e2394cbb932fa12f7a78c9b8c4dd3cf2d751c5",
            )),
            y: Fp(U256::from_be_hex(
                "14f9ccdf6ddd7b3c1d6e4ec230c0fc27fced02d6a4040d4b45d7ba579ea0dfdb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04a41ac135d394631a5f83a409d7f28bff6d2bfbcd745f82b86e5c5a522fe306",
            )),
            y: Fp(U256::from_be_hex(
                "10ab4800db3359843f6a84cd941b7a7848d852382563532f7414eccc360f747f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "23a668b15577b7adac3b8a30532cb937247f2ca179f287bcd500abfa0f4c5af8",
            )),
            y: Fp(U256::from_be_hex(
                "14dae0607aa68ea71f780afdc807d19ed8b463f7aaa01811fab10cabaf6a4c69",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1c9d5602b314d104cc5ac030cb736275ca9cf9181dda43d47e5fd28b94e17b84",
            )),
            y: Fp(U256::from_be_hex(
                "1480dafae0443f2778b3f22d255ce9591ccaa6561e7bf0e915fb6aa8de14dd21",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1faf374902f33e5b4f3a1010798aaa91325b81c39635dfa1f916aacad43348f6",
            )),
            y: Fp(U256::from_be_hex(
                "1d39f5f57fe02d455086bcc89cc89b5810d6b2de65ecdd23b6e4d0af717c0125",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "241ef68f35fffabd0ad2bd3e53002b09f826b95b87899fe0522a4a49d41fb762",
            )),
            y: Fp(U256::from_be_hex(
                "2b45fe8e9e2cba644010d5aae8ffcf7743a056746a9cb9eddd9a50f341370b23",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1640a4720de4c22bbab4f775b71cdf0983e7f05bccbf8f6671c01a71d9e68dd9",
            )),
            y: Fp(U256::from_be_hex(
                "02d7b51bb0001c7913dcaa0d04a95ca4b20966ead6b8eda16e04c43cd4906b60",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0236639c6f77a12fb9cafcb34332c7bc927f7fb750e805a1cd2ce56cfbc1af7e",
            )),
            y: Fp(U256::from_be_hex(
                "0faecd3cfd66d1d4b7a0783965238da49ef25b71b54c5f8ac6b9979016da1883",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "22668dbc4016582e4ecbc88642fa4f4c1db9e3140274d094392984b892a1f8eb",
            )),
            y: Fp(U256::from_be_hex(
                "276cceb4f513815150bd00404067568aad08441032e1a4de7ed6ae8ffbc43007",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1b193d3d9ae21a3d611040a59ed5b04e805c53319f91186f8ea83707091facfe",
            )),
            y: Fp(U256::from_be_hex(
                "2de31cd4a6937f23f2a4777743a369d0e402ac0aa7c2b6e4ddd004752c784eff",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "12f503daaeae9ced9026ccfba044a0efe37ff951f5f14d25342f25a4c2f755f0",
            )),
            y: Fp(U256::from_be_hex(
                "121ddbec4b4befbb1ad1d11e09c61e726a8b3801c50db9889643f0eb3c59c1e9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2ef169fd03532de16aa36cf4f73bdf4056d4670f28d6f248a601d2d8bfd88e43",
            )),
            y: Fp(U256::from_be_hex(
                "2c35ab4f17a8269d0404f53d449ab64b3ae8d955c596b51c1231762e77f56cdf",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "13c117b6caa9669380e0658b8264d166e083f13c601ef1f47d103855830a0e1f",
            )),
            y: Fp(U256::from_be_hex(
                "0a6607c67cca3fbc4dc8cc83b4e8138c9d9fa130ae586d85b3bd6d4874873364",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2cb105f73e3fcdfa973b2a95ca323bc29d189beab87e89dd8845eacadc043f1b",
            )),
            y: Fp(U256::from_be_hex(
                "204b0199ef0100769c7c7585d31ea28c28723703699ec2a9d68ccb99eb89b3c4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0af77fb6ef605951c6daa26d305296b52c8ca728745b7050d3e4bf4d33228a38",
            )),
            y: Fp(U256::from_be_hex(
                "003f7df4c1b660a81e6c2a05c7620ea0c4b1a6e0bbe24f3b6677364f4276e1c9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1fc5942803e1653d925885abdc8bb83c3c42ee801b9f815c702127fbcd835576",
            )),
            y: Fp(U256::from_be_hex(
                "029e110e1bb48aea215b0d0e68c9807b37fdabc0ca21411b9f44f33930c9c88c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2fabb4f16e9a9a2acb8a34a61b244b1e06c57ec3e85c58fefcd661d523025ea6",
            )),
            y: Fp(U256::from_be_hex(
                "1c612a1b466a6d508777f05f2fe3fcea7701e33cb0414298ac98353ed7bba3a6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "01bb262d8ada4e8afd5b804753765d46e980f8e414825d463b62b2ca7ed9df1f",
            )),
            y: Fp(U256::from_be_hex(
                "2f39572265cc4735f18efa431bd5cb214749a20f7689038d4c3d7af996ffd613",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0c1f4a494562dc6ec30dd4efa598fe7ac38486ffd9230a385ea3d28662064c36",
            )),
            y: Fp(U256::from_be_hex(
                "1a208003c505d6ff41fa6eca5aaa0a93739a613cfa015999ca085173d870ec2f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "26090c34eca4326e6b0b9b42a0ee9095fd237d5e681c528197b54d34156eee13",
            )),
            y: Fp(U256::from_be_hex(
                "12c3d34ece889022225485799011ab5b3b2c7cb085c5b41bad643b1011456a1c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00b52dff621e77af0b2c66577ee5ec06802e8729903a9da1cd3afaefcf36fe85",
            )),
            y: Fp(U256::from_be_hex(
                "19bf6f124f3ab991dd190ad97c024d30ffb3ffcf8266a2d0e6312a723d3e0729",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0946e873dba0191b46f0137bf409af9dd14a1d5a40b46320015ecc8564ed39b3",
            )),
            y: Fp(U256::from_be_hex(
                "24af11b181c366f9ad7bcc9de17d3ba0447153cf6df4267f689106cbd69d4d99",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1ba24e2c192bd2e0e3efd77f0735baead203611ecb53fd3af243136693ce13fc",
            )),
            y: Fp(U256::from_be_hex(
                "049cba617021a628bb9ebac97ea8423db1e496bc33f307c70a45a97ea9dfefe1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "198f01c94ee41293357a50b61018014c7c1efadc71fa72b40c6e82cf6b6ca88b",
            )),
            y: Fp(U256::from_be_hex(
                "06cc353fce99149e84ba1d7052ae34d9a84f236b80f44d43c8ab9436036db272",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "22c25abc30cfac2ca6a93c7df5f9df9f6c40c1d77c4ddc017e9792c53acd082d",
            )),
            y: Fp(U256::from_be_hex(
                "2b581b30078dcd71966744a2365b06996563aa80795614ee774d11521bdbbc3a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "023224d3696ef9549f9acb8d720904b3e648c1ebe11b5a853148f7a4549c3c6a",
            )),
            y: Fp(U256::from_be_hex(
                "0a1ad1391ca0519a679f7f64305b92773cd041ce9a2e09c614442726bacff25f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1f362c498c450acacce57c23b1c273e187729a4921eb04f3fc4ec967b07f15c8",
            )),
            y: Fp(U256::from_be_hex(
                "06b0eabf61a62b441c0e01b9829ce1008a0cf6d1bad19c131a051d4a058497e0",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05d17b11e9e0ae91fb9141916a97cdd025527ffa9b109cd87e80ee50810c3019",
            )),
            y: Fp(U256::from_be_hex(
                "01906be275a9e3a25547bd188c24b759ff082f046a7cf47b0930c7661f471322",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1760ec5be91aaa2cef23f6fad5b979f6ff922cb90e0c891a2c0b4a0662a4ed41",
            )),
            y: Fp(U256::from_be_hex(
                "03a05cca0a6ebe7d5fbb5383e7badf090a37c606540f1dd19d640089128a0e7c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "271c190af3bc62498a0164b62997b1cdfdf1adc3da7fc00e6be305754a588476",
            )),
            y: Fp(U256::from_be_hex(
                "1c1b56ea310eb04ac506ce1d4f841592146b5cc108178a29601015baa2063a87",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "25e1758605b1fac2bac117fc865856b424812b7d9e7f27269e23ab819dff1cad",
            )),
            y: Fp(U256::from_be_hex(
                "11be527f0833458ee9bdf82c1dcb839f54b699cad7fcfbaf73c3d165d4f2fc4e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2ca99e457b4b032df61f983fe8ac390d4af90106d1bbfe71989b82a5750ee075",
            )),
            y: Fp(U256::from_be_hex(
                "12a52db26b6a4c7d9ba5e91b46d7544658ba266c66facbfc30f79aad1965aaf2",
            )),
            z: Fp::ONE,
        },
    ];

    const GRUMPKIN2: [ProjectivePoint<Grumpkin>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2bcdfd6d9652ae9a9d797243f068b5968567d4efcdebdda6978d2bc3a0861edc",
            )),
            y: Fp(U256::from_be_hex(
                "2c493e32ec0bbddd14de6f5c22594f9b8fa1ffe5c2d4dc90f753939517e4310a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0e353dee9e198df0f019105c3a147193b73ab06a9d1d825c2a478c727febaf73",
            )),
            y: Fp(U256::from_be_hex(
                "198c296dfaaa44f7d66b24aedfdfc7d17ecfba49e1de095ebdc5848c038577ac",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "295a0adc153e16490a771fae42099133018579fa4403367a0163931c5e1c7685",
            )),
            y: Fp(U256::from_be_hex(
                "142c0e73f9de8f6ada53fac88e8df4ee0fe4dbc05344503c64802c776a7a159e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "259cfb6d5cc31acb05b3c4c137e16a00d9183ef14e05520199610cb0f2bcb44f",
            )),
            y: Fp(U256::from_be_hex(
                "0204462439cd6538a5fa9c7880fc59b95a1846af735529ab339ee87ac773d3ec",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "26749dfc6775f93071d9fcecbf55fd729bb45855dcc73b24caf473ca28f253f9",
            )),
            y: Fp(U256::from_be_hex(
                "27a480343140048e32420832323b0ac4df5e2cc499b6a6eb02bd433234aa0b48",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04d3df42649ea65f1d987929e2364317487545932d8823bc1f560254237b9c55",
            )),
            y: Fp(U256::from_be_hex(
                "2df309f2d74c5c6afa6b2ef3b95b7904e226057019ae59edd3db8e0223fe83cb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "234e42cf6cfa9567e778574f7809ddbc05a097f1ac7868465327eb8b3f4f8e39",
            )),
            y: Fp(U256::from_be_hex(
                "0b0c16044cbc812d135570726c63eac14a111aaee148047de11064e739fd3b1a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2fde40702b8727b7177a4dad2fb36c809bf8ba8d75c314f3fadd35552f02bcc0",
            )),
            y: Fp(U256::from_be_hex(
                "1c7daeae1af95724216e8bb547365ada0a22d6e5c247fefc396eb00c5ca1c778",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "123ffaa57fc31f7dbd773a732b3545cc4e779b40c7436549c8114deef142b43f",
            )),
            y: Fp(U256::from_be_hex(
                "03fe9421951c694289c2d2bc02539f47d92aa51d9d0ea3abbb34d132a238e476",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1445436f5e5a26e081fe9e5612e1e9d5f981fc30852b37e41f7c0f93dc0b8223",
            )),
            y: Fp(U256::from_be_hex(
                "15b5b6736a6b198ba991937d3f96e7cf4ce64d391907391332f8612049dd01c7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "08dc7e16037854f487f7f4b01fddb9b5c1aab6c515fa8508dd22a7dd22110a97",
            )),
            y: Fp(U256::from_be_hex(
                "14a73207a0238f82add4455a47f19d443a97804e4883d6901e765600c3fc0745",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "26e859c62cb24c3fb0261e74eb689fb1d1de4548bdaff761df546f801e293128",
            )),
            y: Fp(U256::from_be_hex(
                "17eb97fa9690137773fb2c47fb4a2c74d8704517beab5b869b9733127608db37",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "045eb91941049256a3547c84850266fddf4ec4cd2a50d8c880514cf6394c31c2",
            )),
            y: Fp(U256::from_be_hex(
                "077e048b23f1e3a45952087449e9c723f314be75b17df403ed75dd3926b82a7a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "10b4ff11f5a21982b25b837c2543543c07704e4ad83f94d7aa87420d941d1b6e",
            )),
            y: Fp(U256::from_be_hex(
                "25a3d89a534b860a563b4246163387052217e54d6a4663d66b5472a033d8a0c1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0a522e46ccf82fbf1b362430938b699a04913919cf0c8fa8e9a19b691e61a0de",
            )),
            y: Fp(U256::from_be_hex(
                "19becf21ee6890b16f7bc63245581c57e86b603053619657f881d592d10657ae",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "25e0d82beb01875f6733568a7b8ff17082544c8b54df86a229c42bdeaa2e801c",
            )),
            y: Fp(U256::from_be_hex(
                "2afa15c2d18ddf020227149381d245aa9bc627f352b40f1e377733f525447a07",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0fd74c2fc377134240ca8a6695f131548313d8f38aaae4fb272fd3156c81cad9",
            )),
            y: Fp(U256::from_be_hex(
                "24885a9ccb2ae8d8a426c805b4af096089b6af71b103afc649edb09c3262892b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1c72558f951dabbb0782e246b40efb94c30cf3de089c1f99153706a1be5d4e9b",
            )),
            y: Fp(U256::from_be_hex(
                "3022ce25408c749d7b053bf523d0c44174054e06978cb95490ccc94dd18d3959",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "28e0d1154d786d7e5e08acea62aee6bf385c6c091b438a21b75ffc5b41a9f4ca",
            )),
            y: Fp(U256::from_be_hex(
                "15c2653333a9b3ebe8049423bde5f88a989edb239974c1daed4be8b0c4a8e062",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "23f7995c45e87f6967a6b48026d62469fb1aa8fa3f89793a3a0e7df8c6b61b06",
            )),
            y: Fp(U256::from_be_hex(
                "03584a9b435fc08215eb39388d77330a2ece5f407b3cafd067d011e81452458c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "275128e8dd7e929ef334ad6e4b485981c74ba672290356b2ed5478aa073158ab",
            )),
            y: Fp(U256::from_be_hex(
                "25e1c43ba71a30f6c752172675d22046ee9620ae1b714ac9bd7b143a53ba1f8a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1f53b697b7b1b5a58c708598dea6eaa63f808a159fdd6abdb816ced5a8ab0898",
            )),
            y: Fp(U256::from_be_hex(
                "1c94aaedbf48a13ae793030a8ff1a8b7b635a8d67636ba236050e7b647201106",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "254d589b6a9ae2861e8bda66c277f7a6e912a6321c00cddf252aff0c9df9bd7e",
            )),
            y: Fp(U256::from_be_hex(
                "0e7e299e76be6f5ed3a905b8c260aeed118ce23ee052279e4498bf90d81f6a17",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2043347c6d627f4b3c5ec85ee951ea4ce8039379c7105054e8ad3895c04b7da9",
            )),
            y: Fp(U256::from_be_hex(
                "151d326ca3d4391ae24d31e9e5d4146bbbea243cfd19a3e3b3040e70c93c2dab",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0aad2152003135e2a12ec331030ae977f4487c0ba608881c37c89c347cedec77",
            )),
            y: Fp(U256::from_be_hex(
                "1565b8616507b25cbb64685384f698ab42902a873da9cc7c2282989f0d445116",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "17e2dc3ba9cd5818cb1233bfb87a15e3a0fb8d5c86efe540d078f77e03fd0787",
            )),
            y: Fp(U256::from_be_hex(
                "026715da434d38f673c3c86754461a67c58396b88d340d40a8396655facdaebb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2812340e4005eb494bf50b12cc6a77c15426beebdbb5e3046d28017ebec9dd80",
            )),
            y: Fp(U256::from_be_hex(
                "2984d6a7999c025fd261f894fc3d93142e4fc3af47c8a0b25c704ce6a1f58a19",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "027ab9782d43c95948c87c9ff4ecc242539cc0915af5ed5345957862ef5bdae7",
            )),
            y: Fp(U256::from_be_hex(
                "218eef1bf15b7c00198d48b9144372cb8ed2d6fb375a2fb179332211dfe92c9e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "093e57da6a18750ee972e16d85abd450dc8a688e611f8be227d19b0618c80f6d",
            )),
            y: Fp(U256::from_be_hex(
                "182f20ee890467db36374f76d1629317eacace345173becba8dcdc051f9b4f30",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2287882fbb67462ba3a919479c0434f841756850599b85f49e7d0b8b1988f37d",
            )),
            y: Fp(U256::from_be_hex(
                "110720a6ccb4be98d1cda52df6af8f55d084052637175eb7a5707db8df9c50b1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0070e0dfd76d3de0b4d9adeb05b32db7a3622c808aa2b8fc795565cf97b3f0b8",
            )),
            y: Fp(U256::from_be_hex(
                "2c5755b2f264b6a61698d7a2809632ee524d5c77da0ef6219646ee800d1dc55a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2f693be0e246488914fb107df1e733f703a4c0e0a1985a1eadcb53233ca93149",
            )),
            y: Fp(U256::from_be_hex(
                "04ed82c25765abd821321e3bf7639175553eeaeff2f780ab86ae5113e70224b9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0371d0b20f312d1b2c54d532f87dee1bcd403aef2f8229ec5dba3c275d42f6c0",
            )),
            y: Fp(U256::from_be_hex(
                "28b2d031e3fa32cdb7db2dd9b4940a964bf47ae6453b03e6884b733f049079f2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "288f62d29793db0f1185988af12f2e0013f81b93963cb5aedf8737784061ba1f",
            )),
            y: Fp(U256::from_be_hex(
                "20be58bf6b1b8c1461ebfaf6f4fb2ba5cd6e31d388ab655d91894a50d465cc0d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "186a933d19f1c64a0dad6526565b45a5099748b053fd9d31de4bf184d4b13194",
            )),
            y: Fp(U256::from_be_hex(
                "0b4d3c671bcb324283a725919604fc6426400d68fd9d67840320417a1d98944b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0fd7b4dbac981f2888544eef4fa8d6a4bc27ec1fee9ab1a7760e252fed86e78d",
            )),
            y: Fp(U256::from_be_hex(
                "2a4dc596b0e03627c83e5539a8c23f10e34e0f41e05cbb9ded1008757d65742e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1d894429e675bfe7e8f16fa5979fe6e460a9de6178d4b6f85663c1c8ec0f4815",
            )),
            y: Fp(U256::from_be_hex(
                "1a421f29010db8508476a2ce741b944aaf32829a02b436e4412118e3e392f731",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0096173ca75440a60570fb3a9e6ecc198e84058d8a323324e050f3514fb80138",
            )),
            y: Fp(U256::from_be_hex(
                "3025fbb9b4971fd0cafd62f19b622d4c2d99e3281b5c82d13833b493906ab47d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2347566bbcd31f89972050e5bcb632c3ce5cd23c397f9c78823e257a9ba732b4",
            )),
            y: Fp(U256::from_be_hex(
                "2b3fe95869a63c5242b00abc0bbfd046ac293df9911f43ad5c3629f3f59539b0",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "29efd6033de4a53121908b7495f541f7c29ac728da69a04ef091c55b39ecc0f2",
            )),
            y: Fp(U256::from_be_hex(
                "02787995f196c60db717f77b0c11b85e9da25534767cf2b3884037e455364e47",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0385916289a9e1030b2f4e3bd55229b5a59bcce6bbea7a5731857c3cd2bc7c6a",
            )),
            y: Fp(U256::from_be_hex(
                "20b8b69fc294ebeea00079364387c56538f8ee8de39b40f442111b4a59157a28",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1dcaecdea54bd45ba11289e5b1e86988db86d735e6977b85f205de9acd896de5",
            )),
            y: Fp(U256::from_be_hex(
                "1395adeb9fbee7da8ac1a0772c93d3a3ae03c5bd06b3ba25f0635f36c8f0f991",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1e57aa2d7cebce6200f3222cc2db02cafebca42e5b3ca9dd8892e4cbc0a7b0a0",
            )),
            y: Fp(U256::from_be_hex(
                "01ecd78fc29a9b77455aea825398ed9f567df258cad2d99742c3ef08ea78abd1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "20af6a604d9cccde762e904faeab29c78693e8714b8306ecd5790d402a391705",
            )),
            y: Fp(U256::from_be_hex(
                "22bf6968a8f3fa2c71661d91940a487feb6426d47e979668180c6475ddca66cc",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2db6a4612a0b096d300c7a568d1eed273bc0c6d6a044da43e0ea9927cf6da55b",
            )),
            y: Fp(U256::from_be_hex(
                "05857dea2c8a43dc99d86f7ff10fa8495144594c1c839a86dcffc40abb6c9a39",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "097f082c100d0648ad6bb39c2745e91c1855bde95dfbe8ef4513f16b346bf311",
            )),
            y: Fp(U256::from_be_hex(
                "13322cde9407bddefc1197d0ea63e3ce1df6dfc3306c62cf69d1d641d4fca336",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0687bf9009727a042887eeb716fb0d71718011b50e2e3fe313ba9c78826e139d",
            )),
            y: Fp(U256::from_be_hex(
                "04f66077f0f25df904a39a093209ba6e014a45e5fc5cdd7e298cbce984f676b0",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02338ea1d43401eeed61249e7a80affb084906faf1df3ca0c0bdb0e2db8453c8",
            )),
            y: Fp(U256::from_be_hex(
                "04f3f094457dce1afe7b3cb28b1c74f14822864760815217475e9a632ec04d7c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "23d1b1d26866e78f211245de095370686169776897101354c7a0b2b3800f5ac2",
            )),
            y: Fp(U256::from_be_hex(
                "13d6129c9f809d1b9e18835a416e1ebf48da065a1029167d75194cd838a763e2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "23b2d503dd823b5e5854eca9658ddfb65c4895a38c3d70e24019795892265c60",
            )),
            y: Fp(U256::from_be_hex(
                "2fd03220259e55ec73713e052b221c2eac416212c1c7b2c45db6dbf744f0af23",
            )),
            z: Fp::ONE,
        },
    ];

    const GRUMPKINADD: [ProjectivePoint<Grumpkin>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "115f00758ca9b808be6f10bb746a355d3e49738aed924cb14dd44926106c18b6",
            )),
            y: Fp(U256::from_be_hex(
                "09a9b4a9b5f0091c1aa1730b2c65542f54f9387ffb9fe0afd11d5eff9ed65373",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1f504ad6edb2729490d6d624147d1ee60064b4d9d340996fefd08d7c75586fd6",
            )),
            y: Fp(U256::from_be_hex(
                "001bc1b9ab3657c0f6a42e8e09522b43e465fc8b583accda891b986bc6d8f40a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0efbe2d0936c773ef9f25a2e124cab3d915b44fe5af9d864d04ccb5f9366652e",
            )),
            y: Fp(U256::from_be_hex(
                "282487f45d27ea922a9d74524869ca421719692fe62ff344ed58cb08fbcf8f32",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "137a1708f32399bd2ec6bb7db9e1e24e80359a91ea0cdb045d24e52c0008953c",
            )),
            y: Fp(U256::from_be_hex(
                "1c49282bd00dee459d8cc4dc245c9dd314d56feb72a523d6646b4fa7a9f9304a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1e6e5d243d50aaa14700ab5994ddb037ddea3db7ac13eeab83326260d00f2906",
            )),
            y: Fp(U256::from_be_hex(
                "11b229501303963bc8db680d74265868630b1b775b373d10c16c72a62cd5d44a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0ba7029fddc88a559b2de36c400d7e481739bac6db63ea7b14354a33d4c24eaa",
            )),
            y: Fp(U256::from_be_hex(
                "1e03444a56d02257f41e619950a16fcd924ef7991a2a33c6b34baee8728c2fb5",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "168b50fc6824de4b816d78b0d199f52280c912b17be034060f76c5668938c1e6",
            )),
            y: Fp(U256::from_be_hex(
                "0d954414642868f8b30424de8d1c7ff8a1ab955bca9e993a955b10df76764790",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0e73a3515f8b82ffa8415cc79eb0b772df5106412b04e25b1dd293c3953cd896",
            )),
            y: Fp(U256::from_be_hex(
                "14bc2d421527f290a78da5f79a2f4f0c61881f6f0962808fdf421b83aa07807b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1ac27b514dea04cfc7a972af7b729ddc61b8aa48954c8bd1708e50945a42b116",
            )),
            y: Fp(U256::from_be_hex(
                "0c0b681f10e9fd0463f6622737ea4f366e19d3a9be92dc21642f86ee97913d92",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "24228b1ecc5c7780cb70454382d11d6faa3418f508f94eda62c13eb425e0c58b",
            )),
            y: Fp(U256::from_be_hex(
                "01e4b5334c3dd404d57a93dbddb1ed6669de048c4f440d6e4cc6937995a59fe9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0f8f405abe1438cd0bc51b9e829f785c5eeeb2fd9f8d051bbc1e0da23ef428b5",
            )),
            y: Fp(U256::from_be_hex(
                "2ae9ed3ec303a00481041272832aa936215b360d9342942660172f1caf18a501",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2b7ad3fe5aaa7241b6771675946ed24f7e32cc391e3e91ff958348d764e28ee6",
            )),
            y: Fp(U256::from_be_hex(
                "126151a53532d3200e37c1562d6dca0b2de2624505bc6d4574ca81f69d4aa0dd",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "21f1bd593421216011c91f1bf59cc642239f0f3356b8436ea0b92a9aef382165",
            )),
            y: Fp(U256::from_be_hex(
                "18bf22ba7219c666529fc36e7f9c65d66359707fb1f54261fd83698b3c2a8f49",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0e62af73c87556291fb6b4e6ee6ecf4a0b807602d2e2759ecb74ab49a4ff7ee5",
            )),
            y: Fp(U256::from_be_hex(
                "11931486f0f71ee41bfeeffd63291cdf6d1da6ba5d2481a96219cf8124cda7c1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "23c384e3468f491cf087154f6d46ecdcc0229d31ee8e52312cd68716d9083308",
            )),
            y: Fp(U256::from_be_hex(
                "09f607d690a16a034a0a50213361fe7a255b755126592fb8353b56b7bc082ee8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "18fe4f8b78484797784ec2575fb26be722054e4cda6afc8e14b6181f3b1978d2",
            )),
            y: Fp(U256::from_be_hex(
                "3028c1edb64c87a9cf998080a682a5aaec16b3da7892f52189f3646ab5a6281e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1a9888f8c0be79426fcda08fb0f382ab6bda57152750b41b795c349f12c8c955",
            )),
            y: Fp(U256::from_be_hex(
                "24b7324a291eeb6c29e3c0548cea96e740d1ad726d4e1d7aa987aceb75189ea9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "21a22f316ba27ec55e978c9c15f37e5a9bad67abe3fe6e914f311af7339adc1d",
            )),
            y: Fp(U256::from_be_hex(
                "1703a379a47695cd58e96439529f0e1a38e7a319b800aac9615de159f96fb891",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "10d41fa265b81546e0a7678d70bac44c6a9c1489c458d5581e275c3061f1f3e9",
            )),
            y: Fp(U256::from_be_hex(
                "01a4d10ad60f2a222adddc555ee5e0c7c4de6fa548516a8135333efc79ff8272",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2766f02150b5d980dd7240b41a36b727801f7d7e8dbddc598b0dfa22e8beec3c",
            )),
            y: Fp(U256::from_be_hex(
                "1013f07a142fd621aee83fc86b60a44ccda3b3c3894fc57b0cd730439dbd782b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "038804825e1c51c73a396f579a6610931750694b4a29a132e1f81aae97ee8fd8",
            )),
            y: Fp(U256::from_be_hex(
                "03d90557d52475be1e243fbc1672c2c7fac1114344699ed88584089473b850de",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "006c47d3af3603434b49ca9f81cca7d8076e07460d5725c57db72724d1fc98d4",
            )),
            y: Fp(U256::from_be_hex(
                "0de389389551c2d91b4076cca4d6eb752b5a55ea25c6bfacced54b2c3536f267",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "284e1296ec8005aa3101cdfe519210b9240fc0461d3e713b61f8e8c2a90b2d54",
            )),
            y: Fp(U256::from_be_hex(
                "040b4527afc6b59ac9747f136e883fe4f2ac67c1794001e74e196045ddaaba59",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "09b5e1bdb3dfa3d1335634396bfd19c213227dbefe5edbd839b983358a69f963",
            )),
            y: Fp(U256::from_be_hex(
                "1241bb7ce2c6e93dc7ad09df7eb0a5c2c0c4a9c9f5c869202524719ec8dd3f2c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0f3aa4763b1fa82f210ec9439dcd08b27500fee4bf05de534e6d4a258d8aca97",
            )),
            y: Fp(U256::from_be_hex(
                "111f99fe42d0373fdee69849e2a1dcb2f23ca3831eb4a37d51f10a9e2046106c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "296eed0c030364f8b87808a0440c0d694934a7e6e3f155f20f86cc9b33fed241",
            )),
            y: Fp(U256::from_be_hex(
                "1e83f830e3092fdcbc0eaf2986f97314a98168ae4af6733757114b932b150915",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "01477612b3ca0eec5a64bc4af6e099b5504bae8baaaa9c0c4fe5a8318ec96de8",
            )),
            y: Fp(U256::from_be_hex(
                "10a95cece9ce6d4553268ea97c0bc73d2447cf5dd39e4d26fa8d03edf96d53fd",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "24c7f7de9e7123dc63625951d5b6f20bac1dd86d819b52c9ca89f655c4eb2e7d",
            )),
            y: Fp(U256::from_be_hex(
                "0a1435223ea32711e54d27e1aa2be5a006fd6a7634c38213bbf97a990ba470aa",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2353d5e59174986ab19d402800b89d11987b03046e39d35589ac96bbe5d9ee58",
            )),
            y: Fp(U256::from_be_hex(
                "29119f2e365f9b87a3f213ac2c79f11d4bc1932f95f72eb257c49e654b9106f9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2c2fac06b7e25ed41f60ca9f28d54156932711587607b9be3df72975a01bc5ac",
            )),
            y: Fp(U256::from_be_hex(
                "1510f8933e271bf601392ed20ce72f0f78e4dd6aa66afe5f6c7e5d25c106600e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0ce1eee3275c03a59ea79e4f25bc9515c66220465a38946937db509bfcf6af9f",
            )),
            y: Fp(U256::from_be_hex(
                "032a53584d023a521e0e328c038e3dccc3b09737b7b113c947f37da3d9e10fe3",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "177b93a0729fd8451febe0e691c6c4013252e4c511d61addfd40560cd00cb0b2",
            )),
            y: Fp(U256::from_be_hex(
                "0a246483c04687630cdd2cc694e97def42ac1bfb1f284ed36a1d290f600d94a1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1f7741bddf3a2daa8329b0735d4ab2c182cc14630d48d5264c364007f561848b",
            )),
            y: Fp(U256::from_be_hex(
                "0fae4cf6d7fc0ce9482ee44d4ae5ef91e6679e0bf88a0d12f949ea0e433b07f7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2a850380e2e5d96273c0da72b7625cbe48ddbc04ddd96f39038dd3d797889599",
            )),
            y: Fp(U256::from_be_hex(
                "2af184d047d5b16ab88ae5e58e7669e29d5d6871e78043838a37377ff66e83f2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0d58d3412cc4106b647e521bf2bf724c8cdf6a63b2279305ff32a559a4c3af09",
            )),
            y: Fp(U256::from_be_hex(
                "2152efddb89ddc05f136167f4e064d5f3e68785b0034df8e0b6698ba4f87b450",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1176eef9a23cc03e83bc87d2e342a423356dd85d0ad072e0b83e9744ca045d9c",
            )),
            y: Fp(U256::from_be_hex(
                "0f08b9ad2577ef7674cbee48c54d185bc6f4afc164f6ed86e0cc17498e7716c7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0ef638868af0e428e2010844b6402d5be36fe7764a47a5f8c33e3dd3edfaff83",
            )),
            y: Fp(U256::from_be_hex(
                "0d53c747b08803ca4e1386693319b625f9b873a19c231922acbf07266c4a065e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1ddf0e7c26f70163d19c0aaba1f6846d44e9349268f159a9bdd15f4fa7122c33",
            )),
            y: Fp(U256::from_be_hex(
                "22e88ec32ef32f7bc213bffe9a451ef2967bc8ab3d67c1939d7b5b4b7cadf9cb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05e7c350ccf178a8003b17d72f3553352f95c34bb0451c35975c051c3e526561",
            )),
            y: Fp(U256::from_be_hex(
                "1788eba1731e24024a2813d9d0e926f34e1da8720e9b6b082c299997d2ad9f1b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2244087c1d3926ce2a8c2b632ee75cf5a49f5ab6f5b6af1dc4fc7e6add982d94",
            )),
            y: Fp(U256::from_be_hex(
                "1de03670d34d53fb9fbcc954ad8176b21854ecf6779ac5716737519a119afa6e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0c5926969639f2d96a4cbd5d6c11c13da1c94f860c0dff0c7669f8921c89840e",
            )),
            y: Fp(U256::from_be_hex(
                "21a73caa9596f6eeea6115673715798ebbd3cd1ea68c770f84bbfbd052fbd20a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "13f0bbed914b17ef1f5e95cb4ddd18dc18ee6f9fc06a1a6de8dd1d3ccc7e6f94",
            )),
            y: Fp(U256::from_be_hex(
                "1c95d43ab3f48e4732f89962205947b72c4650acbcc224f6fe4b4f8247295447",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2db10467aebdc1d1c89688efeb996c78f5cf1f30128807d83ed0b754cca89b65",
            )),
            y: Fp(U256::from_be_hex(
                "0a3160e4418fa2fc4a293b66f83cbd4a9de9b3b46a13156829c1469f63451def",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0dc196102bf19fbcd3ec8bfbe9d73335fb0771659b19fc0414e02d7ef0779d64",
            )),
            y: Fp(U256::from_be_hex(
                "0d2c92edc3629d4564b6bf491e89b71f37e0ca66fca4d961e456045482736dd4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "025a71e04aa8836c785a53a39c0e0fe002d75189898c19a4f0d1f76595b4e2e0",
            )),
            y: Fp(U256::from_be_hex(
                "2eaec479f45a0d89eeb262f4c9fc334c5d3a6410a5458822bc457fab4a2867a9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "09c023fa2d268b0c8ab4ff24268ab5ced6e43e82560e57dddb2396f0a8d3ab73",
            )),
            y: Fp(U256::from_be_hex(
                "02f682a6431e6f07465ecf84863c0c0ec073355a78e65f559f49fd550e33267b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0fb87de2d9374ebb47f8ec5e268671680afb971e7db41c04e3b54cb47890dd99",
            )),
            y: Fp(U256::from_be_hex(
                "08d56c0fb8412f71a2a4a43b5f17c0386b5e97bd4be825911f5c6b710a08b72b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0e4552efbf9aa9dc24dbc40a6099f67cc2be29ae602f686592a79e8b0bc5a7e2",
            )),
            y: Fp(U256::from_be_hex(
                "16343c2453f7cc19c9f233f0a01ceb42133820fc0d881004295e9e758346b4b3",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "089ab747e5e521ac8c7b66d3ff25e9c483720b29d052eac979e64c2d2cd869d2",
            )),
            y: Fp(U256::from_be_hex(
                "0d7b7955ac81071065367589ae1ce560c83687b0479f207f039cb13dfe92f4e5",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2bad15d722b372e4b246a24bb185f55b4a325b657855ecf33e70fe66631e711f",
            )),
            y: Fp(U256::from_be_hex(
                "1a5fcfff95f2393163e172e94df6fc822c47882d1af9a3527fb0bc57e876a584",
            )),
            z: Fp::ONE,
        },
    ];

    pub const GRUMPKIN_SCAL: [Scalar; 50] = [
        Scalar(U256::from_be_hex(
            "1454ec8326aad71ed14c48a522867193cce4d5eca3f1b3d03cf4e5b87806e2ea",
        )),
        Scalar(U256::from_be_hex(
            "281c06bc397daaef223dadaa140874cb51d5ec715c81fb9a68cc7ebe536271b1",
        )),
        Scalar(U256::from_be_hex(
            "05f4ea688f8bcfb68218d1109400961a610f361ed1f476ae8067a42772978d96",
        )),
        Scalar(U256::from_be_hex(
            "1f40007d468571c2b8194fa37199a3dc77483550df97001b9b93d689b2c43f95",
        )),
        Scalar(U256::from_be_hex(
            "2336182bf6e850393f9398a3137cc032a9b2716765ba143c55b400f38ed7d9e7",
        )),
        Scalar(U256::from_be_hex(
            "2ae0bd5c0255b2bff98a6b535b015618926b353cf68c3ed4991446de470c5e6c",
        )),
        Scalar(U256::from_be_hex(
            "254ca71bebd1afc04cd2576053c8affbacfc8601bdaa5fc9d593a2746f10c7d6",
        )),
        Scalar(U256::from_be_hex(
            "2d5f788c5a95dcb6da69def29b586c8a32811e6c2b629ecd0bc001f8e57ede34",
        )),
        Scalar(U256::from_be_hex(
            "146fd1a6820203b734245e0282241e760ab21b71c7443e0d375c7788f442da77",
        )),
        Scalar(U256::from_be_hex(
            "139d2fb4a7b98731f0b7f8f2aea52e59a8a2735a5c45b020c20b8facc2ef6fde",
        )),
        Scalar(U256::from_be_hex(
            "000a38b63db4babff9926e0d2466834926b794ffe88a9d1fc17ee038b3fe9594",
        )),
        Scalar(U256::from_be_hex(
            "2f600c2cacc60a30586b15e1c18d03fd33094426cea963937e41e649703b810e",
        )),
        Scalar(U256::from_be_hex(
            "18cbabbbd520e8ec279f2a585187d3aa4396bb0762ddd7556a01ff4fa7fc9f6f",
        )),
        Scalar(U256::from_be_hex(
            "249d7c384c3cd5ee17cc0d85c45c17380f0845807046ca764504a35c1bfc7f29",
        )),
        Scalar(U256::from_be_hex(
            "14003b4a4a6c94b0cac7c48dc90a79ea3203a459f2a54497cce9c5f46e11dfbc",
        )),
        Scalar(U256::from_be_hex(
            "0b5e7efb88e91a762353c9c0974d708cd4e313e262b6518059962af91a45c828",
        )),
        Scalar(U256::from_be_hex(
            "3019b393f15823858602c3224ab163ce1edb7e7abb35e3485143dda6dc2577c5",
        )),
        Scalar(U256::from_be_hex(
            "1dac2268d8aff7577e6dc7e4366740a827f72cef4b380051503b72487ace5dc9",
        )),
        Scalar(U256::from_be_hex(
            "1140c9ccf7598d9915ec6a2a29199c98022a8c23cafd376e7500c8f63ca19067",
        )),
        Scalar(U256::from_be_hex(
            "06476f76a62430077b9cdac2d03540e80ead5368f859d62ed578e83d2af080fa",
        )),
        Scalar(U256::from_be_hex(
            "0fe3fa82d6b2a7ec4a56f154706523abb950315b3234330bdc2b000d812861f3",
        )),
        Scalar(U256::from_be_hex(
            "171c8473612025ef3f35a0f2745403857d4efc41a1e0f999c00cbf890fc5aa44",
        )),
        Scalar(U256::from_be_hex(
            "264fa760b72bbead47afaa950544b14d2c4bd866176411ae1d8e93ad865dcf65",
        )),
        Scalar(U256::from_be_hex(
            "19eef2fb984bf0db663e06ecee9a6e0e06b9c521b15e56b75dfbcbcef1040a6e",
        )),
        Scalar(U256::from_be_hex(
            "01bf406b0fb3bff14785bb6f90165fc5c73a381fe86f3cdd701ff9628437e84e",
        )),
        Scalar(U256::from_be_hex(
            "239e9020662517cc27cd7935384a1beb6077b5ab0daadeba70ecf13b434a7d73",
        )),
        Scalar(U256::from_be_hex(
            "078f42f38ef0c915ff7912df977c7cf2c29515d22d4d9ddfd815942971c52ca4",
        )),
        Scalar(U256::from_be_hex(
            "08c4d40b852ea1b14da076d22ea618a036b0913f866274d58fcf2e7b80f2c064",
        )),
        Scalar(U256::from_be_hex(
            "2a5b588af9855005530cd451771ddff12343089f4c42cb04949837c19b76a57c",
        )),
        Scalar(U256::from_be_hex(
            "10731b5668eb6af18a0ced6fa690ded682e63798df676b6b03654c3cb1585aa1",
        )),
        Scalar(U256::from_be_hex(
            "19a1a090280ba238abdfbef35c1ae59857923a4f22b92b1ec3089cdc5a8d6213",
        )),
        Scalar(U256::from_be_hex(
            "1904010b5fa23e503847c22c1ac150ee0b593f66f42649e71c6b6b705a89896f",
        )),
        Scalar(U256::from_be_hex(
            "18cfbd2180cb5b729c4933495c87b2ca2cefdf7b989b4d406a62b5cb7ed5c28b",
        )),
        Scalar(U256::from_be_hex(
            "2362f76d0fcdd16eec799ae02006985fe1e791bd9aea6548a94988e5c7fad4a5",
        )),
        Scalar(U256::from_be_hex(
            "057417e662bd43a754d85936baa3b70427c7dad1611e3e23dc76adf64c3da07c",
        )),
        Scalar(U256::from_be_hex(
            "06154f75daaf76e9e135b7dbfa5c835f70ce02a2701108ce285a5ff97d7d618f",
        )),
        Scalar(U256::from_be_hex(
            "0901ebd0f1cc70e6f008664850ad49fe135cce5a3285bc2b6c4983907e85177f",
        )),
        Scalar(U256::from_be_hex(
            "11d8b795c22241f9b59c6fdd8d7b2bc3715d98f016181c7f457fe0da3099bedc",
        )),
        Scalar(U256::from_be_hex(
            "0cb6cb4eb834b1e6ffdedaf90b2a90b7e57b3d6e1f36c9a7c60a42fe3050c5c9",
        )),
        Scalar(U256::from_be_hex(
            "1593b7b70cea84b9094c70f603aa0ee6163888f1d10ad00953e4a58fa490b85e",
        )),
        Scalar(U256::from_be_hex(
            "171c8cb2e43c681abd8b18730ce3140f32cf52dcd73be39dabb9c35fef0972e7",
        )),
        Scalar(U256::from_be_hex(
            "29bf76195bf5b70d4bee1f2ff6ecde73d83547e24b5b1593195d629026c4d2fa",
        )),
        Scalar(U256::from_be_hex(
            "2f1a19649afc3c85fc89ab60cf61a148abc74913b310d5b854705b39dfc3d9de",
        )),
        Scalar(U256::from_be_hex(
            "19a43f58e5f20c10fcd7f41ec558656f7a0fa4367112a40dd77a1766a225999b",
        )),
        Scalar(U256::from_be_hex(
            "2b67fd2f52b075f73e80b9cfa31ed1e1e5aa49aa583497f16ba4317528a18164",
        )),
        Scalar(U256::from_be_hex(
            "2b5d13816e9cda32628b85fe4b3d073d2cc6b4b4b95d8c3466697d857806a5e7",
        )),
        Scalar(U256::from_be_hex(
            "1f14645f3986a99ede00248539999297ff7b5886e8f5593b4e4c4eb4883fe3c6",
        )),
        Scalar(U256::from_be_hex(
            "1fe1fd78fa0571531cf9ca5c528f71c47af4a36715fabc33a652cbd4ebefeb3e",
        )),
        Scalar(U256::from_be_hex(
            "0fcf171b4456667ac5a97fd12d89726d956fbff7d5bf0cecd7ef11803c64d0e5",
        )),
        Scalar(U256::from_be_hex(
            "1a33e11c79a232d57eab2e4b6e3e3ead7ddecb400f977a11db9367c538862fbe",
        )),
    ];


    pub const GRUMPKIN_MUL: [ProjectivePoint<Grumpkin>; 50] =[ProjectivePoint{x: Fp(U256::from_be_hex("0ccdc3ebe0b3b93831c1160e5e25a8b8fd53dbd189fb509bc56c25bdc8d821af")), y: Fp(U256::from_be_hex("0c4271602797355849d34080a7bab8ffd333f06157a29952b951e1cca227c8df")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("1b55ce251fe0c2c87c610aa85be776cd35321fbb84811f2e93bb3cb8363a9f12")), y: Fp(U256::from_be_hex("262e3064bbff01b3fea0fe242873b935f2d8629c69be6f8ea60a443ed34bcca6")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("03b9d73d777138958ad4d41601e53448762f5ceb2f7bc0fa48aedc6d050fbb7e")), y: Fp(U256::from_be_hex("243f38dea6dc2def0d046e0fe570683a46e20c532b29d0e1ff421385f6d41eb5")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("037cdd3ee718b6086f65f1a923b72c8b856744bee55942619e1cbb934f3084d1")), y: Fp(U256::from_be_hex("12b5323f8b292a85ecb461681ffafe0482e78c06ec1c49b452e41cbf97d5ee5a")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("2f22e656c9844e6a1fdcc229159b446e720cd22fedaeeda1c5a9c25318987001")), y: Fp(U256::from_be_hex("1ff149260ec48dbf96f7dde6b6ed438a06880e104990f5d4ab6097a8afd41348")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("295c67140fba05b77d27e974baf79bf94c28a9aa7409b858159310902cf58189")), y: Fp(U256::from_be_hex("01f3f02bb4bcac2fa9aa6d0c80a9bc68580ba764344190863f1bf60f6a89bf1f")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("2015b982e5ee1a56e070d3448289e420223c158c2fc359fdf72ed82993af8cf0")), y: Fp(U256::from_be_hex("12f075cc359b202fe52fa02cf3de0ea8f3efc21b8335b72d040bc3cbe7d8c11e")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("2a23c38f36ba3909a7da4ff80fd90049a27991ac456250a3f00e873dbf777dee")), y: Fp(U256::from_be_hex("246cd8b1338f8eb305620c9d56017a7c7e0a48601bf640b6692bc8712d27e984")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("077599039286d6099c2f369d619fd55639b80cf988f34bb447e35c8f150f6621")), y: Fp(U256::from_be_hex("297554d9d50fc40453760a6661d1fe64390377f5581bb6a1fba439f23ec360fe")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("161e4785a900683edea6fe2ef357591bd9c68084135f4338573367f3078ce8b8")), y: Fp(U256::from_be_hex("22c874c8cca3df4837e0e1569afe02943a6b1ed5b2df1aedb67ae162906e188a")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("1cd61133eb65fc9ca393ac3ca521d545fbe97e0a4c27f310f0f8ca722654b195")), y: Fp(U256::from_be_hex("1eb7f5536ecdbc35808fc469f293ac535043efb3139e497298247b53de22c3f2")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("0edebd7217d18b7f93f01e243d9126bd6cc3a1cadc2dd83339c159af856b3ea7")), y: Fp(U256::from_be_hex("057987200bc6cd8cbbf3957c56747896b9930df6135908cc00b74c14fbc6fff2")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("1a84fc410d52777e405eccba94cda395cab43a13a4da98e1b09ec9a4c5347a18")), y: Fp(U256::from_be_hex("2113a35b611baa79c0490997e20acf97f46aca4e585374e86d0e8403933d8a21")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("1336c4d24ba9307f1caa291505d57479a63688cebf7126444be9b4e0f0681cf2")), y: Fp(U256::from_be_hex("0efc4713cc015d2256c83c9044dd34e9f542b48fa91e063b029f49ccb78b3256")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("12953914ab4285b33674dc6fce37b6dc633ad12d0e58f7d52ca35e995fa5f022")), y: Fp(U256::from_be_hex("0091c82b3908ab3358fc02129e96fa88df3cf4582bbf4f02c86cd1b56c5fe3b4")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("0421b9e12a550df8c0dee4b2bbdaa669d8ba6d1069ca59ac29195b3a13da269b")), y: Fp(U256::from_be_hex("24805fa56ab8209be55b4491464240035b99b428b09296e07e54dc84687ce8f1")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("1506a04d613e3cb1b9e77063e6f4c7ca6357e8c3757f257a80ec958d02a3a4b7")), y: Fp(U256::from_be_hex("09bd168290646867144cc115661845062eeeaf44753198b0ed6922e20f042f5d")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("26deed718eec738b39c95240a230b18d488dc899c0e18a61a5e38b2cfb524c44")), y: Fp(U256::from_be_hex("07008846e49f0cf4cc3f96bc418f26dfc473dd65920d373cb1f3dd90056c1a66")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("124baedceab41c041c684b14bf71ca172e0b308e9d0f1ea3295d2b3983460a4e")), y: Fp(U256::from_be_hex("06e2d020126f67fdc0cc92a88ac97ab21d7a9791bc931be8e88e76ad2bd0e043")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("0a2ab18b38a93c84587b0e9e447d9a2d65f4767b8569cfd65a5238524bb98287")), y: Fp(U256::from_be_hex("22a77fa5d153a2c0893d6822f60c71c9c6192f16c36002c4606eacf660116257")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("2cde9d12219c690683d472d4be4c456a2edc0687d4a0b1e2a0c3b5050ff345b8")), y: Fp(U256::from_be_hex("2f2116de2a3a8ef74c94a55c1a53598e940e2237b6240a81f780d2a57d91bb04")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("0f80930c73c777933398193e707a9337087329f17eeeb62dd3d0877de81ee226")), y: Fp(U256::from_be_hex("117021593f62d42a5e93b9ff52babaf7ff0ce4301c0a3c3e54bb0bf9135613b7")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("29645807fccb0b7485451fc958663f886ee65f270b4aa5a135d6e77018ee38a3")), y: Fp(U256::from_be_hex("2743619455a7595ca730ae247159a44c391b566558e6026c469075610052eaea")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("01db4195d1fe32a78ef9a21d97991e9f0df9d5c4dfd935104549bc4f1ce6ccc6")), y: Fp(U256::from_be_hex("108f8e5e05e5e3c9b642e5a5daac4cd5f72fba65c1cdbc8b3de58cf617aec56e")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("0d8fa68cf3d688566749f8ecdf4bddf5c5e5454cc7c404c72f0a04d0d969f256")), y: Fp(U256::from_be_hex("251897f648927a2b95d7de11d36a62d0cd5d8b9efc539378f6fd1c37f3d91f12")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("1c40a7d3fa94b8ee0e86f7103e62d93d9333a828ed7202fa0278d0bd5f545dfc")), y: Fp(U256::from_be_hex("2aef564d425c631d2180865d43991f487c55c7ddeb9f71f143d02130c292bcc7")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("2e26e5a5b66a4e528a1a3dc9790f454dcc2cb10afdb2686055d0100c36e0a6cb")), y: Fp(U256::from_be_hex("0339c4553af8dd4571c3df4584f8211383566bace400e180e89d58c852fdfb3b")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("108f0276b7946e2dedf8d412bfea918887cbf5cecade831aef2e27153aa597fe")), y: Fp(U256::from_be_hex("00186d6ac64b5ff221d1252ae6c467437ada0a9bc1ea1b54fd6a61379b95b7ee")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("00edd2f6d3c0e206b4827014b594f1bf846d61d7e84ea12a15a2590a4641936c")), y: Fp(U256::from_be_hex("19271bf993ede06284d5a6052eed8321d919700295b054d43b8fa63363b9b2f0")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("196613e94de0be24ad66b7c01dde2f0733176549454ff3ebe1cdf118d84a7f5f")), y: Fp(U256::from_be_hex("277f3d6f74aa3e55e7dfb01c37bbf311c8da5fb089ef46775c8c812e253a10a4")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("25a8b450c747b91be77e695b3be5119b990040f2bf32048a78ec062006d9e610")), y: Fp(U256::from_be_hex("1c8bd7566739201c8da03b588d5c6d267e8df5949967a903fbabe4f75662fbd5")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("059bf9b1fd6a2fb2bfd215b6e38c56739e0bb4b3a1d9d1274ef712e65662157f")), y: Fp(U256::from_be_hex("1a76963e44704cfaac40ab9aac8635a5e270f0e877399acc88db985d485dd3fa")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("13e64a6eda1a721ad495b1deaae530993067ace2e3f1663845ff8e111b65c683")), y: Fp(U256::from_be_hex("187a64a78bb3638f57fe6de84d770979f13d7ed237f408dc70994bff78db0fc0")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("0c6b2b0eaeb5eca8d51b3e562825986752a8e295b3004acc9a4675ec75e3f9ef")), y: Fp(U256::from_be_hex("0361c3c65caad34eaab0c8d5db03382e0f5acd910f120e52f20064cb55241849")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("0076f61122dabf97ea656d3d82249b1a5e5f4e3fa21cb40209202faa182a3ae9")), y: Fp(U256::from_be_hex("16fd541de3359d4b4b7da171611e8fda8c525bc8ddd3150666c029d6da03900b")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("08e40f0092f4a72298731af23ac10676a643b59ae263c1451a32100d00ed8ec1")), y: Fp(U256::from_be_hex("1b2f7eb03f76c0e79c66cbf05aaaeaeeefdf3ff0b03a8eb8a7598796f7609bb1")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("1068459b548010991701338a64396d52de94e5959a002cadbbc92f451298d61e")), y: Fp(U256::from_be_hex("1115807e2ff4c5d7c2e364fade85f6643beec2ab23f3c29bd47f9d1f57ec8f75")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("22c8dea97187ab2952d512f7662e84bbbae8cf5dd8224b367c26a301f967cc3f")), y: Fp(U256::from_be_hex("1fc3031d791aace4457cb883162140859f8943a6d616de11cc77ca13c337a4a4")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("144fe580bed3eb1a561223bf976203b8e4b9da15893f9dd20e430fd1297944e5")), y: Fp(U256::from_be_hex("1e2105109a386348ed7fc051f0c114e80114625f7f5204bb818bb0fe91225c56")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("21dbcf6cc17ddc214a19b7dfd20c80264aac6423b79acb5c987a0ce50bb25395")), y: Fp(U256::from_be_hex("0172f8963f1ff4bb1c3840dc37e079a10c2da05458b645ae996080aca0a4d9c2")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("26699b178fb5e00cd9c37393b9bc4ec7c33e0e621d1e3e765a173961631b4dea")), y: Fp(U256::from_be_hex("1938b475fae0a26a55903a778c2b0b191dcfefc6a0686c647e5d9e3a277d0367")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("181d96a74f577f12b620671ebbc46a54b248ae32d223e630e661ebf8f146dc8b")), y: Fp(U256::from_be_hex("28e8ec6b3049500acd8cbc59192c0edfc5817b29cdd5e8ccc104a71918bf34ef")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("05ed2e8147e700188ff4f378578ee00d3fa12e57ae497a8b5d67136b522b5ef2")), y: Fp(U256::from_be_hex("1a13bdb3b189acc1c3e19cd64d23261ccebd63b00b65e74e21a9fc776d07c498")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("12e00d144fd951aaf73e55f0680d1eceb12e6076084b354decc3e1ea621ef396")), y: Fp(U256::from_be_hex("196f92497d99caac6fa6feb6e7ebf8bc382dcb9dd8d6a13153a4b33d4444304b")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("0e8eb46f0ca87aaad1b4eec744f33390e4325a38f4c6a89a1dfa679f8122f529")), y: Fp(U256::from_be_hex("1bc309c7575602527710e9e3ba41c6880d2ad101de1cdc2234fd29fef0ba59dd")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("1cb1caf3e0e314d9387064b814132d25f2a14bd89b09839980e69a1449413139")), y: Fp(U256::from_be_hex("17e997d6e500026e5ff9216f0da0f8301873874d5771b1b52c93d167b9a6c4e0")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("1ddea942e957dfc1bfb0c959d56fc2bb76cafb6b0556079102385830f7c81728")), y: Fp(U256::from_be_hex("12c9c5a0a9d9395656aea2b01f17196a659e56ab4c1fe26b1f503f2b598ef204")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("2294057757d2281319797d6a4fe49fdf4df3d31609375c9485b22081db07d3f1")), y: Fp(U256::from_be_hex("0910ed965a34fb16f651fbaa71d04465e48f65102f47a7c3c039f4c8a28b1a42")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("0bb0b1e6c608f0e65b447177e58af42fb1298c2ac21df64f69619b446eb91ef1")), y: Fp(U256::from_be_hex("26841352b910d0ddb3d589fc9fcaa7f735644e78785fbb93bced2c4be907a40d")), z: Fp::ONE } ,
    ProjectivePoint{x: Fp(U256::from_be_hex("001e746ad24339561f1d53a7df665ef07bb117ab89f738a498b5c9b7997ccfa8")), y: Fp(U256::from_be_hex("2d79573741d743704df75c43b3e9f941a0545841cc65812048aa59aec231a9c4")), z: Fp::ONE } ,
    ];


    #[test]
    fn checkoncurve() {
        for i in 0..50 {
            let a = GRUMPKIN1[i];
            let b = GRUMPKIN2[i];
            let c = GRUMPKINADD[i];
            let d = (GRUMPKIN1[i]).mul(GRUMPKIN_SCAL[i]);
            assert_eq!(true, is_on_curve(a));
            assert_eq!(true, is_on_curve(b));
            assert_eq!(true, is_on_curve(c));
            assert_eq!(true, affinepoint::is_on_curve(d.to_affine()));
            assert_eq!(true, is_on_curve(ProjectivePoint::<Grumpkin>::GENERATOR));
        }
    }

    #[test]
    fn babyjubadd() {
        for i in 0..50 {
            let a = GRUMPKIN1[i];
            let b = GRUMPKIN2[i];
            let id: ProjectivePoint<Grumpkin> = ProjectivePoint::IDENTITY;
            assert_eq!((a + b).to_affine(), GRUMPKINADD[i].to_affine());
            assert_eq!((a + id).to_affine(), a.to_affine())
        }
    }

    #[test]
    fn babyjubadddoub() {
        for i in 0..50 {
            let a = (GRUMPKIN1[i]).mul(GRUMPKIN_SCAL[i]);
            assert_eq!((a + a).to_affine(), a.double().to_affine());
        }
        let a1: ProjectivePoint<Grumpkin> = ProjectivePoint::GENERATOR;
        assert_eq!((a1 + a1).to_affine(), a1.double().to_affine());
    }

    #[test]
    fn add_mix() {
        for i in 0..50 {
            let a = GRUMPKIN1[i];
            let b = GRUMPKIN2[i];
            let b1 = GRUMPKIN2[i].to_affine();
            assert_eq!(a.add_mixed(&b1).to_affine(), (a + b).to_affine());
        }
    }

    #[test]
    fn checkmul() {
        for i in 0..50 {
            let a = GRUMPKIN1[i];
            let b = GRUMPKIN_SCAL[i];
            let c = a.mul(b);
            assert_eq!(c.to_affine(), GRUMPKIN_MUL[i].to_affine());
        }
    }

    #[test]
    fn babyjubconversion() {
        let a = GRUMPKIN1[0].mul(GRUMPKIN_SCAL[0]);
        let b = a.to_affine();
        let c = b.to_projective();
        let d = c.to_affine();
        assert_eq!(b, d);
    }

    #[test]
    fn checkmul_1() {
        let a = Scalar(U256::from_be_hex(Scalar::MODULUS));
        let b = ProjectivePoint::<Grumpkin>::GENERATOR;
        let c = b.mul(a);
        let d = ProjectivePoint::<Grumpkin>::IDENTITY;
        assert!(d.is_identity());
        assert!(c.is_identity());
    }
}
