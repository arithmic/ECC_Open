#[cfg(test)]
mod tests {
    use crypto_bigint::U256;
    use curve_traits::ProjectivePoint;
    use stark252::scalar::SCALAR_MODULUS;
    use stark252::{field::Fp, scalar::Scalar};
    use traits::traits::Field;

    use crate::stark::Stark252;

    const STARKARR1: [ProjectivePoint<Stark252>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00c381400b37efe38d58a745bb650e3e1e6e81c89b6c58b89c9ff45b5eb6cdc9",
            )),
            y: Fp(U256::from_be_hex(
                "069fe889a0406f0ab8efe1a607f6fb597e78092e0b22b9f4d351ce1614331c07",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "062fdb73420ca60df0729d996363e095fdfa4e469aa7ad7630e01420bcbfe056",
            )),
            y: Fp(U256::from_be_hex(
                "01730cc324bb707e2b880cc0e1c3f50d7016ba9c449c989e023c8c36cdd67ac9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00bb2bbeaaeb4d39c52f8ef49dafc03c3000f9a3d4dcdd6d6a85e7917c85ff2a",
            )),
            y: Fp(U256::from_be_hex(
                "03a85a208c0aa0fd98ca7f98a402443bb691c0ca319919e786fb7b892e222fd4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02707e50686acdc784ca770b6d2684c1cc849e349a56c47d581cffc100f47c14",
            )),
            y: Fp(U256::from_be_hex(
                "07a52c194db71eef4764b5cd9aa3ee0eeab48a9286e095dfe3a6c1ea2e1863a4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0033f0f456dfa200b6fe09e494d99eea98f86c8a6f424d8bc32762918380793f",
            )),
            y: Fp(U256::from_be_hex(
                "051eeb77ae0f7fe4a3f0a378b8ae594a46490e7e77d03aaff788970800f877d7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "012cd0e251041e24105af9cf4564beda32c8845d88131c4c06051c5a945f7388",
            )),
            y: Fp(U256::from_be_hex(
                "078607019ee9f34219126059250622292f7f0706be53acc66114259605c91de5",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0665411f75e7668e0702674254a3c506369a3a8f39a19f2643144af7ec3056a6",
            )),
            y: Fp(U256::from_be_hex(
                "03c5f8e45cac1347a21400c7d41a66b3394c02d67020556a49752d3529b0550b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "07e90c18129e98c7a90ad7429f8939eb8cb46746746bbd541ac422367e2a7208",
            )),
            y: Fp(U256::from_be_hex(
                "00ece5d5559cb34501d7ed192c5225aa9681fb156cccf977e17403f0cf55e221",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "023703545a2522a1fb2878870b4d9cbca78304db3195e89ca5f31e9a7ef8dabb",
            )),
            y: Fp(U256::from_be_hex(
                "006d4128472a19e3fced215c2aab3e071f7141126856778966251a39282d869b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "070bf57b6fb2617e83fe890c8208b89f65a18c4136ceed87f056e83635c8226a",
            )),
            y: Fp(U256::from_be_hex(
                "013d3e1f175545569b97ddbbbce8b8ec38c46a8b5518ca02995eecc4a01afd48",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "044b5f25fa432d3a1aff427044fc7dc983bab36a102e6409b990b290e5b4defa",
            )),
            y: Fp(U256::from_be_hex(
                "038bff2431c87bb275c750d9a4e7cc8bcb09b267013d3863a9f710c6af712477",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "063c68d7e2f4e6a5c06ed91eec1ff3adf446c49d083e1a4e19f4beeea920a780",
            )),
            y: Fp(U256::from_be_hex(
                "027a1d2243006d7bdd9587cadc833c79c39d0599822e9aa7ec55fe5b561cb0df",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0594c27f85fdf5588fc10cc8fcaa791818075acae3d63dc930a8d1ae3514c260",
            )),
            y: Fp(U256::from_be_hex(
                "07e6456418bd23b534e6e94c66e329eca21dfeedd4306fe14a27d2d0a75e108a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02a6cd7e3fc23026043651b3739025a249c810cb9620f5e3dbaf51638c1ab030",
            )),
            y: Fp(U256::from_be_hex(
                "036e59bcd46b16820f7495030242f2585b0250471f1726301912fd22ae3665a7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00aab037ccf00db3352b388135a34ae25710c37d121cecbda7c6ed444623500f",
            )),
            y: Fp(U256::from_be_hex(
                "03186c6670bf6f7872e18d7d7e65c113f96c833bdd43265cd74e246358d4a297",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "072b72c7c84aff63aab8ab646287806e8ba446f2c85e4f75b19d70fc8929cfa5",
            )),
            y: Fp(U256::from_be_hex(
                "078b694e313a71fd0cc2b74b5d04ba5612cac9379d6f1524a3bdce9a6fb028dc",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04b2a3cddc301b829536a9d2e6249900d2a9e3fd02706dcf965eb65798840b88",
            )),
            y: Fp(U256::from_be_hex(
                "000f96c6a922b24365d525fcca3cdc5ab4b73d8e833a9b99eba25742755fc71c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "061be2a2e144afbe42526d8013087ec591e41789bd31ee187ac6fb97cee3cf94",
            )),
            y: Fp(U256::from_be_hex(
                "0559f01131d048e321f02e63b467cddf32ddc7c5ad1029cfe65972b8b8d7fd8d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05a7d2036b0d69d4159762a8effda199be98170d65e252af33fb0692699aac3a",
            )),
            y: Fp(U256::from_be_hex(
                "061def26dc40fac0077e0b62cbdc68902037ebdd747978ffc778c9340fa5df3c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "020f10e3803a561930fa508d6004caff4077c87016e378955f50ce16249cf1c8",
            )),
            y: Fp(U256::from_be_hex(
                "072aca2891ebc5915dc0630efb21637cb37f6190aed2a5b6508f3322d93de790",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "010b535d561d7aa00636545fcb3720bb978182374933a979fc4db2c1d3dc3060",
            )),
            y: Fp(U256::from_be_hex(
                "065a7b6706de5c5ded8c11a9f5a04ef5a5f7a728a18c67d235aa49968738079d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04fe7641a7632081578f1556dcc2c0aec268c438e80dbc681d57dbcc2db15d47",
            )),
            y: Fp(U256::from_be_hex(
                "020c922b37dbaed23cd82dceec9db945e51db79cd6e7c3b7903007a2ce5b4f67",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "01a2470c4deb251d74d303569f7f9699de34424cadbf829d2896d2d9b1590f50",
            )),
            y: Fp(U256::from_be_hex(
                "0491ebfb642eb07af3904984465bb940234f97e675608b08f99111bde8d9b9a6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0392bd06b1ae80876958753aa2ee382a952d387ce818d60d8710bfde6ff71ede",
            )),
            y: Fp(U256::from_be_hex(
                "03725c6e43c585dd1f3972a6b0fe2a2b7607bbe50d156c617ddd4c21e547dca1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "06f511c90d3895d54b97009879e66d7b783848d4b04065509ebf68d1c9123d93",
            )),
            y: Fp(U256::from_be_hex(
                "03fdbb0204872c5529576cc2761448d70bf8f4a4cc3d78a0a437fd82e80072f1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04d5849d720e07391ea80ff8648c6f7dc103b2189980cdf7428733fcb3bae7cd",
            )),
            y: Fp(U256::from_be_hex(
                "05d43bfa8b06b6ea0dbb89c4bda31d278f600b15f7d231e8c99e6b12b4e06292",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03cd7d119b04e54763aa13bcd354b1c41dfaa1294c93a5cbadc2fe244e4a7db0",
            )),
            y: Fp(U256::from_be_hex(
                "0693cf1ca092d7c7f1fa2f802deb89045a9543251cd4b60c295219b3a75cf8ff",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "068760b424dfe9354b3dfd010ba890f49691fae668205a3db06c76f39f649455",
            )),
            y: Fp(U256::from_be_hex(
                "07af2e4a9b19597c3eb52883c93b01f7735ed73fb73dd3d0d63173356bd045b6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "06b7421b78e2ffb97049245e2bed6ce0d8029dd3e602ed2ae556159838a6d858",
            )),
            y: Fp(U256::from_be_hex(
                "03b3f514e83e9500e333906a3de63e3f988191d55a5f45e42a9d7bfd7d01e064",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00a9b9e51484cf0e586f530416e6d5b5bd6bb253ab3067fceab8fe0498f50cb3",
            )),
            y: Fp(U256::from_be_hex(
                "04a758d989bffd2c156464e2847abd8fedcf06fd43ecffe24cba7bb4ab19fe3a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03d95eda59a8e4d7b4ed03099679b74692c0982443036883d45d917dd91f356c",
            )),
            y: Fp(U256::from_be_hex(
                "02e87c74ec4e78b6503e78f302a2e3debea2fbfe0c09f7b06e4cacb49480806a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04692df4701aee35222a819ebfb4f5f03ae46943a99aa3982c2c890c63708666",
            )),
            y: Fp(U256::from_be_hex(
                "05eb4290aca5be2f36e9264baad2b6ce2be9de43e13c6abd6999afc302f28465",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0756eaf60fdd0d4d3811070c799a77c1d12ff35427c3582f29dc46179e5507d6",
            )),
            y: Fp(U256::from_be_hex(
                "017c0f6fbc0352ba1ced3ffdd28a811eccc66baf3a26730aa540c5086e2da7b6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "049eace96f59b3cdf5443c5c40d4546444fc1c6a61953a1b0b9016082400af8d",
            )),
            y: Fp(U256::from_be_hex(
                "014b05b8463bab961b7e3515439e415a4ca780b0bae76998306101d135461c6b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "056cf628d85736660d32d7c9c9047d2e4cb5747a87dfe839a236ff70a9417781",
            )),
            y: Fp(U256::from_be_hex(
                "03a7117a51f974347a73cbfac8346da78fad3074158f7b1141a14531b2c0e034",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "076180ec5e8d91572080a048c2b72fdccbec4ad58f10ec77adeff13ca5092208",
            )),
            y: Fp(U256::from_be_hex(
                "0259a9b25926f4cd5be8046a6344ee5861107bcbde80e9756267adad303cfc08",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03aa905303ef1e52e3a8e2f695ceb2a05977c1f28e4117de05f840d5e753ebea",
            )),
            y: Fp(U256::from_be_hex(
                "0551ccd20c165082b6eb753a2eefbe0277b0fce95f207b80790aa2d2a5e7f303",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0621af547cbb756587cbb7c6e5e6c5df9b83d9b029dfdd2bd6a6b4ec11b14f4a",
            )),
            y: Fp(U256::from_be_hex(
                "022ff143f50c76fc5548dc4a9b63b8406b954825898b7dfbc0e23c037f916c40",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04035c48508d844c989009df4b90164f04439dd7ded4273bc4e43baa9157d2e5",
            )),
            y: Fp(U256::from_be_hex(
                "05b52cdfe32652e2cdae5a7d29204fb0a3e9f09fb6e07d9b9b7cc0b24372b9be",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "050ed0e6d93d7dc695c10e0ca1e2de320d9d71590ed7705b42d36e25f967ab67",
            )),
            y: Fp(U256::from_be_hex(
                "045deeed090bcfac542e0e111636f77a71eba67d0a79dfe99669bdaff3b4e0ef",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "048e9c783bd727de7a09bd6b34dd117319414eeedff14ae22857c83cea477f63",
            )),
            y: Fp(U256::from_be_hex(
                "01c57a526fa4ecf2987d8685e1bd15266a6c51365927a579fe033e6b523b0060",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04d1f6d19d66e690c30537532dd0af14e4ca718f18d3f04e7a85fd854eb7e1c6",
            )),
            y: Fp(U256::from_be_hex(
                "003023a2e82cba0bcec1669e84833aa878e924db6fb8ad7a423922e311b551af",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02c581a4cdac7e94e38fa159917d34afe58c5aa94e064d0f23d9bc6d03782e69",
            )),
            y: Fp(U256::from_be_hex(
                "02061b600d9e3d0a895cd37192274062b8679ea7fb4523f3882fc9a1085d6b09",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "042ccc7eb51616d9a9e90257c2a3b1155e361080d5fb4b23e61852c86582a268",
            )),
            y: Fp(U256::from_be_hex(
                "04af693207f83803d862cc112ef33402d88e841a31e7abe229976d9809cab579",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00021cdfc5430615a152a2c448707625b64327546afc2d6bbeebe91025e9b845",
            )),
            y: Fp(U256::from_be_hex(
                "06081766bbfa90f9f002eb1cc44e301f0d18a45ba685fc3ac6f31d120ccb17a2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "06c69ba7f517f70aaa1fda9b77c999dcf33ab8a1aba2db1faddf972d6b576844",
            )),
            y: Fp(U256::from_be_hex(
                "02d44f58846d994fd54f6120da1d0cfb2ce0fcbd432e962d57006fa1f5af5a2b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "07e0dd37b953f02583699165aecb21dca54677db0a063b369de78f8fbf3ec0d6",
            )),
            y: Fp(U256::from_be_hex(
                "0694dbb4f9902509def67cc20899a4463533351533e4384e2f26091c02384756",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03b211f4197395b0b5792071e2946ca4a2ff4b6bbd85fbc9c56abb6ea2a8ded0",
            )),
            y: Fp(U256::from_be_hex(
                "077dcb93e0fdb26cbae47beb0fabb0f490fabce490c9f19faa3864d78b794bbb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0407e4cf82feb0951a515299eee627bc0b9a3089ee979184bd2af30e1f8c4c50",
            )),
            y: Fp(U256::from_be_hex(
                "06811b92e9e67d0bcd5dc8d74c513c931c083badc87c58f113f8f6e9223bb30c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05632f2e8cbd201ab1e35cd2bd84946951d54cf0b1fb35ad7faded1bab445695",
            )),
            y: Fp(U256::from_be_hex(
                "0281ba8534820d7bba6c0eb8820998548cf95327e3b397fc1e85d0977fc88f93",
            )),
            z: Fp::ONE,
        },
    ];

    const STARKARR2: [ProjectivePoint<Stark252>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02d2ea90a96e17fdf7a352c201a79fcb5c72880aab1af6b7a324097c1cddfa48",
            )),
            y: Fp(U256::from_be_hex(
                "050e12a616794379b8334b3b5a1523147c2bba8334f1b366b5e2dafa8bf67564",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05358e7efa226cc66ffb6ba9d5cf0a0d96f9a919938af710faffce6bb3f358b5",
            )),
            y: Fp(U256::from_be_hex(
                "07cfbdf584913a2a719ae9773d2830a2b0133aa9b044b317935c17a48ede978d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05d19f21a076060ff1d44d12e3fa506721a232b72d3789c3ef5935eccaa20db2",
            )),
            y: Fp(U256::from_be_hex(
                "050da284195a079cf69a13f483fca1c140ec7fbf850a324fe9a6c0ebb675e08c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "023376467cd074edf6d004040e677483a5719360f248ef5024fbda9ad02b97ad",
            )),
            y: Fp(U256::from_be_hex(
                "04b83a38890c331ed69c054150fc8d218c29d161cfc83e2145f56afd879005c9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05748224290609f43cb6d283d5a942b0926d24982ab71cff37d48b998fdfe1bf",
            )),
            y: Fp(U256::from_be_hex(
                "0374962806e7e232431531e4e13501849e1d088b9a2b8f175657f5254e313bb1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "057bae3f23b6bd5628e8e77e3b1f450d3b7e6a7abe6faf770829748b33444fae",
            )),
            y: Fp(U256::from_be_hex(
                "02d772777852598181c2d455fe881d588c4059f4fe2932ef2a1ddcf1b57dc494",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "016804d5bb7ddcb2866e4f875b81ef15463c01719488c6ab5793eb39cf5ef06d",
            )),
            y: Fp(U256::from_be_hex(
                "07dccb8b8831ea51c2d910a905e67505c5bd4b40fe0e43b236729255451381fa",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05bc3aa8d3b1ac938f07303c9a59cf36fd4d510ae0c445d38475357c93bf4a4c",
            )),
            y: Fp(U256::from_be_hex(
                "058d70cc1a49d7433a54525b8403ffe01a8fdb3c2ea36fbc209797071e1a7679",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "036570f38f1e93405825c84fd320b32fd62560c8586318cd0e8503e5f3b5085f",
            )),
            y: Fp(U256::from_be_hex(
                "025343551f6a64fb5deb309e2683a03cd803cbc807cd485bd607474367d6eff3",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00b5b0e4e89980f58d9ca80ff28d686afa5b0a2c80559eba0b0e816ac411e587",
            )),
            y: Fp(U256::from_be_hex(
                "044a28312b080cabd8c67064fc6e90621bd16087f6263c9c616a7aed9b5e6063",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0495842bf65c39276e95b41bb6b7d478c4c96025d22a8dcd666a847f6d38b633",
            )),
            y: Fp(U256::from_be_hex(
                "00153b540eb1233686f87ab67a2d6c40fbf64fd497f7b9804de9b00b9d610b25",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "045ed2b5e03442282113c6e8be406dfbf3663a49c07aa86ad98140d37219bfbc",
            )),
            y: Fp(U256::from_be_hex(
                "02270e78c5249fa1675dc4510487fa0f11cf54ad06b40bc0e75384c07748a850",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0776d25c6f896ab77e3ad24dabfb93ffa537ce08496c333fd56f1233f04cbdd0",
            )),
            y: Fp(U256::from_be_hex(
                "000060b89f3c027418b62cbf588a848a114e1c4c8173d36bdcae8eb8e328eb05",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02bbbe1846d1d03a8cd4f02b9e1698f203b665ebe42b0765bd2494e901728716",
            )),
            y: Fp(U256::from_be_hex(
                "01926037ca78514556599cd85fbc38ca192c9b481630830257172824f2c2f2ab",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "059bd1d544709ca86bc84069b733b7bc0899eb4a3b59e2210b001ced4cdeef1c",
            )),
            y: Fp(U256::from_be_hex(
                "02f2d602d78b6ad035fabed5f4fa40c609e66ac8d31210f713082a4fbf80afc9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "06a7643adb0e2e77aadb7e65a060f4cece7d8bebbd7cf623cffa69159f24318f",
            )),
            y: Fp(U256::from_be_hex(
                "03edcc78dbffe086602cdaf3f8d3ac11591e2f78117a9c8f5cd72c5c5adeac92",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04b4cc68f3e6476f5611224a7a63536a67e0e3c597f682b623d6341d3429bbed",
            )),
            y: Fp(U256::from_be_hex(
                "036fb63a619c1f162b2df5aaade16e2992462d998374823f1c49c0b92449ac49",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "038d2f22ebbc8dcdf9d7fb2163f09956f937f3a4fd7fd2bc94da712d973ce1a2",
            )),
            y: Fp(U256::from_be_hex(
                "01814c318009402f5335026de26f0c38e9135abc60f7dc732c7f39ce810de5a9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "005a8eecc1fc0a4e885bc621632dac8e1e892d4c73cec1aa45f2e7e901bad846",
            )),
            y: Fp(U256::from_be_hex(
                "02280caa8fc30f6ac2023c22c676b351ffad9f25f83be19fd0e6b46361481bb5",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0289b960b1fad326ae4dee6c3072266177bd1b868974e221356de13df5911772",
            )),
            y: Fp(U256::from_be_hex(
                "04214c02d470c0355749243743b8695f348aa27ca51736cff9335c957e98252a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "032ffd0ca3438378b317391de1f0209a440dd1ddfd744c95e21dd4657f2dc099",
            )),
            y: Fp(U256::from_be_hex(
                "027443fa746715d3872f89a1f657e20b697aa7e5e9a4b9626498e08ccaf17ebd",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05fb8d4d5ce112c64ea96b56dd3117d594f377f2b9a06f4534ef04cb1744e384",
            )),
            y: Fp(U256::from_be_hex(
                "0602699f76cbc66399cffe54bde651f97dfaf3780705bd5b17123718358b49b6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04802e0be07297f932b8c68cf3548be8a1c0ba5411c7afff116b5984ed816c6e",
            )),
            y: Fp(U256::from_be_hex(
                "05e94cfddfe8051c8038d4c9e130449d078fcd6c36c74faeeb87377cb4f06e08",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "07d270c7a97e01dc43b39099a7e133d3ab4641ba1cc732a7a42012fd225356b1",
            )),
            y: Fp(U256::from_be_hex(
                "00ccfc509d2636cc3c1a0d3aeca5e473d1e0e6ea65c8f79dcd7461f518fa1d50",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "075e23b1238fa6b131d5529b639a787b148a350d5566f75b94c46e8aafbff984",
            )),
            y: Fp(U256::from_be_hex(
                "003a468af967701f46ef8f6c449996bb2f4a0c38c0f1e152e44ae63038a3573f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "07ff8371c998152e9145256980fcba1673cc2f146af41549b26da79d78f8d11a",
            )),
            y: Fp(U256::from_be_hex(
                "0102266e43daa70b3b094f5a202d749318f41b06737ff2709f3d429d9d3b95b0",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05fa143c7b6e703bba0b200e8c1e172751e3328420c449324b80a4fb4cb7d63b",
            )),
            y: Fp(U256::from_be_hex(
                "05dee3395f54b80aa5c863a92b63c73983fd35881c2bc36b357c363a9be80dc4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03079a458ec5e82b62e0e1bedec4657bdd14fa6807f57cfc917a7dcb9bac253d",
            )),
            y: Fp(U256::from_be_hex(
                "03ac1580657000100a0ad2c8df714f52f14585cf9cd7757cbceb84fdd8dfd4df",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03dc0d274a91ca66404f90e7a96cd0cbd5c6c775112e9b1aae9f4ab0e78de1e8",
            )),
            y: Fp(U256::from_be_hex(
                "03be11d8e3cd3f92804d121f03e17a9c3980fd53381e9c32d37752dde3e7275e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "07bc9067d616961edd8b441c2aa700cbede68bf11cbe052485db98235d5df832",
            )),
            y: Fp(U256::from_be_hex(
                "0759ef7e1940790e19ee776d0d8e80506a98579c8afffa6fe6f650708729716a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "073acbdf120448c72f6cbf3f570b9f83dc06080bb110cf5b9d210e8c4b7fe01a",
            )),
            y: Fp(U256::from_be_hex(
                "01e6344627a44ae4ada33f276197cd7ded6a24394363f399f8c85add6d625427",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "077d6ac1a8d04d0914a74559e2e73c958a7c8ad425677f85bfc383cc24d6a0d7",
            )),
            y: Fp(U256::from_be_hex(
                "01f28c49902130bd89d8315245d5a601c6ae3e16e255343bb0c58fd58c2f2493",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02cfcaa2880bd6f9e263424f8c9c258fe5f08152cd9d432989c2f4eee1d94a58",
            )),
            y: Fp(U256::from_be_hex(
                "04644a075a040c5fc0bdd18cc561da79719be4ec00badd41267f28ffb31fb3cb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "079889d3acc8662b3a49f84ef7e64324e1872b0ff08b330f002e0e71c507c65d",
            )),
            y: Fp(U256::from_be_hex(
                "0499b8f4b847320e66494c69892f3d39f4d8885ac5db59a1c913e8bd8c3b582b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "057ec548ca5fb0be7971b22e481c34dc3a95f4d92f057e00b827d58107bd1627",
            )),
            y: Fp(U256::from_be_hex(
                "05fd1932d887218804f73f56909545bfb5c15f9121735557c24201529036260e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02fae8c1b8bd86593af03e508aa8dcacba4ac72e7c020f0d8d45d290dff2eae3",
            )),
            y: Fp(U256::from_be_hex(
                "073efa0176a1820881597162b279d825479d527e1b078ea95e427e2039f42593",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "012cfcfa503ddc0bd8f14b8d45cb4f66c71a93efa8542608ddfeddcad666a3ad",
            )),
            y: Fp(U256::from_be_hex(
                "064852d2d50b50099758db2eaee911f94b55786a0cc912ffcbf78ab0f001e95d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0396f554188924cc3e2d045f339c079a8e54307549b409a27be9d4589b276044",
            )),
            y: Fp(U256::from_be_hex(
                "00d1e6d3de6927de56a437043152ab01bc9196d8c02463f23a18775b4b1289d5",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04461f04add694dbb70989ad09187b7a572451062ccb9d1054e71d790ec7593a",
            )),
            y: Fp(U256::from_be_hex(
                "01442308244c04a24d97b073d207800e6c926efff21b044dc0621043a8bb640e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03ba3793313ed13e56f97ac73a214cea4bddaa5b79f0eb2762819f8a4e9aab1c",
            )),
            y: Fp(U256::from_be_hex(
                "03e279753eeed627e9b89e0ca5ea2cc1cbdf64a7badae88044841682ff567a2d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0028edc97b6e4eb9f695503e47033d98626f163b5f55e6f647c8d0456019e3b6",
            )),
            y: Fp(U256::from_be_hex(
                "062aedcad645dcbeb785ae945c8b0c3195ceafec9ed014f54bdb5bf6286bfa87",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04f38b320d9e5c150155de64476dbfc55069e7f1f4e67c462a5fad84f42a8f0a",
            )),
            y: Fp(U256::from_be_hex(
                "027a1e510c90783c687159d924fc0edcb535dac9c5b9cef3d0a8d516a07a1c31",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04c81ff2e1f6d799914313b494d0e0f09504eed6583e64a4b14b317296d55c6e",
            )),
            y: Fp(U256::from_be_hex(
                "04a735bedf9ef5d73672c1eb59df632efbc905d99b94343d45b141b1b86e4316",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "050d9a5e8657ace51ef8909aa67b427695b0a1508c85561ac93079437a6ce9f1",
            )),
            y: Fp(U256::from_be_hex(
                "00e5aadd6a5cb810320334bf18c5ab9a749a68f790827d33c3af157d131b0169",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "06e896596d6a2e3df482ef269a6b1211c7f0d8b7c288096e95edda3a7fcb42f9",
            )),
            y: Fp(U256::from_be_hex(
                "02203c5118c5142052ee7b55ef932a4fad8beae879f925503aa17a3f389e9cc1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "055e57cc6aaf62a9c79b1df86949528501eb4863a659d75376a4c1b6045165ff",
            )),
            y: Fp(U256::from_be_hex(
                "040e3714cf6320bf06d4aebacd6b6255948a4a63d49cc923482b9891057c5972",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "042a544f4fcc52e7be349972e4b8a7d1e3493075a0715c35ca0c229edc5c177b",
            )),
            y: Fp(U256::from_be_hex(
                "00dc19d06a14261028b4f64493c5632c89f717dbd0665e4841e22c94779bd6b6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04f286b03495dbebec5eafaf8028a91abc2034c07e8e87ee2cdc3d5c2c41248d",
            )),
            y: Fp(U256::from_be_hex(
                "06a2dc1ed5d5e7174fbdc9c4024cfc535e50d3e45be546e48c438cbc5dd58710",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "071585b8deab6dff1e1200d24d283ffb41b16dda4c07c01acfb4550fdc6bc2c1",
            )),
            y: Fp(U256::from_be_hex(
                "00aa6d85f3046697ea918e3423af252d78527865b2baef287532d596852f0b82",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0750edc6e321791be1e226ac9df0e4ff97f568bc41b410cd4d1c0cc602da223e",
            )),
            y: Fp(U256::from_be_hex(
                "00203ba67028f65fef126f788c03a40b23d03e45fdde0946458d1f88aec07d0d",
            )),
            z: Fp::ONE,
        },
    ];

    const STARKSUM: [ProjectivePoint<Stark252>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "06ce3fda4d40b1613f40d583e1f1ff66abb77d2a3636bbcf00a6e2ddc13eff49",
            )),
            y: Fp(U256::from_be_hex(
                "01dbce989615f3b9fb5942746ab7518130f4a2d42194285f9323323af97d9405",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "07c49c53d96d5bd785405e12531f7b468fda9727a36fdb95272532c0dd7f83b3",
            )),
            y: Fp(U256::from_be_hex(
                "030117b907ed22d42cf8226c48ec6e52c6feedc9e6fc2509b78514aad934a0c1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "07800851f59b19aea938900171632778e3086e8f800304963dec530ab6fc9820",
            )),
            y: Fp(U256::from_be_hex(
                "00e76b416c7de9e3b94bde2f241fd96ae782d84ae2c6bfa8ab4316b6a8125639",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04204f95db9f2d60e971fc6018638c6fde8487bb8b064c505458575293e6ba93",
            )),
            y: Fp(U256::from_be_hex(
                "050b920f7fa710d62d4d219f7dd912af941d3eb8296f890c214d17451816aef1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05520f0cb01aaeda07d4fbe82124a89a2926e48fb204338a080156becc229070",
            )),
            y: Fp(U256::from_be_hex(
                "024774f212645812608124b51661f5d2bb0a57758e7037e0e40298ed566c7ccb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "06bb02302ceb49dc657f01ecc8aca4e30b28d8a07497aa47d4e8aadc5fb63c1b",
            )),
            y: Fp(U256::from_be_hex(
                "073824315c77f64e73cc0bf6e509f7265f67109ca4617b177c2b3f790bb3e778",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0329860a35e1853e59af0d3e67b92b4857d7aed3685bf18a4d3be5299f064e95",
            )),
            y: Fp(U256::from_be_hex(
                "060a4fca00abe618e816540acdeaab69b688d9fee97eaee59f55c23fbaf5426f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "07e2baf17486d38a69e1e2521b38acd5fb5a83964d4e6708434946634c71a45d",
            )),
            y: Fp(U256::from_be_hex(
                "07f6c2d6388d60749deefa627790a971a79c8cef50a644c0814ab1a769ae2129",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03cfc02f92dd48210697711781de4d0a8e21c2b527f875d33514f732b78773a1",
            )),
            y: Fp(U256::from_be_hex(
                "0607909baea8b6bb70758c4704c5a70a33591ebfde49706d068b927c31f3b0f0",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03eb59fad7e5fc7520b17afc496a5cf30e4f752410fe09bc13dda882ee229a19",
            )),
            y: Fp(U256::from_be_hex(
                "04ae20a495941433bb1df5ccd403fa9a86b665d7fd6c1d0c9a4dae23ab24d10f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0456980116ebe2a76b52be4258c7819f8ade493300adf0014087f60b665d689a",
            )),
            y: Fp(U256::from_be_hex(
                "079f39c7fd885d1269f8664b1a6a0dd2e6c08dccd5a1f0319710e38420549abc",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "064e5b547d09264ba2e4c371a708640c898cb5454fe36ad2201ad417738f6477",
            )),
            y: Fp(U256::from_be_hex(
                "06186f1ac1688a5f0bbbe3e4047c7fe0cce9eaba6753192e03fd112c8db36cc2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00eb01ddb0543ab4321d022484c9d1288f3ccf164257dcaebfe26652eec6840e",
            )),
            y: Fp(U256::from_be_hex(
                "06b0b7911b8ecd5b91158f7b57f4bb41782c61d5928fe8bd89dbfbc1736ebb73",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02c72126fa204e6040e778ec30f2d133b9c3d7ff342a7d045b9ecc53d8d5b722",
            )),
            y: Fp(U256::from_be_hex(
                "01024113afd49e56abd75f58c8e04ef32185c3818e9cf26659dc331cba4f36a4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03e74c05398a263af8ea70626fde16cb001d7155b4319044fe6e883986978503",
            )),
            y: Fp(U256::from_be_hex(
                "067daac53143ebbfdfe4c9981b5f3161a6cb531ebdf5bffc5ff7e2b79e811fc8",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00b1f3680bdeddfa6915391c94813869faac1dcef13c9b1eb04471bc72b64c58",
            )),
            y: Fp(U256::from_be_hex(
                "069fd4e171e32b5529650bbad92771ade41edf596acfe19a975760b1de8fc6cc",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "048f02cd4e86b81d77da25b3f7a552f626838d4f7d12fa09429bd3bbfabd038f",
            )),
            y: Fp(U256::from_be_hex(
                "01a19c4a7e117bf73f5da944bd92e28dbac1b397dfe832bc23b3de91bcae6143",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02786ff768b8c326945ed3aa0a7367d10bd7f4f878cdc16a91c015dd2ba5b901",
            )),
            y: Fp(U256::from_be_hex(
                "07b59308ea2927fb293f6eaa985144eb9cfbc72aebdf8deb30e406c653f888da",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00da19be57a1539e78e6e97f91dc61ae39245f0c2287674a565f14dcc4d0a2a6",
            )),
            y: Fp(U256::from_be_hex(
                "022702b454969906420e9d517fae3e9804522f4b5bdede5077d7420664c1da45",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "050bb5e9a8305b91d7b6b99a02180fea1831ba750fb52795bcc7083db9fa8988",
            )),
            y: Fp(U256::from_be_hex(
                "035f521090038444223e491e7897e835665a264434598b67a258b5c3a416566b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05790f558d3c9cd01f773802f6e745a4315db3d10cca49f5be03d3eec0e38a04",
            )),
            y: Fp(U256::from_be_hex(
                "0227bf9e65fd171561c712056d482c040842f14bb9a3c2410cdf653b150b8c20",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "079fe4dd9026710d7d212af34e8a6dcff3739db0b31236ea7e28a8ec8de41880",
            )),
            y: Fp(U256::from_be_hex(
                "06ba3da1fa4fb35a073fae6c3351ccd2eb4394717ab8583189b01f0cef486317",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00a54e0cfa3c53515fafe9f718d6d5a9c7e7c08982b65611c2b34669054fe939",
            )),
            y: Fp(U256::from_be_hex(
                "0624b665b13c801b3af89180e8d4a12b6fcbae0e0f32253a5e0d5294ef8dfaa4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "066a123d8d8c881f4c333385b5318d16fc805f94f2c23043d1010a77b71483e2",
            )),
            y: Fp(U256::from_be_hex(
                "0318e750530ac66ef743f6a8c937012d13e764ccb04d6c403f85c1ad30082db9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02a1dad7c78295a30c6da7bfc8dbe0ec75a8a00e80db74326d2178b68b5bf8f2",
            )),
            y: Fp(U256::from_be_hex(
                "0572dfcc9eb4da8d00196b94218811cec634222dff54c850cc284725ac037146",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "033abc20c3af449297216559bf7d39300744a14f155213776ed01fbceb08e3b1",
            )),
            y: Fp(U256::from_be_hex(
                "02e4b96e0ce19ae8bc06d756471f82037edc554b4c872af6828e40b1ec6ce875",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04657d8ba7db8332f41b9ac17c783aed3f2010c3165db90ea8220ef2b06caddd",
            )),
            y: Fp(U256::from_be_hex(
                "05265c65b25c0a3ebb770421faeabed4a77914ab292cbabffb9cea94a4b1f385",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "076caba6ad695f0310009c3d92032805c153cc549d27856786362bc5c9b60da0",
            )),
            y: Fp(U256::from_be_hex(
                "01ce2db5fcdf2121dc4ecca255654aac4503aefb9481a9f29ddd57f8e92edeb7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0649d4a22fd04abd98a63bef97b3d1757e8f0b67e282e106181ad8fa75418da9",
            )),
            y: Fp(U256::from_be_hex(
                "02dafdd5e61eb1cff4a3c9fb3fa810636a5668cb0f3628003460185b98420619",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05653b9fd2decc91a3de8a891453f8c2cbbb3e44aa336545797f069f5dcfe823",
            )),
            y: Fp(U256::from_be_hex(
                "04c4ce26929d15e42e73ff910698744abd0e4d244dd21d4d03dd431922198772",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "07fd98cd486e98137d7e675a1a59c073fb61ce2a07be9bb4e2ab4d2846a2fdd6",
            )),
            y: Fp(U256::from_be_hex(
                "02e5d00bad6d8d65aeb01ac67b1ee29d0719fd5babf599f6346c32f9a44f3469",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "020830934d6b49c78d6e6a4ba79ca003ea42b44b997b06e9d9df7c15b51776a0",
            )),
            y: Fp(U256::from_be_hex(
                "0032b4e40f74bceae44e89941922d3f11a72eaf4c1819187c3b5f03f24a6eb05",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "020c51f940b86f36ea4a1c784a41407bbec7f6a1783e56575718d48db7f63296",
            )),
            y: Fp(U256::from_be_hex(
                "02712669bb3b634504ca8b613dff2427f20ad363c1490c6a7eb5af8d5afc2bfd",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "07968390b6104832a1b7f060a946b79fd1866f00e1bf04e5fa0c2027906f46c4",
            )),
            y: Fp(U256::from_be_hex(
                "048453875b558722873257aed7ad6b5b2c34d33e2e670dd575aeb29773791611",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "036d6ef4016c1d57ffe61e2983dc4d437cea966c87262b8ec09ec526a4783007",
            )),
            y: Fp(U256::from_be_hex(
                "01fee26202561cd9842a2fd6534bd4da9f0569b08938337d1c8fef2f20215b82",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "032ab62063e80108176a0ec4efcaa4b0af52646f4cb38004e31db1c5a471beb5",
            )),
            y: Fp(U256::from_be_hex(
                "0243e3574cbeeb5565d0eb2ecfbd474232e0ecd8cd8716a01fbe78cfe2338980",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "020300c6efbd29445423302e238be724331efaf38d5bd84782c566bb8dcab3a1",
            )),
            y: Fp(U256::from_be_hex(
                "07362852880f6f93acf583276989bb634ea672a77389b91c935fdceca978831c",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "000e48e73efdb3219c274f792679d372d32067e7fcfc25cf55d9efe2e619ed67",
            )),
            y: Fp(U256::from_be_hex(
                "015f8ff22d12e8fef13d1faffad1abacf570921bea542d287aed51e04835b679",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "045625ff3bef40062e2f57548c66e8d30bcb262372c7b8466b0bc6f95be7db4e",
            )),
            y: Fp(U256::from_be_hex(
                "077a72d898cb2701f65af2c001d455745838b7ae3dda982af5cc47b8f2dc963a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00c0f95187fb1a8d414e58fad04a3acdecb987c8b78a4eb413679309f258543b",
            )),
            y: Fp(U256::from_be_hex(
                "07cd291f5f817e68db03eee924cba5716df279ecaaa1e5350a2e8b252e644bd0",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0297e7dc9f2c6bde4c4a9971234f7362d0d13851e99a6c12a95e1bb10fcd6cad",
            )),
            y: Fp(U256::from_be_hex(
                "06766f3b7f94a7608ad50d6ff8affcf9b91ba980beaa80f0d72efc7b808dc6fc",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "064a7fc4423e9d906e5c7beac4a6432dc95735c04498f34ce89b8d808bbd4a54",
            )),
            y: Fp(U256::from_be_hex(
                "026c7fa0caf12bad86e9d96c6007c6137ff1da2429d6d596cb1165b61b975075",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "04ab0f8f363f7f7853f02ef5948f4dfd895528580547c9924a767a73ae774674",
            )),
            y: Fp(U256::from_be_hex(
                "02550297c014caa0adb55fa84cb11abd950497832428e16d47a597305f227a4d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "044adf03b1ee1ac5c09084628674ce7d5c2b4ef6583ed86b440a67396bd041f1",
            )),
            y: Fp(U256::from_be_hex(
                "033b2db5f3bce6053a906e997fe8c867b09a6ac58843fef8a742386dc7cb6404",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "028d4042c0139dcbde40116e6bfd39c7dcdedaa9a807fdc682353f732c1b2732",
            )),
            y: Fp(U256::from_be_hex(
                "00945ad1d637dbb7659050151cda72f86308c9d325f1c3bfcacf8d49ccba1ed1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03e94712491ee387c9cc6df4f7f16aeea008e743ea10285e936b611e64fa01bb",
            )),
            y: Fp(U256::from_be_hex(
                "001e95f451988f0a299ea26f3278064b09af08f3a56b185aae36aa493fbb09e4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "035daad5f2ae1a6fdd80d0c9f46500fc846d6a6b15e6ff58a8864efaba3601a7",
            )),
            y: Fp(U256::from_be_hex(
                "04e60e88a0a3210738ea2a41ed5732e0108d7b2c771c0d75567ce7c2c1c0c9ad",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "06e5569c1ea4203138cdf9cd38810256ff75219be0826b3a411e3b2290c433ff",
            )),
            y: Fp(U256::from_be_hex(
                "024281248df27e65d3f7d93d9e730717ec9f5dae201da7b3b74f2f75e0198a15",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03e1edf6d209d065c17e9ecc3369fa277d26773e0dc19b8dd5aa018f915c38b1",
            )),
            y: Fp(U256::from_be_hex(
                "03f75cd90221a4f094eb6c88f07b63f7771a9abfdf5cae1bc941336eda5225ce",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05466445ad00a7b88204a72fdd6803f36104021bb787816d6470669731439e79",
            )),
            y: Fp(U256::from_be_hex(
                "079a9ba6d3b619ef70e1ce888f5a182207264f26e4b04ea6334f410799caafbd",
            )),
            z: Fp::ONE,
        },
    ];

    const SCALAR_ARRAY: [Scalar; 50] = [
        Scalar(U256::from_be_hex(
            "03b5823c34ace32694000aa37e0f8dc16e58411a2c0ab2872d8a6095d4672075",
        )),
        Scalar(U256::from_be_hex(
            "078a1a780199a13b21eb07a863885bf2b315a553f7edb03a7c57040b9a8441de",
        )),
        Scalar(U256::from_be_hex(
            "018408911adebfea8b50b63ad02d5cb9ff279a5bbd1999d0424e5f3f7e0dfe7f",
        )),
        Scalar(U256::from_be_hex(
            "03de0e6a4a1b888aaed5b48d319300829f8c0626669756176c706bce83c32ed3",
        )),
        Scalar(U256::from_be_hex(
            "06b89fba71ec226952e4ecf7b55e433e7ea96af2724d727240950f9dc4afcd28",
        )),
        Scalar(U256::from_be_hex(
            "005f1f89af283eae9b8dbf3fab59759aa6f9b36bb8e9c5f202e5d062639974dd",
        )),
        Scalar(U256::from_be_hex(
            "06883d0f9c8bee9dffe558eb422a53f3a2a62bfd9812137f9a46393d67b85215",
        )),
        Scalar(U256::from_be_hex(
            "029a38ae9526ac95a78031989262502c2cb091daa55e9273fab70a32e7416c7d",
        )),
        Scalar(U256::from_be_hex(
            "0333a61c165de90efc87a61bc0253f4eb5ba6f2a5916f0a48953da8bf593173d",
        )),
        Scalar(U256::from_be_hex(
            "07e7fc66b3b3ca90bddd2950599bd1dd4e231fc7e7071eb42221ea01d87be5dd",
        )),
        Scalar(U256::from_be_hex(
            "0115f0f72b04765478bca7af2bb41d913f2731e09110718b4af7778b106f9e9d",
        )),
        Scalar(U256::from_be_hex(
            "0287b130ce41bf00c1e49708ec613b6c9ff353910aba05277e62d3e6319f8949",
        )),
        Scalar(U256::from_be_hex(
            "02d789cd18a3cf4ab12c9c8c01c23af35eed625247cfbf3333c881047dfdb854",
        )),
        Scalar(U256::from_be_hex(
            "038b9a0cad3380459a257ef87ad297f3eff8f95790e4a4c9b116244acbd1a908",
        )),
        Scalar(U256::from_be_hex(
            "06c8da5609f47e68c1d92d6a3a38da20e7f8d7f0df6cd18fd52798fa871153b7",
        )),
        Scalar(U256::from_be_hex(
            "077fec69a544934d2bf7c870c88af7ef87e3a55ed806f5c96e159d3e9cc8555c",
        )),
        Scalar(U256::from_be_hex(
            "02b4ed00b37dd39550b934e51cbddf7b1976f3a81b55e6e6526ec714cebd0d23",
        )),
        Scalar(U256::from_be_hex(
            "07c73094278473a58a25d3f7fb09a8ddf961a54cf35970a489d484e68b0395db",
        )),
        Scalar(U256::from_be_hex(
            "0387405b58f6db64d7ecc6833dcbc4cfa7dc4c14bf239453e843a97b11e366f6",
        )),
        Scalar(U256::from_be_hex(
            "04fa276f5cbd9df7b144a6aab2a53540566192581aaa429a2d9d74e6a86fd7dc",
        )),
        Scalar(U256::from_be_hex(
            "029e4c7ce58e1e5b0bf6dd3d88d86be38c8b87fc9b2577542cdd99978a26f756",
        )),
        Scalar(U256::from_be_hex(
            "04f5a4f25cabdbbbc52b738e5af4eb6e1eddf233c53f5d699f6e8aa91a8e16a3",
        )),
        Scalar(U256::from_be_hex(
            "076efc8d7ffa48fb7144bfe32fc336d1a23816e7ddbdf4008381491ce9e0e60c",
        )),
        Scalar(U256::from_be_hex(
            "06a055ff01598e26396a774fd6b86d69e3b15f93f1f0d8052b38fdf0341103fb",
        )),
        Scalar(U256::from_be_hex(
            "01d3dc2dc2bafe9047f6f8c5648357e9660233d71794c7679e9c00e75b19edce",
        )),
        Scalar(U256::from_be_hex(
            "0663f2b9bd5b25ed6530113365e1699c04f8ba82650d61fdabbcdd44a894a8de",
        )),
        Scalar(U256::from_be_hex(
            "064c19dd19b74b096dff724effe3624e5d218277a40d8b017b95920d20c2f8e1",
        )),
        Scalar(U256::from_be_hex(
            "0063707a2d9455e39ac9b83cd225342015ef882b8a2673b17dab881aa9574134",
        )),
        Scalar(U256::from_be_hex(
            "066a41080cb57c6ad4286e2c589aa1a37b2ab48e92b62b3e2e33526b2b7e724e",
        )),
        Scalar(U256::from_be_hex(
            "07ab92e2317fa69a04695b58705d53f7e73f5781da80dfa733b93d86f8303d19",
        )),
        Scalar(U256::from_be_hex(
            "01d80072f5921f24c2b70606d17bd3d83546c7f2aa91f26968315d0943cc48b7",
        )),
        Scalar(U256::from_be_hex(
            "026186fdb2868c6b49ef399ff5930a832803c54f5c99cfef6ed587f4ca9e6f2c",
        )),
        Scalar(U256::from_be_hex(
            "048c4971416b2afa7598978716556c28403c966777acb3b9e72b538fc3d5ac63",
        )),
        Scalar(U256::from_be_hex(
            "051e6d93cdcbcddef613d0e9eafbe6578575b193cd6cbbe35b60ec3395376355",
        )),
        Scalar(U256::from_be_hex(
            "00bc7e627ee935abcbace0b32cfe522f652dcdbca0e8be5dffeda69722a87c79",
        )),
        Scalar(U256::from_be_hex(
            "01ca4428fd110266d0fa38c1e531df1e614f12979bc81ba0e910454bba753b64",
        )),
        Scalar(U256::from_be_hex(
            "00b7138d556569aa72a3a385a7948019fb5fc92ed74f84900d8c151c9e6d5bb2",
        )),
        Scalar(U256::from_be_hex(
            "04fe5b4ab5b0dc9a7d95557102dcffb02f9c04059be5168ae70dd69613feea35",
        )),
        Scalar(U256::from_be_hex(
            "035766a588d3ac3621bac26ef59c5e07564638a66ce4ce9ae4980958370fe2dd",
        )),
        Scalar(U256::from_be_hex(
            "05a6369cd5b1a771e1689cb7e7cdb572cacf531e532990128c42bf2ce6925284",
        )),
        Scalar(U256::from_be_hex(
            "00677eee1c201ffc773d977d888680dea4dd5dafe232356ea079c34a03f271f0",
        )),
        Scalar(U256::from_be_hex(
            "001e29b956462879824a9f94321c3124a646eded0f0b30c4b13e5a98b5f7b548",
        )),
        Scalar(U256::from_be_hex(
            "026525b9a830e4130e42a3e1e8f148b21a4f695105d5c852bc170dc5a62f7172",
        )),
        Scalar(U256::from_be_hex(
            "060042226bd2c0925a162062b65612366e1e1c11a846709b358a812e0d9805dc",
        )),
        Scalar(U256::from_be_hex(
            "05a2b2f63c66d6fe06a6077c1f086bc176a0f65ae3ec7673ad86e8e7a3fcffc1",
        )),
        Scalar(U256::from_be_hex(
            "00cd55f89e7567d62502a7cf664f59e6de49eddbad736eef97c59208d3a457de",
        )),
        Scalar(U256::from_be_hex(
            "0347cad8e6c760f513c5bb488eaef6b8c1953e0e06bc07f9f79644c1465ee0c9",
        )),
        Scalar(U256::from_be_hex(
            "02f2a49487d4c799fce4d1af6a4cb3b24545c27b040ce8ea9d0dca4dae6506ec",
        )),
        Scalar(U256::from_be_hex(
            "0611a0aeab876deee9647aa04134b18935f9b01e122959c88b996fe447a7df47",
        )),
        Scalar(U256::from_be_hex(
            "07cb83e9bdb308cfc966a3215c0cc6faed1d5d7f8272ffb09ee6270a9e3f6dad",
        )),
    ];

    const SCALARMUL: [ProjectivePoint<Stark252>; 50] = [
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00036aaee4de3c36b4d1a1ac5363e8e725bff9d18e8488f485b3525959a79acf",
            )),
            y: Fp(U256::from_be_hex(
                "0022dafdab4a42a36185011bfa406ec1d3a970fd77b31d0ff4d6118ab9d50051",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05460255ba01fb416b8ff5d576db11bf722d3f83e72174948aa1fe72b93213a4",
            )),
            y: Fp(U256::from_be_hex(
                "06ea8d9ca0dc0d3f23ce2d783870acdc8a191fd3805d31e00d33a08462640811",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0041a289f02e98ca8688bbcbd8bb41c1a80a5e4916373b50b62fc6e6030f7f4e",
            )),
            y: Fp(U256::from_be_hex(
                "00bb333271288a37c930a32704d11dae9b627e4d16d4854a51f6641c985eee37",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00d4f8871f53e13292f74e7e4782908eb0347afcd9b091b25ee50ed613681ee9",
            )),
            y: Fp(U256::from_be_hex(
                "064d3921d34a6ea365bedc1c6e80d9d72e4f2b9206fcb10c0695d9e6608154ac",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "014a99ae84185fb287b016fb6c3b57d4ac4c549f5c8198656d7fb446df1dd607",
            )),
            y: Fp(U256::from_be_hex(
                "0471212af2ec6fa54674bef025d4b6113c89266fc69f7ef8bafd425aed76b8d1",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00033bc27bd4ec0735f56380446591c478047579f37c0bfc3d653ff2469820e1",
            )),
            y: Fp(U256::from_be_hex(
                "032fbd35416152d9c07938891409993bab43d5084e1fbc4901d3e624465f387b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "06d0735d548a3986eace8fa5ea5c1f7e843a181df0b7392e4112358a20b8236e",
            )),
            y: Fp(U256::from_be_hex(
                "0772171bab45263a066a536c1b038b1992f624ab5985f360007f444e6dca7e79",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "026ba1ccb9213e4e61ab83ef927fc9f21bf8dab473355afc4f874bd80c6eda89",
            )),
            y: Fp(U256::from_be_hex(
                "01034c05c2498768a8d1a0e0d60e063d64d02b19837cea352e1f08158cea7a5d",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "01f54738d9da6b362b836fb38180d215dafd50aa00740134c563b78a6d93b2ab",
            )),
            y: Fp(U256::from_be_hex(
                "064986d032686a25765315856328fd2a478999e56dde00b61d727f3809c1573a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "063dc1ab4850cf95197308965b79df5a25166a956e50c3485e35624717426e70",
            )),
            y: Fp(U256::from_be_hex(
                "002c9e8cc259f33d2ba91a739eb5aa95d791836332ac493ee636ef801a42eb3e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "01b6b6005f4778f8362a0a59aa6a34467b3c83ed7ca536c2cf3944ea92c98c8a",
            )),
            y: Fp(U256::from_be_hex(
                "076de3f66fa27af7241d2e70dc3e83a87c678867d128d78653f257e446578140",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "071c6cda05e442e7211b7017afbb7a7b906f9f05576373341319fddaaaa041e6",
            )),
            y: Fp(U256::from_be_hex(
                "0524870e39e1791b1c6eb6aca50d3dd4d54f20410d085573b8816a1f013d44da",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "01dd9a0dde21b09d2f5410ff13780fc4306c095cdf5d72527b03d5aac1ba7bbb",
            )),
            y: Fp(U256::from_be_hex(
                "07c7994838110e7318a92203e319bdfa6f222b56b449511732762ea7b53fbcb7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "031bfae40cc58896a7b22c738f6d5b0bbc7ebd403f3c6d8c6b10172d663d66df",
            )),
            y: Fp(U256::from_be_hex(
                "07e1e57e66aeb4d54b0623470b1df6ca620f9fb36cf8d134e2cf57530d7d8ae9",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02bd40e6bad13fca4b828ba58530de3c31ea1238fbd15bebe67e530869b9159b",
            )),
            y: Fp(U256::from_be_hex(
                "07262857f2f9ad912917f99cfb2568e21a92995cf55b78b5254d161a06ffffe4",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "06376494ab3a57d9646476298424fd0a3198981db2afb3014a537cb8c4d556fe",
            )),
            y: Fp(U256::from_be_hex(
                "024712cf2f645e66e062ee27fb3891a56be8d3df4dfcf539ff1b0a6d954e923b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "007995adcc4f8f5b64d64959b4e9b94d7551ed6bbcbb225f2c45aa5d66ef69df",
            )),
            y: Fp(U256::from_be_hex(
                "0266ede63b06eb7506e5e18d1f2f1069babbaf0d29d022b5e2479036d18210ba",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "01cb1e5323d99bdb3c8514f71badd864c4cf99a4489660c61e0a626a6bea8feb",
            )),
            y: Fp(U256::from_be_hex(
                "0212fc6d34a7a8e8766f95db7048197aa4d6d05aeb97e7ec1e1f82d15d18fa4b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "054df93ff9be6f80f5e25f95501b2d20d43e888d72c73e9975a0ed0e1269abd3",
            )),
            y: Fp(U256::from_be_hex(
                "02b4802772ac6ed1ebaf0a50f1a8160469f313d896739241d0021bb952688950",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "056de4c64222173a5e479a0f5ce1d37bf6765f738ab9f3a259e2cb96fa6a6fef",
            )),
            y: Fp(U256::from_be_hex(
                "0703057bd391c1283235c3e404893f8d19d709d7d8a975869668f599f996916f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "055053fa347a329c4ea857fe0fc785209d7852af8df0934afc6ab15ef5ba14c5",
            )),
            y: Fp(U256::from_be_hex(
                "004be57e8c6173dc1d200f988b409318462da67f6bf3f07ed895c4928bb4364b",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05447fb72d38329deb7e8a499ff90b8870988601223edb03e9d1dee1f9e97b74",
            )),
            y: Fp(U256::from_be_hex(
                "00e0bc2b6efc4b80d1a95833d6edff0d4f68f80498bcbf43def6f0be94154a1a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "06f01cf2ce9b446450eafe4ced5573240debfb69183b11c3002e3e5417360b80",
            )),
            y: Fp(U256::from_be_hex(
                "0323fccc6a5681708bd50b73196edfe78a02f9556a70d1164fd18461c2f7a75a",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "03d537b0c4d32bf68a26442df31b61b48d331a24cd78082c0a99f4dcca4621f8",
            )),
            y: Fp(U256::from_be_hex(
                "02b1471ad840ead16df3e85ce0fed6e9042d2f9b48dc931280c4bff68b7d3043",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02ba6f91cf7da76e22b791218db273f6bf836a63d717c53f45aa9fc56c3abc0c",
            )),
            y: Fp(U256::from_be_hex(
                "042f8708a167048b0ec0b0d1594d35991307755b3a30739fc0c39613678b3f48",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "056bf373d83bba139d300e48163ceb48a4e8e3bd58e69d5d22871dd366ea1989",
            )),
            y: Fp(U256::from_be_hex(
                "079f678e9357f11070f634c689928fad354b856e887dd50e30de4d0d06dd30fb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05427cd8451cea2d973397a034142cb92fd80a5f6aca0cfb76f5252d28a00a72",
            )),
            y: Fp(U256::from_be_hex(
                "024ec3fdedecb5e8f5b275dfebae64116e1641b1227df1fcd1a8981fc6992fdb",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "063c92d9a80875af7acc88268df7cb86df482607979c15a52e79ac21a8fb4fef",
            )),
            y: Fp(U256::from_be_hex(
                "0779cc835671a7070af0101402393c18bffd69ebd4e8371cd328ecd86c812cc0",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05ca31d3b5ddbb6145643b06df36955513024250fafe2bcab42a7f0e4a19b85a",
            )),
            y: Fp(U256::from_be_hex(
                "02fbfc523f9446fd7946ebc0c4844bae98f07b68ba482e18165031720b008383",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "07fffb7bf7602c70333cc419febbaec8bc75eb6c3f076fb4b89b13d76b7f9fde",
            )),
            y: Fp(U256::from_be_hex(
                "04662a3cd853a7dac6871c22b50d31baf40dd9b4ef3436253bb9b04411b07345",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "076a5f6eae0e540999981706f8baebfd42ccfbba62a93e14db6703af68dcc39b",
            )),
            y: Fp(U256::from_be_hex(
                "05ce824a8c5f3bf01fd708b3729aa6ac740decf9d1800af739078f2f21b8345e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "056fd9eb44f5a34a1a26691d775eeeac79e8785e96faaabfe6cc47f2fa5a3ef4",
            )),
            y: Fp(U256::from_be_hex(
                "01ccd39ecd4c813aa479ac3b800997baeaf21d525b9813a71b6c2cba72d1d48e",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00500e2f073fc4328765779ac570203c4b5ad876d5123db3b1dbacd09439033a",
            )),
            y: Fp(U256::from_be_hex(
                "05e033a61b48541fc1e6958ab37e2c859e5b7b36b6c258b0064a484c83a2f2fa",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02011d60c2f10ad3e6b9975fc8b1d4ec4db6f1366e85e896c3aab41ea2b4d4c8",
            )),
            y: Fp(U256::from_be_hex(
                "07ee42be05284c6ca49b0a7ee9fecbabfb141082be2922f5d30d0ff51159db03",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "02d6be68618f9ee7e9e4840e57315994f728dc70de59f63e17a9bbdd56ea0080",
            )),
            y: Fp(U256::from_be_hex(
                "04765fda5d892cfdff332ac63d0b83239395751b391a3eb91e40748a01d276df",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00ccb093ff0ec606d9da79bc6dda5068d8a576c06dc165883f09c067b15d0268",
            )),
            y: Fp(U256::from_be_hex(
                "033bf5ce91ab6f11ab26ed2a3b45e0aa93ab1d92876ba4ea19e7d9fcb0048949",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "06238ace99b8a68d21679c7ca154394ab57d9c21861dd6e4fb2df2b323e35ee7",
            )),
            y: Fp(U256::from_be_hex(
                "00c5624a99df9dc2dc51fe3b1a4209c578adc5b5e217924573ed502d045268dc",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0712f76664a0caf8e7a72572d6937087c5d99e058791c3865061f34e2ec453e3",
            )),
            y: Fp(U256::from_be_hex(
                "0255f3742091ef01b533b6655af2cdd6801989389e04d2acbc58c3e5cba2cb55",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0349c8a4302aa837fa5ec5a6aa1b5a05fe2b04cfb59744ce7045b1ea2e48dc82",
            )),
            y: Fp(U256::from_be_hex(
                "0747196a6415d5ff4a096b6ee6d52fbf4ecd8e91ff6e25d784f915c7f9473e5f",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00d43f7728bf15d57f3bf90d2482968feb817b736ef8b765c0755f9e41ec4c60",
            )),
            y: Fp(U256::from_be_hex(
                "00b662091d93c7c158237eb1d3850103bcdd59e6353f76ec8d9cfdb6160d4f56",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "07cc86dd7173eb46ac88229dacec9125a6493bfae99464b663e2431997583c45",
            )),
            y: Fp(U256::from_be_hex(
                "007f54afc76077b6cd985121bd472143bf2e597f7d19098aac8b748fc6545876",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "0101830b36fc477c68d47ae4bb776bb3271d5565efe2ce5bbc6a87eac6545c44",
            )),
            y: Fp(U256::from_be_hex(
                "0394d1f835a2312589063c9386b8789f2306e623742bba3ae71b6a1a6fe38f07",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00e65b8ff30446c6e55d8c5524a0d368b2e839dc94c5534f4ea48c6c97d87624",
            )),
            y: Fp(U256::from_be_hex(
                "00e1911e48cd112de86f1ec7f6203277203c3c807847d8126fe6014b144e3621",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "052a2a90b27d7e7aaf495f3551d878200c647d4ee7b375ab745807a4596d940d",
            )),
            y: Fp(U256::from_be_hex(
                "04a77b5f774043aa1a465cdfeec8c7617e00250d06bb98dcf09133e588da8cbe",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "022f632387c850e944baa437869d62bb93590072606a23eb21e794af9a543263",
            )),
            y: Fp(U256::from_be_hex(
                "051629035ebf87220cc44059be1fe6e6fbe53980d571b52c80346c9fef9833e6",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "01b002f82d5331d57e2d6204130c91787460e60a7a105a9c6d8354a565ad0a94",
            )),
            y: Fp(U256::from_be_hex(
                "0510002db9748b13d082c0e391554a04e09184d5ae1ae37ff46704fb09a585f2",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00d2d4567a7fc3c40f79f6231e2a7efa70e99047245a7e83d49896ffdda680ee",
            )),
            y: Fp(U256::from_be_hex(
                "05feeac0f456a372993d88b71490f463e5059cc93d028958c1c507ad7646d6a7",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "05a4d0f8711649fa9308f36c1762415be35424b6f4dab4ddec01ca6c43238505",
            )),
            y: Fp(U256::from_be_hex(
                "02a56554cf779b48d98b3bb0f93976aa75bf4aef1e2c9193f7b86eb04c694ef3",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "00adcb24ed49c6e9ba56f3be951804b0fe51608ed20b556e58feb95505546bc1",
            )),
            y: Fp(U256::from_be_hex(
                "0771d09aa4090428b940524c0c1d695ca1f6df4448e99ddff696228db1054b96",
            )),
            z: Fp::ONE,
        },
        ProjectivePoint {
            x: Fp(U256::from_be_hex(
                "039f87535b88c69f61b8871cf35be76f2b3d3f58101c01d29d7a0bd764325036",
            )),
            y: Fp(U256::from_be_hex(
                "043eea4852bf448feab178abd48ed7cd201d252c4eb246cca17caca8c669538a",
            )),
            z: Fp::ONE,
        },
    ];

    #[test]
    fn addcheck() {
        for i in 0..50 {
            let a = STARKARR1[i];
            let b = STARKARR2[i];
            let c = STARKSUM[i];
            assert_eq!((a + b).to_affine(), c.to_affine());
        }
    }

    #[test]
    fn adddoub() {
        for i in 0..50 {
            let a = STARKARR1[i];
            assert_eq!((a + a).to_affine(), a.double().to_affine());
        }
    }
    #[test]
    fn add_mix() {
        for i in 0..50 {
            let a = STARKARR1[i];
            let b = STARKARR2[i];
            let b1 = STARKARR2[i].to_affine();
            assert_eq!(a.add_mixed(&b1).to_affine(), (a + b).to_affine());
        }
    }
    #[test]
    fn checkmul() {
        for i in 0..1 {
            let a = STARKARR1[i];
            let b = SCALAR_ARRAY[i];
            let c = a.mul(b);
            let d = SCALARMUL[i];
            assert_eq!(c.to_affine(), d.to_affine());
        }
    }

    //..............................
    //........Testing for multiexponentiation function .........
    //.......................
    // #[test]
    // fn checkmulexp(){
    //         let mut points: Vec<AffinePoint<Stark252>> = Vec::new();
    //         let mut expo: Vec<Scalar> = Vec::new();
    //         let mut ans: ProjectivePoint<Stark252> = ProjectivePoint::<Stark252>::IDENTITY;
    //         for i in 1..50{
    //             points.push(STARKARR1[i].to_affine());
    //             expo.push(SCALAR_ARRAY[i]);
    //             ans+= SCALARMUL[i];
    //         }
    //         let c = ProjectivePoint::multi_exponentiation(points, expo).to_affine();
    //         assert_eq!(ans.to_affine(),c);
    // }
    //................
    //.............Testing for Pippenger_msm function...........
    //.........................
    #[test]
    fn checkmulexp() {
        let mut expo: Vec<Scalar> = Vec::new();
        let mut ans: ProjectivePoint<Stark252> = ProjectivePoint::<Stark252>::IDENTITY;
        for i in 0..50 {
            expo.push(SCALAR_ARRAY[i]);
            ans += SCALARMUL[i];
        }
        let c = ProjectivePoint::pippenger_msm(STARKARR1.as_ref(), &expo).to_affine();
        assert_eq!(ans.to_affine(), c);
    }

    #[test]
    fn checkconversion() {
        let a = STARKARR1[0].mul(SCALAR_ARRAY[0]);
        let b = a.to_affine();
        let c = b.to_projective();
        let d = c.to_affine();
        assert_eq!(b, d);
    }
    #[test]
    fn checkmul_1() {
        let a = Scalar::from_words(&SCALAR_MODULUS.to_words().to_vec());
        let b = ProjectivePoint::<Stark252>::GENERATOR;
        let c = b.mul(a);
        assert!(c.is_identity());
    }
}
