#[cfg(test)]
mod tests {
    use crate::bncurve::BNCurve;
    use crate::fp12bns::Fp12ProjectivePoint;
    use crate::fp2bn::G2ProjectivePoint;
    use bn254::fp::Fp;
    use bn254::scalar::invert as Invert;
    use bn254::scalar::Scalar;
    use crypto_bigint::U256;
    use curve_traits::projectivepoint::is_on_curve;
    use curve_traits::Curve;
    use curve_traits::ProjectivePoint;

    #[test]
    fn on_curve() {
        let a: curve_traits::ProjectivePoint<crate::bncurve::BNCurve> = ProjectivePoint::IDENTITY;
        let b = is_on_curve(a);
        println!("{}",b);
        let c = G2ProjectivePoint::GENERATOR;
        let d = is_on_curve(c);
        println!("{}",d);
    }

    #[test]
    fn test_invert() {
        let fe = Scalar(U256::from_be_hex(
            "0000000000000000000000000000000000000000000000000000000000000040",
        ));
        println!("{:?}", Invert(fe).unwrap().0.to_string());
    }

    #[test]
    fn checkid() {
        let a = Fp12ProjectivePoint::IDENTITY;
        assert_eq!((a - a).to_affine(), a.to_affine())
    }
    //random point
    const BNARR1: [ProjectivePoint<BNCurve>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1545110d9f267d263bfa2cfcd387074860d7e50e52d4ccd62f5144e8af1e0216",
            )),
            y: Fp(U256::from_be_hex(
                "042d0123d0643e5a3d4477ead0ed9252b31397289d17773510cd0039e49c1d8d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1ed1c997f04f4f59fea65fe26dafef21dbd02d108c6295efd8114480d28e9a73",
            )),
            y: Fp(U256::from_be_hex(
                "2d67c2d0dbcc08dc5e938fe0b4faf3bc20e3aa6caacbbdbb9d7edc842baddaf2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0eeead489bd8f29756e21e386b56bbad48c579538aeff6e011f5cddef050efd2",
            )),
            y: Fp(U256::from_be_hex(
                "3062c192fe12f82f957073819cee9c52479ced4a2ab70c6d5fd727dbe4421c5e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1435b8b27696bda29ce7465407dcf3ef835c4fb7467fb080bf781d824b9a8dd4",
            )),
            y: Fp(U256::from_be_hex(
                "0f1e4298d09764b062a39aee3a110cf5fa7f7a6a5a6f8de0fd2c25e76d89a744",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03d6073c52431070d582d415d742642a6a072ba8496c286232b3c62d153c4c09",
            )),
            y: Fp(U256::from_be_hex(
                "16e973c174b5ed87d0d7737cdc69b51c8db1acf2dd2ccf74c0276ec36f154f70",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "01146955ed827f37bba20d57be1dea8434b6dc05a83a3862fded1d04dcd62595",
            )),
            y: Fp(U256::from_be_hex(
                "20f0208de1023c18566325af84f064e2495547f4917a1b37fd4d25f5c92464c8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "039d7af972e2ecc87818b241e9152d4ab4406a5d753d90ee15860fc6b5d8ce2b",
            )),
            y: Fp(U256::from_be_hex(
                "2fabdbfb94a2ecf1eca9c4645645dd44134c1f352511584981e89ed2198d5314",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1fc26178d27c4bf7f67e26da233c715efaf6bb5eb182092c86c2f9917ed4c405",
            )),
            y: Fp(U256::from_be_hex(
                "0ec930ba48e403beb6865a90e575a5175831f333693e6ab8c9baf58a72f58e71",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1faecaf1faeb84a73b9734c160e43f59c62812b5622c7ecec559bb22b5de1ea6",
            )),
            y: Fp(U256::from_be_hex(
                "29fa4d2be2492dc9e45217068dc6516ac3f34fede6fd6a4ec170a57f9fc5cff1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0418a6922f5abc47cecbe482c562571dc95361141c3df777e902d03244399e35",
            )),
            y: Fp(U256::from_be_hex(
                "19adc796c0b3ae6e6cf3a2455d6c23f02d779c0506b8e4968529ac232e8476fc",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2c94abf4feb9366410f597a089a282d55e8fb3d091bf875db27db24853917ac3",
            )),
            y: Fp(U256::from_be_hex(
                "222fa51dfa4a80e2edbcca770b63e01335333fe94495360f44282282a95b7c86",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "23d63468fcf9399d543c0011719f97ee342bcc551767155c26adf88a2665e1b1",
            )),
            y: Fp(U256::from_be_hex(
                "20f716bc5ab956421bb29de15258a3c67054b1f632ad7f4eb6ba2fbfa91f5ee3",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "207f3d88832e60650a662d5bdeb3ce7fad8198257a0ef064af3237fede307184",
            )),
            y: Fp(U256::from_be_hex(
                "0ecabfcd2d91978b0e3a920f5347549f1556a90c1216115d5bbfdbdece086500",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0bbba6b80a36118d82db92d167d66f625be6673a7cb6994280fc5136317b3e33",
            )),
            y: Fp(U256::from_be_hex(
                "0055397098f110b5871bec4ac7f04a038b0392c2cc72f347ed353054e72d5d09",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2e20a508f247248425e7962f31d07cda6ba8e4ab3876eba3bb1d176c63a0aba3",
            )),
            y: Fp(U256::from_be_hex(
                "296ddcf8fac104b089b453e9a846babf2df0526d65aa9121ae7eace19bb8d1d1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "090652c8e257d779e42fb53d320ee1df16a815974eb85176a9542dde6ea98ab3",
            )),
            y: Fp(U256::from_be_hex(
                "0c1055ac4419f3c20cef1601582b2d7f93286e7d9165bd3aff4043e50a2ae226",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0a6596dcb84ef464a5b8af7e704423a4564ae238dea6864d68f92daba507cf0d",
            )),
            y: Fp(U256::from_be_hex(
                "21216b474b25545f6cce2ac7ca9d56e8ba284460ca434c893775a66431535d33",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1686988b3cc0d8565753b07b3cbaa789690c334d73d0ef24ea9d90d5eb7f7585",
            )),
            y: Fp(U256::from_be_hex(
                "15f7ea7308627f28013457cde9f54108ebb06c617b4c1e77dab8ed1a1d862842",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0ef15e98e2c29bdc7560d5c9b7c3ded548ee2d68080ce5767c4c210f6db18f1c",
            )),
            y: Fp(U256::from_be_hex(
                "20dbcbce62241baead5bf7f152014588515c2c785bf881f8b2651c48a920b2b8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1c8074178814ffd8357ffc255fcc2840e4d66661cac5c61e29c0b166e835d0d8",
            )),
            y: Fp(U256::from_be_hex(
                "119a06e4ce016878b210fc39a8528e16713a96d9add876dfb68ccee9a6241181",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1f6d3262bc53bfb0d9f9d51be43ca41c41e8ada371401681763af2b3ef748a8a",
            )),
            y: Fp(U256::from_be_hex(
                "000597c2d160fe338adab36f8387a4c7b4575bdcae01a795a91b3405efd4fbea",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2b130fcf20f443e99535785338bc6937706c398c1bb89963cb80efb0e8eb3032",
            )),
            y: Fp(U256::from_be_hex(
                "238c97bba0e774c4a7c029894c917928bd23618fdfd0508bbf64cf4bbfae4746",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "07b9a21eaab163f0daa1082902ba714f8aa159d0cff75b09525ab7da19d1e3af",
            )),
            y: Fp(U256::from_be_hex(
                "1bf066980949d39f3eee7d81c01b12ee2166abfd995581e208aa3110dab9008e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "012c3c1dcb4f72d4d3ab9fd8e8f6a2325b8f2008c26fa9269895f4ded29922d8",
            )),
            y: Fp(U256::from_be_hex(
                "29cd599ef5eed7a4049dbfe9c0b71e0b2aa670a6248ca5cce5f40049918aa025",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2f19a7e3ba4160b5e0ce8a966fa656548aa441998f33c11428cc9da8aee876dd",
            )),
            y: Fp(U256::from_be_hex(
                "22345d9ffb879fbdef92b5780e719cf5fe3a663d089f400abd6d8f5fb69b5d21",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "17c6f7421914a3b173becf72a3eeef4fec46c15083e8c69f4b41f29bbceaa4bd",
            )),
            y: Fp(U256::from_be_hex(
                "1cc6c57dcbc1caf6c4a82856621b695019e6fd9ffa1778b864ca2f9ab245ce06",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "06158909756bf241f711b2b5cee3071f4322f94d8ae12719c81990615e1d8eba",
            )),
            y: Fp(U256::from_be_hex(
                "2bc78b0e709626afb92e6d9869a0537f4032897637fd66194e14b468837444b0",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "223d9b1f0fd1cf9c692241ce6f72c24c95251f9f12a2f895f95ba7bbab2b000a",
            )),
            y: Fp(U256::from_be_hex(
                "0f2d729c3fb466fdba8fb12bf5d8c2282e9d69f5b208a07e4999ed9af65cb29c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "09dc2fe726f53d7334a38840c3709fc64c368d2883f857a3c23833d885478f3f",
            )),
            y: Fp(U256::from_be_hex(
                "24e3baff4eccd1fe4dc3f32ab97f62964347598ce6755cfce16ace2f2abf6b2c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "08ab0d2223edf4021874cbe03fd4622c177bd623cc41bdb8d1851a2ec94b6669",
            )),
            y: Fp(U256::from_be_hex(
                "096316d9081ea31916c243da635db7579d7f75d843490186a7e40c82090c98f9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "098200f4a5b8c0c3ba97e0818c21d8990398186a62cfe0d910e2b750fda50c62",
            )),
            y: Fp(U256::from_be_hex(
                "2a37b041bd9ff71078f897a9704c6fd54982e748e5e839f9a80337f82d191029",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0bd4bd5e304f311583d2ec8a3aa064122a9c4739bf015160a1f819d815f5c7ee",
            )),
            y: Fp(U256::from_be_hex(
                "23e03b28e966d72146b8d796e8c3a9f15a302c3a5d77b057c90fda0937323e18",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2185aa94f476a187e181f7b5b1d4e9cc5b913dda00c9d42caf522c866ea0a934",
            )),
            y: Fp(U256::from_be_hex(
                "0a60ecb5e044ede9b9264a52c7bd67848d2a45d3bf8c4f60f0f16735ad38350f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "24917f7b8731de3d8978950518632c6ff2e22e25efd01229535470f3b0ddaaa1",
            )),
            y: Fp(U256::from_be_hex(
                "26ba56da9de85712982c7457a93747a0c13dfdab5f73c75cf729734ee9f5523f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2e3d95c8259f5376f990fafec1df497f08e2ad03a34f4a46a98f6956effa7d6b",
            )),
            y: Fp(U256::from_be_hex(
                "0d17542a3c6499a49c64edba421c8c5b2aaedacd64b98c35d17c9704c3d98a48",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1c18ea64e5f032f882d76cd52906d83d6a63ded27c6f0e485f779d406f6733d2",
            )),
            y: Fp(U256::from_be_hex(
                "23ca05cf23bb0258b2a0f1749327f70059e9c8aa8e452e6cb86df71e75469654",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2eb4d1cc71f681384f64bd339fb8c17988feb46afa11093b60b71cd9f5589221",
            )),
            y: Fp(U256::from_be_hex(
                "0e39da257e99fcb97d6dee407f368a1271444da1691d0d8411cb3f66baee70e1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "26497d4921fcc5efc88d1a3a5182f12a4cd04c3183638534b1bee93a670f4fcf",
            )),
            y: Fp(U256::from_be_hex(
                "0221c9e9c892eb4b1de1fa8c57981f3bfe0df14ed7ac35321db504f1dd6c9604",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "18581f4d5d53a04fc317f4ae5e6842c9b596404ec674bb766b99e35921579e51",
            )),
            y: Fp(U256::from_be_hex(
                "29f4fbead1d52518e87455f496b9e1d2c689a80505c353a38399c8e92b3c727d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "09e5a1a56d54ed2884f4c729f9d2e7972eb8670416f80647fd5a493b3016235a",
            )),
            y: Fp(U256::from_be_hex(
                "0150460e3661d7510a43acaa67f5de1eee77292d1a439e99bf2fbca77ba48540",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "187370e9050d337210be38b0896bfb81c67e790399e1e5ca2b8dce21d44672bc",
            )),
            y: Fp(U256::from_be_hex(
                "089332a0d754c949c5501b7178f0003ad873d9db5c3b81f7d5fa3687a3f1fac1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2e5076d5a790d5c4f5156c8dd277aabf2c96ae0d804f6d5b357587c23aa454d4",
            )),
            y: Fp(U256::from_be_hex(
                "111764efff1d18c2746b11da48323913aabc6b763910f70960e82253ac17e445",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1b3a82108b29d28634949e03f7eafba7307012f046a6f08dc560e2b1103aed99",
            )),
            y: Fp(U256::from_be_hex(
                "020e8106e5aea824b4fce92bd9d427f95620635e2bdb4abe574cb9def88d42ab",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02cac1b048e6b5c8d2f7bc304cd7d24f30c3ecfaf4d80d645606360c4330a2e7",
            )),
            y: Fp(U256::from_be_hex(
                "0d629c3f81e71d99d806b7ef3b33d3c9d1aea4286e97fe23dc6a56eb64ad0b28",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "088a1b1bd2b3af70a7a08df17602624203ea66f9950e1e9dd9c9b4b1b9c2a09e",
            )),
            y: Fp(U256::from_be_hex(
                "21fb753e29947bdb205ac2654b0e2a0df2cabb349b458a2e10bd737454d4f991",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1cd357f2987dd4d917479ad147d43d0676e6ca3077b716a71ed6321086fe308c",
            )),
            y: Fp(U256::from_be_hex(
                "18fc9da11b77488c8c2296a68d36c23f4929de191569c34ab31db30b0285701e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "083a30fb2f1e0899e000562db5342fb8a1fe8e9105258223b885dbc2823b9aee",
            )),
            y: Fp(U256::from_be_hex(
                "2beb8043c85b9e9914fbfea91fa3b3565833be4a4963179c3aad20a0666d0954",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2fc576d3620a7adbe99898341716748b24362d4e6de4a8bd5a315fcdd6e3abcd",
            )),
            y: Fp(U256::from_be_hex(
                "2d8a5b0ecc0c2ea8de7b63aab0da0f09d9aa012cd52238daa93b5a87f821ec7c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1651915a8b9142971636e2a628d7230725196cd3e899e7e5ae95d005c57db4c9",
            )),
            y: Fp(U256::from_be_hex(
                "1164e25040d610e3e176b7cbcff31a2555f146e18e354b74bcae14b9a6ba0a29",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0cd2aa49aac7829ba23ee3b4f4b78b73959d6a991823b2ba92a5a65fa61d1d22",
            )),
            y: Fp(U256::from_be_hex(
                "26e9cfb159d2873f32d4780bf4f69b3bbba863209a88a1d0d748d0a345b68d8a",
            )),
            z: Fp::ONE,
        },
    ];

    const BNARR2: [ProjectivePoint<BNCurve>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "26acc0a2cf6f815351977dcafd98ffb05d412d9d0c1d555f99116d2498e4e165",
            )),
            y: Fp(U256::from_be_hex(
                "04e0c0f9481bc632c3864eda1306f4f05ec36c258740b49f32144a3a268393a5",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "27e07b8ce210f5ffed5c722d3b0571e25cc3e8dd1b35d7dac3aea0448dfea401",
            )),
            y: Fp(U256::from_be_hex(
                "016f49c532d5c10228f17ec25b9f6de868402557f9bf371b8003c2d684b49000",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0df2c4623545953b04bee7a296b328faf041105e18a0405d360508d89f94d099",
            )),
            y: Fp(U256::from_be_hex(
                "0eba136dc0ebed8d4c65154de8ec22e6f5a630658d621100fe8c1df73a6f2a79",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2482f37f7fea708b0c0fdcc1b4dafe92b6ac847e38c9ec080ca762232e0e541a",
            )),
            y: Fp(U256::from_be_hex(
                "2455a7ec2faffa2b3e34a748c36a7a618a095953dbb3003adc5b6fc7b2784be1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "127ba4f7310f1cde2379bfbbbd3ad00de55f8828d116c13be062519de4d0fb01",
            )),
            y: Fp(U256::from_be_hex(
                "0deba7dad777cb59eaea80b6264018cae34454b6ee8618a0fec84957d4961c4e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "28093e79944bd7c02a3dec02a2935f90205952961ec372f1cc6010934bae186a",
            )),
            y: Fp(U256::from_be_hex(
                "053274f2696c439279f32c3c1de5c46d0a83fed35c48b6ddc1a309e5109fba5d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "09e3ad9bb320b252da256f37af60e2eb48e5384fbb7c2c67580c0b7860e51db8",
            )),
            y: Fp(U256::from_be_hex(
                "22fe40fde1bc2c25c53d55fd5c94ae682131a7e03a065bec98c65e14d1603aae",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "298e80aaad37e5123e61b1ad36af20701fbe9ffb0ce40834ee983a4ca9a6ef09",
            )),
            y: Fp(U256::from_be_hex(
                "1d3e44694ef32d3d0c93c2b3acfea75664ec1b3c3c065fb0fa092d4427ff150b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "19295b3f6f67a98f28e708394eaa01674b0ce2d433307895a34cd4f5c4ef5392",
            )),
            y: Fp(U256::from_be_hex(
                "1c10dc3f8ad3178f21ba6e71a693f078161318f01441299725436734c7caa41d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2556f110157facf7916e8a6bed55981446c6bcf8216a8ab2587576575a9c0d37",
            )),
            y: Fp(U256::from_be_hex(
                "05b8c32b85863300ea4a4444d1e43fc06d228eb78de1f5e8f03f6e70844e5573",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0c86238017297e9a8cb71e30b0f7ed8a7643b20544806733b63433f2d8e3ff7f",
            )),
            y: Fp(U256::from_be_hex(
                "304d67a6be3fa702312075663db15debf5423a008d771658d61fb1da5ad831f9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "22f2a86c168e938380d6bb6dc032050bc4a2db28cc7cc39fe983bba88a00544b",
            )),
            y: Fp(U256::from_be_hex(
                "1c4c36ca3534f91f3148eb5cf9c926ef2f5bebedcbf569b8ad039ca7b80fc168",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1f41b6acb75c0f61ffba24867c6cc4947998bcdcd6a1d479419b4f035bcbfef2",
            )),
            y: Fp(U256::from_be_hex(
                "104e411cf0ede4166844457c0b2f927cbe78cec6a782bdc4a4bb8c3dded54e87",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1248389f1b847a70af7ebce5a9b7691d490e50ea73bcbd4e3fe6ed9d157d5334",
            )),
            y: Fp(U256::from_be_hex(
                "01e042bebd421c9863deb6c25f7914d5cacc2ca4fb6383ee137010ab5d37bdee",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0f14583a0eb653a300995a6e701fcccb0a6849f7d624ca46b762eb5a3408d054",
            )),
            y: Fp(U256::from_be_hex(
                "2d87eb88bbb3da8ffca3348a9d70536a597c5719299310f4b4af684c5617fbd1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1a29edceb431e66243b722ca149c5a4241dc46eff9040bb1b7f86b54c7458d92",
            )),
            y: Fp(U256::from_be_hex(
                "0d940df0b9a717ee40f60db64a243537b2fcbdd447583dfdeb3b351e0cdc7465",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "12bc36070df7f1d6bc9dcb2e97c21560a9d57b8e2ca2aafa17d33afb88f62795",
            )),
            y: Fp(U256::from_be_hex(
                "0deb43d407e54215eeaa4abec0a9750629d68db1bd866412e4d8dd55d02a32be",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2e1cf0ef72d35c00eb05fbb6af051ab57210dc535d2f36b81259b9550de63ed7",
            )),
            y: Fp(U256::from_be_hex(
                "2b0dc85075e01ec2a1c7ed8086c4d951abce9d06d12090f64d248bab9f2f68bb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2485b171da075a4f2650bb65f01ca86e17904f18693756826fa63e97a723b3e3",
            )),
            y: Fp(U256::from_be_hex(
                "088e670bc9812ac9a8b7296302faa2b68df60b5dd2734dafc8254492d2cce1fe",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "11ad9141fde2e590102d9b3e4d170d29eebba4872734e825f43517fcc101994d",
            )),
            y: Fp(U256::from_be_hex(
                "2674d28c68300867867455e816dec7c95224ce9f035614a0a463f994f6e4505e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0eb9287bc847e943f058ec4955e7bb4c39152c13834cb20fccce8cdf3cf1b46a",
            )),
            y: Fp(U256::from_be_hex(
                "1093720ce08bc0492352690f748b70a8103e3a3aadeab9a4a09a3e46fd2db41f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "161bd112f1d1e695b7d1897815f444bd617042fe0f6bc60152256b247fdb708f",
            )),
            y: Fp(U256::from_be_hex(
                "22b9068246dbbf434c2b4341e99c3ad7a369e62096f267629fc37a25d8ef4309",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "07ed7c29448a719e84eece1d699439dd170d5f9391a656c4506e5b27f094c2a6",
            )),
            y: Fp(U256::from_be_hex(
                "1d0a890eebd7458cc2138f5831035ffad07f5b7d71474a22f4678d2f92fec3d6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1c30b3d08b1bed8973c98171cc06a8b6b8515553e0c2ed5c679fd4b2c8a720f3",
            )),
            y: Fp(U256::from_be_hex(
                "0a0efce2110d43a50fbd9794d07af7c1f8002e09fe3565fc2554862adb079f05",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1c81e6d0f83c02ee8e5056e32d554b191aa36bc5900c074a677f34e409924be3",
            )),
            y: Fp(U256::from_be_hex(
                "2df6ead1f78027baa4044e6f6b222f4232d2d6b24127666cbdff1ae042d4d093",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "11f32cf6bcb2dec9e343afc007e019484ca7a2fa9be12d8c248a36cb13e148f9",
            )),
            y: Fp(U256::from_be_hex(
                "1ad4a83e1843bc8d9f799cb1b73b003783258f60544d892f415553a0cf2a459c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1cdef6673ede7aba5cf366d3dd764d07da37d2d52e51f168cd455aa427692f92",
            )),
            y: Fp(U256::from_be_hex(
                "244a6f9c4f88aee39ddc35fe75d2b12c6fb5fc079608bc0201bb8d5e9b4dfb7d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "09cadb07d95d969992f3f67b39ff8b35429693ab296a0e74a43465764c764f7e",
            )),
            y: Fp(U256::from_be_hex(
                "1a103fb8147ba65e5657d25907655bfa37f8a7686cbe020dc6a64ac44b5c8864",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2658b8a4eec162256312fd11c35833d53e17dae1602fa017ff592825b83eb789",
            )),
            y: Fp(U256::from_be_hex(
                "148710ce660da1eaa9113fe3b47e38ac649ff265fc0041ea241d0f3405d7ecbf",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03713edb93e70aeb0eec8c7ad243a2ef9e4c08238a13c0643051051c2e773f50",
            )),
            y: Fp(U256::from_be_hex(
                "28b72e0858eaab897179b42fd338e81d980a29887d2fa519ef820c72814925a8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "18e6bf79251bf83a7e656d836abfc65441f3b51c5d9619c024d2b67d7f51cfb3",
            )),
            y: Fp(U256::from_be_hex(
                "17904b619344691cbd9642acfe5071e2be362d889fee1637d4926a7a6211e134",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "13454758a0452a3c75a7e675e2ed5fe82ca925cd01d7b7d8fc8dfed5545bccdc",
            )),
            y: Fp(U256::from_be_hex(
                "19eaeaf93f864d014b9e988f54b4bc41c514aa6ba42de802f4b284203fec78b6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "13c59c87f81537171b580db68287b6182506fa872b63c23acaffdfbf19df3d04",
            )),
            y: Fp(U256::from_be_hex(
                "224aefa736323f7b152c4c33f22cc4680b7a2800facd1c316a683fed089f01c1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1fdfe803652c341bc093d4854586aa35182e8b6a296c45d944590c21a8a4d64b",
            )),
            y: Fp(U256::from_be_hex(
                "2c5b8c5e11d9e419bd965c9ff854b9717def35ccc5c886bad24be97628a4f787",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05dd6411c2ebdaee1b06ef477d62222b339e1384c9bc7213682a2a256a3dc3fb",
            )),
            y: Fp(U256::from_be_hex(
                "14f700b34f6e8c68d82494b2e1d31fff8129dca4e1b891ba3b152a26ceee4720",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2ca10f48ece4bdb2db84c942b7ebc0349bebd7189cbb81ee77f6873ccc589f78",
            )),
            y: Fp(U256::from_be_hex(
                "0cdb05650874c327613d1c3d8df0930717ff427292deccca50613f98a254157e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00881d27b4d02f85dc07291de4decd58260deb6ee692fd2a05faf43a16c0c3ae",
            )),
            y: Fp(U256::from_be_hex(
                "0e81e1c115e5e91d5cde6ead64cc49c9e9752a7045d8e2ca91a38651a6666841",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "10a411c847dbe2af63dd8ac6987d37605fe777a1dafefbde0af6a93a61117a6f",
            )),
            y: Fp(U256::from_be_hex(
                "1e1e0784363d7c967e5593d73a1d152f961f9262922f554fff49e70bb14d44ad",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0a22ca08dc344d90d22754306af7e40b1ba897fbc23a3ff667be08524b0815c8",
            )),
            y: Fp(U256::from_be_hex(
                "07ddd2353cb0e9356aab2523a67393954220b38ff7403e4012519afff066e14c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0ba6a86a69f2f240a9a4544d96b4000722de1649f70f700188dd8c1fc72637b5",
            )),
            y: Fp(U256::from_be_hex(
                "2328b6f364a21347e2a95b9b0a5cf239944ea041ab656309629b8dc1671eef87",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2f4a6bce9478a1f760f538d8590cbb005501777aac8cf45c28d61008a34260a2",
            )),
            y: Fp(U256::from_be_hex(
                "1b536cf4f50baaf1243fd3d8d54d8c7606c345e5ba5fe7b4cf0641f06d10e2c4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2522399589b66c222e8bdbeb4e10b75668bd4e4a16e58fbb45a5ca40d71984e1",
            )),
            y: Fp(U256::from_be_hex(
                "1585bea0b4d54d1fcbdebb9c903bb91d7bfbb56cb778ba4026c44d68c9641c3f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0a916e528c6e5a950d58f52989b03c51f1cd2fe07b999690adc5342c52b83a15",
            )),
            y: Fp(U256::from_be_hex(
                "13def3d8865d65b17e6c67f0ca83123869c762ad98427db431741018397e6218",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0c0ef14754a9c1a89b4d72ccb53fe6b93130050220599b6f05c903a4513991d6",
            )),
            y: Fp(U256::from_be_hex(
                "2c203585cd34ce7ddc05079acd3e17a8ca90d40799734e8eacadf4f2dfc813fe",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0339bec5a2e656979b79d4cdd6497a645adbbb6291c9804c3db18a79d5d5b281",
            )),
            y: Fp(U256::from_be_hex(
                "1abe8f1855bef16a8a5be505372c85d719af03df53bafdbc84b58c3df428335a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2184832081e6c22af2f861ed9b796e45149b2113f742815c4235e24939d35a89",
            )),
            y: Fp(U256::from_be_hex(
                "0d609fe182b38a247bbfe39c1ebd7e860ca4bf2c999588072bbc03a73817196e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "21e80ade9e0e916171304e7bda39f76b79f9c4a9e54a46657bc7bf09a609f79c",
            )),
            y: Fp(U256::from_be_hex(
                "0be334af749121a38ba9b6bc8c4b199f3ba429217a69db12fcb138b2a38869f7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2d1fd89882f47903a9fb0ec29e40ec22f36b6c546fe402b255903e5fe9b919a8",
            )),
            y: Fp(U256::from_be_hex(
                "30137cf675f5002c41e0ab1c248876dc8e74ee583b6e1a4d527f8690508ccad9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1561b846e6c67f2e3e6d7476becbebe432ccebc2fa93b5dc73e805e622095da8",
            )),
            y: Fp(U256::from_be_hex(
                "0349b2e74ce9c3f3fbbc42c0f3a5da94e2dbb7a5130f440c98a0bf29bc1664ea",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0b1151e9038e28dd6925131a1f76b6a8ab2142bf89e4e6d3535065183f1a8bb1",
            )),
            y: Fp(U256::from_be_hex(
                "2584221f0ece1b4e3be682a36be2e6a5cdc354a607a8c3df1f943f2b1b818b99",
            )),
            z: Fp::ONE,
        },
    ];

    //addition values for bn curve
    const BNSUM: [ProjectivePoint<BNCurve>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0cd32146f9b21bd4823342fe5b74834c4097210f0228723314efefe25508fdee",
            )),
            y: Fp(U256::from_be_hex(
                "22ebe8aae6afe99883fb9c183640ebb4fe9f330d29bb71612981ad34e7270fff",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1d33382ce63ba19d5655c2fda825e575c2cb1bc69b2086025c9c308378a623d8",
            )),
            y: Fp(U256::from_be_hex(
                "0b57d8daf1ab092d9ee5af3bf83276080a5928e1df24d4585b2ef6da2dc40910",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03e2abb02e9fe82506959e48c2c239946e85544a8c15a9c7517642d414d4aa20",
            )),
            y: Fp(U256::from_be_hex(
                "2e632aa4334207c1630b9f3083e1e3fd468d6dec05b29c99a919b8285175addb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0660aba05323dd97dffbef41b481b8cb81012464a44162ac006b6fd278027177",
            )),
            y: Fp(U256::from_be_hex(
                "0ba795c36be32017308b4d345fb2f3c82e5307c30331f92fbf15a5842608bb04",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "21717cead4222ff5f98ce7d5c9aa90b97b800fb21733798055feae63347de85b",
            )),
            y: Fp(U256::from_be_hex(
                "2bce93cc09deb4fd032887f9263bae4a565cee602eacda4f4007734af8ce4072",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "232fef85843fb91797f36383eda943795b42b549d201114948d2be873282d85b",
            )),
            y: Fp(U256::from_be_hex(
                "2ec23abd05c784c2e3d8c3f6d6e633683d57292e4f740609d8f6a9384e030123",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2f241562eb9c95189d2a821d8cc52620097322c63fd2e1ab68dcf6184d865e35",
            )),
            y: Fp(U256::from_be_hex(
                "27d5060c81fef976bef1af9b575ba992d66065359d6a0094b6155ee3439ba757",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03428bbfb0eb0cb1b9f901b7884c03d29bacedbaf88031e5111fdf7dfc924224",
            )),
            y: Fp(U256::from_be_hex(
                "22ab86925a0fc18a11051eadfd91fc0c9101658fc94dd6da5283f35b8c6954fa",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "244f070894cc1c05280e8f4766fae9d13dc11f9049f9b16c339a94c5e6a5c480",
            )),
            y: Fp(U256::from_be_hex(
                "18c2f0ccc1bc7e6ae3d38b6c56e2801d5c783d79726cb37256cc5dd785017abe",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1ec4fa802c5e628af94b37f50d0cd63a74a7da87e7ebb784fe92811e96422d78",
            )),
            y: Fp(U256::from_be_hex(
                "2a01483adb715192f286752beb122482dbe3dfebf32cc8db96af24b914fc70b6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1917db59601aac83868d4a91ffa54d0d254c81ce748dea8f2f05a9c4558640b2",
            )),
            y: Fp(U256::from_be_hex(
                "10f98d6c8154e44ac0232ea4be3c9002890ed2f976168870f3875d9d7d3500bf",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1734cd43f230930bf524d20551b0961989cddaa2a0168f39203052836c2ff567",
            )),
            y: Fp(U256::from_be_hex(
                "1fae0949887ad18e2f39449f6f86097c48ed985879dbcf2335a8bc9d181a16eb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "25a30b559523e55ec646d386b601688eab95a7536d58bc44d273db82010283ad",
            )),
            y: Fp(U256::from_be_hex(
                "2c4e22e580be7e40615f663a1cc65af41a103f5bf23b9553f49d64a9f57495c0",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "26208b41c7de3b4ea44060ed8c3c24ad6b92401287f76f3e2214320d4c83d016",
            )),
            y: Fp(U256::from_be_hex(
                "1d642caff1fe8613533338633744c63aed62ff8a5f60e0f7144e4689d3a85c6c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "297c09fe55c2dc360ba94a469f4123469abafdad43583dd3bc782227a6a68c59",
            )),
            y: Fp(U256::from_be_hex(
                "2354c940ddafcb7d1752e8b85b97d0f025e9f1c767d2e50f7ecea317ea8dca03",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "29d91da9ad3fae4f0cc9ff8ce5eada0edb6e5dc7aa14166265e91731291080f4",
            )),
            y: Fp(U256::from_be_hex(
                "0694861cd7dd10b15d511c2b033d04bf64377f045f6f623fb6a6810dd26f3c29",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2f07b887673190366d7b4ba4e178fc644c03756aeac67746d92ec692d8fe18f0",
            )),
            y: Fp(U256::from_be_hex(
                "0cccf476f81d6586e226125f0bb9cf08362a459a14aa10f5cec4047c1461168c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "29b13f717d4e8e4dcc431cc0ee59f6c6edf06460b4731b849debf71b2ef28fcc",
            )),
            y: Fp(U256::from_be_hex(
                "0fc21f59a3a9cecd451c813971b791705f85ae494b2b7ecfc36decd6435e55a2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1aa5b0a954853e3b3d0ca8133e24511bcd8ba511c63244042f71309fd980336a",
            )),
            y: Fp(U256::from_be_hex(
                "0df5fa79edaaa427014dca8ff98fd6c9a404baa5810c8e0fb0c7aced7b88fb96",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "304ca1b7aa64c6dbc124e4f0fa6e1573d4999c95c0a7df8221eb2aa799e1b661",
            )),
            y: Fp(U256::from_be_hex(
                "22ae4332d608ca6292cb6052bf6c8208848152b0b4199ac44b6e5ac2f3c39d64",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0c5cabef6fbc5f7cbaf5dad53f2f0b435c97fba616933343d0e6f91e9bc755cb",
            )),
            y: Fp(U256::from_be_hex(
                "155a7031341bad5c6444177e82417d23713abca7f2945ed9680c9fd21a3f4a7b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1205aba20abcb37810fee9d91f52b3a7566ba747e7e59fea0598d57afa71bfbd",
            )),
            y: Fp(U256::from_be_hex(
                "0ef2e249b12de3f8b98333a5c60ead118f72b4a7275a6df92236ec524b26d58c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02f3b4a09b8efcc8dbbc62f3b08d6c39c7c8a4d200123e9091554aa19da62352",
            )),
            y: Fp(U256::from_be_hex(
                "119af6a352e559b422c2b8ca141c15c8cebfad8d67b217fe8dba9cc6326932be",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "18fd167460e493d1d152a20026483a95de9511842a7ddfc642269a12fa6d0cb0",
            )),
            y: Fp(U256::from_be_hex(
                "133284a556de8c2c5b0971a231667472db4bba4a8a68674025c0d2e7fc9aba30",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "29ca2a602fcf41d868378a7b87e8bb765a8297d52d18eb3ad1e7e0119da0cee1",
            )),
            y: Fp(U256::from_be_hex(
                "19d7367e7c14f972f0be6427384c301d0243ed88b308753c4769d4bcba64266a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1af6785ba12523da0559abaacc6e14ad3ae5e072f80b60555006ed91ff387d0b",
            )),
            y: Fp(U256::from_be_hex(
                "281ce2f94220b129e2c3fe84b8dd9bcc28799df97da31c5ace070b10ee8912cb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "21a145573abad1d3852591a589a49440a25a2d62a7f53b91b8bdfc2dbe53afd6",
            )),
            y: Fp(U256::from_be_hex(
                "2b501964ea6e950ff9a71d4bfa6f6b593d5170ce07eb8587422996ab339efc0d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "208d57dc35f0e335417eab773d49b92789d426071d55e939d25b62d239b22ede",
            )),
            y: Fp(U256::from_be_hex(
                "19cd30f7d6210e75bd945c0168826b24fb3c6e1d7e565890a33648ec923f95a5",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2cfac167b2878da6d7288b2bfe9a63a29b6ae12ddcf28e8c867ec1e3db33430e",
            )),
            y: Fp(U256::from_be_hex(
                "0904f52da1bda65ba6050b608bcaf0f3232688f57a3934a8bf27b9ab6d546545",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "095ccb9bfc41d25bae74ee4298f66787d605d172d3fafb06ba265c7c55e8053f",
            )),
            y: Fp(U256::from_be_hex(
                "0a027c6c06247d62027f21e8b52f7c3ddd866acdd50a2429ee68526c8d331c36",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "23d74582fee1f682cd846d607c0334b568eb89ebc6f5fd59aaafd7f125a4913a",
            )),
            y: Fp(U256::from_be_hex(
                "198223fda7068833c7c794395b6c7d2fabbe9f3a6c02ff7100ee9518b9943988",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0122fe2d37f27c6f46685de4092ef6f682f63e72272d24d280e562b311c5110d",
            )),
            y: Fp(U256::from_be_hex(
                "1efd9d29e1b6c2d73e277237222e486fd46b0781b5b4d6a49a2f87f9180400ec",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0ad2aa0fd67e549af8c296963548dd23387eccb5198b0d5918a5c4f50d08d5d0",
            )),
            y: Fp(U256::from_be_hex(
                "20e6963e490220780d4e52e18359d75c629fa870ae2c1568c84d5389500267d8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "22035645bcfc86ea28fc9c328e34bed83e48eed28a8121399759fd4c68eaec14",
            )),
            y: Fp(U256::from_be_hex(
                "1d995ca2f0767c82470b95bf73e05156569902b8348e106c0b7486050b69e923",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0ad03e27882b9a4925aa781c157e33a7affeeb95db65c61e608848306d939067",
            )),
            y: Fp(U256::from_be_hex(
                "0d47493df9ed55190baa33e08d4273f5c76cf3a2ceb508ee69a8129993f75e88",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "27703c7e48e43df5f61e28bc0c4aabc43c208bf5133921219358872d6af17986",
            )),
            y: Fp(U256::from_be_hex(
                "0e04e4bad19ba20ad40f1c02c78af7af05995e755579136862801aef4f9de97e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "014429fae4616d597c7e71ed83e39647e86de16e46cf94d82c5280561bfabb0f",
            )),
            y: Fp(U256::from_be_hex(
                "175b3a23e5592b6ec17b6f41dfd5e799e53e182a48a0474d9fb4de947950a3a5",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "253e30ca11c868fc532f93c01e2fc944fcfd9e50a288d7c35fb924eb56c128b8",
            )),
            y: Fp(U256::from_be_hex(
                "137990cd0a425d51ef075fed832cd2bb4b1d5ac2dd70ef15e47e46cffaf03c1b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1d0da8b3d2ac59d520c9ff5851486b821eefe3af1b871c007bd5c6dfac8061e5",
            )),
            y: Fp(U256::from_be_hex(
                "1b6b6a5d96c61fa9970c83d66374cca931133de75a962b83b74454288942f6e4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "08b3fea1ac4bb60a1b9bcc56231ff51960bb8f4e35bcabd2d5f348835fb581c4",
            )),
            y: Fp(U256::from_be_hex(
                "06fc0480d2e7522f7ae895d520494021990e6166b3b4dfdedffc182219f54e96",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2a767d6ecc0da6b2a6daa5e4b1d3c145f542894f95226b7108bc5fc34b4d3c8a",
            )),
            y: Fp(U256::from_be_hex(
                "0d96715f8faed67bab2df66513267be0e3fd3549a7e994070f12393b8a377851",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "17697df831e3d1f395c385697fb1fc635fecdc5d7d4e63018aa51225c86976e0",
            )),
            y: Fp(U256::from_be_hex(
                "1139289376e6ad5080b2688c7110844b43c2d313d4caf86c33eb6cd1f258caa2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2c3f6160daa68867d4302fa034cc03a4e0d569f589cd6250c0551463b06c891b",
            )),
            y: Fp(U256::from_be_hex(
                "2549b1e8fc921106c035042cf38453822bb3895e9209e571e93782ec3e0ca4c9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04a94b5c21e4b2ee51a83fd2f17a8c8b8a905e12960b12ee1b061c1512f2c3be",
            )),
            y: Fp(U256::from_be_hex(
                "262cb6af89ca268d9e2ef89c5b01755efc8b12272cf67a2aeb8ea12470c32354",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "08a25b6f922b95276a015ee8e7dc186afdeb6c12c7c77414f78a25e6c22dc12f",
            )),
            y: Fp(U256::from_be_hex(
                "113efe697756d7377b6578cee37e957319f9026dedbb558525961f3923468a50",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "23aa40d8314f29f4ea1f2047ce7c07ff551d8454f023216cd871f862c1a25fe0",
            )),
            y: Fp(U256::from_be_hex(
                "086785cc6bf4f82da78991433098f3297229067a3193f8709c4c431d49f6e03e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2473e8d656dd34cefa58fad2397912905f5075ee8d271be45027da385235504a",
            )),
            y: Fp(U256::from_be_hex(
                "07835162908742e07cbaad6b8ef325c53dfaffb77b427a8a6bcce8acd667e38e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1bc8246dcd420ec568cd2eac10b7e17e7b25a32a21400e549805352fe13126fa",
            )),
            y: Fp(U256::from_be_hex(
                "0564b641823e6e45261c0eede09b2af9b967926e53d3592625965460abf89c30",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1833181e530e517a9271d16d6442ebc6364427e766a528f8fc3864935b8464e8",
            )),
            y: Fp(U256::from_be_hex(
                "110757bdc81f1ac6b08dc9e2a97cb8171b71a81f212b35209d1ac7912f2481fd",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "206b31a9251c3612aebbff34443c2f99722237e7534d74f13d8d972d0d483fc2",
            )),
            y: Fp(U256::from_be_hex(
                "12fb00ab515bf2f1f8ca8c707df029eedeee67c81ad28aa100d08ce7f8cb7aee",
            )),
            z: Fp::ONE,
        },
    ];

    //scalar random values for bn
    const SCALAR_ARRAY: [Scalar; 50] = [
        Scalar(U256::from_be_hex(
            "0ad589e242812a3bf5afad0cf0f1c5d97c2e05f63d32b7737ebbc59a88db35ab",
        )),
        Scalar(U256::from_be_hex(
            "0d4a04d706e1cb04a85f0934a380b89f217f56235ecc0b22e992e38c7a3bd976",
        )),
        Scalar(U256::from_be_hex(
            "0299d2830cb34c55735481b81448c4537ab9a6fbec9268c766938f1087716f0a",
        )),
        Scalar(U256::from_be_hex(
            "12d6fa862c73a57282471e5b44c58f139fbf8e2ebeceb7bed38fd2e433eb6320",
        )),
        Scalar(U256::from_be_hex(
            "1c2099b3e02aa3889528f334c67b0051e2c0ace5edbadf93cfab2f985f3f02d3",
        )),
        Scalar(U256::from_be_hex(
            "0f776bca30bcdf9fc46dd0645f983c5b31f9c9010bfc3b8edaba25f3b3650167",
        )),
        Scalar(U256::from_be_hex(
            "09bf7fba53e7b7102bc94baaf2181ebb17aef63d7c79053e84445e825bd899d6",
        )),
        Scalar(U256::from_be_hex(
            "28c97a7f05cc596b08d0b5eb7dd840dfa36e7af2580b5c193e7ae874a51ba79f",
        )),
        Scalar(U256::from_be_hex(
            "0c6de8a9bb5e4f42b971332662617c420d89597be9864a72cce0b3f32745c6c6",
        )),
        Scalar(U256::from_be_hex(
            "017d31c94c126a5409cdd54f35e2bfebb82b65d45d897a88f017cb4223bb67fd",
        )),
        Scalar(U256::from_be_hex(
            "195ce654d77db8583aea7a724e10872a339778d30f0bc4384affd8527b63682f",
        )),
        Scalar(U256::from_be_hex(
            "22480e1e23310ebc42b67c84933d9fc967bc368ead4130d4b1902e458282cb88",
        )),
        Scalar(U256::from_be_hex(
            "25c86531b18312ad8fe299d10f8fd253ba31c4b4bcca8e3a9cbcea351f70946b",
        )),
        Scalar(U256::from_be_hex(
            "0de71cbbf87aaad8c6d8b4c84a717aa3308be055b0585339594ec35c879077bf",
        )),
        Scalar(U256::from_be_hex(
            "2ea8c89d604d95eae54d2db73b18f3193e44c6b5b2dc16fe46445817d157ccc7",
        )),
        Scalar(U256::from_be_hex(
            "085685a811caac1a8d82be5e3da7a76c478ab00e6a0b382d6996861b3add1170",
        )),
        Scalar(U256::from_be_hex(
            "163cb6f255f811a76e042c9f9b8c0d9cebd1562139b5c5473a5ef9f3646dadf2",
        )),
        Scalar(U256::from_be_hex(
            "035517809b5701fefb30919a3d93db81f2c7aca05aa72607e89f35deee6ebf2b",
        )),
        Scalar(U256::from_be_hex(
            "21a7cef4781ff70ee3583167041f2ed652aa9355de1bd339d5945ea4c53676ed",
        )),
        Scalar(U256::from_be_hex(
            "2213b8c6068395fca249e9e96bd385e2b0bc5fa25a240fb56ffdd17dcfabb10a",
        )),
        Scalar(U256::from_be_hex(
            "12952598194821d6c77b5df0c068e3912fa046638c9611ae8832b4018701a792",
        )),
        Scalar(U256::from_be_hex(
            "03d7b121ce9cf6d2181a9d12031735b6f6111b218231b8184f72f0e01803d981",
        )),
        Scalar(U256::from_be_hex(
            "1d51622358dde1d469b3eaad06840d303a454c165e28d176d9e5148121b34fbf",
        )),
        Scalar(U256::from_be_hex(
            "2f2dba204dab1dc24ade6a8d126d130b232774cfe9bfbb064a78c9ec462b8e19",
        )),
        Scalar(U256::from_be_hex(
            "23fe3eabaa4a46ef39fcb750e06b8936382dee0787985340047ee0df6fe37132",
        )),
        Scalar(U256::from_be_hex(
            "15114d451a20108f6ea9e01f9b68dda9974bac6dc88077cd5414986683c34be1",
        )),
        Scalar(U256::from_be_hex(
            "1bffd9ce82b3cddbfce5698b66f3dbc7e295d55b564a2d13c1f984de67ead8c9",
        )),
        Scalar(U256::from_be_hex(
            "0d8a3ee530f1b6640ac2b83fe39f20690fd9c1aa4c06dddced1f904b346123de",
        )),
        Scalar(U256::from_be_hex(
            "074d863060f3b36475bc9f4c34ba872ad6662f8f4551fc7afae20cfc09ae78a9",
        )),
        Scalar(U256::from_be_hex(
            "25b1147300b2bc3abce74f21eb1c405a62421223a9c9568c14a316de2e6bf4d1",
        )),
        Scalar(U256::from_be_hex(
            "0635b4453a1a24ca7250c145cd1097f1047ce27f3db769aa8c58b9bde13ea6b0",
        )),
        Scalar(U256::from_be_hex(
            "20973dbddf95029a7186e66a9ace2be49b489422151a957de31dcb5a54953e4d",
        )),
        Scalar(U256::from_be_hex(
            "2f721185dafe21cb604fdb9bb9b78302fe8767d5f2e98d93f2d178dacc960087",
        )),
        Scalar(U256::from_be_hex(
            "0e17f5431de5a883bb46d92270356fbf50f92cd2e75c8b60fc8302282da0f573",
        )),
        Scalar(U256::from_be_hex(
            "286dc82b37119fada1eca5330e9fe3de1123bd4e1e4e07ec3127b7e70f9b2a0e",
        )),
        Scalar(U256::from_be_hex(
            "2bf003282734dcff18197f5fcdafa136ebf6eabd9e3fd58aaf2ebb351a6d9459",
        )),
        Scalar(U256::from_be_hex(
            "2d122c9d3b884c5a335d7d62b963ae5324a82bf226b825c611c1e5970d90b462",
        )),
        Scalar(U256::from_be_hex(
            "2b91e6b44c814bbbe19e27bc8557b19b89b7acb7dd09731f5730404f03c007d1",
        )),
        Scalar(U256::from_be_hex(
            "2065cde76d24ddb783daf47710f0fb2672a69a572d5376ee5b0f508308fd02d8",
        )),
        Scalar(U256::from_be_hex(
            "13142bc1d94706bd73b9d3060831226eebf622a8d21c3ec78bb58ae38fa8d876",
        )),
        Scalar(U256::from_be_hex(
            "0cdf5679d1b6253366e9500a5ac9fa1065db32dabef505fcc095e29df740d5da",
        )),
        Scalar(U256::from_be_hex(
            "131ff02555a5291e1651f0dca58f26d79d1d50e062fac4a549a223d2de5a58e4",
        )),
        Scalar(U256::from_be_hex(
            "0a762f08483b5387db8091ab00d2f56a3d49e047eaaad581076ca7d3e18ffd7c",
        )),
        Scalar(U256::from_be_hex(
            "2707f30d35a341f4df6da09c30e955923e5b1df756d05321a23a0c7cae56f434",
        )),
        Scalar(U256::from_be_hex(
            "154ac6911866090cff92280d02bebfe8d6ffcc5f7d882609726929617a1087bd",
        )),
        Scalar(U256::from_be_hex(
            "08501ca7266382cb16aef270c7244ec7fe96eb7af3fa5803727c6c3c0be07f4c",
        )),
        Scalar(U256::from_be_hex(
            "2b8b67120c1e0abff83204ba2d4b019517cdf861be1d99d5d177d85faa23f683",
        )),
        Scalar(U256::from_be_hex(
            "1e12effcd06e783afe2a7eddd5ccaf1a24edccb7eb5f78a514ce578a84bc36e3",
        )),
        Scalar(U256::from_be_hex(
            "07c074bd2e1af90099526601633d83936663d06919bf90c1e16f6e5bc2618151",
        )),
        Scalar(U256::from_be_hex(
            "04b34c45633aadcb5190476ee466b9574800cab11a3099e43552a4a86a04267a",
        )),
    ];

    //scalar multiplication value
    const MULARR: [ProjectivePoint<BNCurve>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0fa5541ccc4e82caab0f5c92dad87d97a993fc49a3414d931fc3819e82daeb3e",
            )),
            y: Fp(U256::from_be_hex(
                "002d9462f1c74b4276c58cacd4a10b6c5a6d3177224c61e8582b12f0dea339fb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2016026455303ef9b95149e18ef3f0178267492e8ab1f39f3f5b0f3c48eab2d7",
            )),
            y: Fp(U256::from_be_hex(
                "096a011ca5f80525dd878ed1f53033c0f56d7ef42ad399f216d3b00a2ebfd238",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1d6e91d8da7a9cceea5f678c88ffc7249d1fa9740be7251f1fa7f172e51122c7",
            )),
            y: Fp(U256::from_be_hex(
                "1882f49ba634713bbb86ef1a6db822dfd2dd4cdefa5913ce67c0376ec1a0cee3",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "302e6ec8181ad8fd48ed5870507a7c99ea74904eec4531afc5911fd5a55db117",
            )),
            y: Fp(U256::from_be_hex(
                "0bcdd4196a8b8ebb7361ee4d6460327d1c3720e9b31c7819c8f9c62c0c2ad04a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0200b4f8b9202bdf8b7106f60f57560b00ab58754055535274c8fa091217b366",
            )),
            y: Fp(U256::from_be_hex(
                "2710d6ecf2f7a932f562cb918bf8917347e2d4596ae447ecb4050124bd16f215",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "160cb0a37259368fc8219c7daf2d427cd657c3cf7c8892e83c0e1cbe54ad09ef",
            )),
            y: Fp(U256::from_be_hex(
                "20fc4703bb31475e78ceeecd897371cf13a8562c68934b2d7499b431e87b485a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "23d2f165092458edf1ffed560122aed5ff4f95cfe778c70e0662cd8486ed2106",
            )),
            y: Fp(U256::from_be_hex(
                "23faa44ef873d7f91fa4955f195a6a542558f0397a4f28b7bf8d0c4f41d491a8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0d2e3fb0eda6e47fd12fcfc1113835389d2521799107dd023e54e13017e04e5c",
            )),
            y: Fp(U256::from_be_hex(
                "12b424a80a5d0d10ee9dbe65149a3030869d5ba4ed403b82d681f9dbc1f4cf48",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "180a0bbef1bba76c5f07f48b49d81c55607158604fff2c44d1930aef2fe6bbeb",
            )),
            y: Fp(U256::from_be_hex(
                "217707da1ab20a23fd4a1a46fdbf3af2cf171ffe5786b6916f1a5a65507bd506",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1fb370af01775be3f76cc4c18284671480563e51a498f97eb061277a5df706af",
            )),
            y: Fp(U256::from_be_hex(
                "0569c001ee8aec40889874e74e888b1315d7a0ab9855c73c09f954041ceec72e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "22148f788f9507d4393b3ad93052861b80497a51ea44c81621200d875e792cf6",
            )),
            y: Fp(U256::from_be_hex(
                "1549cc8f05b5ddbdf27e577849570fa3f1f1d548a2d2876421d45d28fb24fc58",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "279c05c87125c65b18b0c6f5d9f3937c451911580303e4d9e690ba8027628a6a",
            )),
            y: Fp(U256::from_be_hex(
                "1176b3cc5f09302d3811309d257b94f4ae7abc18ef6ee4ae4db0407a8e4113c8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "27ca818af52ecf77232b53ccbc4fe3733c00c2f99573cb83cf805e1537ba5a82",
            )),
            y: Fp(U256::from_be_hex(
                "01f0bc7b146788598ffc02bc0d741e758e9f2af8708988913e33e80a746c2378",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "15106c74358a97d435c84f2d176826ed56d241256a686c1e359cd89045bca717",
            )),
            y: Fp(U256::from_be_hex(
                "218ae642cfda1cd2cba2f4a0c0a15245ec399803e4322220e2488ffccca220f0",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1a16a700eec2509484b50b03d34159a048c711608c43fa5fe788af644ee69d7a",
            )),
            y: Fp(U256::from_be_hex(
                "0a841ddf9445c835d295e2d167554ef977d7ab3693ccb940932ed916fd5d06bb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "222d3b57dd298fa14e58a3a13a911917194dcc86b53a448e60ff8ba8f48f863a",
            )),
            y: Fp(U256::from_be_hex(
                "27fc57c59dba40d49330e6e0855e9a83747785fabb52708b4b8a864d336be190",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "21968bda60770e7f1fcd41c7b6dd6f93cdd583b5042d90732de0a9b198ad18f1",
            )),
            y: Fp(U256::from_be_hex(
                "1f398ea114f883e8ea405e6e51dfb57283af2dbf34e162dbe0b1194af475ee6a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05aca9dc50f609083b3e487c9b784240d1ed5a6cf07ac43822e2f6ae35fb3f5a",
            )),
            y: Fp(U256::from_be_hex(
                "0ac0f454ea31d82ef11d70d58bcae55fa19ad52ba45bcdb65176de13bb87780f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "17ba366137fac58a60f4baca48515fbb4c3f449e7d66ea44d7f75dc80c4a3200",
            )),
            y: Fp(U256::from_be_hex(
                "0350184db9a4169185bc24d2a9c390f8986b2ac7289b92f34b8d43c3be42f682",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1b81fb88a2a360a6b9ee26777bb2ea0c34f3f318f2611f60690c06ff7d35550a",
            )),
            y: Fp(U256::from_be_hex(
                "30376d1c008f968a34de982eb5041a80f9a3e4070d052fe343c02ab5bd58d4a7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1409fa3fad8f43e5fb7bca7d0f7cbe7c04f14ffaac9b63fe99ae91e762063116",
            )),
            y: Fp(U256::from_be_hex(
                "2f2a38998c09fcefec3da6eb8e4b1522acf89bf02498752aadf6dc589e13d72a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2541de31e7f943d041e3e9cbbcedc080f8bd4021f5a132f95e04510b1db7663e",
            )),
            y: Fp(U256::from_be_hex(
                "29c83ce1427763aeb545290adf321f5aab500dea27c051eed912a0044cbef5ae",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "260d185aff1cc44877be20e307346a9b2746be53b454ff27cd3cf3563a24569f",
            )),
            y: Fp(U256::from_be_hex(
                "1c781fd6adda58d3404d7cda8dbd1082458ddc2cce6a25bec1fe36c93d95a149",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "14a3ea5a3c74833d7104059feab2a4b2751adf137e270ed75333822d727bf4a0",
            )),
            y: Fp(U256::from_be_hex(
                "14ff192c50844034f4e085cdc033478453cf42064047c9112b8df7f0696dfedb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "039e33b3b5f2a8e6512a3d565406e4800d5814f13442cb188631274fe42eee34",
            )),
            y: Fp(U256::from_be_hex(
                "036b1276e65fbbfd6309110e48509cecf18289b98a6b7c97154ec4ceef659b5c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "224e97ffdcca22f658054867dd2ce6ee54527ed1f37cb34329d8cd20e5271059",
            )),
            y: Fp(U256::from_be_hex(
                "1873bb7b0fc3f7a84d93e0481826f53092e594462627200bf8a467401e0891c8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "06024aea3ce22699f82d6a37829547639e0f07d3bc4345ea8d1119e3e3b9506f",
            )),
            y: Fp(U256::from_be_hex(
                "2426cf22b0a12a6509f28796bfcab0d57af5a5a72ee9214972f50c8affad22aa",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1c62712cbf1be71ad219a7ea8ace6615957405a719dc50158f8a6c26d96e076f",
            )),
            y: Fp(U256::from_be_hex(
                "1dfc6fdd727a79b1c11912a64af75de17d9db9028802409a85aa42c59bf8e7b2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "15342474c8d3aaf21b6233e1b6f373421ebfe1bce05b83248ed804957066578a",
            )),
            y: Fp(U256::from_be_hex(
                "291f670b93e2693f23a7b020db60633e53ca70c5d5e179b7f5e6b9600b10bd77",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "230ccf9ccb2fbda6511c599ef881f8307ff19ee8dcaf97ac50e05ec672583c72",
            )),
            y: Fp(U256::from_be_hex(
                "1b682b78bce5da9b25d78afb4ad04caba333f1e259d69e86428b19ab15b94cb2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1553cbc9a7411ff66e8f9b09bc3d45ff956e609b70dafc534949bf47853386dc",
            )),
            y: Fp(U256::from_be_hex(
                "2a33b105fd846d1d50685747076bb7be0cd6be6dc523bbf2bd6218128cc48426",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "30048d3d8fd136bd858f9496b18f11aef5dc7b852a9560f7f91ea552976f827a",
            )),
            y: Fp(U256::from_be_hex(
                "237ac14949cf7235fcca7f96c2ca74de15f007589c503fc99b9e2d83e438031c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2b18c26c18cc71316c4074fde69cc7d08b262fe37ac4e8cbe6633682e8caa641",
            )),
            y: Fp(U256::from_be_hex(
                "0f2f081ae48dc30a0fb93fc314106acb64840a134b059d7fd079cd9861581623",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "261a2c2a6d038dcfed4ab1e542377b8d93beab1dfb28ec26739b1021584adc82",
            )),
            y: Fp(U256::from_be_hex(
                "15aa513aeeb5c5a44e8bd03ea90b7d56d33b799872463754fac274665849747a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "023b08ac9317f4d0b1e3e97a32311bed86061e7fe37469f01ad6d71e5af618a1",
            )),
            y: Fp(U256::from_be_hex(
                "19255c4c55a0670fb18413c5ded797b1bb2b6929717daacd5e39ea49b6261f54",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "28e93c7429626b97a36ec38d8f5198eb180643ec0f97710bed7afa40e84324e6",
            )),
            y: Fp(U256::from_be_hex(
                "2f20334ae89054bfaafafa84d8742593ce6b6b9ed2aa001a5f817d51f3493f69",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1336d7b517b303bc419df432975a7ef9ff3e918ea8843fa69ef7a66156967cac",
            )),
            y: Fp(U256::from_be_hex(
                "01eb15975a2358f479d23595f0022360b9b46de683dcd9ea02b78dc9bc0bb36f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1a7ba963448ddc395b69d0a9e5721b7f0076c9a92373f4e58407779afc5a33ef",
            )),
            y: Fp(U256::from_be_hex(
                "03b1bf4e7e3149fb65caf0411df1b7ec0a725df327a02bca57fa059a32cc30d2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "098d1f019b792fbcc4cce310d2afc07dd799fa7e84132e30d0d4bc70a6a1af11",
            )),
            y: Fp(U256::from_be_hex(
                "08cb0e25207d78a09da29c5fcbdee862e4e007e8c330e9116a0cc891abb486ef",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "13f3cb388eb3318018e70510fb9a72a8e7b64a1129616d8be8aa40a34631d433",
            )),
            y: Fp(U256::from_be_hex(
                "120608961b981bdd0f4bb77cb3d2dc34b139815ecbbb7c435872f7877ada3ec9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "046ddf68362c2424df77187443df83cb8894c9e831e6964f314cd333a0aa184c",
            )),
            y: Fp(U256::from_be_hex(
                "177561236e077ab0ec8a2f4e392fd2ea10685ca47816fd54a4f9e87d62ed07e4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1c65efc47f5ff721d6c49e216241a0d79ac72b7e67885c77200c726015ace15a",
            )),
            y: Fp(U256::from_be_hex(
                "1fbcc38dcf5849070084721df1e0acf1d231a276f0dc238656f1a86eb0f1b3cb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "28f251774bfc5b38a4299e7b42f3605749cd254111e0ac5683ecc3e62f636987",
            )),
            y: Fp(U256::from_be_hex(
                "07842333a740d384ff88932a77932389c8450105098f8de7f01b84d5acb38366",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2e670034c22fdf5d00ec90f97824064378fd064ffcc3180617b17def9dcb885c",
            )),
            y: Fp(U256::from_be_hex(
                "11b2a20ff58bda8459baafb90f21520c7cfda329082d0d69060664ec354b7fb6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2568162295e56b1bfe5057c7d0e688b9ebee26e1282e0c458617bc9350ef106e",
            )),
            y: Fp(U256::from_be_hex(
                "007635e2e5b7dd82ce5880c95e54d944f85da0bc7964e334bf36a6dcb0d8dcbd",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2718231545aa843693229ac2bea7a93c8cdd02b00c59151a69b26fca0804824b",
            )),
            y: Fp(U256::from_be_hex(
                "0223d7f1241feed0cd83a041cef837b9624e26320d61cee9263e7743f3b251dd",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "232c072ca3622de321ad66af27fb7a1ac71243de1a95d8c1b8a2c968f861255f",
            )),
            y: Fp(U256::from_be_hex(
                "1070a3af73356202815f354ba878b470369afb09527593369a402d5b251b7f41",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "254c4021e54d5347eafa48df9a556a031239dec60d35ff1a0a06e199cea5310b",
            )),
            y: Fp(U256::from_be_hex(
                "07e326c5c03279d60583cc0addd50f17ec4de9ea6caa51043729e9dc19682d7a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "2dc4d65cce2eb28f926c43349858e1ce31098f31ed2ec52a16fdb5361a9e0bc2",
            )),
            y: Fp(U256::from_be_hex(
                "109a0d655e79fab85e13887d94ab067946ea9a2452018fb0f7da97148ce6db2d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "1623b06bdf4d571cab1c221262b8c0777e0525a3c9bea0bca3533c876d495c58",
            )),
            y: Fp(U256::from_be_hex(
                "2f59ef7aebc6e5e83efb979d14a8f1723381ac3c629be206f6518540b803d8e2",
            )),
            z: Fp::ONE,
        },
    ];

    #[test]
    fn addcheck() {
        for i in 0..50 {
            let a = BNARR1[i];
            let b = BNARR2[i];
            let c = BNSUM[i];
            assert_eq!((a + b).to_affine(), c.to_affine());
        }
    }

    #[test]
    fn adddoub() {
        for i in 0..50 {
            let a = (BNARR1[i]).mul(SCALAR_ARRAY[i]);
            assert_eq!((a + a).to_affine(), a.double().to_affine());
        }
        let a1: ProjectivePoint<BNCurve> = ProjectivePoint::GENERATOR;
        assert_eq!((a1 + a1).to_affine(), a1.double().to_affine());
    }

    #[test]
    fn add_mix() {
        for i in 0..50 {
            let a = BNARR1[i];
            let b = BNARR2[i];
            let b1 = BNARR2[i].to_affine();
            assert_eq!(a.add_mixed(&b1).to_affine(), (a + b).to_affine());
        }
    }

    #[test]
    fn checkmul() {
        for i in 0..50 {
            let a = BNARR1[i];
            let b = SCALAR_ARRAY[i];
            let c = a.mul(b);
            assert_eq!(a.mul(b).to_affine(), MULARR[i].to_affine());
        }
    }
    //................
    //.......Testing for multi exponentiation
    // #[test]
    // fn checkmulexp(){
    //     let mut points: Vec<AffinePoint<BNCurve>> = Vec::new();
    //     let mut expo: Vec<Scalar> = Vec::new();
    //     let mut ans: ProjectivePoint<BNCurve> = ProjectivePoint::<BNCurve>::IDENTITY;
    //     for i in 1..50{
    //         points.push(BNARR1[i].to_affine());
    //         expo.push(SCALAR_ARRAY[i]);
    //         ans+= MULARR[i];
    //     }
    //     let c = ProjectivePoint::multi_exponentiation(points, expo).to_affine();
    //     assert_eq!(ans.to_affine(),c);
    // }

    //.......................
    //..........Testing for Pippenger
    #[test]
    fn check_pippengerexp() {
        //let mut points: Vec<AffinePoint<BNCurve>> = Vec::new();
        let mut expo: Vec<Scalar> = Vec::new();
        let mut ans: ProjectivePoint<BNCurve> = ProjectivePoint::<BNCurve>::IDENTITY;
        for i in 0..50 {
            expo.push(SCALAR_ARRAY[i]);
            ans += MULARR[i];
        }
        let c = ProjectivePoint::pippenger_msm(&BNARR1, &expo).to_affine();
        assert_eq!(ans.to_affine(), c);
    }

    #[test]
    fn checkgen() {
        let a = ProjectivePoint::<BNCurve>::GENERATOR;
        let b = a.mul(Scalar(<BNCurve as Curve>::ORDER));
        assert!(b.is_identity())
    }
}
