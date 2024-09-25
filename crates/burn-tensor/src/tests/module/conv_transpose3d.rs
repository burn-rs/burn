#[burn_tensor_testgen::testgen(module_conv_transpose3d)]
mod tests {
    use super::*;
    use burn_tensor::module::conv_transpose3d;
    use burn_tensor::ops::ConvTransposeOptions;
    use burn_tensor::{Shape, Tensor};

    #[test]
    fn test_conv_transpose3d_simple_1() {
        let test = ConvTranspose3dTestCase {
            batch_size: 1,
            channels_in: 1,
            channels_out: 1,
            kernel_size_1: 3,
            kernel_size_2: 3,
            kernel_size_3: 3,
            padding_1: 1,
            padding_2: 1,
            padding_3: 1,
            padding_out_1: 0,
            padding_out_2: 0,
            padding_out_3: 0,
            stride_1: 1,
            stride_2: 1,
            stride_3: 1,
            dilation_1: 1,
            dilation_2: 1,
            dilation_3: 1,
            groups: 1,
            depth: 2,
            height: 2,
            width: 2,
        };

        test.assert_output(TestTensor::from([[[
            [[96., 124.], [180., 208.]],
            [[348., 376.], [432., 460.]],
        ]]]));
    }
    #[test]
    fn test_conv_transpose3d_simple_2() {
        let test = ConvTranspose3dTestCase {
            batch_size: 1,
            channels_in: 3,
            channels_out: 3,
            kernel_size_1: 3,
            kernel_size_2: 3,
            kernel_size_3: 3,
            padding_1: 1,
            padding_2: 1,
            padding_3: 1,
            padding_out_1: 0,
            padding_out_2: 0,
            padding_out_3: 0,
            stride_1: 1,
            stride_2: 1,
            stride_3: 1,
            dilation_1: 1,
            dilation_2: 1,
            dilation_3: 1,
            groups: 1,
            depth: 4,
            height: 4,
            width: 4,
        };

        test.assert_output(TestTensor::from([[
            [
                [
                    [238452., 360588., 363756., 244488.],
                    [367929., 556353., 561186., 377163.],
                    [380745., 575685., 580518., 390123.],
                    [261192., 394896., 398172., 267564.],
                ],
                [
                    [394083., 595827., 600822., 403749.],
                    [607635., 918648., 926262., 622404.],
                    [627831., 949104., 956718., 642816.],
                    [430353., 650529., 655686., 440523.],
                ],
                [
                    [447075., 675747., 680742., 457317.],
                    [688419., 1040472., 1048086., 704052.],
                    [708615., 1070928., 1078542., 724464.],
                    [485073., 733041., 738198., 495819.],
                ],
                [
                    [328656., 496632., 500124., 335892.],
                    [505611., 763983., 769302., 516645.],
                    [519723., 785259., 790578., 530901.],
                    [355428., 536988., 540588., 363000.],
                ],
            ],
            [
                [
                    [286729., 433489., 437629., 294061.],
                    [442288., 668620., 674911., 453466.],
                    [458992., 693784., 700075., 470314.],
                    [314653., 475573., 479821., 322321.],
                ],
                [
                    [474274., 716842., 723295., 485884.],
                    [730837., 1104544., 1114345., 748522.],
                    [756865., 1143748., 1153549., 774766.],
                    [518320., 783208., 789823., 530434.],
                ],
                [
                    [542818., 820090., 826543., 555004.],
                    [834949., 1261360., 1271161., 853498.],
                    [860977., 1300564., 1310365., 879742.],
                    [588592., 889048., 895663., 601282.],
                ],
                [
                    [397669., 600637., 605101., 406201.],
                    [611074., 922906., 929683., 624052.],
                    [629074., 950014., 956791., 642196.],
                    [429625., 648769., 653341., 438493.],
                ],
            ],
            [
                [
                    [335006., 506390., 511502., 343634.],
                    [516647., 780887., 788636., 529769.],
                    [537239., 811883., 819632., 550505.],
                    [368114., 556250., 561470., 377078.],
                ],
                [
                    [554465., 837857., 845768., 568019.],
                    [854039., 1290440., 1302428., 874640.],
                    [885899., 1338392., 1350380., 906716.],
                    [606287., 915887., 923960., 620345.],
                ],
                [
                    [638561., 964433., 972344., 652691.],
                    [981479., 1482248., 1494236., 1002944.],
                    [1013339., 1530200., 1542188., 1035020.],
                    [692111., 1045055., 1053128., 706745.],
                ],
                [
                    [466682., 704642., 710078., 476510.],
                    [716537., 1081829., 1090064., 731459.],
                    [738425., 1114769., 1123004., 753491.],
                    [503822., 760550., 766094., 513986.],
                ],
            ],
        ]]));
    }

    #[test]
    fn test_conv_transpose3d_stride_2() {
        let test = ConvTranspose3dTestCase {
            batch_size: 1,
            channels_in: 1,
            channels_out: 1,
            kernel_size_1: 2,
            kernel_size_2: 2,
            kernel_size_3: 2,
            padding_1: 0,
            padding_2: 0,
            padding_3: 0,
            padding_out_1: 0,
            padding_out_2: 0,
            padding_out_3: 0,
            stride_1: 2,
            stride_2: 2,
            stride_3: 2,
            dilation_1: 1,
            dilation_2: 1,
            dilation_3: 1,
            groups: 1,
            depth: 2,
            height: 2,
            width: 2,
        };

        test.assert_output(TestTensor::from([[[
            [
                [0., 0., 0., 1.],
                [0., 0., 2., 3.],
                [0., 2., 0., 3.],
                [4., 6., 6., 9.],
            ],
            [
                [0., 0., 4., 5.],
                [0., 0., 6., 7.],
                [8., 10., 12., 15.],
                [12., 14., 18., 21.],
            ],
            [
                [0., 4., 0., 5.],
                [8., 12., 10., 15.],
                [0., 6., 0., 7.],
                [12., 18., 14., 21.],
            ],
            [
                [16., 20., 20., 25.],
                [24., 28., 30., 35.],
                [24., 30., 28., 35.],
                [36., 42., 42., 49.],
            ],
        ]]]));
    }

    #[test]
    fn test_conv_transpose3d_dilation_2() {
        let test = ConvTranspose3dTestCase {
            batch_size: 1,
            channels_in: 2,
            channels_out: 2,
            kernel_size_1: 3,
            kernel_size_2: 3,
            kernel_size_3: 3,
            padding_1: 1,
            padding_2: 1,
            padding_3: 1,
            padding_out_1: 1,
            padding_out_2: 1,
            padding_out_3: 1,
            stride_1: 1,
            stride_2: 1,
            stride_3: 1,
            dilation_1: 2,
            dilation_2: 2,
            dilation_3: 2,
            groups: 1,
            depth: 2,
            height: 2,
            width: 2,
        };

        test.assert_output(TestTensor::from([[
            [
                [
                    [810., 776., 832., 796., 854.],
                    [756., 712., 774., 728., 792.],
                    [876., 836., 898., 856., 920.],
                    [810., 760., 828., 776., 846.],
                    [942., 896., 964., 916., 986.],
                ],
                [
                    [720., 660., 734., 672., 748.],
                    [606., 536., 616., 544., 626.],
                    [762., 696., 776., 708., 790.],
                    [636., 560., 646., 568., 656.],
                    [804., 732., 818., 744., 832.],
                ],
                [
                    [1008., 956., 1030., 976., 1052.],
                    [918., 856., 936., 872., 954.],
                    [1074., 1016., 1096., 1036., 1118.],
                    [972., 904., 990., 920., 1008.],
                    [1140., 1076., 1162., 1096., 1184.],
                ],
                [
                    [846., 768., 860., 780., 874.],
                    [696., 608., 706., 616., 716.],
                    [888., 804., 902., 816., 916.],
                    [726., 632., 736., 640., 746.],
                    [930., 840., 944., 852., 958.],
                ],
                [
                    [1206., 1136., 1228., 1156., 1250.],
                    [1080., 1000., 1098., 1016., 1116.],
                    [1272., 1196., 1294., 1216., 1316.],
                    [1134., 1048., 1152., 1064., 1170.],
                    [1338., 1256., 1360., 1276., 1382.],
                ],
            ],
            [
                [
                    [1405., 1317., 1427., 1337., 1449.],
                    [1243., 1145., 1261., 1161., 1279.],
                    [1471., 1377., 1493., 1397., 1515.],
                    [1297., 1193., 1315., 1209., 1333.],
                    [1537., 1437., 1559., 1457., 1581.],
                ],
                [
                    [1099., 985., 1113., 997., 1127.],
                    [877., 753., 887., 761., 897.],
                    [1141., 1021., 1155., 1033., 1169.],
                    [907., 777., 917., 785., 927.],
                    [1183., 1057., 1197., 1069., 1211.],
                ],
                [
                    [1603., 1497., 1625., 1517., 1647.],
                    [1405., 1289., 1423., 1305., 1441.],
                    [1669., 1557., 1691., 1577., 1713.],
                    [1459., 1337., 1477., 1353., 1495.],
                    [1735., 1617., 1757., 1637., 1779.],
                ],
                [
                    [1225., 1093., 1239., 1105., 1253.],
                    [967., 825., 977., 833., 987.],
                    [1267., 1129., 1281., 1141., 1295.],
                    [997., 849., 1007., 857., 1017.],
                    [1309., 1165., 1323., 1177., 1337.],
                ],
                [
                    [1801., 1677., 1823., 1697., 1845.],
                    [1567., 1433., 1585., 1449., 1603.],
                    [1867., 1737., 1889., 1757., 1911.],
                    [1621., 1481., 1639., 1497., 1657.],
                    [1933., 1797., 1955., 1817., 1977.],
                ],
            ],
        ]]));
    }

    #[test]
    fn test_conv_transpose3d_stride2_out_padding() {
        let test = ConvTranspose3dTestCase {
            batch_size: 1,
            channels_in: 2,
            channels_out: 2,
            kernel_size_1: 3,
            kernel_size_2: 3,
            kernel_size_3: 3,
            padding_1: 1,
            padding_2: 1,
            padding_3: 1,
            padding_out_1: 1,
            padding_out_2: 1,
            padding_out_3: 1,
            stride_1: 2,
            stride_2: 2,
            stride_3: 2,
            dilation_1: 1,
            dilation_2: 1,
            dilation_3: 1,
            groups: 1,
            depth: 2,
            height: 4,
            width: 4,
        };

        test.assert_output(TestTensor::from([[
            [
                [
                    [2144., 4366., 2224., 4526., 2304., 4686., 2384., 2422.],
                    [4584., 9324., 4744., 9644., 4904., 9964., 5064., 5148.],
                    [2464., 5006., 2544., 5166., 2624., 5326., 2704., 2750.],
                    [5224., 10604., 5384., 10924., 5544., 11244., 5704., 5804.],
                    [2784., 5646., 2864., 5806., 2944., 5966., 3024., 3078.],
                    [5864., 11884., 6024., 12204., 6184., 12524., 6344., 6460.],
                    [3104., 6286., 3184., 6446., 3264., 6606., 3344., 3406.],
                    [3272., 6628., 3358., 6800., 3444., 6972., 3530., 3592.],
                ],
                [
                    [5280., 10716., 5440., 11036., 5600., 11356., 5760., 5868.],
                    [
                        11152., 22616., 11472., 23256., 11792., 23896., 12112., 12344.,
                    ],
                    [5920., 11996., 6080., 12316., 6240., 12636., 6400., 6524.],
                    [
                        12432., 25176., 12752., 25816., 13072., 26456., 13392., 13656.,
                    ],
                    [6560., 13276., 6720., 13596., 6880., 13916., 7040., 7180.],
                    [
                        13712., 27736., 14032., 28376., 14352., 29016., 14672., 14968.,
                    ],
                    [7200., 14556., 7360., 14876., 7520., 15196., 7680., 7836.],
                    [7632., 15432., 7804., 15776., 7976., 16120., 8148., 8304.],
                ],
                [
                    [3424., 6926., 3504., 7086., 3584., 7246., 3664., 3734.],
                    [7144., 14444., 7304., 14764., 7464., 15084., 7624., 7772.],
                    [3744., 7566., 3824., 7726., 3904., 7886., 3984., 4062.],
                    [7784., 15724., 7944., 16044., 8104., 16364., 8264., 8428.],
                    [4064., 8206., 4144., 8366., 4224., 8526., 4304., 4390.],
                    [8424., 17004., 8584., 17324., 8744., 17644., 8904., 9084.],
                    [4384., 8846., 4464., 9006., 4544., 9166., 4624., 4718.],
                    [4648., 9380., 4734., 9552., 4820., 9724., 4906., 5000.],
                ],
                [
                    [4000., 8096., 4098., 8292., 4196., 8488., 4294., 4364.],
                    [8368., 16928., 8564., 17320., 8760., 17712., 8956., 9104.],
                    [4392., 8880., 4490., 9076., 4588., 9272., 4686., 4764.],
                    [9152., 18496., 9348., 18888., 9544., 19280., 9740., 9904.],
                    [4784., 9664., 4882., 9860., 4980., 10056., 5078., 5164.],
                    [
                        9936., 20064., 10132., 20456., 10328., 20848., 10524., 10704.,
                    ],
                    [5176., 10448., 5274., 10644., 5372., 10840., 5470., 5564.],
                    [5440., 10982., 5544., 11190., 5648., 11398., 5752., 5846.],
                ],
            ],
            [
                [
                    [3009., 6149., 3143., 6417., 3277., 6685., 3411., 3449.],
                    [6529., 13321., 6797., 13857., 7065., 14393., 7333., 7417.],
                    [3545., 7221., 3679., 7489., 3813., 7757., 3947., 3993.],
                    [7601., 15465., 7869., 16001., 8137., 16537., 8405., 8505.],
                    [4081., 8293., 4215., 8561., 4349., 8829., 4483., 4537.],
                    [8673., 17609., 8941., 18145., 9209., 18681., 9477., 9593.],
                    [4617., 9365., 4751., 9633., 4885., 9901., 5019., 5081.],
                    [4785., 9707., 4925., 9987., 5065., 10267., 5205., 5267.],
                ],
                [
                    [7873., 16009., 8141., 16545., 8409., 17081., 8677., 8785.],
                    [
                        16769., 34065., 17305., 35137., 17841., 36209., 18377., 18609.,
                    ],
                    [8945., 18153., 9213., 18689., 9481., 19225., 9749., 9873.],
                    [
                        18913., 38353., 19449., 39425., 19985., 40497., 20521., 20785.,
                    ],
                    [
                        10017., 20297., 10285., 20833., 10553., 21369., 10821., 10961.,
                    ],
                    [
                        21057., 42641., 21593., 43713., 22129., 44785., 22665., 22961.,
                    ],
                    [
                        11089., 22441., 11357., 22977., 11625., 23513., 11893., 12049.,
                    ],
                    [
                        11521., 23317., 11801., 23877., 12081., 24437., 12361., 12517.,
                    ],
                ],
                [
                    [5153., 10437., 5287., 10705., 5421., 10973., 5555., 5625.],
                    [
                        10817., 21897., 11085., 22433., 11353., 22969., 11621., 11769.,
                    ],
                    [5689., 11509., 5823., 11777., 5957., 12045., 6091., 6169.],
                    [
                        11889., 24041., 12157., 24577., 12425., 25113., 12693., 12857.,
                    ],
                    [6225., 12581., 6359., 12849., 6493., 13117., 6627., 6713.],
                    [
                        12961., 26185., 13229., 26721., 13497., 27257., 13765., 13945.,
                    ],
                    [6761., 13653., 6895., 13921., 7029., 14189., 7163., 7257.],
                    [7025., 14187., 7165., 14467., 7305., 14747., 7445., 7539.],
                ],
                [
                    [5729., 11607., 5881., 11911., 6033., 12215., 6185., 6255.],
                    [
                        12041., 24381., 12345., 24989., 12649., 25597., 12953., 13101.,
                    ],
                    [6337., 12823., 6489., 13127., 6641., 13431., 6793., 6871.],
                    [
                        13257., 26813., 13561., 27421., 13865., 28029., 14169., 14333.,
                    ],
                    [6945., 14039., 7097., 14343., 7249., 14647., 7401., 7487.],
                    [
                        14473., 29245., 14777., 29853., 15081., 30461., 15385., 15565.,
                    ],
                    [7553., 15255., 7705., 15559., 7857., 15863., 8009., 8103.],
                    [7817., 15789., 7975., 16105., 8133., 16421., 8291., 8385.],
                ],
            ],
        ]]));
    }

    #[test]
    fn test_conv_transpose3d_groups_2() {
        let test = ConvTranspose3dTestCase {
            batch_size: 1,
            channels_in: 2,
            channels_out: 2,
            kernel_size_1: 3,
            kernel_size_2: 3,
            kernel_size_3: 3,
            padding_1: 1,
            padding_2: 1,
            padding_3: 1,
            padding_out_1: 0,
            padding_out_2: 0,
            padding_out_3: 0,
            stride_1: 1,
            stride_2: 1,
            stride_3: 1,
            dilation_1: 1,
            dilation_2: 1,
            dilation_3: 1,
            groups: 2,
            depth: 2,
            height: 2,
            width: 2,
        };

        test.assert_output(TestTensor::from([[
            [[[96., 124.], [180., 208.]], [[348., 376.], [432., 460.]]],
            [
                [[2997., 3089.], [3273., 3365.]],
                [[3825., 3917.], [4101., 4193.]],
            ],
        ]]));
    }

    #[test]
    fn test_conv_transpose3d_groups_different_channels() {
        let test = ConvTranspose3dTestCase {
            batch_size: 1,
            channels_in: 2,
            channels_out: 6,
            kernel_size_1: 3,
            kernel_size_2: 3,
            kernel_size_3: 3,
            padding_1: 0,
            padding_2: 0,
            padding_3: 0,
            padding_out_1: 0,
            padding_out_2: 0,
            padding_out_3: 0,
            stride_1: 1,
            stride_2: 1,
            stride_3: 1,
            dilation_1: 1,
            dilation_2: 1,
            dilation_3: 1,
            groups: 2,
            depth: 2,
            height: 2,
            width: 2,
        };

        test.assert_output(TestTensor::from([[
            [
                [
                    [0., 0., 1., 2.],
                    [0., 5., 11., 11.],
                    [6., 23., 29., 23.],
                    [12., 32., 37., 24.],
                ],
                [
                    [0., 13., 23., 21.],
                    [30., 96., 124., 86.],
                    [66., 180., 208., 134.],
                    [66., 161., 179., 107.],
                ],
                [
                    [36., 103., 113., 75.],
                    [138., 348., 376., 230.],
                    [174., 432., 460., 278.],
                    [138., 323., 341., 197.],
                ],
                [
                    [72., 166., 175., 100.],
                    [192., 433., 455., 255.],
                    [222., 499., 521., 291.],
                    [144., 318., 331., 182.],
                ],
            ],
            [
                [
                    [1., 28., 29., 30.],
                    [55., 168., 174., 120.],
                    [61., 186., 192., 132.],
                    [67., 168., 173., 106.],
                ],
                [
                    [109., 284., 294., 184.],
                    [355., 853., 881., 519.],
                    [391., 937., 965., 567.],
                    [283., 648., 666., 378.],
                ],
                [
                    [145., 374., 384., 238.],
                    [463., 1105., 1133., 663.],
                    [499., 1189., 1217., 711.],
                    [355., 810., 828., 468.],
                ],
                [
                    [181., 410., 419., 236.],
                    [463., 1028., 1050., 580.],
                    [493., 1094., 1116., 616.],
                    [307., 670., 683., 372.],
                ],
            ],
            [
                [
                    [2., 56., 57., 58.],
                    [110., 331., 337., 229.],
                    [116., 349., 355., 241.],
                    [122., 304., 309., 188.],
                ],
                [
                    [218., 555., 565., 347.],
                    [680., 1610., 1638., 952.],
                    [716., 1694., 1722., 1000.],
                    [500., 1135., 1153., 649.],
                ],
                [
                    [254., 645., 655., 401.],
                    [788., 1862., 1890., 1096.],
                    [824., 1946., 1974., 1144.],
                    [572., 1297., 1315., 739.],
                ],
                [
                    [290., 654., 663., 372.],
                    [734., 1623., 1645., 905.],
                    [764., 1689., 1711., 941.],
                    [470., 1022., 1035., 562.],
                ],
            ],
            [
                [
                    [651., 1388., 1405., 750.],
                    [1485., 3150., 3188., 1690.],
                    [1539., 3264., 3302., 1750.],
                    [873., 1840., 1861., 982.],
                ],
                [
                    [1695., 3578., 3620., 1910.],
                    [3789., 7967., 8059., 4233.],
                    [3921., 8243., 8335., 4377.],
                    [2181., 4566., 4616., 2416.],
                ],
                [
                    [1875., 3956., 3998., 2108.],
                    [4185., 8795., 8887., 4665.],
                    [4317., 9071., 9163., 4809.],
                    [2397., 5016., 5066., 2650.],
                ],
                [
                    [1191., 2490., 2515., 1316.],
                    [2613., 5450., 5504., 2870.],
                    [2691., 5612., 5666., 2954.],
                    [1473., 3062., 3091., 1608.],
                ],
            ],
            [
                [
                    [868., 1848., 1865., 994.],
                    [1972., 4177., 4215., 2231.],
                    [2026., 4291., 4329., 2291.],
                    [1144., 2408., 2429., 1280.],
                ],
                [
                    [2236., 4713., 4755., 2505.],
                    [4978., 10452., 10544., 5530.],
                    [5110., 10728., 10820., 5674.],
                    [2830., 5917., 5967., 3119.],
                ],
                [
                    [2416., 5091., 5133., 2703.],
                    [5374., 11280., 11372., 5962.],
                    [5506., 11556., 11648., 6106.],
                    [3046., 6367., 6417., 3353.],
                ],
                [
                    [1516., 3166., 3191., 1668.],
                    [3316., 6909., 6963., 3627.],
                    [3394., 7071., 7125., 3711.],
                    [1852., 3846., 3875., 2014.],
                ],
            ],
            [
                [
                    [1085., 2308., 2325., 1238.],
                    [2459., 5204., 5242., 2772.],
                    [2513., 5318., 5356., 2832.],
                    [1415., 2976., 2997., 1578.],
                ],
                [
                    [2777., 5848., 5890., 3100.],
                    [6167., 12937., 13029., 6827.],
                    [6299., 13213., 13305., 6971.],
                    [3479., 7268., 7318., 3822.],
                ],
                [
                    [2957., 6226., 6268., 3298.],
                    [6563., 13765., 13857., 7259.],
                    [6695., 14041., 14133., 7403.],
                    [3695., 7718., 7768., 4056.],
                ],
                [
                    [1841., 3842., 3867., 2020.],
                    [4019., 8368., 8422., 4384.],
                    [4097., 8530., 8584., 4468.],
                    [2231., 4630., 4659., 2420.],
                ],
            ],
        ]]));
    }

    struct ConvTranspose3dTestCase {
        batch_size: usize,
        channels_in: usize,
        channels_out: usize,
        kernel_size_1: usize,
        kernel_size_2: usize,
        kernel_size_3: usize,
        padding_1: usize,
        padding_2: usize,
        padding_3: usize,
        padding_out_1: usize,
        padding_out_2: usize,
        padding_out_3: usize,
        stride_1: usize,
        stride_2: usize,
        stride_3: usize,
        dilation_1: usize,
        dilation_2: usize,
        dilation_3: usize,
        groups: usize,
        depth: usize,
        height: usize,
        width: usize,
    }

    impl ConvTranspose3dTestCase {
        fn assert_output(self, y: TestTensor<5>) {
            let shape_x = Shape::new([
                self.batch_size,
                self.channels_in,
                self.depth,
                self.height,
                self.width,
            ]);
            let shape_weights = Shape::new([
                self.channels_in,
                self.channels_out / self.groups,
                self.kernel_size_1,
                self.kernel_size_2,
                self.kernel_size_3,
            ]);
            let device = Default::default();
            let weights = TestTensor::from(
                TestTensorInt::arange(0..shape_weights.num_elements() as i64, &device)
                    .reshape::<5, _>(shape_weights)
                    .into_data(),
            );
            let bias = TestTensor::from(
                TestTensorInt::arange(0..self.channels_out as i64, &device).into_data(),
            );
            let x = TestTensor::from(
                TestTensorInt::arange(0..shape_x.num_elements() as i64, &device)
                    .reshape::<5, _>(shape_x)
                    .into_data(),
            );
            let output = conv_transpose3d(
                x,
                weights,
                Some(bias),
                ConvTransposeOptions::new(
                    [self.stride_1, self.stride_2, self.stride_3],
                    [self.padding_1, self.padding_2, self.padding_3],
                    [self.padding_out_1, self.padding_out_2, self.padding_out_3],
                    [self.dilation_1, self.dilation_2, self.dilation_3],
                    self.groups,
                ),
            );

            y.to_data().assert_approx_eq(&output.into_data(), 3);
        }
    }
}
