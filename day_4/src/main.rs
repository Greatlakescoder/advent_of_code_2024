fn word_search() {
    let input: String = String::from(
        "MMSSMSMXAMXMSSSSMMXSSMSAMASASXXAAXMAXXAMXAXSXMAMXMMMMSSMXMAAAMXMMMXXAMMAMSAMXXAMMMAAMMMSMMMMMXXSAMXMASXMXSMSXMXMMMSMSAMXSSMMXMMASMSMSMMMAMMS
AASAAAXMSMMAMAXAASMMAAMASXMMSASMSSXASAMXXAXMASAXXMMMAAAXMAMXMSAMSMSSMSMAMMMMSMAMASMMSXMAXXAASXAXAMXMASAMMSAMXMXMAAAXMASXMAMSAXXMSAAAAAASASXS
MXSMMMMAAMAAMXMSMMASMMMAMAAAMAMAAMMAXAMXSSXSAMSMSAMMMSSSMSXMAMAMXMAAXXMMSMAAASAMXSXASMSMSXSMSAMSXMAXASAMAMAMXMASMSXXSAMMSSMSAMSASMMSMSMSASAS
MMMASXSSMXSASXAXASMMAXMMSSMMMXMMMMSSSMSMMAAXXMAXXAMXMMXAAXAXASXMAMSXMSAMXMMSXSXSXMMXMAMXMAMMXAMAMMSMASMMASXMSSXSAMAAMAMMAMXMAAMAMMXXXXXMAMAX
SASXMAAXXMMAMXMMAMXSXMMMXASMMMMXSAAAAAAXMMMMMMXXMAMXMMSMMMSMXAXXXMAASAMXASXMAXXAMXASMSMAMXMASMMXSAAMXMASASAMAMAMAMMXAAMMAMXMMMMMXSAMASMMXMXM
SXSAMXMXMSMAMAMSAMXMXXAASAMXXASAMMSMMXMSMAMAMMASXSSMAAAAAAXMMMMXSMMMSAMMMMAMAMSMSMMSAMMMSSMAMXAXMMMSMSXMASXMASAMAXXAXAXSASAXAXXAAXAMAMAMAMAA
SASXSASAAASMSMMAAMXMSXMMXASXSSMXXAMXSMMXMASMXMXSAAMMMXXSMSXSAASMMSMASAMAAXMMAXXXAAXMXMASAAMAXMMSAMMAAXAMXMMSAXXXSXSAMSMSASXSASMMSSXMASAMAXXX
MAMXSASMSMSMAXMSSMXXMASMSXMASXMSMXSAXAAMMXSXXSMMMMMSMSAMXMASMXMAAXXAMAMSMSXSMSMSSSMSMSXSSSMXSASMASMSMSSMXMAMMSXMXAMXXMASMMAMAXXXAMXXXSXSSSSS
MAMMMAMXAXXMAXXAMXMASAMAASMSMAXXAAMMMMMSMAMXAMAXMMAAAMAMAMAMMSSMMMMSMSMAMSAMAAAXAMXAASAMAXXMAXXMAMXAMAAAMAXXMAXAMXMASMAMMMAMMMSMMSSSXXAXXAAA
SASXMAMSASAMXSMMXSMAMAMXMAXMSAMMMMMSASAMMAMSASXMSSMMMMAMXMSXAXXMASMXAMSSSMAMXMSMSMSMSMAMAMXAXMXMMSSMMSXMASXSSSMXSXXXXMAMXXAXAMXASAAMAMSMMMMM
SXXMMSMMASMMAMASAAMXXMMMXMAMAMXXMAASAXAXMAXAAAAAAAASMSSXSXXMSSMSXSAMMMAXAMXMSAMXAAXMMMAMMSMSASXSXAMXAAMSXXAMAAXAMMMMSSSSMSMSSXSMMMSMAAAXMASX
XMSSXMAMAMAXASAMSSMSXXSAAMXXAXXSXMMMASXMMSSMMMMMSSMMAXMASXMAMAAAXMAMXMASXMAMAMXXMSMAASASXAXMAXXMMMMMMXMXAMAMSMMXSAMAXAAXAAMMMASMXXMXXSSSMASM
MMAMASAMXSXMMMMXXXAMAASMXMMMMAMSAMXMMXMAXAMXXXXXMXAMXMMAMAMMMMAMSXASASASMSSSMMXSAXXSMSMMXSSMSSXSAXAASXMMXXAMMAMXAXSMMMMMMMSAMAMXMASXXMXAMASM
AMASMMMXMXAAXAXXAMAMMMXAXXAAAAASXMASAAMXMMMAMSASMSXMXXMASXMMSMMAMXAMAMAMAMXAAAMMAMAXAMAAAMXMXMAMSMXMSASAASMMMXMAMAXMSAMMSMSASMXXASAMXSXMMASX
MSASAASXSSSMMSSSXXAXMXMMXSSSSSXXMAMAMSMSMXMAXXAAAAXAMXSAMASAAAMSMSMMSMSMSSXSMXSMSMMMMMMMSSMSAMXMMSMAXAMMXMSASASMSMSXXAXSAASAMXSMXMAAMSAMMXSM
MMMSMMSAAMXMXMAMAXSMMXXSAMXAAXXMASXMAMAXMAMMMXSMSMXSAMMXSAMSMSMAAAXSXAXAXMAXSXMAMXXAAAXMXAMSMSMSAAMMMSMXMASMMASAAASASXMSMXMXMAXMAXAMXMAMAMMM
XMAMXAMMMMAMXMAMSMXASMSAMSSMMMAXAMXXAMSMSSSSSMAAXMAXXAAAMMMMXAMMSMSMMMMSMMSMAMMAMMSSSSSXSSMXMXAMMMXAXAAMMAMXMAMMMXMAAAXXMSMXMXSXAMSMSSMMMSAA
MMASMXSMMSASMSAMAAXAMXMAMAAAXSMMSSXMXXAAAAAAXAMSMMMSSMMMMXAXSMMMMMMXAMAXAMMAAAMASXXMAAMAMXMXMMSSXMSSSMXSMXSAMXSSMMSMSXMSMAMAMXXMMMXAXAXSASMS
XSASMSAMXSXAASMSMSMSMSSMMSSMMAAAMMXMSSMSMMMMMSXMASAAAXSSSMMXSAASMMSSXSASAMASMMSASMSMMXMASAMASMASXXAMAXXMAXSASXMMAMXAXXMASMSASASASXSSSMMMASAX
XMSSMAAXMSXMAMXAAAAXAMMSAXAMSSMMXMAXMAXXXMXAMXXSAMMSXMASMAMAXMMMAAAAXMXSXMAMMXMAXAXXMASASASASMAMMMMSAMXMXMSAMMASXMMMMAMAMXSMSAXSAAAAXXAAAMAS
ASAMXMXMAXAXMAXMSMSMXMAMXSAXAAXAASMSSSMMMAASMMMMMSXMAXXXSAMSSMXXXMMMMSMMXMMXSAMXMSMSAMMASXXASMASAAAMASXXMAMAMSMMMXMASXMAMMXAMAMMMMMMMXMAXXXA
MMMSXXAXMMAMXSMAAAAMAMXMASAMXMMSMSAAXAAXMAAAAAAAAXXMSMSMSXXAAXSSSMSXAXAMMSMAMAXXSXAMSSMAMXMAMXXXMXMSXMMMAMMSMSAMXMXXXMSSSMMMMAXAMXMASXSSMSXM
XSXAMMSMSAMSAMMXMMAXAXMSAMXAXXXAXMMMXSAMXSXXXMMMMMSAMAAAXMMSSMMAAAAMSXSAASMMSSMMAMSMAMMMSAMASXMSSSXMMAXAXSAXSXMXAAMMAXAAAXAXMXSMMAMXMXMAAASX
SAMXSAMAMAMSAXAXSSMXMMAMASMSSMSMXMAAAMAMAMASMSMSAAMAMSMMMAAMAXMAMMMSMMMMMMAAAAMSAMAMAMAAXMSASAAAAXAXXXMXXMAMAMXSASAMMMMSMSMMSAXMSMMAAAXMMMAX
SSMAMASMMAMMMMSMMASAAMSMXMXAAAAAAMMMMSAMSXMXAAAMMMMAMAAMSMMSAMMSXXXMXAASXSMMSXMMASAMMSMMSMMAMMMMSMMMSXSXMMSMMXMAXMMXMAXAAMAAMAMXAASMSMSXXXMM
XAMXXAXAMXXXSAXMSAMMXMXAAMMSMSXSMSSXMSMMXAMMMMMMMSSSSMSMAAMMMXXXXSMSSSMSAAXXXMSSMMAMMAMMAXSSSXMAMAXXAAMAMAAAXXMAXMMMXSSMSMMMSAXMAXXAAXAMXSXS
SSMSMSSMMSMMMAXAMXMSMXSSXSAXMXMAXAXAMXMASAMXXAXMXAXMAXAXSXMAMMMSMMAAMXAMMMMMAMAAAMMMSAXSAMXMAAMAXXMXMMSAMSSMMMSAMMAMXMAMXXSASXASMMMSMMXMASAS
MAAAAXAAAAASMSMSMAASXAXXAMMSMXASMSSMMMMMSAMXSMSMMMSASXXXMASXXAAXAMMMMMSMAXAAAMSSMMMAMXMMMSMSSMMSSMXMXXMAMAAXSAMASXSSXSAMMAMASAMXAXXMASXXAMAM
SMMMSSSMSSSMAXAAXSXSMMMSAMAAAXSXAXXXAMAMMMAAMXAXSXSAMXMMMMAASMXMAMXSAAASXSSSXMMMAAMSSSMXXAAMAXXMAXXASAMMMMSASMMXMAAAXSAMXAMXMMXSXMXSMMMMSMAM
XAAXXAAAAMAMXMSMXXMXAXXMASMXSMMMMMSMMMAMAMMSMMSXMXMAXXAAAAAMAAMSSMASMMMSXAAMXMMMMXSAAAASMMSSSMMSAMMMSAMXSSMASAMSMXMMMSAMSMSMXMASMMASAAXAAMAX
SSMSMMMMMMAMXAMMAMMMSAMSXMMXAAXMAAAASXMXMSAAMSMASXSMXMASXSSSMMXAAMXSASXXMMXMAMAAXMAMMMMMAXMMXAXMAMSASAMAMXMSMXMAAXAXAMAMXMAAAMXSAMASXMMSMSSM
XXMAXASXXSASMSSMMMSAAMAXMASXSMMSMXMSMASAAMMMXAXXMXAAAAMMAMAXXMMSSXAMMAXMAXSSMSSXSAAXXXXSXMMASMMAMXMASMMMSAMXAASAMXMMMMMMXSXSMSXSMMXSAXMXMAMA
MMSAMMSAAXAMAMXAAAMASXSXMXMMXMASAXSAXAMMSMAMXASMMSXSMSAMAMMMMMMXMMMMXMAXXSAMXMAAAMASMMXMMMMXMSXXXMMMMXAAXMSMSMSAXXXXMASMMSXXXXXMXSMMMXSSMASX
XMAMXSMXMSSMMMSAMXSSMMMAMMXXAAMMAMSAMXSXAMASMMMAAMMMXMMSXSAMAAAAMASXMXSXMAMMASMAMXSAMXAAAXXAXAXMASAAXSMSMMAAAAMMMSMMSASAASAXASXMASAMXMMXMASA
XSAXXMASXMAAXMAMMAMMAASAMASXSSSMSMMXMASXMMASASMMAMXSAMXMASAMSSSMSAMAXAMAMXMSAXAXXMXMAXSSMSSMSMAMXMASMXXAAMMSMSMAAXXAMSSMMMMMXAAMASAMMASAMAMM
XSAMMMXMASXMMASAMSSSSMSASMSAAAAAMXSAMMMMXMMXAMASAMXMAMAMMMSMXAXXMASMMXMAMAMMMSSMMMMMSMMXMXMMAXMASMXMMMMSSMXXAXMMMSMXMMMXMAAAAMSMASAMASAXSAAX
ASMXMASXXMMMSAMMMAAAXASAMXMAMXMXMASXXAAXMSSMAMXXSXXMASAXXAMMMMMAMAAXSMSMMMSAXXXSXAAAAXXXMASMMMSMMSAAAXXAAMSMAXAXAAMSMXMASMSSMXAMXSAMMXAMMMSM
MXMASMSAMXMAMXSXMMSMXMSMSAMMXMXAMXMASXSSMAXSAMSAMXMSASMSMXSAXMAXXMMAMAAMAAMASXAXSSMMSMXXMASAXAMAASMSMSMXSXAMMMMMMSAAMASMSMAAMXMMASAMXMXSXAAA
XMSMSAMXMAMASMMMAMXXAAMASAMXAAXAAXMXMMAAMAMSXSMXMAXMASXAAXSXSASMSMSAMSMSMXSAMMXMAXMXAMAAMXSASMSMMSAAXAAAMMXAAMXMAXMMSASMXMSSMMXMMMAMMMAMMMMS
MXAAMXMAXXSASAAMMMXSMSXAMAMSASMMMXMMASXSMSXXMSAMXMSMXMXMSMSAMAMAAAXAXAMXMXMAXMMXSAMSAMAXXAMAMMAAMMXMSMSMSASMMSXMASAXMXSXXXAAAMXSMSSMAMASXAMX
ASMMXMXSMAAMSMMXAMAXAXMMMSMMAMMASAASAMAXAMXSAMXSAAXXAMAXMAMXMAMSMSMXXSAMXAXMASAAXAMSAMXAMXMAMSSMMAMXMXMAAMSAAXAMASAMMXSAMMXSMMAXMAAMXXAMMMSA
MXSMAMAXMSMMXMASXSXMAXAAAMAMSXSASXXMAMAMXMASXSASMSMSSSSSMMMAXXMAAAAMSMSMSMXSASMMMSMXMMSXMMMAXMAMSMSASAMXMXSMMSXMAXMMXASMAMXMMMMSMSSMSMSXSAAX
XAXMMMSSXAXXAMASAAMMSSSMXXAMAAMMSMXSAMAMAXXMAMAMSAMXAAASXASMSMXMSMSMAAAXAAAMAXAAAXXMSAMAMAMMSSSMAAXXSMSXXXXXMXMMMSMSMASXSSMMASMXXAMMAAMAMASX
SSSMXAMXXSSSMMAMMMMAMAAXXMSMMXMAXMAXMSMSMSSMAMMMSMSMMMMMSXSAAAMXMAXMMMMMXMSMSSSMSSMXMASMSMSAAAXMMSMASAMXXMASMMMAAAMMAXMAAMXMAMXXMASXMSMMMAMA
AAAXMSSMXXAMAAAXXXMXSSXMAMMAXSMSSMSSXAAAAAMMXMXMXMXMXXXAMSSMMSAMXAMXSMSMSMXAAAAAXXAXSAMMAASMMSMXAAMMSAMXAMXAAASXSMSXXAMMMAASXMSXSASXMAMXMAMX
MMMMAMAMSMAMSSSSSMMAMMAXMASMMSAAAAAXXMSMMSSXSAMSAMASXSMAXAXAMXMXXSXMASAAXSMMMXMMMASXMAXMMAMAAMAMXXXASXMSSSMSSMSXMXMASXXSMMMSAASASASMSASASMSM
XAXXMXAMAAMMAXMAXAMAMSXMSMSMAMMMMMMMAMXASAAAMXMMAMAXAMXSMMSSMSMXXMASMMMXMAMMXMSASXMMXAXSSSSMMSXMXSMMXAXAAAMMXMSASAXXAASXXXAMMMMAMAMXXAMXXMAA
XASASXSSXSMMXSMSXSMSXMAXXXMMSMSXMSXMSSMMSMMMMASMMSSMAMAXAAXAAAAXSSMMXSAAXAMSAMSAAXXXXXXXAAXMXXASASASXMMMSMMSAMMAMXMMSMMMMMMAMXMAMXMXMSMSSSMM
ASMMXAASAMXXAXAMAXXMASXSXSMAMXXASAXMAAAASXMASMXAAAXMAMASMMSMSMXMAAAXMMSXMAXXAXMAMMASMSMMMMMSSMMMAXAMAXXXAMXSAMMSMSXAASAXXASAMMMMMASMMAASASMX
MXAMMSMMXMAXAMXMAMXMAMAXAXMASXSMMMMMSSMMSASASXSMMMSSMSXSAMXAXMAXSMMMAMAMXMMMSMXAXAMAAAMAMXXXAASMSMSMMMSAMXMSXMXXAMMSMSSXXASXXAAASXXASMSMAMXX
XXAMXAAXXASMSMXMMSASXMMMSMSASAAXMXAAXAAMXXMXSAXXAAMAMMAMAMMSMSXMAAAXSMMMXSAAXMSSXSSMSMSSMMSMSMMAAAXSAMXMASXMSMAMAMAMXXAXMXMXSSSXMMXXMMMMXMMS
MSMMSMSAAAXAXMXSAMXSAAXAAMXMMXMXXXMSSMMMMSAAMXMSXXSAMSASAMXMAAMSSMMSMASAAMMMSAXMAMXAAASAAAXAXXMMMSMXXMAMXSMMAMMSAMXSSMMXMAAAMXMAXSSMAXMXMASA
XASMAMXMMAMAMSXMMSMSMMMSSSMSAAMMSXXXAXAASMSSMSMXMXMXMSASMSAMMMMXAMXXXAMMXSXXMXMMMMMMMSSXMSSMMMMAXXMAMMSMXXASMSAMXSMAXASXSMSMSAMAMAASXMSMSMAM
XAASMSAXMAAMXSMSXXXMAMXMAAAMMSAAAMMSMMXSAAAMMAXAAMXSXMXMAMXSAMXMXMSAMXSSMMMMAMXAMAXASAXAXAXXAXXSSMSSXAAMMSMMMMAMAMMASMMAXAAXSXXAXMXSXAAAXAMX
MSMXXSAXSAMMAXAAXMMSAMXMXMMMXMMMMXAAXAXXMMMSSMSMXSAMXMAMSMMSASMMAMXMMMAXAAMMMASMMMMXXASXMASMMSMMAAAMMSMSAAMAASAMASMXSMMMMSMMMMSMSXAXMSMXMSXM
XMXSMMSAMSAMXMSMXSAAXSAMSSXAAXAAAMSMMXSXMAAXAXAAAMAMAXAXAAMSXMMMASASAMXXXXXASMSAASXSMXMASAXXAAAMMSMSAXXXMMXSXXMSXXMAMXAXAXXAXAASAMXMMXSXXMAX
SMAMAAMXAXXMAAAXMMAXMXAXAXAMMSMMXMAASASASMSSSMMMMSSSMSSSSSMMAMXXASASASMSSXSMSAMMAMAAMXSAMXSMSSSMXXXMASMSASAMXSMMXMXAXSXSSSSSSSMSXSMMSAMXMMAM
AMAMMXSMXMASXSSSMSAMSSSMMMMAMAMAMXSAMASAMXMXXAAMMAMXAAAXMXASXMXXMMXMXMAMAASXMXMASMSMMXMXSAXXXAAXXXAMXMAAMMAMAAAAAXMAMMXMAXAXMAMXMXAAMAXSAMXA
MSMSAAXAAMMMMXAAAMXXXAAMXXMASXXXSMMMMMMAMXSAMXSSMSSSMMSMMSXMXAXSSSSSMMMMMMMAXAXMMMAAMXMMMMMSMSMMSSMMSMMMSMSMSMMSXSAMAMXMXMXMSAMAAMMMSAMXSASX
XAXAMMSSMMSAMXMMMMSSMXMMAMMASMSSMMMSAXSXMAMAMAXXMXMASXAAMSAMMSMAAMAAMAASXSSSMSAMXXSXMAMMAAAXXAAAAMAAAAAXMAMAAXXAMXMXMAXMAMAAMASASXAAMXSAXMAX
SXSMSAAAMASASXAXXMXAAXSMMSAMXAXAAAXMAMSMMMSAMSSSXSXMASMSMSAMAXMSMMSMMAXSAAAXSMXMAMAASXMASMSSSSMMMSAMXSMSMAMSSSMXSAAASMMXASXSSXMAXXMSSXSAMXMX
SMXAMMSSMMSAMXMMSASMMMXAASXSSSSMMMSMSAMXXXAMXAAXASMXAMMMXMAMASMMMAXMAMXMAMSMSMMSSSMMSAMAMAMXAXXSXMASMMMMMXXMAMAMSXSXMAXSASAAXAXASMXAXXMXMMXX
SAMSMXMAMAMMMSXAXXXAASMMMSAAAXXAAAXMMAAXMASXMSMMXMAMXMAXMMMMMMAAMXMXSAMXSXMAMMAAAAAXSMMMXAMMMMASASAMAASXMXXMMMMXMMMXMAMMXMMMMSMAXMAMAMAAMXSM
XAMAAAXMMXXMASMSMSMSMSAXAMXMMSSSMSSXSXMXAAMXAAMAXMXMMSXMAASAMXSMSSMMMAMAMAMMMMMSSSMMXAAMXMMAMSASAMASXMMAMMMMXXXAAAAAMAMMAMXXAXMMMSAMAAXMXAAA
AXSXXMSSSSXMXSXMASAAASAMXSAXXXAAAXAXXASMSMSMSMSXXMASXAAMSMSASMXAAXAAXAMMSAMAAXAMXAMMSMMSAMMXMMAXMSAMXSSMMAXMAMSSSMSXSAXSASAMXSASASASXSAAASXM
MXMASMXAAMMMAMAMAMXMMMXMAXMXMMXMSSMASMMAAAMMMMAMMMMSMMXMAAXAMXMMMSMMSSMXSXSSXSASXMMAMXASASXMXMAMASASAMAASMMMMMAMXXAASXMSASXSXSXMASXMAMMMMXAS
XXSAAAMMMMAMAXAMMSMSMSAMSXSASAMXMAAAXAMXMMMAAXAXAASXMAAXMSMSMAAXAMAMXXXXMXMAMMMAMXMAMSMSXMASAMAAMSAMMSSMMSAASMMMSMMMMXXMXMAMAMXMAMMMXMSASMMM
XXMASXSSXSXSSSSSXXAAAMXSAASASXAASXMMSSMASASXMMXSMMMAMSMXAAXAXMMMMSSMMMSAMXAMXSSMAMSSMSAMMSMSASMSXMXMAXASAXMAMASAMASAMXSMMMXMMMAMMMSXMAMASMXA
SMSMMAMXAAMAAAAXAMSMMMMMMXMAMMSXSXSAXXMASASAMSXMMSXXMASMSMSMMSAMMAXAAASAMXAMAXAXSMAMMMMMAAXXAMXXMASMMSAMMSXMMAMXSASMSAAXXASXSSXSAASAMAMSMMSS
AMAAMXMMXMSXMMAMMMXASXMASAMAMMMMXXMASMSMMMMXMAASASMSMAMMAAXXASASXASXMMSAASAMXSSMMMMXAAAMMSSMSMMMXSAAXMMMXMXSMXSMMMSAMXSMMASAAXASMSMXMAMXAAAM
MSSSMAMSMXSXSMXMAAXSAASMSAMAMASAMXSMSAAAASAMXSXMASAXMASMSMXMMSMMMMMMSMXAAMAXXAAASAMMSXSSMXMXXAMAXAXAMMSMSMAXXXXMAAMMMAXXMXMMMMMMMMMSSSSSMMSS
XXMAMASAMXMAMSAMXMSMMXSMSXSXSAMXAMXAMXSSMSXXAMXXAMXMASXAAAXSXMXAAAASAMXMASMMMSMMMASAASAAXAMXMAMSSMMSSMAAAMMSMMMSMMSXMSXXXAXAXAAXAMXAAAXAAAAM
XXMASMSASMMAMAXMAXAXXAMAMAAAMMMXSAMMMAXAXXXMSSSMASASXMMSMSXMASXXMXXXASXMXAAMAMXAXAMMSAXXMASASAMXAXAAAXMSMXAAAAXXXXMMXXASMSXMXMSSXXMMXMXXMMSS
MMAAAASAMXMMSMMMXXSMSMMAMXMMMXAAAXAXMSMMAAMXAAMXXMXMAXAXMAMSXMASMSMMMMAXSSSMAMMSMSSXAMSMSMAXSMXSMMMSSMXAXMMSSMSMAMXXMSMXAMXSAXXMMASAMMXXMAMA
AAMXSAMAMMMAAXAXMAMASXMAXAXAXMASMSSMXMAXXAAAMSMSMSASAMSSSMAXASAMXAAASMSMMMXMASAAAAAAMXAASXMASAMAXXAAAMXMSSMAXAAXSAMXMAAMSMAMAMMASMMASAASMSSS
SXSAMXSXMAMSMMASXXMAMAMSSSSMSXMXAAAXASXMAMXXSAAAAMAMAMAAMSAMXMASXMSMSAAMAAAXXMXMSAMMXMMSMAXAXMAAMMMSASXMAMMMMSMMMSAAMASMAMMMAXMAMASAMXMSAXAX
MSMXSMMMMSAMXMMAMXMMMSAAAAXMAMAMMMMMMSSSXSAXXMSMSMXMAMMSMXMAMSSMMAMAMXMMASMSAMXXAASXXMSAXXMMXXMSMXMMAMXMASXMAXASAAMMXMMXXSSMMSMASAMASMMMXMAM
XXXMXMAAAXXMASMAXAXXAMMMSMMMAXMASAMMXSAXAMASMXXAXMSSSSXXMASXXXAAXSMAMAXXMMMXMXMASAMMAMMAXSASAMXAAAXMMMXMASAMMSAMXSXSAAXAMAAAAMXAMXXMMXAAMMXM
SMXMAMMXSXAMAXSSSMXMXSMXAXAMXXXMXMMMMMAMAMAMAAMAMAAAXXXXSAMXMMMMMMSASASXXAMXMAXXMAAMSMMSMMASASMMSMSMAMSMMXMMMMMXXXASXSSMMSSMMMMXSXSXXSAMSSXM
ASXSAMMMXMMMAXXMAMAMMMMMMMSMSXSSXMASASMMSAXSXMXAMMMMMSMXMASXSASAAASXSAMXSMSASXSXXSMXMAAAAMAMMMMAAXAXASAMXSAMXAMMSMAMAAAAAXXXSXSMSASAAXXSXMAS
MSMMMXXAXAXAXSASAMMXAAMAXAMASAMXXSASASAXXAAMAMSASXSAAAAAXXAASASXSMMAMAMXAXSMMAAMXMMASMMSXMSMMAMMMSASMSMXMASMMAXAXMAMSSSMMSAASXXAMAMMMMMAASMM
XXAXAAMXMMMMMSAMXSXSSSSSMAMAMXMAAMAMMMMSAMXMAMXAMASMSMSSSXMXMAMXMAMMSSMMMMSXMSMMAXSASAMMMAMXMAXAMMMSXMXMAXMXSAMSMSXMAAXMSMMMXAMMMXMAAAMMAMXS
MMSMMXSAAXAXXMMMMAMAAAAMMXMAMXMASMMMAXXAXMXMAXMAMXMXXXAAMXMAMXMASAMXAXAASAMXXMASXXMASXAAMAMASXSXSAAMMMAMMAMAAAXAASXSMMMMAXASXXMAAASXSXSXMMAS
SAAXMAMXSMMMSAMMAAMMMMMMAXXAMSAMXMASMSXASAXAAXSXMSMASAMAMAMASAMMSAMXASXMMMSMXMMMMMMMMMSXMAXXMXAASMMSASAXSAMXMSAMXMASXXMSMSSXMASXMXMXMASXSMAS
MSSSMMSMXMMMXAMXSXSAMXXMAMMAMXAXXXXAXMMMXMSMMMMAMAMMSASAXSSMMASXXXMMAAMSXMSXSAAAMSSMMMMASAMSAMMMMAMXXMAMSMMSXXMXSMMMAXMAMXXAXXMASMSMSASASMMM
MAMXXMAMMSXSSMMXMXSMSMMMAMSSMSSMMMSSXMASMXSASASASASMSAMAXXAMXSAMXAXSMSXSAASAMSSMSAAAAAMXMAMSAMSASAMSSSXXXAAXMASAMAXSSMSMSMSMMMMMAAAMAAMXXAAA
SAMMMSAMXAAXMSMAMAMXSAASAMXXAXAAAXAXASXSAMXASASASASAMAMXMSAMMAAXSMMXAXASMMMSMAMXMMSMMSSXSMMMAMXASXSAMXMASMMSMSMMXSMMXAXAAAAMAMAXMMASMXMSMMMS
AAAAMMMMMMSMAAMAXXSAMMMMAXXMMMSXMXAXMMMXMAMXMMMMMMMAMXMXXMMMSSMMXSAMXMXMXSAAMAMSXMXXAXXXXMSXSMMXMMMMXAAAXAMSMXASAXAXMAMSMSASASXSSMASXSSMXSMM
XSMXSASAMXAMMMSXSAMXSAMSSMMAMAMASMSMSASXMSXSAAXAAXXXSXMSXAAAMAMMXMSSXMASAMXXMAXAMXMSMXMMMMMAMAMAXAASMSMXSMXMASAMASMMAXMXAMMMASAAXMAMMSAAASXS
XXMMSASXMSASXAAMMMMAXXXAAXASMASXMXAASAMAAMASXMSMSXSMXAAAXSMXXAMSAAAMASAMASASMSSMAAAAMAAAAAMXMASMSSMXAXMAMXAMMMSMMMMXSSMMXSSMAMMMMMMSXSMMMSAM
XSAMMAMAXSAMXMMSAMMMSMMSSMSMMMSMSXMMMSMMMMAMAMSMMAAASMMMXMXSSSMSMSSSMMASAMASAMXXXMSASXSSSMSAXXMAMAAMSMSMSXXXXAMXSAXAMAASAMXMMXAXAAASXMASAMMM
MSAMXAMXMMAMMMASXSASAAMXXMXSAMXASAXXAAXXXMASAMXASAMXAMXXAXAMAXAMAAXXXSAMMMXMMMMSXAMXXAMAMASMSMMASMMMMAAAMMSMMXMASXMSXSXMASAMSSSSSSXSASAMAMXS
ASAMSSSXSSSMAMXXXSMSMSSMSMMSXSMXXSMMSSXMMSMSMXMXMAXXMXSSMMSMAMMMMMSSMMMMXSAAMMAAMSMSMSMAMXMAAAXASAMXMMMXMAAAASMMSXAXAMMSXMASAAAMAXASAMXSMMAM
MXAMXXMAAAAMXXASAMXSMAMASAMMXMXMSMMMAMXAXXAXXSAMXXMMMMMAMAMXSSMASXAXXAMXASXSMSAMAMAMAXMAMSSSXSAASXXXAXMAMSSSMSAAMMMMAMASAMXMMSMSSMMMSMMXMMXX
MSSMMMMMMMMSMMSAMMASMASMSAMXAXAXAAXMASXSMMMMMAAASXSXSASAMXSMXAMASMSMSASMMMAXAXXXAMAMMMSMSAAMAMMXMASMMASXXAAAASXMMAAMSMMSAMAMAMXAXXXAAXXXMMMS
XAMAMAAAAXSAAXMASMXMXASXSMMSMSMSMMMMXMAMXAAMMMMSXASAMXSASMSAMMMXMXXAMAXMSMAMSMMSMSXSAAAXMMMMAMXASXMASAXMMSSMXMASXSXSAAXSAMXMASMMXAMSSSMXMAAA
MXSASXSXXXSMMMSAMMAMMXSAXMAAMAAAAASXMMAMSSMMAASAMXMAMAXXXMMXASAMMSMXMXMXAMMSMAXMASASMMSSMXXMAXSASASMSAMXXAMXAMMMAMMXXXMSAMSSMSAXAXAMMAAASMSM
AASXSMXXSXMSSMMSMMAXSXSASMSSSMXMSMSAMSSXMAAMXMMAMMSAMMSMXSMSMMASXAAXMAMMMSMAXAMXAMAMAAXMAXMMAMMAMAMXMMXMSASMXMAMAMMSMSAMXMASMSMSXSSSMMSMSAMX
MMSASASMSAAXXXAAXMAMSAMAMXAXMAMXMASAMAMASMMMSASAMASXXMAMAMXAAXAMMMXASMXXAAXAMXSXSMSXMMMSASXSSMMSMMMSXMAMMAMASMSSMMAAASXSSSMSXXXSAMAMMAXMXAXM
SXMXMAMASMMMXMASAXXAMAMMMMMSMMAAMASMMAMXMAAMAXXASXMAMSXMASXSSMMSAXMMAXAASMMMSAXXXAMSMSMAAXAAASAXASAXASXSMSMAMXMAMMMMMXAMAAXMMSMMMMAMMMMMMAMA
MASXMMMAMAAAXAMMASMMSXMXAASAAMSXMASAMASMSSMSMSSMMMSAMMASAXXAAAXXXMXMAMXMAAAAMMSAMXMAXAMMMMMSMMXSAMXXMMMMAAMXSXSAMXSMSMXMMMMXAAAAMMXSAAAMXMAS
SAMAAXMAXMMSAMAAAXAXAMSSMSSMSMXAMXMXMAXXAMAMXMAAMASAXSAMMSSSMMMSSMSMSAXXSMMSAMXXXAXMSMSAAXXMAXAMMXSSMAXMMMSAMAMMSXMAAAASMMXMSSSMSSMSXSSMASMX
MASMMMSSMXAAMXAMMSMMMSAAXAMMXASXMAMMASMMXMAMASMMMASAMMMXXAAAMXMXAAAAMSMXMXAAXSAMXXXXAXXXSSXSAMXSMAMASMSSMXSAMSMAMAMSMSASXAMMMMAAAXAXXXAXAXMX
SAMMXXMAASAMXMXAMAMAXMXXMMSMMXMXMMSMAMSAAXASAMMSMXXAXASXMSXXMSASMMMSMXXAMMSMSMXSAXSSMSXAXAMXMSMAMSSXMMAASMSXMAMASMMXAMAMXSAAAMMMMXSMMSSMMSSS
MASMXSMXMMAAMSSXSASMMXSMMMAAMXXAMAMMSMMXMXAMMSMAXSSMMMXAAMASAMAMMAMXXXMMXAXXMMAMMXXAASAMMAMAMAMXXAMASMSMMMMMMSSMXXSMMMSMAMSSSSMXSMXAXAAAAAMA
SMMMAMAMXSXMSMMASASMMAMAASMSMMSMSAMXMSMASMSMAAXXSMAXSMSMMMMXAAAMMSMSXMASAMSAAMAMXMMMMMAXSAMASMSMMASXMAXAMXSAAMAMXSMSMAAMAMMXMAMAMXSMMSMMMSSM
XAAMXMAAMMMMAAMAMXMAMMSSMSAMAAAXSXMSMMMAMAAMSSSMMXXMAMSXMAXSAMXSXXASASAMXXSMMSMMSAMSXSSMSXSASXAXSXMASASAMMMMXXAMSAAAMSXSMMSASXMMXAMXAMMSAMAX
SSMSMSMXAAAMSSMMMAMAMXAXSMAMMMSXMAMMASMSSSXXXMXAXXXSXXMASMMMSMASXMMMAMXXSAMXXAMAAMAMAMAAXMMXMMAXAMSAMAXAMAXMSSSMMMSMXMASAASASXAASXMMMSMMASMX
XAAXAAXXSSXMAAASXXMASMMXAMAMXMMAXMMMXMAMAMXXMASMMXMXMMXXMSAXAMASASXMXMXAMXSXSASMMXXMAMMSMXSAMXMSMMMSMMSMSMSAXXMAXXAXAMAMMMMAMMMMXXAAMAXSAMXM
XMAMSMMMMAXMXSMMASMMMXXXSMMSXSSSMSSSSMXMAMXSAMXAXAXAMXXMASXSXSXSAMXMAMMMAASXMAMMXMAXASXMAXMASAXAAXAXXXAMAAMMMMSXMXMAMMXMXXSAMXSAMSSMSMMMXXAX
MXMAAASMMMMXAXAMAMAAMMSMMASMMAAAASAXXAASAMAAAMSSXMSMXSAMAMMSAXMMXMXSAXAXMMSASAMMASMSMSAMMXMAMXSSSMMSXSASMMXAAAMXMMXMXMMSSMMAXMMAXMAASXASMSSS
ASXMMSMAAMXMASAMXMMAMAAAXXMAXMMMMMMMMSMSASMSSMAXAAAXXAAMXSAMAMAMSMMMSSSSMXSAMMXMASXAAXAMXSMXSAMXXAXSAXAMXXSSMSMAMAXXAMASAMSSMAXMMSMMMSASAAAA
SXMASAMSMMAXXXAMXASXMSMSMMSSMXXXAAXAXXXXAMXAXMASMMMSSSMMXMXXXAMXAAAAXMMAMAMAMAAMMMMSMSMMXMAAMAMXSSMMAMSMSAMXAXXMXAXSMSMSAMAMASXXAAAXXMMMMMMM
ASMMSAMXMSXSMMAMXMSAAXAXAXAMXAXSSSSSSXMMSMMMXMASAMAMMAASXMSMXSSSSSMXSMSSMXSAMMXXAAXXMAXMXSXMXAMXMAAMAMAAXXXMXMASMMMXAAASXMXMXXAMMXMMXMASXXXA
MXMASMMAMSAAAASMSSMMMMAMXMAXAMAMXAAXMAMAAAAXXMXSAMXMMSMMAMAXMXAAMXMAAMAAAMAMMXASMSMMSMSMAAAXSSMMXSMMMSMMMMXMXSMXASASXMMMSMASXXMAXAASAMXSXAXA
MMSASMSXMMMMXMXAXAXMXMXMASXMSSXMMMMMSSMSSMMMMMMXMXXSAXMSSSMSXMMMMAMSXMMSMMAMMMMAAAXAAAAMMMSAXAAMXXAXAMAMASXMASXMXMAMSAAAXMAMAAXAXSSMASAMXSMM
XAMXSMMXSAMXSXMXMMMSXMAMXMMXAAXSAMXMAAAXMAXXASMAMXMMASXXAAXMASXMMXXAASXXXSXSXXXMSMSSMSMSMAXMSSMMASAMXMXSAXAMAMMXSMSMSXMSSMMSSMMSXXAXAMAAAMAM
MSSSMXSASMSAAAAXMXXAAMMSSSMMMSXMASAMSMMMSAMASXMASASMMMMMMMXMMXAXMMMSSMASAMAXMAMXXXMMAAMXMMSMMMAMASXMMSMMASMMSSSMAAAAXASXAAAAAAAXXSAMXSXMXSAM
SAMAAAMAMXMMSXSASAMMSXAAAAAMXMMMAXXMAMXXMASXMASMSASAXASMSMASMSMMSAMXAMAMAMAMMXMAMAXMSMXSAMXXAXMMMXASXAAMAMXAAAAXMMMMMMMAXMMSSMMSMMAMXSAMXMMM
MASMMMMAMAXMAAMXSASAAXMMSSMMXXAMMSMSMSMXMASAMMAMMMMMSXSAASASMAMASASXSMXMASXSMAAASAMXMXASXXXSXMMASMMMSMSMMSSSMSMMXAXSASXSMMXMXAAAASAMXXAAASAS
SAMXMXSXSASMMSMASMMASXXAXAAMASAMXAMMXMXASAMXAAAMASAAMXMMMMSSMXMAXMAMXASMAXMAMXSMSXAASMMSMMXMAXSAXSAAAXAAXXAMAAAAXSXSASAAXXAAXMMSMMASMMMMASAS
SAMXMASAMXSASMMMSXSMMMSSSSMMAAXXSSMSASMMMMSMSSXSASMSMSXMXMASXSMMSAMXSASMSMSMMXMASXSASMXMASXSAXMXMAXSXMXSMMMMXMASMXAMAMMMMMXSASAXASXMAAMAMMMM
XXAAMMSAMASXMAAASMMAAXMAAMAMMXMMMMAMASXXAMSAAXMMMSMXAMAMXMASXSAAXAMAMAMAAXAASXMASXMAXMASXMAMXSXSSSMMXSAMMAXXXXAXAMXMXMMMSXAXAMXSMSAMSSSMMAXA
MSSSXXXAMXSMSMMXSAMSSSMMMMSMMAAAAMAMSMMSMSMMMSXMASXMMSAMAMMSXMAMXXMXMMMXMSMMMAMASXMMMMMMSMXMAXAMAXXAASASXSSMXMAXMXMXMAAASXMMXMASASMMXAXXXXMM
AAAMMMSAMXMMAMSAMMMAMXXSXMXASXSSXMAXXAAXSAMAMSAMXSAMXAAMAXMXMMAMXSXMSAMXMAMASXMASAAAAAAAMMSSSMAMMMMMMXAMAAXAMSMSMAXMASMMSAXAXMXMAMXSMSMSXSSS
SMMMAXSMMMSAMXMAXSMASMAMMSMMMAAAAASASMMSMMSAXMXMASAMXMMMXXSAXMAMXAAASAMXSXSAMAMAXXSXSSMSSMAAASXMXAAASMSMMMMXMAAAMXMAXXSASAMMSMAMAMSAAMAMAXAM
AXXMSMMAAXXMXMSMMASMXMAAMXAXMMMSMAXASAMXAASMMSXSASXMMMSSSMMASMXMMMSXSAMAMXMASXMMSMXAMXXAXMMSMMXXSMSASAAASXSASMSMMSASXAMXXXMAAXASASXMMMAMMMSM
MMSXMAMSASXMAXAAAMXXXMXSAXSMXXAMXMMSMXMSMMXAASMSASAMAXAAAMMMMASXSXXAMXSAMASMMXAXXAXAXXMASMXAAXAXSAAMMXMMMAXAXAAAXXAMMSMSSSSXSSMSXSAMXSXSAAXM
SMSASAMMAMXSASXSMXMMSMAAMSMMAMAMAXMXMAMMASXMMSAMAMAMSMMSMMAAMAMAXXMXMAMASASMAXXSSMSSMMXAMAMSXMXXMAMMMMSAMSMSMSMXMSMMAMAAAAAAXMASAXMAXAASMMSS
SASAMASMAMXMASMMMAMAAMAMXAXMAAMSSMMXSAXMAMMXMMMMXSMMXAAMXSXMMAMXMSMMMMSAMAMMSXMAAAAXAMMSSMAXASXXSSMSXASMSXAXMAMSMMMAXMMMSMMAMXAMSMMSSMAMAAMA
MAMAMXSXMMMMMMMASAMSSSSXSMSSMSAAXAXASXSMXSAAXMXMAXAXSMMMASAMSASMSAAAAMMXMAMMMAMMMMMXAMXAAXMSAMAAAAAXMMSXMMSMXMMMAAAAXSXAMXSXMAXMAMXAAXXMMMSM
MSMXAAMAMXAXMAXXXAXMMAMMMXMAAAMMSMMXSAMAMAMMSXSMMSMMASMMAMAMXMMMMXSMSMSASXSAMAMXXAXMSSSSSMAMAMXMSMMMMXSAMAMAXSASXMMMASMMSASAASXSASMSMMMXSAMX
SMMSSMXAMMAMMMMSSSMAMAMASAMXMMMAAASAMAMAMMXXMASXMAMXMAXSAMMMSMMMSMXXMASAMXMXMSMMSXSAAAXMAMXSSMMMXAAXSASAMXXXXMASAAXSAXAAAASXMAASAMXAAXAAXASM
AAXAAASMXSASASXAAXSXMAXAXASMSMMSSXMASXMASAMMMMMAXAMMXMMXASXAAAAMAASAMAMAXSMSXAAXAAXMMMMSXMAXAAXXSSMSMASAMXXMAMXSMSAMXSMMMMMASMXMAMXSMMMMSAMX
MSMMSMXAASASASMMMMAMSSSMSAMAMXXMAMXAMXSXMMMAAMMAMMSAMXXSXMMSSSMSMSMMMSMAMSAMXSMMMSMMXXMXAMXMSXMAMAMXMAMAAAASXMAMXXMXMMMMXMSXMMMMMMAMMSAAMAMX
XAAXMXMMMMAMXMAAXXXAAXAMAMMSMSMMAXMASXSMSSSSSSMAXSAXAAXMXSXAAMMSXXAMAXMXSMASAMXSXXASMSSSMMSMMMSMSAMMMMSXMSMSAMXSAMXAAAAAAXMAMXAAMMXSAMXSMSMA
SMSXSASAMXMMMSSMXSAMSSXMAMXAAAMMAMSMMASAAMMAXAMXXSAMMSSMASMMXMASXXAMXSXMXMAMAXAXMSAMAMAAMAAMMAXASMSAAAAXAAASAMASAASXSSMXAMSSMSSSXAAMXMXMAMAA
SXMAMXSAMAMAXAAMASXMAAASAXSMSMSMAMAAMAMMMSSSSSXSAMXMXAMMAMAXAMXSMSMMASAMASMSXMMSMMAMSMSMMSSSMMMXMAXXAMXMSMXMMMMSAMXAXAMSSMAMAMAMXMMSAMAMAMAX
XAMAMXMXSASXSSXMASAXMSMSAMSAAAXXSSSSMAXAAMMAAXAASXAXMASMASMSSSXMAAAMASMMAXXAXAAAAAXMAMAAXAAXAASAMXMMXAXXXSAMXAASMMMSMMMAAMAMAMAMXAAXMSMSXSMX
SXMASMAMSAMXMASMMSAMXAMXMASMMXMAMMXXXXSMSSMMMMXMASXSXSXMMSXXMAMMSXSAMXMMASMXMMSSSMSSXSSSMMSSSMSMSAXXMASXMASXSMMSXSXAASMMSSMSXSXSXMSMMAMXMAMS
",
    );

    let rows: Vec<&str> = input.lines().map(|line| line.trim()).collect();
    let matrix: Vec<Vec<char>> = rows
        .clone()
        .iter()
        .map(|row| row.chars().collect())
        .collect();

    let mut height = rows.len();
    let mut width = rows[0].len();
    let mut count = 0;

    let directions = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // diagonal down-right
        (-1, -1), // diagonal up-left
        (1, -1),  // diagonal down-left
        (-1, 1),  // diagonal up-right
    ];

    for x in 0..height {
        for y in 0..width {
            for (dx, dy) in &directions {
                let word = "XMAS";
                let mut is_found = true;

                for (k, c) in word.chars().enumerate() {
                    let new_row = x as i32 + dx * k as i32;
                    let new_col = y as i32 + dy * k as i32;
                    // Add bounds checking before accessing the matrix:
                    if new_row < 0
                        || new_row >= height as i32
                        || new_col < 0
                        || new_col >= width as i32
                    {
                        // Out of bounds - don't try to access the matrix
                        is_found = false;
                        break;
                    } else {
                        // Safe to access the matrix
                        if matrix[new_row as usize][new_col as usize] != c {
                            is_found = false;
                            break;
                        }
                    }
                }
                if is_found {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}

fn main() {
    word_search();
}
