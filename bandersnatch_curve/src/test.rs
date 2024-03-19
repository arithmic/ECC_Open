#[cfg(test)]
mod tests {
    use bls381::bandersnatch_scalar::BandScalar as Scalar;
    use bls381::bandersnatch_scalar::BANDSCALAR_MODULUS;
    use bls381::scalar::Scalar as Fp;
    use crypto_bigint::U256;
    use curve_traits::ProjectivePoint;
    use traits::traits::Field;

    use crate::bandersnatch::Bandersnatch;

    const BANDARR1: [ProjectivePoint<Bandersnatch>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "41564bbbce92baadc50ae66408c9778381104e045e5a31cd2af39387643c2c9a",
            )),
            y: Fp(U256::from_be_hex(
                "6bcecfde106caf9e662656d3990a04d7b27c176fd57ee415ee313597edee8d91",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2732dcf89c88e54f003a4c6164317a0be3067c56e59a8cff625054d858fce47f",
            )),
            y: Fp(U256::from_be_hex(
                "036b72fe5a77e50a50dc3a8a12d8ff9ee31a48b76aae408a1d6a39dec9fbe358",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02dbf8c623b6372a369ce8665d07c39bb1b022530dd8deea9a60a482cb413e02",
            )),
            y: Fp(U256::from_be_hex(
                "42675e52a09516c2b245bb79d5b074b8f2d4b741a2e0536ac849924a7c46fdbd",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2f6fd395cb6d1d0df3d3866f9ee4b81c5c7bab46db5ab2945e40a70073e72d4e",
            )),
            y: Fp(U256::from_be_hex(
                "1a2de98b17f1541025c83d5709c7f4f6c1d0d272fb2ba824242528ce746dfaf4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1bbc61568df2122074faf53e3c80e678e3a6a6460199101fb5fabbeb7cffe48f",
            )),
            y: Fp(U256::from_be_hex(
                "4f4ae165dbf7a73bd01c8ec26d93266f69b4d6ec8948bd11db05903608daea79",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "22616c87680be15fdc77795b72699685c64ac7793636ecc8ccf7b469e5dd4da5",
            )),
            y: Fp(U256::from_be_hex(
                "5f5ed3ebb5a32c60fd6c5ed77ff5d03a9ad046113f2f3b62d29f6fed3dc5d4f7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0b070c5c4b6ca6d9cea317ab2e3a3930f338b4a5efc52a043b46a9f3f02dad54",
            )),
            y: Fp(U256::from_be_hex(
                "51317ac1c7cfad9fefd7a311be9585eee3992b5c18b0f852aa8c61cb5398167e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "5cfc8642f38f24b7037ba35eee365fa8f08325f292896f20cc775ee11f5f27d2",
            )),
            y: Fp(U256::from_be_hex(
                "1d9a126c195309cebfa84975fe1ee33023dcc5c64009ee7b0aafed19ba160ae1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "31d354bfab2122375518a964a5817445e63689ae54d5088f0acdea6cf8b01670",
            )),
            y: Fp(U256::from_be_hex(
                "3b6bbe1a62cc4570d157469e3d95a7a896c58a881301e839f7b45ee7653b7463",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "20c9bfd91de56b3efe1281103f7c3959da6bbc1d8081bf70735695bcba01ba2c",
            )),
            y: Fp(U256::from_be_hex(
                "17f911c629a4852ebc6c9bdb3c5c9cf27e4066e8a11b795dea4a87e547eda73b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "5a45fb0dcf5790e4987d9a7a8a62d8057f616a5cba08068e01c07613b274a74e",
            )),
            y: Fp(U256::from_be_hex(
                "11378ab4103525b17581e61630734ae6a0d1e0810d3e5f244dab5331df379ac9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0cc1fb20584f420199cecd863865be2383d3af6f73351c2f2739d7f53ec64302",
            )),
            y: Fp(U256::from_be_hex(
                "4979942ea38b7cc1219a06d80a50bde7e151ef2f787a9f0f2d0d0c5ed64b7ced",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "62db6a1b7906b269a11db5ba9681cecd2b0e521c072a47019abcd4c334cf7521",
            )),
            y: Fp(U256::from_be_hex(
                "07b41487a70059cbf6802ed4a8c6d1b81feea3d8ccd21f0dc28d26ea97b05cf8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2831b1b58870efeb0d9da5fe97b13faf9ec9e96c8a9fc37fabc7f2ac5c515a3e",
            )),
            y: Fp(U256::from_be_hex(
                "60ca9d15a14f0f0d0341cd50be22a84302dc9ea2666840d5824d990c6bc1e3cd",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "68d5b62bb152576edd4e59cd29ed7180572bf38c784dcfaab88b3d17b075da3c",
            )),
            y: Fp(U256::from_be_hex(
                "6138a3a29ab8eb80ff63633806623bb64230081811cb85062e3a215f47893774",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "4007940ba155deafac69f229f89d961cda72ee10effb15ed53eb8c2d30e17da0",
            )),
            y: Fp(U256::from_be_hex(
                "487402e27f9a2961c634e3874f1efea16ba791c5d50ded6d5f8f2876851613ea",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "26b2894496ba52bb8bf4be0659755dc8b2789782db98e79357c30c92fccb695d",
            )),
            y: Fp(U256::from_be_hex(
                "440c1b79033d0f2e54c2617e6154b6d8f5cd14267cc74ed94625dad3d3301766",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "6d4788dc92ca75da4d32241b3a1cfe1e0aed8f15ca3671ab5993b5f1bfba98ee",
            )),
            y: Fp(U256::from_be_hex(
                "51508d8c3b5e52b87bfb015c03b7d77828c378641e7d20b989c53ceae73db10d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "25b7f5959bc152884c4443ae30aab185aa33f1713a73cac691965ce33c432157",
            )),
            y: Fp(U256::from_be_hex(
                "6ab1ca173b6b278fbb7a6dc93e9aaf591fa5f358090eb3f229af0082791303ef",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "10103125e29f7bd3507721094e5590cd0fb5099acd8edc513bcfe575ed257f0f",
            )),
            y: Fp(U256::from_be_hex(
                "4ca8dfe60e9f062254cbf726bac97a2f77a64b4e82dd848b49edab83d6752d1b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "418ae161b6ffd9b022187317427256eea9ff32a71a50386501f33d2da335ce58",
            )),
            y: Fp(U256::from_be_hex(
                "57532b63c5c064a6ce6c3b84f8ff6e30854699dd8353c452c6e5d85673bb021d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "26dfa54171c506e9deff2dcded843d1b3ece0b119d3a9341cefbb6d08b63318c",
            )),
            y: Fp(U256::from_be_hex(
                "5efc649b34fe5c0c79870dd3e81c55d41bdc8ade71f65d7a9263b9251362334c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "46f92c1a704b365268f07c517561254f0d5f1052dca1d530f16a1cbaa14020a4",
            )),
            y: Fp(U256::from_be_hex(
                "3957783c0cc266a7e7aac0792a1934417bdec0e4a7146be0a29bc644c08e2cb1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "11dce9b902679319d2c58d873d92bb8b18e77eb5547940f8f48f9168a1ff7294",
            )),
            y: Fp(U256::from_be_hex(
                "3cf3b7366308123fbd320611c63fc8a71901b3b219742e1ebc135fa1ae851f99",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1123cdcad8ffe4631d422b2d7e19bf464926aa602627bbb77439946566e0b33d",
            )),
            y: Fp(U256::from_be_hex(
                "722c4c53ae88fbee47fe7973039d6f3e3578149d8856f48626059fd86871cb1b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "648268d27588ef9a68d7ab339a28749bad32119a9f53a52fb3d1934aad21d798",
            )),
            y: Fp(U256::from_be_hex(
                "1fa969b83a736c8dc786bdd774d1164e10aff6bc4cd6d4858f2050f6d192f010",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "10be09bc1282628c93bf40a6d195b0e09e5599f6d0a4b1396ddce47da9c9cc4c",
            )),
            y: Fp(U256::from_be_hex(
                "5c907d646db017bf992cea5e62edaa14c1ab1f2d65eb1fec86d07aa148eacd36",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "46b59bd587a4938f4350a5a246c6e0fc62b9e7059adcd4c365b7025c04d4d649",
            )),
            y: Fp(U256::from_be_hex(
                "5f2ce76e8dc01b404a303d040c9f3b316a73e5186ac91ff7baa08e42fdb434e2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "70f08d070cc3525e2e647de2df8218b5feaae62d5cef883d104f2adacd0293d2",
            )),
            y: Fp(U256::from_be_hex(
                "16282d3244d4d8757dbf531a2d7f607e7ee620685ccbdfb1b80ae15599fc5a40",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "5538a98452481616870c0445e23ee497542afa6c748c7c568ea865a416e9e4aa",
            )),
            y: Fp(U256::from_be_hex(
                "3298c0962ed31468ffd9b3e84964ec3b443ad4fcb69c77aad3c0b5861e73b695",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1d4f8782c741b93cf7411561994ec6b53b5c14cf0de46aed8be412997e9f4fa1",
            )),
            y: Fp(U256::from_be_hex(
                "4e9fe02e462d843eaac77773370ae0bcc05b01463eab9d69f8bb7fc6339e5e98",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "60dc8b056b1fbc7e9bdca0592e8797537f1733704fab44f9542aea1e406983a8",
            )),
            y: Fp(U256::from_be_hex(
                "04103224cec66933d386a45c45a6973e810b542d0445f6fc503de461c194009e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "50aa046e51339767178a74c020fb8666e73040f6e6ffb09eceba3e03b1e03f01",
            )),
            y: Fp(U256::from_be_hex(
                "1f937f81350095fc7ab79053c9feddcdcd1ebec038632bd343587d13d8c7bfa1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0bf9342def892d9fff1de7cf8bb344641c2e2e8cbbf4db75c32ad59ab0a936cf",
            )),
            y: Fp(U256::from_be_hex(
                "1b3e7cd19a8636cd9af98889e1a2501df539bab4ce3df3d8de5dfd5d622c2de7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "097f6a0a6b3b691a577385e297a5513c543abf6157fb10cf4f5d1b6c3fa32fac",
            )),
            y: Fp(U256::from_be_hex(
                "4eed4986c0ed3463fb727b48b6b09f905a622fd0fe0bbeae3d532771b7bf35be",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "3cd66058277777cc5b9476b53c42311b4888e3b46e574d8db9b2ddf8d92c0a22",
            )),
            y: Fp(U256::from_be_hex(
                "1337f13bd5b6ac186db5dd64b203128cbdee7c076fc6f3124ab07a889153f12c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2cfa6e5e7d3bbc7b939e1f3f6446f9f2ba8226e74c5809a1f31234849f313dae",
            )),
            y: Fp(U256::from_be_hex(
                "72a3c4e4d506f3511ab7e288ad5e8ab5db6f9ac4bd0b7782d2a9a8b3fc960e96",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "694d91c3d925087e8388f0bb4cff80e39aa58f2225d252a6d560421ec0d34ce4",
            )),
            y: Fp(U256::from_be_hex(
                "02edea6aa45d786f2f6ab88d130b4255bce9693f2916a7dd49f6bce0ebc44879",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "596a0b418ba471a09df12d6915390c537af5d7eb3361649d79437bc20d43ff68",
            )),
            y: Fp(U256::from_be_hex(
                "3a909a4b1d83bcfb5ba72678c716f6cfd4e5251aa472809402825070e641b1e6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "4c2be4438e030ab752ed1e633cc297b2d9b631c57cea802b97d0d7ff740b625c",
            )),
            y: Fp(U256::from_be_hex(
                "67758494bc88af8afadced8812691c1bbeca6298fc516f7aa19cadd3de947f1c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "4058bff483b687b293039ad74a9ab85792d13dd356418e2ccbbe6347bd38ccfb",
            )),
            y: Fp(U256::from_be_hex(
                "613dabf25375537ab0e230aa10a2a2f35f605d0af1c380068b4bbd97434c5f33",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "509107faeaae0d6b0ea75e88f2212d0d020766f86cc77995ecb85e8c6bddff2a",
            )),
            y: Fp(U256::from_be_hex(
                "21f25be9aea81e9501f0969fcd750847f7df880ece163a7efde523640b0c6995",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "28a16cb0cd55ba5a7c29c9a936bf81502f0387f4fa3e91120f58bbbf38b30b4c",
            )),
            y: Fp(U256::from_be_hex(
                "5873d52ebc3d531b6364abc52b303a8852829e99c241020ff5709e7a14d26cb9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "5dcab0a3cb652c1c32cc609b8ee00ba080e30e6b4745e994a728e921061aca29",
            )),
            y: Fp(U256::from_be_hex(
                "3dad11c58d18d3c4f7a4c231f3b233a2f77b42a645827d3b4d6bd611bb86f04d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "49578b55e7855a6ec997340d2c2bdc9d7d26a187f8d5faf39dbaf0025d9ba05b",
            )),
            y: Fp(U256::from_be_hex(
                "08a507a69f3eeb4d4d09b1f2c88fcb9f93ab3c96b3f4e057143058243f861d72",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00d3a97e0b709aefb7ee871b5eaf352a7d680c9f536754b97d452720614d07c2",
            )),
            y: Fp(U256::from_be_hex(
                "1eb88011ceeacb993dea52544eb80fcf323daa2e6276b261d23ff6db1922762c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "5395fffcfbe18b78eb84fb1dc82f8728c36bef8aa0799098458ff4c8b1b084e5",
            )),
            y: Fp(U256::from_be_hex(
                "098e0b5e89036e8c5bbc6954b67bc853bb8401e666ddd1215015a1d27bf33368",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1894483e1c25fcafa66c2a80480dde5de1dcedde83af10ef222a5b824b67f534",
            )),
            y: Fp(U256::from_be_hex(
                "19da607d74bac9e13149b7085578d724474fe104752e67b22a67c4f29f4d36d6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "62f247ab1e3e62862b5ef180fd126c151af160adfce6408ed521dd4190e63792",
            )),
            y: Fp(U256::from_be_hex(
                "54a9976cdd13b02ff214ab382f585449b7229b1e22c3934e7944408a7075f213",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "3fc637eb6b73428084c165d37826616f157d56cab6a9ba7bdb8c01d1f372f25f",
            )),
            y: Fp(U256::from_be_hex(
                "47b6df3258c3f368ba7f67139f0f75f271a57c11b85cff15180e6254581db517",
            )),
            z: Fp::ONE,
        },
    ];

    const BANDARR2: [ProjectivePoint<Bandersnatch>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "550932590c0118060e44d9f10ce5147a1936ae873ae7dcf7d245571785bdce86",
            )),
            y: Fp(U256::from_be_hex(
                "1199df1dffba79564b8e700d435bcdf4e3b9f319af8e49b4bc869708c327b4e6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "56880d62897d08a19197287279241b65c6d28ff49946ad35a659234550cd0328",
            )),
            y: Fp(U256::from_be_hex(
                "24261dac9024b309d142146f4554f5462bbf72b63c48dab095d264505a76da31",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "12ac88028cfd7bee0789f07d210e30269c52d663fee49b02778aae231f8a5d40",
            )),
            y: Fp(U256::from_be_hex(
                "732a7edd53908c722e13c2c8751ae001c6c1f39aba4272946fe07afadac9d9f2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "6babe47a030dbc9d1a3610891fff5298d2c8d75ccb3cd5f12110414b8eb0dcb1",
            )),
            y: Fp(U256::from_be_hex(
                "5ea92126265c67c8f5a29f235e8af287aad448b41aa0dff1d330325e3ae02cb9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "41c2e47cb37b7c8b21affd4e0ad778f9caa70cddc4f47ba5a0a81cd2c15752d5",
            )),
            y: Fp(U256::from_be_hex(
                "729648e759efea28f4ec9b8a2b6c48d86ff4859861270f1e7c4d356236c01efe",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "62c75bb1d298f80e6f34688f450224581c7a5b8b518a0d69a386e8bcd230072d",
            )),
            y: Fp(U256::from_be_hex(
                "135d6c143be70d1284bc0326f40fc04df95e20f6c876a50e76dcd2994a2c6101",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "23929127952d0613a0b04bf968430fe9cf9ac0a29e3ad6065cd8fa388ebf86ab",
            )),
            y: Fp(U256::from_be_hex(
                "3ef315f2cfc6940279a2eda4605efee375ef445431ca1a8e7831aeeff55de855",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0aa6f83918d002ced89d307a18552f67f3b37a3b5bf00eea5f3be3b5f5464624",
            )),
            y: Fp(U256::from_be_hex(
                "0c6a25f5424bb6e953d7e92d8c3777b6fe0d9637227d06836e05ada7a8424213",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1f2e228061f4bbade3bdad2cbc8a238ea07937cc1553c65394e756e0d1f09d4d",
            )),
            y: Fp(U256::from_be_hex(
                "5ad517d1f0f5fe3c30f5a3c4968976af28f7d8e0b010ca0b0f7f1c264c25df39",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "37233663a68bfbf8118a4eabdef00330bae096ff2b7118ccd22e1563fe0e6567",
            )),
            y: Fp(U256::from_be_hex(
                "0d0ccfdd21c115ba42f0b3c57f7de9f643da08dbf88378764af839efc1a5b71c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "72e06596a82769f49cb7bfd0cc26d7bc87895d8d579184fce2c0276afb211fe3",
            )),
            y: Fp(U256::from_be_hex(
                "3809bc580234adc5feedbd05b2808939b8f221906156c906e6f1d6bc031a1a88",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "415eddac3fc13e6ee9c426b0acb8894c1f72c3298437ed9f9abdf441a56faf82",
            )),
            y: Fp(U256::from_be_hex(
                "0bf6bf1bfc591dea254a8d9a541a4037ed65683b0dd30fa8645b804a15bbca56",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "6fd5fdff9d9ceb7d797dab7ab38c5eddd6481ca6379738cca494db17acfc2a85",
            )),
            y: Fp(U256::from_be_hex(
                "011ee1f8e860425450a7f216a3a4283983a5a767b50bab059d87cdd2802d397c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "7029b42f4764bb33cfa5ed625d3550f6aa897486416f7a943dc0857861fd730f",
            )),
            y: Fp(U256::from_be_hex(
                "5f68138e9eefb167d8e57f98e9ebe97ff7311fdcc44db428070b04edc1c29064",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "127db6e9bbc2b2d884b73ece4f04761e993f9fb4f09792f12ee64fa504e5bec4",
            )),
            y: Fp(U256::from_be_hex(
                "1e166bc3bdccbb7f94926f4d82ee7e239e4f1dbf179c14f9f3c563d1e13cb60e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "47c859010adaff4291f9a08735ea18b107bce4088c69e9e1baa19ce545923ff4",
            )),
            y: Fp(U256::from_be_hex(
                "3ee2479b2be65a20b37863a911f08331afa53d62e76134c89be917d443051037",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2f68ee9eba6705661c10c805d396e5f297de9319abf13aa2ceb7efff2dec2f9d",
            )),
            y: Fp(U256::from_be_hex(
                "15ea4d83a7fab53ca94c689178ccf206c5f9cb103e0e6eb9cc4a783c574014bf",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "18a5117715a686f36a0e7e892e2e8fe1b995863c707c8acc6c7741dc0dd9fa7c",
            )),
            y: Fp(U256::from_be_hex(
                "52a2328977634801ca4e32a4154f7f6470c87d1c16c3002b9a52c838510a70df",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "73eb9753d618a83aeb331c3084aa7050e2930bbc3cf10f751af92104308c56a7",
            )),
            y: Fp(U256::from_be_hex(
                "2e4131107f776ac88d1531b63ae24e6c9f695eb961e8bc3045dbbc0741b71964",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "60b13ec73d5e3bb68db9ebaedd6da0e60a1688082aa6bd6c1640b01b3d6dc883",
            )),
            y: Fp(U256::from_be_hex(
                "3362e700a68994fef6a599588677e8a0700501d0570358efdcfbf4b407e4ece2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "18ba0f52e55fc1c506de0a04d0e5050770e9e9f4eeb653f9e6ebb8aca405bb2f",
            )),
            y: Fp(U256::from_be_hex(
                "4518436196b7da379a32826d074f8f8e95d7a41b9b3ee768f275528a1e392aca",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "43a707eeaf9f470a2e35bafcb3f0cc2ff44803015ed8a8e2efa393f7df41b49a",
            )),
            y: Fp(U256::from_be_hex(
                "55cf87e3d57bbb2fb1223415911eb4e384ba10d48f72dcaf59214e57f88bdcce",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "4d18c7745767669d5a28ab5605db7c7d7f9344627ec945bef3b0fd43158dc6dd",
            )),
            y: Fp(U256::from_be_hex(
                "673026bde2f52017e8e69319aabb83bbdbce9a7b97dfa6050badd091c261e0e9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "048470d4214c95e727aad1fe59b715402d17dcdb912ca913cc0c7339cb0b388d",
            )),
            y: Fp(U256::from_be_hex(
                "1e33b697151aa8147456d0514e21c9dd95e7db1b9cebaf93050e82d07f3c8cc1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "501bc0dd581fb78eccf6feb5d2c5712bf059c5c85b3bd915be5f6682acf88ad4",
            )),
            y: Fp(U256::from_be_hex(
                "514d646d1228ccb1d50d9cbe13f436258bf7f1bffb097c994a1a2cc516ce1602",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "3051421fa6d3a88d0b4e202e4734f71f6db7157e77b709b32f6780310185ee44",
            )),
            y: Fp(U256::from_be_hex(
                "47b4ff6acb08bae156e473456fca5aa11a47d55af5a0982f1a66c4e82c6bf705",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "694fc2084ac02f3459aaba808338b943856328d10414bacd765fcd3ce611b25e",
            )),
            y: Fp(U256::from_be_hex(
                "6e4efeaed049bb7245fb9ae7f5124a61d6025e8e0b6d6ac4b6d05afa8a9fc898",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "09eb66f424050708f5af05028c815761d3ec3e11a7878054779f9c8962f8e6fd",
            )),
            y: Fp(U256::from_be_hex(
                "0bb4835704fdf451b09031bc4d98faacaf7dfacfa147032b744adec7657dc46a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "45b0909cad7a3701ccf710abb522fd8ebe8bce9cdbb6d6621a4e022d8fbe6c97",
            )),
            y: Fp(U256::from_be_hex(
                "1eac540d8f44975eeda2c804764316a62ca63212e60f4814c9e19b9d19806510",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "15a1007ecda6d45bbb3f639e4bd976c00e06384b8616fb1af9da788cbf033344",
            )),
            y: Fp(U256::from_be_hex(
                "60c3e1db2120f59952d8d01867ec0c39fa7475f4fe3137803ce8b6cc7615959b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2fc9c85bce40b70988f6cdf5e31bcbd4a2416845d78cafe071ac779b9e19893a",
            )),
            y: Fp(U256::from_be_hex(
                "220fb96807e4c3b20c1978282be637a95b4a2ab8d6b2660d512db19219929b0c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "5ccff7cc66f0cbde7e84e17904b96c05f880c346ab79500d7acccc3891b40787",
            )),
            y: Fp(U256::from_be_hex(
                "554daa575b44619d09679da4bfa228e66fa6f6e3386e61b33fa365dbe77ce26d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "3dfa5b193b555f88251d4a017ca8abb5e8af02f497258227b80047327ec36af2",
            )),
            y: Fp(U256::from_be_hex(
                "120465dd4119ceefd00854a3ba74a15b00efe5bd28ccdad0be787f4bc96a48ca",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "71bd23b1650c6208b6bea21b119ca42516c9c6ff7171e95a4df34a28baccf352",
            )),
            y: Fp(U256::from_be_hex(
                "6b2e45922837b55ba2738f2e28b159655cb09ae6f47241d39a65c284c706aeb4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "21badf8a280300512a96043c30565e2717fc5330e64edf1aba9251a14cce4cfe",
            )),
            y: Fp(U256::from_be_hex(
                "567ddaadbe7824df7859fa97bc462574248518bf67160de7171421d379c924e1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "595ea1295feb644804570e0d79ee4ae0c1e10c62634d81308813d56da6afec93",
            )),
            y: Fp(U256::from_be_hex(
                "2f79ae1f2993c10f0292f20936bf6a8a4152e7f28f60b7d09419678016feceb7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "4b79637b4d9e6e4cdb21455dc8e30a9ee49e99e16fada016a0d2b82efca5350e",
            )),
            y: Fp(U256::from_be_hex(
                "6bba9786fc02cc7eff7c2a4b908188cfdc7d6e8540a4c7960d47628c4d854790",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "3b90d9f3de6a1ffd4ca4694bee3a285723c237dd5ea352f9ad045e736e7891d5",
            )),
            y: Fp(U256::from_be_hex(
                "5b66a96a9be91385d6cd35713b5fa2101143bc7b2a4820c627efd8d5e13ea756",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "30da61ff1e1bb53009564616db090e0a93d242130af90b7a34c0e44597d91e5b",
            )),
            y: Fp(U256::from_be_hex(
                "13fc1a470b95f9392dd87c15f02f765901ff81225a7b80970a5eb61093bcc3ef",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "6f98943088a934f198d11b12d022074bc7c6ea21ab30c8f86001021f998b59b2",
            )),
            y: Fp(U256::from_be_hex(
                "698dd3edb802897c4bb6696a5eae56517db705db52b8c119594d2ad7b334f92a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "4f59f5c41c66c421eee057f4a4c6653715085015e3b8d31c9bfccf4c2cf5e401",
            )),
            y: Fp(U256::from_be_hex(
                "4c972a82dae9141e3e71fa91a7336e7f982203c3d45118ef6a5edd918f25cc0b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "35e1009f3f8dbe6bab3e7ed70f7762dec6b9f4ab8819d4ce8d719e9eb38719cc",
            )),
            y: Fp(U256::from_be_hex(
                "6f24407a55af6b73ec138e0323014dd3bcddd1f59fa3648876487f30c72a9668",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "14be9b12ab90b4aa2019201c16b9bfb2129ae83272510e0909e37a80bbe4682d",
            )),
            y: Fp(U256::from_be_hex(
                "3a3141378ab9f49bf11b7239fcecfaf9879d7169b892743f3b754a32776ebbbc",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "3494fd6b3064dd745b0b079283621955e99533d6aac7bda8c355d4bea22c8f44",
            )),
            y: Fp(U256::from_be_hex(
                "49e33d6b0ce04fa43a519d7044ab180577630ec5b33c72fab1e48395dbbadd18",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2e9ccd608afb2f37a3314597be55dabdba9806bddb426caba0b613ef85c9777d",
            )),
            y: Fp(U256::from_be_hex(
                "37db7568879cd9827af28f0c7f65498b40b22ba94a1cd9ebd4d7fd1693f4a27d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "13f81f6c877fcc3971a3e3c08213e830d29f8bf4538a6d94bae26d2df1f53428",
            )),
            y: Fp(U256::from_be_hex(
                "2f2f3739cc12f2b0799abd36c8336714a74ed0d3107cac733e07fdccb0afca8c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "49aa618bb1c7504a75eb6b6383bde04709e5db11ce27252a3660577c8c0ff191",
            )),
            y: Fp(U256::from_be_hex(
                "4af0da5ccf74da1ca94bce267f52cd47e2aec60f3d7947d2908e29b545acec28",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "416a5ecb815e226ffe8cea8ec2b82664fdacf828d78d67db029d816a2c6cf2ab",
            )),
            y: Fp(U256::from_be_hex(
                "341a139a11ce5ef1b30df8d72b2e09d2ed86d86ad7a251a75c8899e4343a03eb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "475c8f05345f34e8912c77528457c985071636f51846e9a25e59241bf2b6be92",
            )),
            y: Fp(U256::from_be_hex(
                "159103d0b220db6402f27e7850637127df98b257f8450765569fc31895f8b5d3",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1521e4e813f7b2d3785e5bd5c53a6d191d61e08fa04ae75846549fe8f055d8ee",
            )),
            y: Fp(U256::from_be_hex(
                "5121b49e026f093bfca1db355d983dcf0cf42d061d0976414fef7e3e3555c6c0",
            )),
            z: Fp::ONE,
        },
    ];

    const BANDSUM: [ProjectivePoint<Bandersnatch>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "12ce8ccdb286637f6c8be12371d4a2407b26828dfcc4561cca388cdceb647a07",
            )),
            y: Fp(U256::from_be_hex(
                "22a08b67a7ee7770d7c0f3ee7254bc77f56338a99d173959bb553ffe102c3dcc",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0a7844df7f7208d12cec14c904328cfe55d3675dccff85acce088cbd508e03a7",
            )),
            y: Fp(U256::from_be_hex(
                "211ae035a0c40ffdca226b942ea0e182cb7531a98e02348afe19cf66bd83c882",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "6d3f950abae472c2f76121593a92da029687eeb26aa488d19fd1bddebd69be76",
            )),
            y: Fp(U256::from_be_hex(
                "40b4f18b7744bec3032bab7fcbbf83a2858971997745c385489580a2bd9c8b55",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2a8d337959a58ae763cbbe236ff6206f10e62a8d62325f1f23a8bbefe9b8ff7e",
            )),
            y: Fp(U256::from_be_hex(
                "0aeb05eabfa0a5038a5704aca2ed555277175f88424576436602ae892c11aac3",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0c861fc9eda3f5eadaee6c15b462d32485dabf834185761c9cb5c45a4c6d2d35",
            )),
            y: Fp(U256::from_be_hex(
                "4c81ff2fbf67cd20914b481a19a0deb5ba52df4f52c8ad7f1e1bb9aa63d6edb4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "29e1b7d76a0c75c95a8f29661f8868b2ec264d334afb3327ab3ec01d5b67e37b",
            )),
            y: Fp(U256::from_be_hex(
                "2e819f0ce6bbd34abf8d573597caf129f4d65b44ac1c1d518aa8687841d9b98b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2a2ac4ba27e6aa1ee1ce8410c15aca545f78535a442e1ef5721848013e73f319",
            )),
            y: Fp(U256::from_be_hex(
                "4553465d72a7000e91a56ac60911de180f997205a118ae1e37cafdc0713a4215",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "4973540a1da2a07bd8d8cf1018ec3a0b0d8c39e3567b27d40ccdef5faaa3c054",
            )),
            y: Fp(U256::from_be_hex(
                "705ad76bc2388ed9ed4286f39749a76eb4ee665807892452af599646a9d57935",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "6f0453080f0cfd073e43189d6313cc3a5b911eed68d63b15a5f8fd66765142e6",
            )),
            y: Fp(U256::from_be_hex(
                "440215e75a9c05b054afdc96b3ef01294faa7b67c8761430ef8ef494eac04df7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "58693d20fd8e976d2d01911ab2d899c6111f1efc621b91e6faa284f5eabdfa7d",
            )),
            y: Fp(U256::from_be_hex(
                "5166f510646429ade8af09764fdc237bd1b5170c05f2ae7e85a47117328f3030",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "17f30c18fc31cc65748c51a1cb6f734211d67c186341116e384d92245b7e1499",
            )),
            y: Fp(U256::from_be_hex(
                "0254194d88340b1d07253a09cf656930408e89a56acc86bfed0a3ec8a429478a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2372d4b71ebb09e83c3acae2b14491b3f3c30441f43c112dd33e3d5b7eed166c",
            )),
            y: Fp(U256::from_be_hex(
                "3b9d2f955bcd8ad3f7dba8d21bdac4fd40a68be8d9030aec814b1d64a9b89b4e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0aeb3352a384c871c04429fbad1d3c857d7025512576b4ebfb40ec5435e74b7b",
            )),
            y: Fp(U256::from_be_hex(
                "0b4f3eb3b3641a9263e8f362fe59c9f3c04b8f0eb9a21ee4692c01286e2376b8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00211bf5fcc12ac2077cfbeee0762dd1475275a3bd53502e192b1f69ee04fd6d",
            )),
            y: Fp(U256::from_be_hex(
                "401970e996db79514a667de54ce3e5a4068664a7f2bfb10e374d4e0b41aadbc2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "51e7c608858ecd81e08e47373206fffc50def864d6f2df890fef124388904236",
            )),
            y: Fp(U256::from_be_hex(
                "09e5a5adfb40908aa21dd4989a53363a05cb8b827fc05668a0cb00073043520b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "232a095952106c85eb44e1074043fa04d3ce9aa72dd6eb0c7e663ab6f7edf864",
            )),
            y: Fp(U256::from_be_hex(
                "5922c98c1482030532d9b1889a4c122f56919fe42a58bec5ab9035981122e9db",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "166c535024932c7b648d303679fb05d068acf3340ae2f77b6f416985880a31e3",
            )),
            y: Fp(U256::from_be_hex(
                "6725358ddf007fb479fa105e1584eb6d9ab56e207a7653012ea6d0b1dc44788f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "6f38c331a048f8ff6b151e664e22376e4948178db36c30d59e8a1ecd5807d463",
            )),
            y: Fp(U256::from_be_hex(
                "2349230b07694627d052c5f7540f96809c9a8c4c6ac685184daa3c66692d28fa",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "594794dfc793655ffe9512c0c64109e1ff14d46f4e12774f5ebf30ef69668ba9",
            )),
            y: Fp(U256::from_be_hex(
                "5429e259690b0ed5c27ab956169c02663f2355197217579a9a52d0e17d14dc6e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1b3d2f5abffecd3cf65c375e5fdf19a6e562232eb791e1d7261c9b800989ee42",
            )),
            y: Fp(U256::from_be_hex(
                "576dbccc7a3493f29f9a7d80c1b47a701f81a9a4632bd34a41350be25f842428",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "3365003665f9f12c1a1a843a4d8b6c63e6d997a680619e70bf3cef8261557108",
            )),
            y: Fp(U256::from_be_hex(
                "5112a5f686bf8be91bd0293b078af5c32cb1e46b0ae90ff238dfc8a4033e99d8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "18fd08c0f3267e3c25716d929456449bd8721259d7884658e1202ce066c135f9",
            )),
            y: Fp(U256::from_be_hex(
                "3bcece112190baff5b9c49c5555b819c0a08fd16091f6feabf4c2fa4311aac34",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "08eabe784d04ebb27cf5c50fedd10f74fd1337780da49bbd8989cb614887db79",
            )),
            y: Fp(U256::from_be_hex(
                "05d384c5b67cfad20f69bd102a695cf93120063b82364c6a9cd2c768ff4eccd7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "47f65447bb170c7150772ae07098081eeae5cf8af2c556862464424897aa36bd",
            )),
            y: Fp(U256::from_be_hex(
                "63d11172745390755eb9999b1114d61c7b76c7362c38096f41ec5444c6edf381",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "485b0c6582c7818c2c2b212e3be61b6f1ed1498bfd35264fc34742aafd789b12",
            )),
            y: Fp(U256::from_be_hex(
                "674328c9363396ce5827e81038e6600e9711b9249dfc44517118ccffef091761",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "297c8ed1fd1684e8e8ef8a68820f50f897112d06202609c21b556c60daf88c4d",
            )),
            y: Fp(U256::from_be_hex(
                "3bbcc9131ca5f534a65a9a3b9ca3524d078375aec00a5e215afa613ca21fddce",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0dc31f6417b751d9255f897818ab8a62d2171cc1eed2793afb7a3b200c937956",
            )),
            y: Fp(U256::from_be_hex(
                "55e29c3ebcd3e2631f37332e21c8dc2381425f78c3c02b9c2eb33448ff8f7b5a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04147ec8c38b8d3b18006aa5c64d907994055e8e092eb4ce173fe4fb25e390c4",
            )),
            y: Fp(U256::from_be_hex(
                "1c8bf2b60c39efd6b62c334588c19b8ef9a9a60c5cb28f62deeeca2aa55a3c48",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "533c7c46fefbaf614fd1ae4082c6aced85b800073837c2b3fd3611a5e1a69b32",
            )),
            y: Fp(U256::from_be_hex(
                "0ad17c7dbd701b107fdc8750ef96ee8ec3bdb38278129ef357d01d926e00f4db",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "334f0e74d6b41a26bfdbfaf37140c6b5ba33061b218c6dde149686a068cdba84",
            )),
            y: Fp(U256::from_be_hex(
                "664ab272f0f919c6fb7c6be1e037aac85b0b827f90f66c441907aef1e4a6ef61",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "42bfbaeeaafaf12e955c4a039c8df0d717e44be26647a435719ce060c7917c0f",
            )),
            y: Fp(U256::from_be_hex(
                "0ba56c7e4bae2fadfe73f633266b3890ac51d9cf4a1f0e1a1ed70217f003355c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02d0dda628809a2269fedb8b8fc173bb0208c962b1be8787a42625ef94eef11f",
            )),
            y: Fp(U256::from_be_hex(
                "55b9c94f9a517af26c904ea7f9d680c746d6651d7492eaf96ba5c91dc9e45cb6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "005f7680cb4ecf71ce471d9fcd09a15f9ba3020e05f49fa7db03811755852008",
            )),
            y: Fp(U256::from_be_hex(
                "6cf567df95dfe0c2ea6fb3194d3281ede4ab5c320aa85062a3a5a9a01ef90e01",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "5bf6d3fb4b413dfd63014176cbc1000b0f19b98e63491f8dce012947fbfa6cb2",
            )),
            y: Fp(U256::from_be_hex(
                "5a21c1aa4c28fed86ef02778f9e19f2bbecc568be46bbb682a86e90b5ceb34df",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "54862eac27d702e189e60ee2eaa0e1a4db24076c6d34b3ea6d41be030a3ae01b",
            )),
            y: Fp(U256::from_be_hex(
                "2d372586709fb9709f2e1e5d2a8a81aacfc7b54ef702ee328ef3bc114ac9d19c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "6d5b1d601d0a7c4633b5ffb69809a12be432a75ecb692b7ccf944f4adc2e8d7f",
            )),
            y: Fp(U256::from_be_hex(
                "21195f7c2ba598ba363fce8f3cca046ef2aecd42970d53655a28df97a4f7205b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "5f2123a82815689cd1e820ccd88b8ae6287dec85220d6bfea3302af4258fea92",
            )),
            y: Fp(U256::from_be_hex(
                "4f9a8c2d086c5c536be83b9af85e17798950f2ab98b91c11774f074c2d8465a0",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1a6c90f531e88ca7b85e54f92007f316b4e3d6fc1f336c7a07c4f1a990108fb5",
            )),
            y: Fp(U256::from_be_hex(
                "1b44c4690182102acc1ab87466ae1a946cc45c4b07409f15af286777bebb4512",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "6b1032ac8e003842e8f4de9b39cf411cd4be54cbd5699806f032a9d6f36157f0",
            )),
            y: Fp(U256::from_be_hex(
                "143d7b8a2ddfb7d44fc43d19e24439b8424fff9f92e666c48e2d72a2214c1edf",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "22bf496e9d3a8b70a212bcba9ae089995300e8fabdaa4150cbd793ea07cd67d6",
            )),
            y: Fp(U256::from_be_hex(
                "72e396e7a9dde05fe12eb0f5bb4fda1ccd1e22313a48f3bcb9a87c1ed1b00ca8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "201093006f7a907fe17efdeae0eb419c06d97143cf1832ed4ae613387862e0ff",
            )),
            y: Fp(U256::from_be_hex(
                "54f78055fc8ce3a79de4befa31a155c9b03eca234b7f702e9652d9567b5307e1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05f4ffa4925f5c95e8ea09b743fb8ca700505ab980ccb0d30b132044379c8761",
            )),
            y: Fp(U256::from_be_hex(
                "360e4d7462c7893ce06c4d194d8db7c73bb8b8c7fb33e26cceb4631862599e2f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "68eac2fdbf4f19bd5169f165d7819915f89e84aa3a3119f63813135695dcbb0c",
            )),
            y: Fp(U256::from_be_hex(
                "62a2bfee46c883005c4e60b9857428614715bb099160d659f9e3557b5ded5755",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "586b40c5c381dc61b63470b164f66919ab2ebbebb8f2b94d45721cb0bc71c511",
            )),
            y: Fp(U256::from_be_hex(
                "33594a7396b90f486f75c109708b031ce05beb4dd4cdc126df79a81d27e25c3a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "60ddb82a3b72f3d8bbbcdcf09ae3bec1c5ff3edd24118051003006a7a754a2d8",
            )),
            y: Fp(U256::from_be_hex(
                "215221bb77fc6e0ac73226be777bc145182f534353acf90e331870f7b2f7ced7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "6778a6affc6010e2760e2d31b728da7a7e5302de1a89434c4c0ebe872bab3ccb",
            )),
            y: Fp(U256::from_be_hex(
                "6152e61ddad1ac300922c8eafc8d258a4a3c23ce367ef8959c6349aea57eb747",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0c96b9171ca7c985ac9f4ac7ef502db839d527c7c8a142ea8a0554d616d18d75",
            )),
            y: Fp(U256::from_be_hex(
                "048fb58f8b6badf5fffe8c71b2535308aea3ab46365ea6d58eb87eab2a74226a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "16c2db3a8ccb1f9abd0817b57527902e65a622eae746df46292efc75738c2144",
            )),
            y: Fp(U256::from_be_hex(
                "3705af1813151391d8bbae9183ca986bd3e72654a62566423a229e12a5a8cd09",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "54b203fd85c985f1cca72a554089f84ef7d32b12ed0133358ec0f9b8016da690",
            )),
            y: Fp(U256::from_be_hex(
                "5a7cb1dbf0388134f0d1ab5294c3451e8660008fbe43ad532218b1bf6f2656d0",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0c575e85804e5a33d1b8617b8e004adb130cf76b27fdf7d0a9b244006d9ff4aa",
            )),
            y: Fp(U256::from_be_hex(
                "3883e96f73f7a5cf119a35be95ff33c564297c256ba44590fa80f2264da1e69e",
            )),
            z: Fp::ONE,
        },
    ];

    const SCALAR_ARRAY: [Scalar; 50] = [
        Scalar(U256::from_be_hex(
            "00d9f6b70f74f15f978ed8fece09b195d982ee18f4bc6e02e405e28eb8515f03",
        )),
        Scalar(U256::from_be_hex(
            "0e12e00a53da089a5af8fc9d7a7c8227ba9472c9f354e911802afff097d76a1b",
        )),
        Scalar(U256::from_be_hex(
            "0745326fbe4ee89e1a3842e2395b5fab14bc44c30054e6e5d62cb28a72d4f7a8",
        )),
        Scalar(U256::from_be_hex(
            "0aaf109c05a43ab42a315dd390d6c56be58a32615b9d92e40c1ffee67a2898a9",
        )),
        Scalar(U256::from_be_hex(
            "0d13e8b3371e4843220198a9889211ce1cf962fdaa7574383064da0dd162b451",
        )),
        Scalar(U256::from_be_hex(
            "070e93eca92b4d3b6cf887950ca9397c41224fcc023e2cd1bb8bd535b7894e06",
        )),
        Scalar(U256::from_be_hex(
            "098043d6264dd25fefbbbdfe1ca5c68181466f20a0b402e58bd0bbd519c77b82",
        )),
        Scalar(U256::from_be_hex(
            "00941440710b56d0821b6f58161d041f8d818751a994e2555975b973ee176f42",
        )),
        Scalar(U256::from_be_hex(
            "046595f55b67a31772666c676a011edb9fd816593179edea1aab9fec19662a59",
        )),
        Scalar(U256::from_be_hex(
            "082be96057a7a2c9f4528dc746311dee75245a49d75ec117a9b33a93f0b55571",
        )),
        Scalar(U256::from_be_hex(
            "0c75851bd2158255fc3a1b0f37cb512934f70bd265c24cec47a6e63735791e34",
        )),
        Scalar(U256::from_be_hex(
            "09a50713b4209c7aab5b3c359364a74133e3f31a455bcb072e9fceb95ab55c96",
        )),
        Scalar(U256::from_be_hex(
            "080ac8ff030f7be5259de4b76abfdf1893e39a805d030070a312501911ffa643",
        )),
        Scalar(U256::from_be_hex(
            "0436c24541d06c8d0c69fadadb1fed374c2e721368d55477d3879666a425ee4b",
        )),
        Scalar(U256::from_be_hex(
            "07690b90f776ec6d342bcce0feb25e84260a026f7d7919405d7b158c6b56a1cb",
        )),
        Scalar(U256::from_be_hex(
            "0d442cd6b307981783aa715b8951cd2f16b18daff52eb3a8aa7b36e70226f2e3",
        )),
        Scalar(U256::from_be_hex(
            "036ad813012d746c59ab6f3fbbd83d9838367807baf8080f97a1db948b7f6b79",
        )),
        Scalar(U256::from_be_hex(
            "027202cad037c4749eefdcf3ecf0cfe79f2712b2938663a2433d2794d3e708f7",
        )),
        Scalar(U256::from_be_hex(
            "0d35eeaa4af6e8ae9eb24f4e035a099fced7c481342ff845e7c682d35218b151",
        )),
        Scalar(U256::from_be_hex(
            "04ddce336615f350b28e9896a55e5252bfdd4501a4e093971ffe2fb7ecbc7be8",
        )),
        Scalar(U256::from_be_hex(
            "09a8aca8d77fa33485e0cc0bc14ae448e9a501951bb213a107832bdbed7f8775",
        )),
        Scalar(U256::from_be_hex(
            "0e1a6c2a2e8c16b9bedf7d777b018220ad0261bc757ee5e3c3821101d35d759a",
        )),
        Scalar(U256::from_be_hex(
            "0150b448673297d17dbcf5447e8daa6678575e2cdde59641b0cece1ea5c8d75f",
        )),
        Scalar(U256::from_be_hex(
            "0b53fcd80dc4601edda4b88c8417ded30528348051e8c2f291d4ee19976b641c",
        )),
        Scalar(U256::from_be_hex(
            "0c5c919574050767312694c7c73588c9bd01a075b581030246535cc42255de2d",
        )),
        Scalar(U256::from_be_hex(
            "06b2460a1cdde2ee7326a487e5f509409d39aa2dd681e55e208a74019b93e4b5",
        )),
        Scalar(U256::from_be_hex(
            "0dfdbb5f89449e5d966a88a8caee3dfd5e627212ce35626281f18714acc52ee2",
        )),
        Scalar(U256::from_be_hex(
            "0035f33ad2ab72109669da6b9998249e6610fa335e4e771b60e931bbfee812c5",
        )),
        Scalar(U256::from_be_hex(
            "0929d7a0f49941d887905eabfc2d8e045cd78014ab84867f3a6cd1fd40eed8fa",
        )),
        Scalar(U256::from_be_hex(
            "071b9607d56e2713c706ad779f240ddd1b266e1af17a7906fe17559c06029997",
        )),
        Scalar(U256::from_be_hex(
            "0b47f095180b4e4997e33c045da6d0ce8cb9ba0902c967bb593e64cdda0f0989",
        )),
        Scalar(U256::from_be_hex(
            "0973e2ccfa2f8ef345e7517a20e49b2d41b3769379440b96cd6408749db2f933",
        )),
        Scalar(U256::from_be_hex(
            "0d564f02553ba0f3b9e186f0bf135bb5f23c8246a383f505e88fb107a26851b0",
        )),
        Scalar(U256::from_be_hex(
            "03eadace724820ddd64ad2a5afd9029b9736d5f96798da28ebeb73ec671f4215",
        )),
        Scalar(U256::from_be_hex(
            "0dc6303c90451e15ed522696e3fe82a29744acf8e287aa79c0102e5e562667ae",
        )),
        Scalar(U256::from_be_hex(
            "0aa17070922a2c1833b864f6c2e934248c7b5b402545720a441bbb0104a47d7c",
        )),
        Scalar(U256::from_be_hex(
            "0ae60dbc9c88a16559c2a86a0ffc27f6d6df5350cab530e7653dfd8776e49e37",
        )),
        Scalar(U256::from_be_hex(
            "05111b08633033a9964b5887d3d2559e3462180add4bbf86c7c6cbf26f1ca6f1",
        )),
        Scalar(U256::from_be_hex(
            "0800656ae73eb640663b7644663d682e1ba7f5e6946ebf31c745bfa493e82e36",
        )),
        Scalar(U256::from_be_hex(
            "09d7c366b0cea6781504b267425e86383a7cd1e8bf7a61ceb796d9b4a7f32651",
        )),
        Scalar(U256::from_be_hex(
            "0c16b70603b68417c3db50b3e3f5d0cfece145f896283c3a5274b4aa3ff9e1cb",
        )),
        Scalar(U256::from_be_hex(
            "082225f045939e71a1d731dd36b021389a5105e0e0bbad37cf4c3ac3e4ab7766",
        )),
        Scalar(U256::from_be_hex(
            "03de301f19563c2e37c1d1fb01b7bc66ab6417ef8dbd20f60b41c5346799058f",
        )),
        Scalar(U256::from_be_hex(
            "0c4b1d354ff273e1ae9c4dac6ca072a759690e84d2a271a47f8b825a801792b5",
        )),
        Scalar(U256::from_be_hex(
            "06163df64dc7c0e9023f783c00735e0cb4f0cedae7e9d3927be56b05387cee1f",
        )),
        Scalar(U256::from_be_hex(
            "0a66cc2edbbfb02c1a2db73e79cf7f24176167a7ba702ca1e4df7a4ba12f20b2",
        )),
        Scalar(U256::from_be_hex(
            "0b5267252b3cf09d04d924646e55ca57f24c225147ada12b9423ccdd5e2157ab",
        )),
        Scalar(U256::from_be_hex(
            "034270201f134240aba0ce5ea9d0941b2939ed3dcf56216105f5b7029b4518c6",
        )),
        Scalar(U256::from_be_hex(
            "0e2df0d7d79907cc3977ae743a6166199a57555f086c7275088b51d81a74a3c9",
        )),
        Scalar(U256::from_be_hex(
            "06d6e6147c43bbdff30e1359a9d7e25912294e052250b070ab3c32b1b9e9d5be",
        )),
    ];

    const MULARR: [ProjectivePoint<Bandersnatch>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "47120b90b7782453d718adb083272a14e34c1d92fab5cbdf76c45ad366acb6e1",
            )),
            y: Fp(U256::from_be_hex(
                "7042a5f9e5bb4928b334aaa3f4716187aa70e0fc79357bc9110074aacf40635d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "18080754e700f46d0af447dad244fb1fbb5b974036404c630feef49882e2621d",
            )),
            y: Fp(U256::from_be_hex(
                "6369e41d0d73490e40ec743e2b91a9ffe3dc6f6064ace8b0ac8542412681708b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "251d751b6b5164db3207f10dc1fa6703044cce79e8f1bfcc280de8cdf97074f6",
            )),
            y: Fp(U256::from_be_hex(
                "0c10f1d3b0c35f083ec5a25c1d806f273a5077b5ce96bb2847677af3fea64c3d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2666ca5faa7f4e1e8bfc5aec1a69d6fd94ad32bcc0281021f1b20598878f6e4a",
            )),
            y: Fp(U256::from_be_hex(
                "6d9387e023c6ffdea95cd0deef1f67b36d363e75a091d16c0215cb6731766f2a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "6a6437e14951c3f9a878ee53ec6d7e7fbb1894be41da07081c61c3c0a89a18d6",
            )),
            y: Fp(U256::from_be_hex(
                "025e8fba5373201ac459570d378d39ebf8739d250d5543a6021a3dee64176ed0",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "56f82b6d107738bea9d59fe9046c9917f92cedcddc934ea63b07ba7120ccfb06",
            )),
            y: Fp(U256::from_be_hex(
                "654231888c4904ca369ef4e9fa6d6f1e7b6aa95277ea151a6fd96c2b7e5c1bed",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0a2941cfe25b5219fcad91a2f2105b01b4cffc4b96ffd0b350764b2a1edc555f",
            )),
            y: Fp(U256::from_be_hex(
                "354f9a26ab8bbfb52cec3fa15ddcca1c586a25c53504a60456085e341bdb4b29",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "62f7356b901f219f35fc8e5b55b82cf1374457ab0d49e91488f1c02d214d9b59",
            )),
            y: Fp(U256::from_be_hex(
                "0322695fd79aefb49a265cce718fc080fe5a80d36e3d8d82701cb15385d56c1a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "3ab0eece8f3394e5e7c407b4992a92e5744d2a1c6c82a67c70a74e64e512c9c6",
            )),
            y: Fp(U256::from_be_hex(
                "4c80ec33e1c700ed4d066ceb6441ed76213d454a14c16e080b37da2148d9b702",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0a8e6b4c89861f39f655fbe69342f19c0ddc56cf3a2c760d5e6ec444621a4ae5",
            )),
            y: Fp(U256::from_be_hex(
                "43e9686c0eef2cf4e32ed8e31169adb9d2f1bc448329637f590b3cd9c25aca8a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2e84e9f3a62e369124e39c2077ce6da5c9fbd9bf11d00521453a06a1a0e1aaf3",
            )),
            y: Fp(U256::from_be_hex(
                "2e99ad65dc01493045898830f754243221bb75d9282e99c7ff4802aee43f9bdf",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "391a8614a4e049329b084ad6c4cbe9f499264014e3e2ad392899fa9306a40edb",
            )),
            y: Fp(U256::from_be_hex(
                "042c48ccfa5bbda5fd0859e6ca7b0a3fed9c2b07c63544ca31157cdd8b007fd6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "3166ec33c18525142fcf8ce5c8373cc17aa8caf775acea0f855992bf8b6f7b2f",
            )),
            y: Fp(U256::from_be_hex(
                "5a0dddcd1bf9ccb5980ef1e944e4e08cbcfda9e6fa67576c166c6a64d88910eb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "14cf655f78dcf0fe1cd05a0623c4338c182881625a09423ee119232a20ca5df1",
            )),
            y: Fp(U256::from_be_hex(
                "3675cc9903054da41803fa781d27fbb31c1da3220be523a822f9eaf9c22e32be",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "4c47413d1e384cf33e4422df848c147e589466fc43bfc3878aa054b7049bd0af",
            )),
            y: Fp(U256::from_be_hex(
                "73e22eeb85711cdaebddd1c1fd5c113f30bc7ffecc25cd283a540181a388f9f4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "40c5e865fc582e7427db612b48affb773b92dcdfe9942486624d8329528f0a53",
            )),
            y: Fp(U256::from_be_hex(
                "13adeeef0ecebcb0df39306aba2be16b1f20f4ed36e67a90f993334c2c5065d3",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "5990730dd18990d680ee39b079d5e7ef1d98f65e18c6e09a2e3e73401990bcaa",
            )),
            y: Fp(U256::from_be_hex(
                "39ae6fb8a188ca0b3bc2171f034209d80d596418cea63a435af1a1c13d2e5348",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "703d899c22fe9b74a7ba58de38269deaff6c9cbe599f54f2726350cf6b72fa39",
            )),
            y: Fp(U256::from_be_hex(
                "4edee666c19a910c696af5201e034084b600c641cdeed59e0f9b778bc52a4dd9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "5f4f1d55adb7b202c0ecb3c911063528d6175c6a3f17cbbd80994af248aa23cb",
            )),
            y: Fp(U256::from_be_hex(
                "71f3dfbd97ea7154c0402e5f1b1b3a6daabdb60c6cbfc42ee2603bd0034212e6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "61c3b500423fed48e1a53b0647d3a7588976df7a4a63e9341b0e5ee8fa280cf9",
            )),
            y: Fp(U256::from_be_hex(
                "6fa6f02997768f6e9ae2e9bbbcb710cc609a15eebaf1dedf2331090459134d33",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "5e721628f1d60dbc0fdc870b532fa6630bdc2934d81335e7b00e788a0fdbca8e",
            )),
            y: Fp(U256::from_be_hex(
                "485a8375ab9fd3e945f983bd8220f27ccf10cddfad671f511cd7a40d7c66eb99",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1b16500c7725b6656e091c9f44a84fda8012b123f3b878aee49a4a66f7b1ea37",
            )),
            y: Fp(U256::from_be_hex(
                "12ea93311bc83d27a76ade7d07d98f13b745dc4890072023264c09b5bbf98330",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "234faecd263598dad6f45c72ec4cb8efe98a7471cd4aaf6ff61a133acce25317",
            )),
            y: Fp(U256::from_be_hex(
                "47fcc2bff7c7fe7e1e161cf901ab4d67a7c94460091272fd546546e5f244b0cf",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "6bda71b97d409c1f45b4650fdc245b7d52a7ebdbbbe6cb4b77c4323a76bf7d85",
            )),
            y: Fp(U256::from_be_hex(
                "55107a8163e9c0e06aff7a0519910d3183636d8361b9285e4a80a4c701a5f592",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "736dc0654ab604deb9a4bb276e8d2211d4952952cc49138155114b9e1a794996",
            )),
            y: Fp(U256::from_be_hex(
                "24fb17b808b82f1e35d11abc96061efbb7c9774a7034358c9b530eddd3c70e81",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03856b305dae4ce7e6b2264d9dc709ba22852f6714b4421df406936f34f31bcc",
            )),
            y: Fp(U256::from_be_hex(
                "59e64ca8575a57b2cec4d5f891600fca00ef0574e5bbb1ec7d8dea7e0ef67033",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "40b36711be2852f603153cd4f9d368bf9c6654cc63466e9514c4ad14fb1d6d23",
            )),
            y: Fp(U256::from_be_hex(
                "12a08b7ca8e70c32c449f3e6c78a09ed899b0a66bd1f3afa32dfc5a41ae65862",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "7045eb0de120fbcb875018605a916e24aa32c22644cc9a66c32130019e126127",
            )),
            y: Fp(U256::from_be_hex(
                "5d22a32bfccc6054d1709c36f9fa8f3d96862eb0154b91a8071ac61c9bcd7ed2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "64ffffd484677f3b68efdd8604eb39eb39e64c0ae58ff908d9ba7c9b69081951",
            )),
            y: Fp(U256::from_be_hex(
                "466903d7872da78ab34b90f5013c3da6b0b2f849971507acb2cf178447d72aa7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "01a1c0b1648461585eea85afad67b290bff30fbdfc0159afa967bfe258bbd8b1",
            )),
            y: Fp(U256::from_be_hex(
                "5bfc9b855288ea145a78246c7a664d340776a61713a88090cd0e8354d4a0829b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "5cafbeb0ea274cdef9497dc7b6ecce214984a1ef82164fbdd0dac72a7fad0b12",
            )),
            y: Fp(U256::from_be_hex(
                "05285499b61106de49e7d00212d080b29ceb3dc991d51ec5b2b3c8336b5cf1dd",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "161162a840ac7ce8539c0bf1aab04ed0508fcacda10400a739bf4f9b58b206b2",
            )),
            y: Fp(U256::from_be_hex(
                "0ea01a3df11f8d6f84bb2bf6e1142b77c9963aeb7bfec72b3e3a0c4cda04f5b0",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2197f92c55be1b174ed4f5d4fbdc90808baaa7e923bc57341575b76059126637",
            )),
            y: Fp(U256::from_be_hex(
                "57fb7235bcaab8559b7a6ea44280a06cc8d2a7f19de525ebe74573b4d55418db",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "62907d1ed5c0dedbb3dadb6fe7774310c4f5e62c7dcba8a175132a84d384ff0c",
            )),
            y: Fp(U256::from_be_hex(
                "65eda85963687db3a82135fea4cecfe1fe1a92106891f7b00cde61c4aab360ce",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "5a3078f939e20c535268a57b43bf9497604b2fc98ea59dc895f7b43f47756b64",
            )),
            y: Fp(U256::from_be_hex(
                "61d93c7e515a5f02dc76a5a23332aa298162f1c6297599f89a3fa04539a6a7a9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "4258aa8f022149aea12fc5d3588a3768c53e66c36f501d60dfc45d3dbc9f1bfd",
            )),
            y: Fp(U256::from_be_hex(
                "2d0ebf3864b5f8a2ac31fb69a9a8f4b36a38fe76fda0a9d20e7139cbb2379d72",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "7289c2d96fb1f205b54a526180ab71a2b2ed96d081ab96ef8b0650dd87fe2425",
            )),
            y: Fp(U256::from_be_hex(
                "63ee6f12c03b3f154da5100c40bd6db49de5cf13ea583dd99429b292ca9ec6a4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03c9ede544d91232eec619398717e87badd0d883b2fa4aefd5b2a5ae374132a5",
            )),
            y: Fp(U256::from_be_hex(
                "30d28b6cc567fb24eaa0c7f510b57e20a58300bd9bb84f0f29013d363b9d15df",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "419eae4f8f150790e05e6f44eabdd45cfef86e76e0a90262dcd7b8ea495c3bd5",
            )),
            y: Fp(U256::from_be_hex(
                "3f8d5757936a3b9b69d93cb2be56097a82357177c699a250e648b1ef0a7ee25a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "4676442eceb39e24fca750332abae057fb84570656ba2063f96ad5deff9ed148",
            )),
            y: Fp(U256::from_be_hex(
                "266769b5941cc9e20864e2836ad89ba0893646339b061e663a0f470da98c77fa",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02614a99907daaf86cc1155fa9fae3cc7311a73a9cb1ccda5755f7977a1a2e53",
            )),
            y: Fp(U256::from_be_hex(
                "3abf8db0b6c0d1de1670ecfbb098b3625821fe94ae73c4143091185afc5cfd6c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "45edeb922af22ac97201f1bcb0958bef9fafb8cc1673d61f8e5d0dffe7b2b7f6",
            )),
            y: Fp(U256::from_be_hex(
                "49817b76eb62a8f4ab1cd19a1d95f489d319a46f5948a600b5e26c081a7ba090",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "22b18ed31b6b04ded6dcee85d35488f474d185f1258fe28f4b750f277176e42d",
            )),
            y: Fp(U256::from_be_hex(
                "467e1cb60295106350101c04ead995a384ff4e3c6daf37eaea15614bbbe89b62",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "53a8a1a35a566d5ae1cdb4728f03dc5a757d2e4ba687aa658df2dec44399324d",
            )),
            y: Fp(U256::from_be_hex(
                "124cf01a05ad0d40949b20a0f6f3a80166e010cd0967a5fcaed2099834e73fd6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "5d382a8e66bfc783dc4130d5a56a203053af98b40cca17447ac16bc038ef2865",
            )),
            y: Fp(U256::from_be_hex(
                "1aa3f8fa47878799c45bcedfd11c45e340b73326636df35e4b18a8ede92c75c8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2818d7dd97101daf1112fc44a62ab021be9ac629035d048338c7cc4ed62a3f70",
            )),
            y: Fp(U256::from_be_hex(
                "4340ab2eb665df826fc5060266d3ce29b54c27e94006f0178170b377cb6edccf",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0457b687389970b8ec2dbf3bca31cb0d16cc5ac6144fd8ebb7ab90d13f510b26",
            )),
            y: Fp(U256::from_be_hex(
                "038e2fd847a72f442e9b43d9a0e16cc66c551bc6115b9dc32922dc1464e0fe3a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "34d3e93c16a6b7e8137636a66ccaff011a72042ce11399e42dcb19f0f7b8012e",
            )),
            y: Fp(U256::from_be_hex(
                "645bcdff737851d401d48d254427d33852d80bbba194562f95c9c0cc99e22178",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0e01820c050d8bc065f8ce948d531e611d738d62b3052cd93555f83d865a4147",
            )),
            y: Fp(U256::from_be_hex(
                "528e3e6596df37ee9fd4cbb7ab102e489d997e10bc52403f07f56a9862a329e8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "18ebb72bdd96912797b215e4daca9c25873047dddc8c92605c428103e027f8c4",
            )),
            y: Fp(U256::from_be_hex(
                "1ec584346786ed075bdae58184b305d3598a7f07e21f25fd10cc7804140fdcfa",
            )),
            z: Fp::ONE,
        },
    ];
    #[test]
    fn addcheck() {
        for i in 0..50 {
            let a = BANDARR1[i];
            let b = BANDARR2[i];
            let c = BANDSUM[i];
            assert_eq!((a + b).to_affine(), c.to_affine());
        }
    }
    #[test]
    fn adddoub() {
        for i in 0..50 {
            let a = BANDARR1[i];
            assert_eq!((a + a).to_affine(), a.double().to_affine());
        }
    }

    #[test]
    fn add_mix() {
        for i in 0..50 {
            let a = BANDARR1[i];
            let b = BANDARR2[i];
            let b1 = BANDARR2[i].to_affine();
            assert_eq!(a.add_mixed(&b1).to_affine(), (a + b).to_affine());
        }
    }

    #[test]
    fn checkmul() {
        for i in 0..50 {
            let a = BANDARR1[i];
            let b = SCALAR_ARRAY[i];
            let c = a.mul(b);
            assert_eq!(c.to_affine(), MULARR[i].to_affine());
        }
    }
    //......................
    //..........Testing multi exponentiation
    ///...............................................
    // #[test]
    // fn checkmulexp() {
    //     let mut points: Vec<AffinePoint<Bandersnatch>> = Vec::new();
    //     let mut expo: Vec<Scalar> = Vec::new();
    //     let mut ans: ProjectivePoint<Bandersnatch> = ProjectivePoint::<Bandersnatch>::IDENTITY;
    //     for i in 1..50 {
    //         points.push(BANDARR1[i].to_affine());
    //         expo.push(SCALAR_ARRAY[i]);
    //         ans += MULARR[i];
    //     }
    //     let c = ProjectivePoint::multi_exponentiation(points, expo).to_affine();
    //     assert_eq!(ans.to_affine(), c);
    // }

    //........................
    //........Testing Pippneger MSM
    //...........................................
    #[test]
    fn check_pippengermsm() {
        let mut expo: Vec<Scalar> = Vec::new();
        let mut ans: ProjectivePoint<Bandersnatch> = ProjectivePoint::<Bandersnatch>::IDENTITY;
        for i in 0..50 {
            expo.push(SCALAR_ARRAY[i]);
            ans += MULARR[i];
        }
        let c = ProjectivePoint::pippenger_msm(&BANDARR1, &expo).to_affine();
        assert_eq!(ans.to_affine(), c);
    }

    #[test]
    fn checkconversion() {
        let a = BANDARR1[0].mul(SCALAR_ARRAY[0]);
        let b = a.to_affine();
        let c = b.to_projective();
        let d = c.to_affine();
        assert_eq!(b, d);
    }
    #[test]
    fn checkmul_1() {
        let a = Scalar::from_words(&BANDSCALAR_MODULUS.to_words().to_vec());
        let b = ProjectivePoint::<Bandersnatch>::GENERATOR;
        let c = b.mul(a);
        assert!(c.is_identity());
    }
}
