#[cfg(test)]
mod tests {

    use crate::bls::BlsCurve;
    use bls381::{fp::Fp, scalar::Scalar};
    use crypto_bigint::{U256, U384};
    use curve_traits::Curve;
    use curve_traits::ProjectivePoint;

    const BLSARR1: [ProjectivePoint<BlsCurve>; 50] = [
                    ProjectivePoint{x: Fp(U384::from_be_hex("0c1a1d4430e3d305c717291800a25a61344a90b97c9f24df01cde3328cdd563eb2f3797972cf9a50ad47695bfad29584")).to_montgomery(), y: Fp(U384::from_be_hex("026a2868abace2a9df2f4796d93583d1bad5638027b0c7ea7814a9c2975d9e699fc6e2d62e7c92f0d207f9137a73642f")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0e26ec56c1defc78e4fabe9948d02524486e27bb72c1c93766a88b98d64af7ca18d238a2abe63010abfdc79d5229298f")).to_montgomery(), y: Fp(U384::from_be_hex("088e4565e62dc2cc9c60a687e14334acd496edb5dfe490eeae2fd3ae327229f1d972b75a4c672f2fccb77de500545bf1")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("01fc043ef3c91e827aebbc176335a9c1f5c9c101a8c7fc6301c64b60a43a212e7ffebb7fddaff477e40e117586329dfd")).to_montgomery(), y: Fp(U384::from_be_hex("0bc46e463b24d40b94ecbbdc4702e172b19656d0f560801355888f2c2e688744627cfd618586c6c6265f62f953bdaa92")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("08423c61b295e62a3336f5bcea9f5d1efe6dabfedcdd9d3944ba4811cc469ab03d367fe0538b8721f2d9ea95e78b0cd7")).to_montgomery(), y: Fp(U384::from_be_hex("0c7765f9da919bdd3f31af71b9243817617cc118df4a6964f6d110354ed1177e7d97f8765008521102f7b30281489358")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0856113fd44ee31ed59ef71a8db017bf6bcb8bfa79164fb89cf6d53229d88240a8df70c0f97694989b00635d713b9888")).to_montgomery(), y: Fp(U384::from_be_hex("07939c1ffdca5c305a46ae495f4e40e88e5c8ba56f250cbdf1dbedc0e11659955a7a0e6fe2626f8d4192d8f1d48526c5")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("188144e63302abe3a4d62d878f31f74e189e1a0f586af4478e950c8fd9332d4fb45672272b71919479101c2503c3e3ff")).to_montgomery(), y: Fp(U384::from_be_hex("057058fec52483ee4412eb80a48239574a463f3b6bb760265afd852a504810e369345317d703f24d412874bdb884b9fa")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("17b92430c720e566d5ca78918fec3ec27b32bbd7ffaae92280479a3ca60e2ab5f1bbbae11c6575453761fd6395ef5f03")).to_montgomery(), y: Fp(U384::from_be_hex("0a358eba4829d649d55082c95e4cf1861f448f21bbeff48700e6aa251c774d731339f5cd773428032b0f7705a4ac2c3c")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("192508529b200e6e0b7ef1461867c4d159d7e97c8ced2097259238c9a188879e62ac9bed247c72c2fe15e1647f3e489e")).to_montgomery(), y: Fp(U384::from_be_hex("15d6d71135214fc49b71e747f2daa46b2dc6919c29b13755ff4eb166a835168112bcc108f11d9f3ab846c631834d0f34")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("1158cd37340ae4a1d6ee4a1471f41fe9b375049235771d5bb7936474dd0eecd94fb91a454af04757080234b98a2b9949")).to_montgomery(), y: Fp(U384::from_be_hex("103d3982701a455d8f143669d38a88a2c01b4cd0c5b86968230fa5353fc85247dc97cf71bcf98174d6771d726d1e2aca")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("052e759c406c77021b779febef388568c3bbff2b2c78e61c988e6252f89a28ec247f82bcc7814747c8841d697217115c")).to_montgomery(), y: Fp(U384::from_be_hex("1535a0a3fcd35ff84d17868b0328d71b7491df6dcd213c2f4f3b3585967f791e5b026af3d9981d2e5a0b0f03c6dae443")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0a6dc2dc641a2e7692fc02817e0958c41f17dcf85337313990e50705230c5cf4b019ecf1af20818263f2dadde95062f1")).to_montgomery(), y: Fp(U384::from_be_hex("07dcedfac309008eff72fbdd0c5530f401158fe6129493fb236f923827bf253386a418572bb5b6a6ed531a49f704b705")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("103968c1cd30a67613b2fc3bf43e045cfacdb878f1423b6b5bfe83411a72bbf725da2727905096afece0760483aee610")).to_montgomery(), y: Fp(U384::from_be_hex("0ba8ea6aa8dc9d36215327217ca169fb9a6fcaa211027d47f7b4df28c023f9c0af645de069d9d1ebedad2aae8743685b")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("12b99d320a0d6448131c35f86cb3c6c7f2cc01f74d2855dcb38ae7423c5d8b6e831d2aabf583d587ea1691506126dad3")).to_montgomery(), y: Fp(U384::from_be_hex("08dbfbf82292f2bf124343c3c51d3c1e6fa3ae2c2e8587e3a943663d288deb6c0eecc6fb48e9bee785f713ee063df7dc")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("13e460518ae40baca3b37afec9f7746b11f8cb5e3752673037f721bdfe653256baf767347da4f08fe35c7aeb593affe4")).to_montgomery(), y: Fp(U384::from_be_hex("17cc7455f21982732a5d379b0367b8860434d63dcb9773078cadc023c92c671176bf74f7e1c13af5424e217821d6509d")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("00aa2bbc50305a4ab4ddc65c87ca5d70224f98e9651dd5856b971fb5bea559e433d754f8c8b8acdc4e84954b9f70486d")).to_montgomery(), y: Fp(U384::from_be_hex("0b038ac78dc20f25002d06d5e772e5b5777c5a858d58e4ebf24e42c720f373b81d5aaaadac87f43214c41df1f23016bd")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("009f0995d2838af08708ef6c00dd09f0bed01aff91f3b743f4d16ee51189b0e3c1bc324a648d8458391ae4e4a27cdeab")).to_montgomery(), y: Fp(U384::from_be_hex("0620e2da5e64e2aaf07e4e44ee14d1b30c871cac44647b58d13e660544b3ad42536b6e2e95463fb923166d8c2b0449a4")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("17578396b9eb8e0ad97c2c59fd20fc0941021296ba383b72d911e37c60cda064ca58a010152ee9d34788fadfc6bdd26e")).to_montgomery(), y: Fp(U384::from_be_hex("01c1d9e97d412f4824b823248ecbaf0ae2ce26a8b782936a94104b5bdae880284957cff76fccf041e9aeeb4508fd066b")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0eda8463343e6a09e94e2e1e71a8e52f215d2ff63771ff26184d37dca2a4eccd5e1450e02f26c317479ead73cde12b55")).to_montgomery(), y: Fp(U384::from_be_hex("09f86f37cc0c6598f225179fe41d13369fbd7f2e16cf4450d0c36abb5f2d197283bd15ee81f75f8e42a791704e97fdfa")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("145e0d20e09307e9d52f4071243cf31cac3a4d43467581d2b5e04f6e572836421cb330a70f0ecfd09ff703ec1829ee75")).to_montgomery(), y: Fp(U384::from_be_hex("01945d667e61e9a1179dbf96d0417826f194562cebeb2533d23d79e33da910a1b801d7ff509175e0783ce62d35bb9eb0")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0d353b8fa2d67eb1f354f2f3b5bb5ae783f186899e000c45d532ef8a80f68fb53212170a7f55de3dc0dd04fb40795ff4")).to_montgomery(), y: Fp(U384::from_be_hex("0910ea23700606e1aabbf8edd3ecbd71cec8c2d5b817bd102cc0fe1397c969640e5601b198e2b46730941d36c1cb2e27")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("141d665c06250ab378cd7eb0fc5fccfe6a9be8974b7887696fd76eb94adbca1dafd5bf6d0ebc49a11c216309d63567a4")).to_montgomery(), y: Fp(U384::from_be_hex("0a5f6c6b1c177e2fe4dcaf9636fb6897bef668c60ef75e9f60b91562915f9797ab09152374d6b1c3e78017439493a055")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0bfe1b7ac5508627476254c7047995d08d4418de6c22d59a67c389d9b4073982597fb822c0b134787638879947a0cd9b")).to_montgomery(), y: Fp(U384::from_be_hex("14b1f3766d488edba22e56f094a73b1709821f48acadbd81ea3b864c54746ebcb7ef41b71b3a3592e6551eefcc3be327")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0965e9fe4fe3e3b3bb3a744f95eba1f0cd550a2ccee419959f873038a6a762d1cae4e7a8c43245b10a978bcd51c7fd86")).to_montgomery(), y: Fp(U384::from_be_hex("10db406b41135610e90942821ead148ef9fae32588b14b64932f488896a91fb2cfb240acc4cd4fe65dfb29d3cab3fc09")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("19d6b786b2702d01f3bbf1e020756c3cea363e38168601ae77d58a99b5855b3e9c2f23c73acd881e01d9141762c300c6")).to_montgomery(), y: Fp(U384::from_be_hex("0333f0f9db87452c2e3a0911895d01d2183873052ec2d39411e4991b70b9299f2cf49a736ff10ab8601af168fd46a426")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("171454da9034fa869f34ab0e717dcc1bde2826c5684710ad11bf2e5365859d218fc0d5f33e93ba7f44080e029f853154")).to_montgomery(), y: Fp(U384::from_be_hex("180234ee97cad0e2ed28ce2d036606d13e34f36ca194820cd9b0e1f38c766c36fb358208458cd77bd10d29bfa64147df")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("07905032e0b4c6d3530fc14d546187ac11ec5eb4c10c0668358aaa2e0bff322792d0a7ff185d297ae5f528147b136b34")).to_montgomery(), y: Fp(U384::from_be_hex("05685a5650f2e750ed63fa04e82acbee430f4432072d27016500725ad7ed5ffae7d14a15c701c050862548f13449e14d")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("18c1355edbe50db6f101393a170784eff38cd9c9cc8962b92cc1ee303c7bfbf30067c85705fa03d54078cdd5eede67eb")).to_montgomery(), y: Fp(U384::from_be_hex("09e1179c3d93ab35ff7027d826d2ecee8516f6f436777b863250e092cbeddcbb5cd838d0c45b110964e77db4a501f029")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("18941ec3b982c43efce524e18ac90f023fdb7f96d84594ab11e78f2d1121dc445da062c71b89695232844c4a95667d61")).to_montgomery(), y: Fp(U384::from_be_hex("0e550d22e397fecaaa17f908ba0bf007242def5b51b72c9a864ad9177b0e8f6b7dfd926f16efa7b42b326e5e0e9f6276")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("11dccae7d937bdd4045866dcdc61f9c62a40828567fe4cf86b05e583c3a8ab734c57a00d243823966a58425fffd1c8e9")).to_montgomery(), y: Fp(U384::from_be_hex("040bec713584a6a881e6fc11064094fcc8a83089b81b269646ee29c1ac7091b8da9147fc6305bd22ac7fcff06e997473")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0325199dc7f60fc588b67b4614f56efbff36b48b381ee6d501ba8a92b154b6203b4cf7112582c4042b239338fe3f49a1")).to_montgomery(), y: Fp(U384::from_be_hex("0526fe567b30281de6d938699630de68596e1dcf3cb667bd70396b23ac72f44fc6ef3a0f767c5fd0176d3d9a866adc4f")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("1533add952f9d5a9964cbeca8a00f47261cfb1b540e6ce9907c30d88f3bea67432c229241336d3e9c5aebe7f18791510")).to_montgomery(), y: Fp(U384::from_be_hex("170f6db30e00e3c2eccdd4cdbf1f3161b57fe1c03d661e9824a23c54ca4082cf2825797e40f9ef203009c9352a36c108")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("05a27f4c22ca96ec24a2434de64250a25701e592f2cd90e3c1e2b66447035f3a884364102672c756281cb8f8c2384e3e")).to_montgomery(), y: Fp(U384::from_be_hex("099cd259bd846abce19e08686358ba67b1782af56f45d302d28a8be3dc052201d9b9db6d254ea2d7b864ecb613b610a0")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("00e6d2087d954427f66372628859a8d2663795c20e2b5cb7cd61a929b876d62cf24065f9445e5f9f08a6ae1db4eab76d")).to_montgomery(), y: Fp(U384::from_be_hex("05f32a987b0cd57a8d4659fddd0faf83b94d7b054c61a93c0a44130a23cd5f5318b7898925c70e035bb36d84dc844ae8")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0388eb49b4a4f7b782497f0e41b6fd0ea6cf69cbc82a914738ddb9d798950d3016ba088cc8a1a42769208760b6340c00")).to_montgomery(), y: Fp(U384::from_be_hex("0c04f6140e41f8b1d3250fd4815259bd458c8cb295a3c410f57c6703b3f46e669558a7328936ac41d187901e465a1504")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("02ea0fce73ac9dc0c358bb58b5f94bd1e7326bbcee40747c81b84b5c4db1e14a40b4ebb49e0be419a554aab0b576a855")).to_montgomery(), y: Fp(U384::from_be_hex("1464450811734d7c652e6b47aeca038a74cf522d7e42af9f9a78466a66d2b06e8e8ac8e1e600686cdaa72ed961377731")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("179b03019819308965751b8dc25c38f0d48f8f6e2151330c7fd82667212ea18a151a8fbf4fdf4ca4cf06986596fd8d9c")).to_montgomery(), y: Fp(U384::from_be_hex("0731d36a6703a309d5ca48c16d6c895ec202658e919871347576563586f24a19761d97ba9f6b19aff5cd84287d8675e4")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("174b97b36e216eab47f3e67ec5e30b16b8e749924e0084297d2f020e7c1055f01e6eb46700757be7edf2d1964792a95d")).to_montgomery(), y: Fp(U384::from_be_hex("1173e506c6962f60a1e1c0f9557d44aa57a5f235f20a81fabf4d65b1fdae1831b0b7b6aa490ea1ff4a6e1bd5df420fca")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("13b364191a9720064481ce56118bd1ec24050ad049f7d1de7be26407bf000b59237248472d92d3eb8c06465814e9d7ff")).to_montgomery(), y: Fp(U384::from_be_hex("094be2f10132791efeee3183d529b6e0feac285400d6e9e1a3eb120ceefd6e8faf39c86804fc21bc8c193c415cd82c69")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("189d1c3c441901db0ffbbf24fbe7257139aebc6ac5afc1bda02ad9ddca5ec0c9d7ab664c4c8a04c84adba1b29ebc10aa")).to_montgomery(), y: Fp(U384::from_be_hex("00728136862d31e25092b7e06d791abfa03af2c15a3263effc1d5a73be813e1b5676c704a408ada753deb2b093df5c6e")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0f7ffbb842ddd547e86ffcd11fc83da6339e957935b14d05b7445cce9338adc1af22e448454cf23d66337e4d01afda4a")).to_montgomery(), y: Fp(U384::from_be_hex("0df7188d08851ad6229ed26a0cdece0472e0977b85316072d7f05de1cf50eb967dcb8ca8ff9529e2674a270adbcfc3e8")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0c80bea9e679219af9015188d997c0d160ce2073dcf6b5c73987d129d045f11a3c21058c4970b4920d6e9a264ac5e83e")).to_montgomery(), y: Fp(U384::from_be_hex("10a96562dbd3d18d9e2b343ae06bf877614f56a837e3ba5c94cd9f1ee8c854019002b7996735dc5dc732c092e63e5bfe")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("061063736a796c10d051b88e6752ae4e109cfdc09ce9b29b8ffa7512780165709b25c338a2ffd3870abcf2239b75a1b8")).to_montgomery(), y: Fp(U384::from_be_hex("121064dd45c283c97e32de2f7d30c4821640fe34fd47246ed34e5d24477247687e61ee56a7ecff0c77014fd7ba542d8c")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0389490357d50f7196629bc6c044fef5687fef493b1480298764ee7c5ad13c1338f62b3aeaba4bf9974bf725c76051e5")).to_montgomery(), y: Fp(U384::from_be_hex("0a27132972bb5785075f16aa1de00f756abe93917dc6d05d25124d7c477ab0762e4a9a9568b0b5cff5bb863e73300874")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("12c95ea94996b7849a1590e1b13881768eb96c7000b30ce7bcf21a1106b63cad0f107cfb4d4b35e60e6ac44a16f2d295")).to_montgomery(), y: Fp(U384::from_be_hex("001b10139c5136629d6e6bd19bcd640a22632d305b6f5e61df2d39227bcd61da6be7b611ac7515b60bad9380296fee92")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("016d3fa4e095a04781f4f77a0bdba5a225aee10fd7d3e06057e963b201f7627d721a1ecaa2a01ceb575daee92e4bd355")).to_montgomery(), y: Fp(U384::from_be_hex("16d72cd3d5b9d6edceebb41e209cef3039d13d875ac58655e263118bf909aa6a8ab41ac74f147e1e0bb5eb7873617e80")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0c4b4dea0f3eb94c77044fe3fc22dddc57c2ea22e82bb4ac91ee32315d4ee42289157c7723bb6a2d8b280e9f443152fc")).to_montgomery(), y: Fp(U384::from_be_hex("0653ada51f87b957c370a583251ee898989528e20a1757eb8bea86befa058e333d84ad5cef363502be7af122720c7936")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("017f5c0c65ab2406ff8e86c9a3af139f2bdb2978c75506eba51d1699134b2589cff06cf62735179c0fba5a5160e9a4a4")).to_montgomery(), y: Fp(U384::from_be_hex("0371ae54441d72b9f3ecb9f4a6ada4544eb3f0aa286184056a6deec12c450a8a8e1d68729cb87a3cf1fa5dc6dcf43513")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0dca99bc1d642802ab39eba822e40dd75759533557888a01143b7c7ff5c34ab40ba6d62c2bf4822f3e5344e8ef307c38")).to_montgomery(), y: Fp(U384::from_be_hex("0bb60bf65204360fb8823b39670d8e2bfb55c4884d94db72a2a8dd62f58a53ac33c078ee781ec92e82afda38d14d21a6")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0fcdd438f77f2d082617c2df17c1233e418db6fa8e1f37802cbdd2add4a0f5a846af86e581e25d55af5827aa61aa01c7")).to_montgomery(), y: Fp(U384::from_be_hex("040a18fae610c46ef8547c1177f0c2741f1de370fb7adcd5b56ddbb0aeea19a4f5e01d41d1e12eae11592b4b425c1a11")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("004f3960b1910aba65cb0df725c44c309303c782a8880b5653e426ff65f40601ee2328e17f69333dcc7fa7e24995d5ac")).to_montgomery(), y: Fp(U384::from_be_hex("0bc152cb7778300ddb8e2117caebce2df12100384071ae5717d4a7673b8b5f81cabd2fe1f737f0d18299efc73ca2011a")).to_montgomery(), z: Fp::ONE }];

    const BLSARR2: [ProjectivePoint<BlsCurve>; 50] = [ProjectivePoint{x: Fp(U384::from_be_hex("09ec62f6721da7eecfe2a8bb51dbf09b7453d47fdef90598983635998955cd522e2832e795e5ecbf4428a9ee5fb8bc5a")).to_montgomery(), y: Fp(U384::from_be_hex("052bb753939a226fbebbdd276d3d0b4ea7eda9c3c2fd54db827821606abc7263575adf200629eca9c70b99a927c8f91b")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0c94947257db952636b532d73f0ec93c6cc55e7e699e4200f92204ee033fdf13747246eb2519654476397f6e52ca3cbc")).to_montgomery(), y: Fp(U384::from_be_hex("09ceff1208e85afc041fe89fd7bba352ef409ccb62fc165c84487149c61ce13acef9c285ddf69f3fbd23baa76cbfcfe7")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("116921a15cc13d3284db68f5309bd2bb0667247955f7ec22912d1c2a61326e09f9b094f376e54a7a44448289301ecdb0")).to_montgomery(), y: Fp(U384::from_be_hex("175073efe5296a7e611aab2eb73a0b3890a74c0235c43667c39a0b0e302f5e77d5a7b49b88278b0b549383f2278eb647")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0af8dd720c173d2a800c056d767415b660375e05bfd3791e59d195f2ae4e62bb1d4251eebd15113d68e88cc5d1f077ef")).to_montgomery(), y: Fp(U384::from_be_hex("0d5be460240e8ce101206ce590dc633520eaefcffc39d2282d4eadb53971a367732e591fc98d67362cc872b3ff93123c")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0eaee3df2274e8721cbdab2015db140731c817ed89ddd107505951aca1130cc7842c3d17d07762089502774a9445e663")).to_montgomery(), y: Fp(U384::from_be_hex("042588ae2c18bfc52fed61b6c2e2cbdff7d1bbd6cb804924a3b099aae9a354cffad553f84e0e5f99272f5d69dd9abe3c")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("17b893275529467d69ae0802bf559f85b6f9a4b8249d608d1079bcf1f7f85293f5e386645b9e2751f725f3d0c3ea7066")).to_montgomery(), y: Fp(U384::from_be_hex("12b671815208ab5282708f272c065a9ee3770d8bd98d00d2b04045cf2c7620e75a09fa5e3a113ed326603986c40762e9")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0d0aca7b4dce0fdc9f124e64b4f7c7b9bafd6ca8fb4b966ef60f9df2f5515792f238e2ef59201734aa0e4581315e0ca2")).to_montgomery(), y: Fp(U384::from_be_hex("0cf99e80ec5030d1470491ef3ffe320206f01fff246889f4556a7b4b46a322fe28fbeb603d6a940decd502e1658fce3e")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("073369430a46a535990f25f6bec35e66da995e8ccb3ee3533c4617d10fc5aa0917b0e5e94e2efd8dc6efc246615c5a8e")).to_montgomery(), y: Fp(U384::from_be_hex("0acbaa5e112243b4509e473b1cc698a3a8cb6cd869178c9000ea2ce3f02a5df2a29f4738828da9f45445b4fd7d09d039")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0b3f5547b34be26f6140cebc2e2ad0d7c3a55b92c2b916d9da03b1ec5f4a72f86745d63ce5b305f09b8c89f189695f56")).to_montgomery(), y: Fp(U384::from_be_hex("14e744ebd43392079bea9827b81e7c77e215507e4d86250ec713d6556c5bc7ed201a6e40187b1f0c663b6ca3ce3cea31")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("113c9043275ec239286c0f05c8c5ec50d9bafae0c189cfd971643717f479a596b1df2ddf394f975adfe88f2f0d01b5d9")).to_montgomery(), y: Fp(U384::from_be_hex("110aa57278375aa8b827e28f7817eb09b8ac5eb6d5ef3b56eb773023e8b5d58f2ba350fd4f0d1f914ba6ca737e33a06a")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("19b8abb23f7de734dd5ab31522fde8101192e5cf610cb31cdc56237ad23daa1cfab91e5534f35b0261b094f43efd1c06")).to_montgomery(), y: Fp(U384::from_be_hex("0bf017516243be2c22b47c09b0fc2906940e110dadac06baf855476a3330f4968137706f7cb2c555883b9e72f12a0f24")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("051f7bd69c9298b42ff1f796652f8d1d21296d210b897dbe28eb3cf36088f8beb3aade4a950861629b94cdebf0743c57")).to_montgomery(), y: Fp(U384::from_be_hex("077b72d172ee48a3231c83a7044a91294f0dfd7bd36d72d98882d461d529c99cc7aeaffc5d72ce8c4009e1d847c01a57")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0011bf5c49f980a905f101b545b7e7bb6e9077f8c58aabfbadadc7c82fd880b39b73acb456dbb38c8f63365d743ddd44")).to_montgomery(), y: Fp(U384::from_be_hex("17acc19760f8f52ffa14a22d1dc5641b9667dfb714bcf1d917ccd0d4aa9e6199ebe3a1342bb014fa07d703e52ac8298f")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0c6f7c37f1b3aede8fc1adb9d021862bf49c356d83c7f99f148b12bf4ccd83aacff6213c756cc4c6e55b5b7711239c72")).to_montgomery(), y: Fp(U384::from_be_hex("19ec4fe19ab10f40e45b51a1e7af5c1432bf195b51e436644d83d4802c8439c2e4d87222e9e14875e7b9aac041ada8e6")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("16969dc556642cccefe75dbd00c83dc7f71e11ca158f31fd3ac8504b2639b8057562237d2fef07325bda0c31cda48867")).to_montgomery(), y: Fp(U384::from_be_hex("1054f101d8657ae6b7ee2d1d05e3976f11e4dc68061a33ef236425d1febd6ab18b972bd7ca78657c3853d5aea5bcb154")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("1425f6248f201fb9a75936e8b0bdf860f32d8bd036ae3f9cfdf732139020a71f89a6faf15b85d73f0247812983856493")).to_montgomery(), y: Fp(U384::from_be_hex("096e26e1de1d04c722c93c7f4c61c71b46b5db5e4ec071cecc90c0eb21708938c2bdb0295cf07a742e7c0792cbd2e536")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("18aea1bc9c365b879ea5757aaac29e09a2ce04fc02531b5f1850ed7226540aae8bc96abcb3a2630d144b557a783f9551")).to_montgomery(), y: Fp(U384::from_be_hex("1814cdf9092ee23130ec9db90ebf301ab85229cd4663507be9d94655d75e38614bd77705813a41265c67ed69a1a1b2c5")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("018252897f2a26ff04b74e5f65b09b6e2eba50fcd335c0d40b3054e91ef30c3ec489e9696108fbbb5daefb34e6cce843")).to_montgomery(), y: Fp(U384::from_be_hex("069f19448e1954d20a3574dda69311ab76428f3673a9ff865e891a39d26a8bf31f21b124e2c87961db40beae78c6dadc")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("1029a6b0cacb6febbba4f822e37faa0948ff8a6475e1c1e14e5e832f57b6c318a3ccdd4285fc6eb3eeaae1c95e29044e")).to_montgomery(), y: Fp(U384::from_be_hex("14350d29e1aee75abbfbf43f27fcef3a436432740db0cdb10925b7b8f3b781db1530ea3f3c5c43df7747ed5c4cebf843")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("08edb065a756938ba5529cd5126be9eb86395a970e91a4259a191a85504400f0ae133396696de023c3343081e38f51c9")).to_montgomery(), y: Fp(U384::from_be_hex("0ede2f0f484b931927404934c8ecf41255f7269aad37dd6aea562d155358eca224718f697cab193667a28dc3427d8636")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("031c8998471fecf30364808def091f4dd886e573de0bfb4daa4a04e2dd97c2c573a78eea9179c6b4bee9b2af8c89b084")).to_montgomery(), y: Fp(U384::from_be_hex("188899d139c6c86d43f4c28b8fc0e0ab7b48eadfd4c7b1a9dacd89bf70173d0f80f3a6c0c7624a5e5dbac64aca39ba6d")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("08d5416ae9e8da1bbf75ef645dfb21d18a1912cba819ecd808e26392df5a2d7f79a9597e7129a6a9c533fe19a73ff97f")).to_montgomery(), y: Fp(U384::from_be_hex("00e9be391a793d69cc8f09e25d6ac617809c24057ff6c76a8e4c0e67b9ea9e6bdfa2b1688c874b6d5d726c9078d7a757")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("13bc8d0528bb54fd149f7be6ba25bb237ffcabad7177494af7701f9e4ae7cc294fc98b15f9ac2012762bba2ddcd9ef0f")).to_montgomery(), y: Fp(U384::from_be_hex("07ac319793273df7c281c9ebd55b1c789b1be7b0dd98291f26dc6586f0ce1e626244d327d19b98ba9eaef0344e116fa0")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0082d267a3766e883115c1786d646b69119a472ced35c4277f833e8d5ba27cc20c21286d74a4533718ae692a5a795060")).to_montgomery(), y: Fp(U384::from_be_hex("04bf1d7c4721bd0b545ca8f52c2aecae4c771cb1181c492e49528ae73d01c48ef87d06fcadab6b7c337f02cdc110ccf5")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0a0064958586425b736ac295af85d1cf9af01f1f4b2dbdffc40807cea4aa33f6d31e27d958ffcd58420acacf5800126d")).to_montgomery(), y: Fp(U384::from_be_hex("1844649d256254faea88e65da9cbadc96dd7ade10f5edb4ea58d57d81fcff0dbf14081e9818a5469d0da78ee205dd784")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("12b54cc86e0dcb6451fae25933fbc28dea49dbe273a4d5045e10c853fc98192d9fe12dfa0a92983ba116336651752027")).to_montgomery(), y: Fp(U384::from_be_hex("00b2332946e5d9be403112952ddf656d25f84c2b5f06bf545455c2032e065a06a179bb66f95a55b68fa4771dd5ba7656")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0ee00ed108ec4a217d326451fe8631a212adb119bafd863b4ffe2423da7ce4c837085f53adf73443763f0fd40501f360")).to_montgomery(), y: Fp(U384::from_be_hex("12e8780a644c6ca38587ad8982e2f7fedd5fee4eb3df247ee8c1a9445da3c1bd56a7751abf66de1aca6b46758892d790")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("1985981571d430a269006d2c54c1d854d62a4c3db014e7eae64918d09a4b9347d3864039d2ec3e02db043cf7f78e7503")).to_montgomery(), y: Fp(U384::from_be_hex("17a39c6157a846fa686948a65532d5b36cc0d0fda6b9dabda7ba97557c0f1584a3488660684c725baec26435b5949fdc")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("1965148c82148b6de4795e95c0862a700cf35c81477ee05848641607205071854bddb9b70aa273120687eda7c3df9b63")).to_montgomery(), y: Fp(U384::from_be_hex("012088439e47b6aa1336589e31b8d617cd6aceafd19d2bc338e0c56364d0374955f589e32ec3d80cdb6d6dfa4943533f")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0c9f018984bd08f2ac6c7b5bc8f3d5cbbbd1db6b52a5a6cd530fc8f0dcb191992caeb09626aaaf0ec30b71f41c03c10c")).to_montgomery(), y: Fp(U384::from_be_hex("02381890f076d0526e42cf31e10f495e4333e96d3f68cfba2868adffdba0891e7995cc766deba28dce8fc436fe8cf25c")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0716603269f5b858c550dedb372d9b1d428a5ea60f18772c5efaddcfd659d1ad7608b4a9652232d34655c5e34a63a77a")).to_montgomery(), y: Fp(U384::from_be_hex("155f48ecc5cec2a3fa4210fb3d1eb5036ea5e930160351dd57680586a91885236dcac7db03e356e92dcbf435305d0784")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("185e9bcd97edfa9e51eb255f45edd85102c94a2bae6d018ec5c6d7847eaeb07f35e0dc5e901db156c68e13e1df7c782a")).to_montgomery(), y: Fp(U384::from_be_hex("0b0f50e77780cb0b7486cc759ec15ff9a1d7dfb53b3a6755ea15b6e8681b4e3084ca44fb089dfa8c860603cb4b25ba94")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("1218982d8178b19aae8094fc04a81cee18fd0ab186367698b41c74af7b3e319ef3cb44133f3fbd9b5b5763bcae5c2098")).to_montgomery(), y: Fp(U384::from_be_hex("104c37b874d1cd5a5bb99b4a07804afd2f51721cb1b955b7f90ebeb207aaca9b88704f4c9ade28e69552138effd3caac")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0d9d0150a73efc60f50e10f219d53754b62e394429c7fbbc27b9c65e2c2483ec64110ac240273f7e5fbeb50791254050")).to_montgomery(), y: Fp(U384::from_be_hex("15985ebe14a839a2997f0b34fa5c9e8204884807c2059697c51268656890314db3c960a3c2ab32cc236d01de7b8337d2")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("1696634cb8834f37911477ef9627d9eb757ac3e8156bfd15a33532ce79e0e63a3a570d0c01d2df039556d7bcfc2f7c16")).to_montgomery(), y: Fp(U384::from_be_hex("028c247bd82ce92b837bf901955b34c6234e1d71fa4b760c70d9311b8aabbe52a3227b336b2de2c1430c8bf0e3d9d62b")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("18019b3f3753945baa7232e31f6fe570fcf95f32a673f6fe55632cb09b7a10bb92c30630eaab29e6f87deff550690c91")).to_montgomery(), y: Fp(U384::from_be_hex("1384326bfd18e97588d50c581f703785e4dcb0b7ee4c97ef866f17925fd5ac083c8aeb0e85216f421afb40ea2e0679aa")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0f0112321306b08ccdb96471edbf9d70b09e4fb5c0084ea6ee05ef217f34b9cca1bead845e3ad73e686e94d2db3161cb")).to_montgomery(), y: Fp(U384::from_be_hex("0dbb694485b3a5266e4cb782557c27eb6e2a2c2c424877e9fb5a1ab81fa95dc785526e36e612a2b4bd8fa2509601b762")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("13c7101198d096ac9abc53fdba3450c08489b9573de0d71addd4fa8c60e4e1de9498ec4a24a2e9e198f243482c488334")).to_montgomery(), y: Fp(U384::from_be_hex("0b26970ecf639364346bae84bb951767f112541b4b3742e6ad0bed68275de0dbd317a7aa1ab3f104f999c5435a9446c2")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("09cb55c6e39a85a9f56c13033da04f0813b4f64a0427b2a5a1ce9ad6d3028b48b8626f1db609bb31fd807b1713745be7")).to_montgomery(), y: Fp(U384::from_be_hex("0615f0b98bbf22bafe6a4bbc8d3b35267e753d6c32e1559f8c3310879cb599993dbc26949626c983b82b7c84e5ceec4a")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("08e4734dd77217b76f205d4ece35ca0d31c5c1608658696a5eabf0eb51a8368c4c9e8faf0a6f12c1cc7aded370746866")).to_montgomery(), y: Fp(U384::from_be_hex("0c34db21e7d9b15490d75bbe3670bb8d690b9be3aef8e606828b069d1fec807826bb047e70e62a618e2476a68be4fae9")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("19fe8c7db8659fcbd247f160dfaa0669bf1ad333e9dd6723096a5ee3095508e0d2233142047282443a62b4303102da4a")).to_montgomery(), y: Fp(U384::from_be_hex("1171fb9ee1bb6fd764b775a8d22f0fa93e6a02329c9b1f1d5d6aa0ec8814e688a55453cb8a03c1d407469d7709de287f")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0f08182f91c8f9d28446ed6c18d485fa8539bf2f5aad4a7871e50a65156c9e281cae7f2257e865a49806655d735d2c37")).to_montgomery(), y: Fp(U384::from_be_hex("02c0e85ee60b4d5a56c1cd8267bc8c85e06a9c61957fb555cb2fcd42733db179201b8b6717d51caa6e6c0c0230e28ef3")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("1605e871352f46fdabd269ee13422e6b83e4bd5875d668421df6ebf8ba209f049375150c7941a8e6112ea64407513bc0")).to_montgomery(), y: Fp(U384::from_be_hex("03e8275ce485cb9591467611e7cafbd93e3b3604c4e12edd5465defafaf73ddd98070ec94eb79bcaee30922ae79dfd59")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("17e1ce4531587b63fc057c6e36ff9c0a2c87f9ed9c0f93b13d761680220dcdb76d121204ff027415bccab063b4e9730f")).to_montgomery(), y: Fp(U384::from_be_hex("0861ba26a480fa3ecf564b1c84b116a4f9d186dc7c23016be5527ee3628534a553f7ddb7f72c71654b06d542f65566a6")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("02aa68d580f5fadd0fe34bd47fc0333c3123db7526c794f5fccbb35afef6a9349d5e471f1017bd5a93013e2df399129a")).to_montgomery(), y: Fp(U384::from_be_hex("01840481f2ff21ba0900ff53b7b5e740ee28c73a3a292cbcf8ada24d960c5195a7d274ebff3bed5412f6fecd2b0ca502")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("17d9cba479811ac8d7a8e4a5e9833ca85e7539bf7e4648946545b0bd71da4418c28b84730f59205a460af286969090d1")).to_montgomery(), y: Fp(U384::from_be_hex("1052ca0d96ad4210511fe884f5ac286f1d2c75423a2dd13424d004737f88f83d2aae2eba213d199d449ba4afc36eca62")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("18df47bf5c609b8eea1c3abc84bfadaf24e7277fbd734689fee7c257318e0933ba7ac02c9eb28574f125f9c74325e574")).to_montgomery(), y: Fp(U384::from_be_hex("0775cf19bcb0b32c379f2bae759bc38528096a87640780f32876707b3e0ba5efa9a7cb175f7ddc9a86dc755a6af1642a")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0072c9873adf6691567eba7f34b4d2a997cda7e4f5803f8eed2889e388fa4e97986b6f2aa1bc5e5f3c485a2ae90aea51")).to_montgomery(), y: Fp(U384::from_be_hex("08d974506b11d4aacc76c1928f664d49ed1be4bcf392782a08f9d845037aa759e3db67f2c1bc1e3357e2df07b5751585")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("1782a0fbeede0d93d9beea430245543530efd1a8fdaf59f8aff543c96871bd24948c5b3f937a21afac2e3ece8168d9ff")).to_montgomery(), y: Fp(U384::from_be_hex("00b6270ef8e1e8c4ee8a518dbf2c7cd2570bdd59e162be998e20e76c68e3f7d752abe557ad09549ccff8b5a24c61aec2")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("14bfb00f632cdc9055f2a230bc525b49371c838490ad82c5ed24dcbf7b0b1a1199341c01e44b8535fdcb4e9dac09de5d")).to_montgomery(), y: Fp(U384::from_be_hex("0973c63f7e2683e192386cc416af0e5fee0675c69384d2708d7d9e8e64729c1999427ab7a788a448b6b0c404d454a5e2")).to_montgomery(), z: Fp::ONE }];

    const BLSSUM: [ProjectivePoint<BlsCurve>; 50] = [ProjectivePoint{x: Fp(U384::from_be_hex("0f9a51c06aeb4b887cd7c73c14b775843f50787340af998dca42736f42b9fd7f0f2e5872f48c6e22f26c637b25ce7bec")).to_montgomery(), y: Fp(U384::from_be_hex("0c146d4155d78fe93e56f871ac233dc316deda9a17682a8683aa073dae4ab997d6fcad008a461ed72092d145bfc9c268")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("109e429836266e52ab468364f8c86ad73f472c10a31e73428ff722b65e29a85aab51abd1df1b00f3097e7c5dd117dbe5")).to_montgomery(), y: Fp(U384::from_be_hex("07a0bb60b338db244b3b5e9f01c9f49608638b12cc73d6cfb73026d7e1e266af3935bcf99c4c7dbb38a24cdeeefb65bc")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("11b1f1b1723d6c13443dd3b5a068a3345b8b3a4da60ee318efec06f5e61f75f06a62d6737ccf5e250f60ed6a00e383ae")).to_montgomery(), y: Fp(U384::from_be_hex("1998be4c7eea92df87c7a3cc557e9b09d018457a6b34fdd901995ee7e0a8edf3fc82f9245bcf903c0d03ff8ca354209b")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("13cbde35956fb87c29616987884732aeb5c02e681e2c0a518471fa4331199a6d7a8a2a6bf16be4e775662a406d376b58")).to_montgomery(), y: Fp(U384::from_be_hex("103fefecbf3227a33cf27d3856bd814d970ad33cfcf0ebd70f1653c484ad51f69fed11aa5fe9478c49d827757c9baf60")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0aa50e0725631ee5d070be850669b2e57fe2d74c744bcd4f650f8243b63450f4d74e88b50a49c9f462e56b8156e4db34")).to_montgomery(), y: Fp(U384::from_be_hex("150ba0490ee3e0acaa2cf743359a9c7881b3524c2ca15ea7185c0c2baf556f4677b2c8ec47096b2b366c92c8866adddb")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("15a8127801298a845e8c4caf2dc7847a3f40c1c3e7e4fbba305af41d5134896c235b216a210ed87165b8313c0bcf4ac7")).to_montgomery(), y: Fp(U384::from_be_hex("017763e6e649b5bab4168286b6e69d5a4abe2e6b341a9508281de9a41c25f5eec4ff2b6dcdad9880ada7e34e8a065503")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("092c9fc54191244f76282fb562ce3d87131e663f400a7bcebbf2bc2f456192a6bfaebd870bcaa162b25587fd31456331")).to_montgomery(), y: Fp(U384::from_be_hex("017da253946f578ccdff4004810eec94156481af7ba06c1c37d312c428cb61b2be65d19192bdc67219b3010643fb65f3")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("1791ee3745baee5c800b0e4673947ac1285c59dca82e78e6805dd923859c10cc5bb3956749000a2a3afb59bd42b66f8c")).to_montgomery(), y: Fp(U384::from_be_hex("170a925fd0bd42e5001761bf73edf0b8e1e4ed75688a41cfdf2ed4b0e2a68cb2920827eceaf95c5780835c67afbe0800")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("1164eda77d39563da395f2156a1114214238ddfbe0fc7ef870024cc5f67747e300004585abcd1493f1dbebfa56a796de")).to_montgomery(), y: Fp(U384::from_be_hex("16cf391862988d86b80f75ae04e6cee5cf789bd83ee0ed8e5fda54c06ca5998d109398390e3fb2c93ce911ac7f7e9308")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("02918b45b3982d51cc940388a77d5a2787c791265173eb5a03830f7556df839bac4a2cd8294466c632d6c78b1185cca6")).to_montgomery(), y: Fp(U384::from_be_hex("003e90dc9670210d248eaee7b99a75c42c87a3a48b5b8e8c50ed4bd6a6f35ac4bc986bac43fac5b971afb4d9c4800744")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("04463e104c14c4175b995b0e093c89b8db3efee3c2f1ad365e5ffb6dcaf05fb20dcc22221015cff029640d778cee2b30")).to_montgomery(), y: Fp(U384::from_be_hex("18a4402293575eed9d23fe237202942e7a88c9e78e207a2e8b54039925d56fd3758441044356abe4de9e566172323834")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("16959da0ff90e4a26c1fde5ba6cb807e86ce20311687d9d6889b874f2daf55abd45ea8fbcc6c812feda3e843a4fb0c77")).to_montgomery(), y: Fp(U384::from_be_hex("05954cd8b087856b58e3612e25ffc152b2ddee4c7d5417c656ad7c3333d9c87039f9c7ad05e44b740fae3f2b262ed989")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("04a311489156a50df459ece8c83699a5e6405b1eb4c8ca56d954b45211db43a5dc9e23c8ca8eed2c6937627785a2afdc")).to_montgomery(), y: Fp(U384::from_be_hex("18b416ec0d8126a682db3fd65741be726d0f50cb19d4369faafebf4885b1045376a26709550140b14cbf1ae1b88d4c95")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("118aeddd0bc3ba0f741765d34033335f786934f6bfd188f8d0c238e43e03faf7212fa37088edddc186dbc9d8fb0bbd42")).to_montgomery(), y: Fp(U384::from_be_hex("099c26f92ed234fc24004b018cda31233ce806bf2434ec743244142c2b86fd85e9a297a233d3ef10436b3bb92668b2b6")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("01293f7c9677da6d62ba0cd6df5af8942863a6f1b54e34802171ab18185a470f29e0e8aa792993e52ae8ff40b33c96f0")).to_montgomery(), y: Fp(U384::from_be_hex("047847bfef1d038599ad7980541e343a4e924f57c7fb087d6d0d2b51495483a8e29b7041a73ed1852d5735fd133bc49e")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("03d5b4248fcf7f86a6f04f6c872ec0d6df013e94a6ff3483c2f4f5dd982bfd58cf512c412be090444cbecf20230ca48f")).to_montgomery(), y: Fp(U384::from_be_hex("042e2950b6a16ed999d78d73bae0630830b6e1f7dbd1ba0e2914d85a199e11de49674e3030b98299231839885aa3b9ba")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("12507b3cf6743c5d31dab0dd7fe72011e20da986bf630d6141490aa20f59f48f776f5bbaa07bb6681c919c763322501a")).to_montgomery(), y: Fp(U384::from_be_hex("0d80022b6682192f9fcf3b14a7fb75fb8adc1711fdb2659603feb79ea8e82a103f54156c44c611eed852f67a537722f6")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("00afcd6981b8f064dad7bec372801f58c535cd91aa23f57fa1d1fbd37f227c3d08b37f28f048dd707b06847aa11050ac")).to_montgomery(), y: Fp(U384::from_be_hex("0c49b643f0594a1bcf66c07d6bdad97f2880d1d18b9f37163b6b34ed0c37005c71d371ac147a657c8fcf638aebf2b8e8")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("11776a39c8c3e04838bde3eec30b77ad1a6306e64aa964d90f88ea02bfea6028ed881adea945e43b4b8d7e08dc3cbbea")).to_montgomery(), y: Fp(U384::from_be_hex("146aa07bebff0e121e9051fd143e8581bf556c19dac4ec304dcabc90967dcfb73873f42b36c7701001ea9ed43e3a5de9")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("06d23ad3fd3ffd58877c2c97946b30ad871edf732578d10872f319089c65bacbb8bbb210ef7dc9a278a2d59efba6a40e")).to_montgomery(), y: Fp(U384::from_be_hex("070ab46f3b58f87cff2954ae84611b104b2e658da317a319e5b2cfc0f452301fc9f301d2a8f5ed8e19a1e5220b3252a3")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("18ce37445ef053b4309b7ee58f7c72c73a955e5485995bd7ff8506f08e3b329ebe74b61a2e310b6de2c62a8a5836636f")).to_montgomery(), y: Fp(U384::from_be_hex("0da8838711fe560f0aa4b891c82fa0de8c743450e37519a79970c80dc4f5508b46a11b77d0871c2b1c9761f294bbf8b9")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("1892c3465bc7f92da9b8d487163d6b066162da990e4002a69a82e916b67244744c94f83ce6f1d1e8cfaec4014b796dc8")).to_montgomery(), y: Fp(U384::from_be_hex("0b63150467135195c9320c45a2bf06c8032ea575e89fd5eb471b05df58fd8bc802975e0a20c361475c9ce94b30c22fd1")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0fa4877a3db530456e8048076885b3e56f88af6c0287fbc6db7bbfc3f620325a773abb79f297d7a6cd9b8a44d4a154c2")).to_montgomery(), y: Fp(U384::from_be_hex("10a2cb0759f5570ce899e02f91c4bd170346745a2f65565404204d3d246a713f76720a4af741149472278f11ee88cdbf")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0e7e7b885eef6be4d1c91fba4b29cfc685c90c6d28bb74588a15ce43bbb21e04dfa8b15558c9d1fefa63244a338c2b76")).to_montgomery(), y: Fp(U384::from_be_hex("0dd1dfc38bf4febf2e0a05f1a36cb132987503735dd6196b560274a916637598e3a8a9d226f13497153203b2c005ffea")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0a7e45c028aa7cd6acd097b53ca061602ba3801d61d2995d1a09a53d75fabaed50fc386569c44e588d3afe1a5af4cef3")).to_montgomery(), y: Fp(U384::from_be_hex("17e4dbfea1d180e5258613215a31459fbdd5b1070dec49aa57f5364497afa3d676eb255b408b98041a159fd23527e646")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0efa5a87f90a5ebc61c35f4aa867fce2929db4e9c1624fd9ec895c42539f846efa4e5ec9c591a168816a2c09bc8dbc8d")).to_montgomery(), y: Fp(U384::from_be_hex("09c8a1e2bd692cafb238081d841799df8a198bd111ca580019de815265f977701471474dd70cb185979dbbfa2f3bc68c")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0515c2020a88a7adcc5aa2953e05ca7a683e80fec8c2759154c3581b4432ee718e02a73905e671caca314530326f9136")).to_montgomery(), y: Fp(U384::from_be_hex("0ed738663fab01a149c6ac1d522cffe9ffdfd44579f3a5a280d68b3d4e291aa8f77b9f0d3ef6ee475a4c7bf7abf5b06e")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("170fa8314460d0f0ac9ef59d6c56acd6acb68ead3ed8b7dd1e906570f7e8302ed4ffbeb38a2ba2160a1425c296f4c488")).to_montgomery(), y: Fp(U384::from_be_hex("0482fe634b27bd8d516dd5bdca6081aeba006d670b300da8d98cf478e71a11257ddaa63024198458033a1ec7cc98ff1a")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("08a0903706da58987cb822fccfb25df5f82b6e4200b8f7adc9f345277bc333af870f0ef0a823fdac5862db2fd1194071")).to_montgomery(), y: Fp(U384::from_be_hex("18f82f99b4c8483607a9ed42e6781080aa3fbc2d3b0b2dd1f8d784fc6cebacd37b42e47a32d49352c784ab8fdf895039")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0de61c1656b27c4c59135d0026eeb121265b03ecefe693a061ee8ea735a937fc84bbbc5df0ff3a8cdd9f1789557af242")).to_montgomery(), y: Fp(U384::from_be_hex("18738a31076ce9c10065455491b9f599212610ba7896229721893b9bc6e58e74f4c5e7bd7f22bc1fd3973d616bd9a897")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0e63369a7a091e254b1668ef3a0c2848fa3b19d86b53190f57f6ccd29cd54d0888bccc1a7545dde8a29fa9ec18ffb821")).to_montgomery(), y: Fp(U384::from_be_hex("10e69bba8f74a242e971f362cd8c8d343880303f9956861f0438788f428d9b6861e30e5f4f3028aca67d4ee3cffce133")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("02cc9a7e2ce6754cfaaaaf87cd400000e30bba82b89e85272834ad6dd76f9fb87cb44aaf09ca5bb473c3d916dced54d0")).to_montgomery(), y: Fp(U384::from_be_hex("13a5e492ea4b7569d023a7ddac47f4904fb1edfbed55ccb87d9f8debed0f4c4f54bb536fc41b2874ac71a713bff47bad")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("04bd02128d6199c8e7afa085692f4fdf37416cfc0ac74c60916270fe559b53eb6c9494fce8fdd24b0788263ca2a91705")).to_montgomery(), y: Fp(U384::from_be_hex("1717f50605db51fb7cd4c2e4aee000cdba6023221cf60556bef5d546db34b7b3b33d6ff2476ebbbb5544103164899d0d")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0d8dc6f382a04d7f994b7d8ab54cbc2ac3cc95f9d39a85093885be0b46e7a029071dca86e2a678b2e2184f0203fd11cf")).to_montgomery(), y: Fp(U384::from_be_hex("094e0b569ed3662922aee467f16f768f25f9c0583cfd107d452bd93ab34cf2796b26dc295d05b28d7e4e74b6ddb8a65b")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("11b3260e60caced87fe9aedee75db81b84c015c0b9a24e73147b3f712f70084a9166c8359cf4954a7c5464c416e40870")).to_montgomery(), y: Fp(U384::from_be_hex("0aeb25531fe5263edc486b9214d4fdbff02dca3a72d66283a3d462d80e3746662344530359412b85e5238e62f142963c")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0341829d8f0fe9bced52f483955eff0cbdb9dd8b069aa840b038fe6995ddf1203ecf71a4ae9a1cdcada54a7bd914da91")).to_montgomery(), y: Fp(U384::from_be_hex("07106efffd56d4a96e7b6b46091e1c595f67b2ae0c8e2a345fe538592b94de731ee40eff19851972f86a6eb6430fde56")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("05b1efb87e97d001c2d7b4f3c928d470396249ad98743879811d9d27edea9ef43ebd7752dadf98f9f87f5db21bdda20d")).to_montgomery(), y: Fp(U384::from_be_hex("12e4606e9db8003686613bccdebd2eab13a6f7741d09d2cd383e2474be8c0e563a5b66f484ec6ede5ec6af245a11ad5f")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("091326aeba5bed8f6b5ae71307249f7fd54ffddf46cf4588c2e06b50a00efdb41be1d248c6176b377068e534021a8f5c")).to_montgomery(), y: Fp(U384::from_be_hex("0ce5f167fb3d43aec9d95681b85bdbc2f50fdf5aafc6987170ac169b908733c006e762a7902625bcd72f4e3fbec22aa8")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0bce0a0e465b8ef1b65d90c91ef030f3ef21742ea5a72c8fc78e2930bfc1a9e16cd8f2d908aae37b7255306582e6137e")).to_montgomery(), y: Fp(U384::from_be_hex("09f0c48388bedaca1021589a35ac199e50c3e851053fc89fb3d19f5161ab0b22e5f2eec666bdb9034391c0675a8fb179")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0cddba8ac0a9d48972a70d916924a9c188b40a3b9567f66626b9d8aec0334531cfb012cdabc0ff3177800970528efe01")).to_montgomery(), y: Fp(U384::from_be_hex("15abe5c4425ff01d17558eadb1c9423b85731ee2431f2db902b63f63f8fad8ddbc21644fecc9d187cab0162ce24f9e72")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("00d18ef86007e508a1166949c3acc58a87a1f9d07f21db42efec450a646e51c07591f2f76451a2d4f34ff702d51ce7ca")).to_montgomery(), y: Fp(U384::from_be_hex("0fdba040ad1c4ad2853540a1074c6218f8578c55ee87746d62fa91dbc28f8bb8205899e0665d3987d5acc230b5055dbc")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("19593afa7a54b5200fa248e75a3f7167ee35945d7ff11467e6053ee7d2463776e1a435942e9e3156215021cd365210f4")).to_montgomery(), y: Fp(U384::from_be_hex("00f735c18f8602ba98a4d7e3cc26e741b8db2402733c94feb40e4f7a496c26853c7938259e7baa269d191a094625b853")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0a4f0347d83e9c2de04dc229917b642ff2810f0bee8ba5917445b02443a205efa741b2e19f4074d5642f174928737e4a")).to_montgomery(), y: Fp(U384::from_be_hex("07b000c4f32b17c2842a1dbec3fb97187fa2b66c6cf7d967c4cf0ea2fcefb0c97f153a3e801e6e15ad855d599fe31bef")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("048d3839feb5874182d836188e2cef3d09c488c01fabc3a499b5ef90aa048f02ce575608d8cab4a451b5a35f416d9e07")).to_montgomery(), y: Fp(U384::from_be_hex("06269095d06619665f2fa702526fa4ffaf12f7246fb84a122b9cdcedd1679d53c0c7351d1fb029ff702461aeca9b0194")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("196f2f936f7e59d814334aae12ffc39b9f04db669ad7a91339721410c87ac167138c376d02634b815662f67e6fb241a6")).to_montgomery(), y: Fp(U384::from_be_hex("08bcd0c78c6a68b17ed672dfb1bdf18c588e6596db4f65d0af2cf03e9bb3bcc0b2e27605d2a5f3ee49323204217b8b27")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("19fb9ce4677468e91afbc7f00027d455bf2bd26729685dfbcbe6a72d965dda53f0f53a691573f2db07ac538b45fe3903")).to_montgomery(), y: Fp(U384::from_be_hex("0d5fa093123adc3e94ac6fdaa49016be19cb2f5444d0d681e900480161c30f5caa05e289c2da0e021c9edd331b12f28c")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("170442c7e619d85f02054be7f17620756830f06a91937afa1edf0e8c94e5c948b59b810fefa41e828cbd42147a2ba122")).to_montgomery(), y: Fp(U384::from_be_hex("18515f5e52f141271e84d5c2bd12510de5c0637cb143e3392a81bf78343c2c7f2b76a47647db94e14446eca9ad8bdbef")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("058bd9cdcf5d6efe83c906f768abca08cb150019f8806375627de50149e9ce9c307214c3639618b652050aada5965ec0")).to_montgomery(), y: Fp(U384::from_be_hex("0231c298df55f7b3b5359b3b7c0089519812f2f428ce09b2b9573c99bf791177c296331e3bcd95c925636191387af607")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("00fb787ae8d4222f846ca22214100c99c49e1de190b7860309d73cc33a4715617ce95f2f5742d8edde28fb07a21c84eb")).to_montgomery(), y: Fp(U384::from_be_hex("09ea467779f30462a009344a374c5bb925dda540667174e67483a7444c0f1b481a2fea2be9d0e99f7eb55234f61cf81b")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("11b5055f74b40068c13ae54ffcfcd11623f8f386e083893853f6443720c6496579261a54fc99b7c6dff0ebc48fb4ce3e")).to_montgomery(), y: Fp(U384::from_be_hex("0a131360a96273dc97f5aa92d014c472f3454c778c6418a7f5fae03fabf4361e31427156da3d2b0b720223661053ac2b")).to_montgomery(), z: Fp::ONE }];

    const SCALAR_ARRAY: [Scalar; 50] = [
        Scalar(U256::from_be_hex(
            "01f2dd021073426e93d94b4fe53dade94a3b7e1b730c42d6365b3edf662192ee",
        )),
        Scalar(U256::from_be_hex(
            "39bb04495692197fc4f63c183a5541a1968382cbc36048568e463322f112fd1c",
        )),
        Scalar(U256::from_be_hex(
            "0f6a8336301683b13a8a1f1a4f19e7ce8591bf1f2329303fc1cb3f51e8f7f971",
        )),
        Scalar(U256::from_be_hex(
            "63d27d4295c671ad4933ad7e6a4ff4455a1a7a14bfe7f0d9b8a5a682738aa477",
        )),
        Scalar(U256::from_be_hex(
            "1732e329b99b4efac9a3a9f52d99911c13074b06665287fdacf7d265f9fcb50b",
        )),
        Scalar(U256::from_be_hex(
            "17f214363036ef2e1786c1e5210cc28b25cab8fa5005066dc58974dd1478f1bd",
        )),
        Scalar(U256::from_be_hex(
            "7284620625cc66a9dc893fc6772390985e1c47ce4e37aa818a9184d82aabba79",
        )),
        Scalar(U256::from_be_hex(
            "61778e002e0c2bb43ec1f0263fb170a9d0e91c33e6835ae227c86f1a394f269b",
        )),
        Scalar(U256::from_be_hex(
            "4d8d0c7084ca85ffcef0f4bf536ed9697e3648dc880fcf0f7bfe2a8e75234e5c",
        )),
        Scalar(U256::from_be_hex(
            "22744a162c92aee7e3df08a99be9598af6942602418929542848226aeca9599e",
        )),
        Scalar(U256::from_be_hex(
            "03844674889ce1bf1e764f3e74f5648e2f2cbd737f1ea531b7e6fa8fbfea1d51",
        )),
        Scalar(U256::from_be_hex(
            "07f71c292b3d975c2d26a4c814aa8ac0b145500123193d2761960d5fca3d44b9",
        )),
        Scalar(U256::from_be_hex(
            "01a00f78d2c9b616c950cac564b9a1c586e379e8887dd4ca87c75474fa196e75",
        )),
        Scalar(U256::from_be_hex(
            "3ba9c93c3e71642fb32860f095015f36252f6110906e6c63f7450f4d08b9e988",
        )),
        Scalar(U256::from_be_hex(
            "251a45db0638aee06f138030d16465d87b338e8d2fbbbb4ad059f533ea3589fe",
        )),
        Scalar(U256::from_be_hex(
            "6b201d2a26653696d455b2245f196ccede456b9ebcf90ad487a39d4644125de2",
        )),
        Scalar(U256::from_be_hex(
            "1fad5cdd9a8e96a6bf7e7387df1da558723eeca5a97d6c051e1dbdb4d2b70f8c",
        )),
        Scalar(U256::from_be_hex(
            "2b2dab8f35388edb5f4da6b3e8b263570e578c01c3286dd76d12f39a480503ba",
        )),
        Scalar(U256::from_be_hex(
            "4956f6e8153e6735f8e8756248090ea362778180eb32d08648877b192a209626",
        )),
        Scalar(U256::from_be_hex(
            "30a36ea9d50167d42ffbe4eba2b7ad071ad12f381ef8e828c51ddfe591a8fd7c",
        )),
        Scalar(U256::from_be_hex(
            "6fe85797fa6a8394a0b8a7a94ba7f07d1a2a9a59c2f844ab7e23c99e7b143a53",
        )),
        Scalar(U256::from_be_hex(
            "2b3bf31f8981c21a90edf571b8240478d485e1d9f0baabafb2a838303be03bd0",
        )),
        Scalar(U256::from_be_hex(
            "38a17a66334fd511373967178e4e92cc7931e725aae17eb00127f58a6d654d10",
        )),
        Scalar(U256::from_be_hex(
            "6f1d81888e930605d6719a8d0a3d88029e2172db8b70d2021b45eb7f189c66e1",
        )),
        Scalar(U256::from_be_hex(
            "0a1e1069faa9b3ee8ea6ccc16d2c77b9c10cdbbb0c5d0c1e38f17fe623839487",
        )),
        Scalar(U256::from_be_hex(
            "4708ca8d0f1f6ad3d49031af42ce3924e4dfd755220594bd9ea73b4804abac85",
        )),
        Scalar(U256::from_be_hex(
            "4b73cdd36b99ff922944b8d96fbeba702c955e78fdcf291e3a400d0bd26c09dc",
        )),
        Scalar(U256::from_be_hex(
            "49e812c71a887e26d023b59542815cfcd708fb688ab450340bcd7279338e92d9",
        )),
        Scalar(U256::from_be_hex(
            "4305fb42d530ed9c1636f3a0d495762edfbaad8bee110b4cde0d7d583682c078",
        )),
        Scalar(U256::from_be_hex(
            "1bd7180fc0ea52250c9ca0771fb4077411d3d7218fe5bd6e69ca907daa893a5b",
        )),
        Scalar(U256::from_be_hex(
            "16aca9709d2ee8514d0323f2cc758d55c1dfa054a2c0df09b1e6e9d2d6841f5e",
        )),
        Scalar(U256::from_be_hex(
            "6093f7be689cdf2858c8038db4615c2d31c0a790d321476414ee047cf3768353",
        )),
        Scalar(U256::from_be_hex(
            "04e90554f7909dc515b08780ed9e5451f97c897b513a24b1f832e6dd0857892d",
        )),
        Scalar(U256::from_be_hex(
            "03bfc910e49d3b613374c330d4b1b8dab60ed73d8b092a5281b35738bbeb67ce",
        )),
        Scalar(U256::from_be_hex(
            "48d5bed29d90471e076673743b4469e3bead049d25108ac2f9b4ccf571b75b31",
        )),
        Scalar(U256::from_be_hex(
            "4c4b49597be08a73e81cccc0768298a179fb18b40cbfc4a366fa2b7bacf3d99e",
        )),
        Scalar(U256::from_be_hex(
            "41451be9a377f164a2dc7b81588b8d061ff5001d10b9a854ef37961cdc84e523",
        )),
        Scalar(U256::from_be_hex(
            "0f99c7dd8573d94471a4bfefaf1ea12ee928251aed15384b3f85aed5fa449fae",
        )),
        Scalar(U256::from_be_hex(
            "60393680a65359f934d7249ce153ef626ec1ae3b3fcb27fa616b5814aceb9e99",
        )),
        Scalar(U256::from_be_hex(
            "69047da749937748064f18583b6d8e81ab61c6dcebcf933ed4fab532679546fa",
        )),
        Scalar(U256::from_be_hex(
            "1b24acc4178e709a1a1e235446c100084156ae9c1b97cc482ff83b324dd456b5",
        )),
        Scalar(U256::from_be_hex(
            "4681ba54c9e7815515fe585c33b9444d45d6e7495e77a654856da61a92a44157",
        )),
        Scalar(U256::from_be_hex(
            "4f4625b7f9967fd9ebb59736f0a6e4751be7e5f11d0e6fdeb39100bdbaa555fa",
        )),
        Scalar(U256::from_be_hex(
            "47fa01ab9c335650ba5dbbc0e4a59c3fbdced5f06323c5d0699ad3313463c4c7",
        )),
        Scalar(U256::from_be_hex(
            "08f31192b5f65a74a64fc9449dc4d84a341df664e1da2c1d0f335095ab577b4a",
        )),
        Scalar(U256::from_be_hex(
            "44c0eff4478737fdeffaf96706454a3f0a4896b9aa5f6e6368e8d8c8605496f5",
        )),
        Scalar(U256::from_be_hex(
            "0fa8bb200be10b1613a22faf937aeaba72bd684e7a380b7309e44221659b25ad",
        )),
        Scalar(U256::from_be_hex(
            "55f5aa338e09e0db0210e90da5d7212ef51be81561c454af2ff0b2bafdf1aef1",
        )),
        Scalar(U256::from_be_hex(
            "55075e49a823fb144d817fdb1158f835cc716dba46e166c37692843000bfce92",
        )),
        Scalar(U256::from_be_hex(
            "13926f7baf5db3e939c402fbfbce76af25b365631602430dd76a914ec005ce02",
        )),
    ];

    const MULARR: [ProjectivePoint<BlsCurve>; 50] = [ProjectivePoint{x: Fp(U384::from_be_hex("108469aee63580b9c27bad83d8a1358bd0641295ba08975cf02797de1290fdefda1d61c593e69b8d10e3692bdafaeaee")).to_montgomery(), y: Fp(U384::from_be_hex("00297d0c5bbe865f02fc8650b6775980e7edf12f32d01a7e864593b5a5cb7601deb2f60755c1195766d8e5e747e5bc35")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("06889dc96c5df67f13b8c2ef2683e516f89dc331563b04d88ca5d9b1ec6114d0e7c4dc1b57e96f859a47db4151edc709")).to_montgomery(), y: Fp(U384::from_be_hex("004a2afd2bc8786c44e47a56d5db527cf2c59ff82644e3ed917e1e37c7373a32e65c9046c77eb79a18f2cb057e9950ed")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0890b3ae9b41b85f36aff4f1e6d538a19a0c1129d7cf847e008256d71e197716ba04594e9da7c619bb3fb7cc6f7147a7")).to_montgomery(), y: Fp(U384::from_be_hex("0652516578505777fcca8f85aeb4783a3189e3a99c8d9f4fc5ebe5a94e5a4e9bdb69af8ee0ff36081d367c9bec31d478")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("09c4dc5d6b23e816c06a417e23414ff9f6dd31b53b1fd8392b4cb9d6cd12ff300b2842ed0b9a48fccb4d5f2894c06270")).to_montgomery(), y: Fp(U384::from_be_hex("0a4bc353a658a72ca31640e37317cf1a2def3c8f4c829af8cd1f7073dbffc294635565b8cc748a08dd7374cce0149658")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("01408b9e3d9900741f65abea6c51e0f855c8164e3cdab0848b0b39e2d500a469ce2392001d7467f824471ab70ec13a2f")).to_montgomery(), y: Fp(U384::from_be_hex("05712878def3a58fae50692aff8da69a84f9b897d444b23db50296382946b1596f97ea86bbbf79574dd9e5a0ca35e3f1")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("156fd4f04d97f13e11fb611ea5d93416c901c42da662d405af6919141ce0313380dd5728c65f6cdba18fc1231a7a4fd5")).to_montgomery(), y: Fp(U384::from_be_hex("15bd6ec2b2957162f2ca926db320a9367c9d845d509fa819508e77d5eec4ae66b1fab4e55fe3305fb15bd75d4e21b65d")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("1060694191be8032f9f62a5c23f288294c9ad5149623d156626e53b3b10df3ba1997c13b65ff0a0f15a7bd4291b69aa2")).to_montgomery(), y: Fp(U384::from_be_hex("10de7c173de05623f0e2e42a17cf59c1d915b1e53d6c54cef7dc95ad267a2940214ca13e9c7e7dc9ac559342863421c3")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("143a03b1fc6fda839c144391c1ef7b9e59c3eb1bd13b7754e83d4bffee79c641dc9907b239881ed4b4a166c6d4c17802")).to_montgomery(), y: Fp(U384::from_be_hex("0b99f1ae3d534d85206e681e1da80870d6cd0ce2d05efc369a147eb16098ada65098927dd88e7185bae6fa39f2d1ffba")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("1002321e4fcbaec5cac5a54f88f1872356be3ef9cc6536f6d71b700c749e9c3d81222df12bdd995f96e519dba8d1093b")).to_montgomery(), y: Fp(U384::from_be_hex("10cb166a97278650b9b6040d14a301c48d56872f5bd7aef050aa022ca95fcf5367a0aa19c4aeb5b089e40b28225acea9")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0f7b60f142bda8ef2e814b49d399926e41276be84a03563b0a46f7606079dce1f4283f40d73e5aa428508237ae05d925")).to_montgomery(), y: Fp(U384::from_be_hex("102638294c92eeb040dad8738b635e577050c71a9102932c9b7ccb92702a128ba105b10923cb741e0660fdabe80257f6")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0488466aff29251ce206966ccd74a8d1766a702ee07b654df8aba2a317e787b914450cded525ec3fb4e0195b7db37e56")).to_montgomery(), y: Fp(U384::from_be_hex("06c442aad6dce66dd2c4b46a4a0a6aeeeb4a6573072c84bd2b47c51f15516a30f95463b4e449e1d7fadf646cba36369a")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0f32feb3a06265918484cce3a1baa80d3ff1cc41e4ba48f792c5766e739cb2f70c1e8529a61a8a45b67c3dd9beefe40e")).to_montgomery(), y: Fp(U384::from_be_hex("1228f91c1034c3dcf273cfa215d27b41c015263c7fd261be9bf268568310b93ba72ae4ecb8a593c844132cc0b2de208d")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0134179eafd0c2ef198a6aeeed2261427da13bb8218a2030409bc2411124af5ed0a371d9cadf0de8d234c311d0fdc5c1")).to_montgomery(), y: Fp(U384::from_be_hex("0beaf05b2c105ce1de04a43f475f8c52204d86b336f46806f08d5dc0e530833672db5c99caade2635aaf0cffe81b9244")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0a5e9d948510b7d0fdb8ba29cf7352315ba41fc2bc17688600619423c7a6615aa7dc97a7a65c56a6d3eabe53fc962358")).to_montgomery(), y: Fp(U384::from_be_hex("160a20345b557fd66ab8f314c113caec891102955bc1f88cda559b434b3555d947cc435aebd17ed2885c440a2c00c4b9")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("038ba198560147bd5208858e9fded513d75b4da194859d71c33a22124ecc79d822c676991890fe680b4a92cff75ac035")).to_montgomery(), y: Fp(U384::from_be_hex("0e2c610ee5e1ff3fb319f55698e62d6715d7ec94234cd6181d145c2ad290d86889c64fefeb10484c39f4963ab824bd25")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0f586287dc99887451c3be67d740866ecd06120057ca642f9380f6d6e50105d6b2fbbc08275ce39337d93bd65022f8c3")).to_montgomery(), y: Fp(U384::from_be_hex("10794ad962a1c76b82708d63839cfe96250aff350fed1e00e140c5d5c7e6684d951a3bce4bdb182b50e5618dfceeeee3")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("10c42ed4014be5aa578e8a8f53a52bc914cb500c93bf5032ab5aa5aad6f4f1d2a1d932034191e9b9771b4c9a52c9aefe")).to_montgomery(), y: Fp(U384::from_be_hex("14ac58c5f8d3c7cf85fa6900552b44994fe91264510e676097d296952da1e75a05d36288e76357d5c96e4a0b7cda9c98")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0b69519a62b6fe9d3afead107fa783080d016032c90071f6fc2f7f18ea26cd9161bf39580949731c9f28174a8f5ac058")).to_montgomery(), y: Fp(U384::from_be_hex("0636ee0609aa665366334bb33d6884bd7fc2a15ff5f035878dac424fe44a566229206738b5ee968c19146ad25e73db22")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("06e9890da09288d399c3d69b73e2b61359e2b42342b960d8b3861fbd1480cda231895286426b850101f07da2dffd20e2")).to_montgomery(), y: Fp(U384::from_be_hex("00bc4d00edddab2b3770d7e5fdea932ceb0e9fb6c215706ce07145b6fae71f26b058086bd659269a43cbc88b48c450f3")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0ac7f5d7522ce98c02a6c28c629f43d9b02590a2ad95703c4e927b02ccf1bcc405edd73bd57621c80c4533b940a14fe3")).to_montgomery(), y: Fp(U384::from_be_hex("12411ecd9a3488b27215eb25a47eb7bcfc9f4b74271c7cb53edc2ff7a86e3d4b5c1a0e9ca0452673399b2822a5953167")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0ec2708e912050242f50c30af18f9935685ccdc2906a50ffb9868e2e6270a23b69acba09e8a89ec6e30f1fbabf229be6")).to_montgomery(), y: Fp(U384::from_be_hex("0a64299d7e3fd1d549ecfdd2926afde036ffac6430260ce52de3afd200ae9c0f0797e97a16d43dc5fda913c048e62174")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("151c91631d116d8d36065bff6da621ffa7c23742736bb887c9a6bffad367f03a4770ba05a92c4d7d50da182de5f0daf5")).to_montgomery(), y: Fp(U384::from_be_hex("14e8e18472158f7ad66f7c7fdac3848958166ac63b58771cad35b06bc8ec18a8ca1dba95e77e2ea1e3e165cdcd5ebc49")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0cdc7d2281d5dc2b5329171f8737bb3663db6a41183c1e12501474e78d407c3695b157151a6cfbe713ed88dad7b464af")).to_montgomery(), y: Fp(U384::from_be_hex("17b3d0e44c1a39749cc17c1c32c7823630ac1da0d3a7fe6ea0f63fcbfb62ede3bfb3b7025301c2181c38b86b6f2805cd")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("18e0a2f813e4eae89a82012435eb8049d109d8aa8f7ee15c37630ce57b43ca2c19b3758f810dd3652c7f11ec5bf0f76b")).to_montgomery(), y: Fp(U384::from_be_hex("055216087656c56f18f85284a345f70aae3d6e7f5f224ca41c2088653d186128bc6dfa6a843ab0391a027644c86000f9")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0e45584f1ee79e256522e93a779386dd3d245ce32ed631a6a4f5ac4a92c886d0793d2978bb4719999e729add3d9e8b6e")).to_montgomery(), y: Fp(U384::from_be_hex("0a54ad4506655d1dd43fb18d42e4c564b2ba34884c7b874205544be55d22ed07abb17ab42bd6ff4051085cfd4a1e4b9c")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("041dd487c9bcc4b5801b7cb5894cc38be830c114dc1a6e1258c39b3d02edd44e74df0fb5c162b25381e2bf75d091caa9")).to_montgomery(), y: Fp(U384::from_be_hex("044e0c4a2526869f653bc0cde3dff2ea2514c458e8b6d28f7243051f000f8543f32890c64451152550d8cc086186e711")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0fe51654e860cc26667015fe12228e355da276f0fb915de9ce5800e31adf4c795b018574190cdabfa6d2ed2740ea0907")).to_montgomery(), y: Fp(U384::from_be_hex("16825c9ffd9e8bcfdca3746727c2cf054d3db62214d391e32317da92d32d88ea7c9112daf24735ee41e9b91576a6c6f4")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("119e38f297b1e91df02ca5043d50dae909002d9793c8bb61957af0e598eb65d7f16a035e367aa32913e08f47d40c3069")).to_montgomery(), y: Fp(U384::from_be_hex("0625ed6897f855e6eefd02ec8fdd2c3a87c76b7860e8f2c55f1293644c3d9f4bae23eaf5f4021a533feefa4a7da2bbb9")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("168d240ebcabbaf5644185eb447db37a8dd7ab257f0ee09c5956cca86ca2ce140db50f18d84b310179980b2f95de548d")).to_montgomery(), y: Fp(U384::from_be_hex("16a82e945932fb317959a62d4a6764107d9e87c7d054a9c2ded788d0981d6da460fa1f5f193083ee2989a8f48fcb1ed4")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("04cb929f43609c6f45bbadd77b1e0924ea646f449acb52b2538866fd04d0925857d684447df5f8a8c2c897e2c25e9667")).to_montgomery(), y: Fp(U384::from_be_hex("0daf631acf55d9d9aa0712be0b801d50f86ec785b6c6b0a4d4164c06ec45ff8540ff5162fd4c7db80daa22972fb38983")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("074dffca2c072dbeb2e9243e3b773d641263aed1582dfc598cda6f769e6872d5b6929356acaf68be60504067eb1c9601")).to_montgomery(), y: Fp(U384::from_be_hex("0618586e929f2127c6c33ca96be5566c04104b349a379c617c8032a607a54c45da731fd50a75e2661228f11e3247cae7")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("156dda65ca3db309435454256832ed1944368e09e92a25a2810302b9bda67662b44dd808de11fe103afd851b8cb377d6")).to_montgomery(), y: Fp(U384::from_be_hex("08602a09c14cb102093616b0eb0859ec11fab0506c2b6a4faae95f077b9266222d1cd95a6cb75a6ca0efe41f98b59d34")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("167adcfbe0828d6f4e61f77b9c04aa6b40fed9421c9015138b7e5bb084059643b64151b6cd44e381312056e4a76af13c")).to_montgomery(), y: Fp(U384::from_be_hex("13b42154f7616ac65971567ce5eb454eab4b3b51408eef9edbea0714a6c180b7c511d8a90d6b8bf195201bc298688c1d")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("14f8b83fa2b4955a9a1f7b9fcd5f7d7a1756c205dfa9cb1e9c71a5ffcdf1119b9a7f37456e9994c396d3d02cf784191c")).to_montgomery(), y: Fp(U384::from_be_hex("151b4e1194c7c92a4b8cfe000c4633ac75308521eb54ba903d195f419c81a43d6f3f4991427398a712670fe7285e8324")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("026eef887387a214a6c6cffd039f4d8dc8b26ba2dee87cfd12d4862e2ff20dafb80f5c945ef2635727b11cc6d35d00b0")).to_montgomery(), y: Fp(U384::from_be_hex("03c31fa0840fe18668c8e7718299d2819afae08ba49da38c04f8aff37929dfd318107e75e59ea94bd41f7ccb677ca169")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0e812ee5bb20c8aed34849962f7f9f8925a49380252a7e8962a39da5f33f0179d4895796a42814070b8e777a7e2ebba3")).to_montgomery(), y: Fp(U384::from_be_hex("18268ac5ea008d8b20f104ee7f3c73b36f02f97fc256382df48fcdb245c80685450db96ff4701d5d39ab1df9abe18341")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0a8ad5efb651dec7ce40eace2c2fc380806c663351bdbb39639b9b06c3b4d0deea9b6bf886dce27a159cba19766daa58")).to_montgomery(), y: Fp(U384::from_be_hex("0202b9e7eeb22d0d50f1b97e302a47623f0a8b19873edb12ce4c3cf29b79b9c34fe2c903fc5031e87d5f8f5f5709bc61")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("19c4168f6eee10e74f06f8e35f83d1eb20df4a2d6ecfea03ab3171b67c402ef284e7ec8f6c9803ff83451c80c6fbee72")).to_montgomery(), y: Fp(U384::from_be_hex("0b9ea959a0b8800f9af8143a6b9e12155f6d582b3409f21b4358f54efe700171ab041ef05d8ae7bf26fd4e4a69fec549")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("05cda93e4aabe1207fd17c4dfb9c6055a2c9e5f35779b2d46b86a49fcd785c12de9bb11eec02846d469e31ad3365e3f2")).to_montgomery(), y: Fp(U384::from_be_hex("19f36b32626991d1ef4b5c5e91310eab40d6e1326143ca0c47d0a88c18823a7fbc8a8de8f1155ea798c01b9c961a3a64")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("14154504d24918e4a1bbba1494f6290fd251420e5e046126bc814b3c48c8b78cbc3c935a56fe01ab001ef518c037a1b3")).to_montgomery(), y: Fp(U384::from_be_hex("02f2f9006b9cc6ddf096d38cf9100904623d205164ca13c0b7a70e718408b034e1e71ac2039ed1ed09f755ea4dee9c9a")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("08fae99d0c8cba922d927bcce9c6c7cfb8797cb5ad1f045505c5971ccfad1e9927e5b01b8b9de6acc852e362457c4e04")).to_montgomery(), y: Fp(U384::from_be_hex("171035bc605cdf1df65e461907bb4ce4872c7b813fe070164ddc359e94715ed9651958e3611b43c0694b74159bae2b46")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("03600b4b8f83e4df8b4078956d7dac08a31b60ad0795dffa2839f9a5983388b80826f6f280803fa3e90286739fc994e1")).to_montgomery(), y: Fp(U384::from_be_hex("16af599defe4abfb367a8047220ecfdc99125017d55ea199700479a6eab039cfc9a38ef5a130a499ee4e63cd2a29ca52")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("00f4d31ed979443096f7d75a18a91a1ed463092994354170d28026e300545dac87b7d26d879a042b7ee324cda7b1cfb8")).to_montgomery(), y: Fp(U384::from_be_hex("0698c6c11bbe5054fc7afd44d30dd81c086195ae4fe8fb8481029f907f492e9847c113ecf2d1275edf9317e8f8fc53a8")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("139f7713d986b72cc02c807c797c134ce59834122be4a00aa9ebbc02c51a7af9349374424d4b528405d962a3835a7d8f")).to_montgomery(), y: Fp(U384::from_be_hex("197146fce190b2125f0afc3800ce28fba4a83718b3fb8db7f28a8b46f8e51d89f299bc30baebb715cd96219223522452")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("09686ba30afdbc11273b66bf1c7b5312dd54139b8ac593579e3f4f9969f96600ec4b91c1835d92ac149aa20da7d5a2d6")).to_montgomery(), y: Fp(U384::from_be_hex("09fdd1d94f73a955a661095f368496dbfb6b9d0a790f7e2fc6f3378c7953610bf55f74c868f1a51bcef707763a44861d")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("097060a3ca5b736f676c82f88bf3bf1589044ff81c81b853af7e127b6137642bff11a8325e2148d4ca55413280a0fd27")).to_montgomery(), y: Fp(U384::from_be_hex("12ccfc14466ffa157cc1ee48674525e7e159ed996cd03d3c0a7b0c904d29eb97fd9b107004fcf746b6fa5c1dd365ee82")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("14c18f42f19cf61aab32c33054704323f01286aab7fcb51b1e39df6ea9785e838cdc5447adc25d54fa94d6876b6dbbc2")).to_montgomery(), y: Fp(U384::from_be_hex("18e2dbf9c19eb1fc8e69840712e3fecc9e40e50e58c0a35b0b1c507c2ebbaa1cd146bfa9a6745295b13ccf04599322ae")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("00c60bfc64f53ef73d58414fe6e83fcfacfcf15b2dee0d4df792ab3d4b07a119f116910a7bebd023c781d7faf1335927")).to_montgomery(), y: Fp(U384::from_be_hex("0d9bf8deaf78d7f5368e2b6045fd97f0fde5f114ebc893f02f1176be6313818772c7a4287551312f079dbcfde22b62a7")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("054e511db0351db2705cdb6798b9eb8f340692bc8a9f1f498355cd91f147e00fefdf6d2f4042a9d86d3268c21e3b8c32")).to_montgomery(), y: Fp(U384::from_be_hex("0cfd3bbf27c64e3e06c5117532b3f2a190b58a8dda60bd9e4a1460b6436c4d4de3649e9af9c1109752a7b1c16deb8c38")).to_montgomery(), z: Fp::ONE } ,
                    ProjectivePoint{x: Fp(U384::from_be_hex("0b41aa318f23bef524afc5282b07ffaa7c3f721c96ef2004db90b1f891279c69ffbee596400a118b94a1f837258a1229")).to_montgomery(), y: Fp(U384::from_be_hex("04588f68aea1df51f66e03086b7bdf21d3f6211fe7b5f3ea05b4302dc45006c66bd83c0b0eda07acdc6638662f982331")).to_montgomery(), z: Fp::ONE }];

    #[test]
    fn addcheck() {
        for i in 0..50 {
            let a = BLSARR1[i];
            let b = BLSARR2[i];
            let c = BLSSUM[i];
            assert_eq!((a + b).to_affine(), c.to_affine());
        }
    }

    #[test]
    fn adddoub() {
        for i in 0..50 {
            let a = (BLSARR1[i]).mul(SCALAR_ARRAY[i]);
            assert_eq!((a + a).to_affine(), a.double().to_affine());
        }
        let a1: ProjectivePoint<BlsCurve> = ProjectivePoint::GENERATOR;
        assert_eq!((a1 + a1).to_affine(), a1.double().to_affine());
    }

    #[test]
    fn add_mix() {
        for i in 0..50 {
            let a = BLSARR1[i];
            let b = BLSARR2[i];
            let b1 = BLSARR2[i].to_affine();
            assert_eq!(a.add_mixed(&b1).to_affine(), (a + b).to_affine());
        }
    }

    #[test]
    fn checkmul() {
        for i in 0..50 {
            let a = BLSARR1[i];
            let b = SCALAR_ARRAY[i];
            let c = a.mul(b);
            assert_eq!(c.to_affine(), MULARR[i].to_affine());
        }
    }
    //................
    //.......Testing for multi exponentiation
    // #[test]
    // fn checkmulexp(){
    //     let mut points: Vec<AffinePoint<BlsCurve>> = Vec::new();
    //     let mut expo: Vec<Scalar> = Vec::new();
    //     let mut ans: ProjectivePoint<BlsCurve> = ProjectivePoint::<BlsCurve>::IDENTITY;
    //     for i in 1..50{
    //         points.push(BLSARR1[i].to_affine());
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
        //let mut points: Vec<AffinePoint<BlsCurve>> = Vec::new();
        let mut expo: Vec<Scalar> = Vec::new();
        let mut ans: ProjectivePoint<BlsCurve> = ProjectivePoint::<BlsCurve>::IDENTITY;
        for i in 0..50 {
            expo.push(SCALAR_ARRAY[i]);
            ans += MULARR[i];
        }
        let c = ProjectivePoint::pippenger_msm(&BLSARR1, &expo).to_affine();
        assert_eq!(ans.to_affine(), c);
    }

    #[test]
    fn checkconversion() {
        let a = BLSARR1[0].mul(SCALAR_ARRAY[0]);
        let b = a.to_affine();
        let c = b.to_projective();
        let d = c.to_affine();
        assert_eq!(b, d);
    }

    #[test]
    fn checkgen() {
        let a = ProjectivePoint::<BlsCurve>::GENERATOR;
        let b = a.mul(Scalar(<BlsCurve as Curve>::ORDER));
        assert!(b.is_identity())
    }
}
