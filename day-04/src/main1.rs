fn main() {
    let grid = [
      "XMSSMSSMXSXSAMMMAMMMASAMXSSMAMSSSMMXSMMSSSMSAXAAMMMSMSMSMAMMMMXMMMMXAMXXMMMSMMMAMXMMSASMSSSMSMMMXMSSMSMSXMMXXMASXMMMMMMMXMASXAMSXSXSXSMAMSXS",
      "SMAAXSAMMAAAMAASMSAXSXMXXXMMAXAAMAXASAASXMAMXAASXAXMMAXAXXXSXSAMASXMSSMMSASXSASAMXXXSASAASAXAXAMMMAAAAASMMMSSMAMSAAMAMAXAXAXAMXXAMAAASMSMSAA",
      "MMSSMSAMAMXMMSXSAMMXMAMMMMMMXMMMMXMASMMSAMXMSXMAMMSMSSSMXSAMXSASAXAMXAAAMSMASMSAXAMXMAMMMMSSMSMSAMSMMMMMAAAXAMAMMMMSMSMMXMMSMSMMAMAMXMAAAMXM",
      "XAAAAMAMXXAXAXMMXMMASAMAASAMMXSAMXMMMMAMXMMMXASMMMAMMMMAASAXASAMXSXMSSMMXAMXMXSMMSXSMAMAMAAXMAMAXXXSMAASMMSSMMMSSMMAASASMMXAMAAMSMSMXMSMSMXA",
      "MSSSSSMMMSMSMSSSSXSASASXMSAMAASXMMMSMMXSXMAMSXMAMSMXAAMMMSXMMXMAMAMMXXMXSASMSXMMSAAXMAXAMMSMSXSASMXAMSXSAAMAXXAAAMMMXMAXAXSSMSSMAAAXAMAXAMXS",
      "XAAXAAXAXAAAAMAAAAMAXXMAXSXMSAMASAAAXSASASXMSAMMMAAMSMSAMXXMXAMMXXAXAXXMAAXAXAASAMSMSSSMSAMAMAMMSMSSMXASMMMSMMMMSMSMSMSMXMAXXAMMMMMSSSSSXSAM",
      "SMSMSMSXSMSMXMMMMMMSMMMXASXXMAXMMMMSSMASAMXAMMXASMSMXASXSAMMSXSMASXMMSMSMMMSSMMMMXMASMAASAMMMASAMXAMSMMMMAAXAXXAMASAAAMXXMMSMMSAMAXAAAAAAMAS",
      "AAXAXMXXAAAAXMASAXAAAAXMSMXSMMSXMMMXAMXMXMMMMSMMMAMAMXMXMASMAAAMMSAAAAAAAAAAAAMXXAMXMMMMMAMXSXSASMSSSXSAXMASAMMXMAMSMMMSXAMSAMSASXXMMMMMMMMM",
      "MSMSMXXMSSSSMSASASXMSSXSAMXSAASXSAXMAMXMMMMXXAAAMSMXSAMXSAMXMXMXAXMMMSXXXMMXSMMASMSMXAASMMMMSASXMAMAMASAMXAXAXAMMXMMSXAMMAMMMMMMMMAXMMXAAAAM",
      "XXAXMXMXAAAXAXASMMAXMMXSMSMMMMMASASMMMMAAAXSSMSXMXXMSMSMMMXMXASMSXMMXXASXSSMXAMAMXAAXMSSXAAAMMMAMSMAMXMMMSMSMMXMASXXXMASMMSXMASMASMMSXSSSSSS",
      "MMSMMAMSMMMMMSAMXSXMSAMXXAXMASXMMAMMASXSMXXXAAXMSMXAMXMAMXASXMSAMMSMXMXXAAXAMAMASMMSSXAMMSMXSAXAMAXXXAAAXAAAAXXMAXAXXMASAXSASAMSAMXAMAXAAAAX",
      "MAXAMAXAAXSAMSAMXMAMXASMSMMSASMXMXSSMSAXMMSMMMMAAAMSMAXSMSAMAXMMMASAAAMMMMMMSXMMXAAAXMAMMMAAMASMMSSMSSXMSSSSSMMMSSSMSAAMMXMAMAXMMMMAMMMMMMMM",
      "MMSAMMXMXMMAMSMSXSSMSMMXAMAMXSAXMMMMAMXMXAAASMMSMSAAMAMAAMSSSMMAXAXXMSXAAXSXAMSSSMMMMSAMAMMMSMXXAMXMAMXMMXAXAAAXMAMAXMXXMAMAMXMXMAXAMXAXXAXX",
      "SXAXMXASXSMSMSAAMMAASMSSMSXSASXSXAAMMMAMMXMXMAAXMAXMSMSMSMAXMASMMMSMMXXSMSMAMSAXXMXMASASMSMAAMAMXMAMASMMMMSMSSMMMAMXMSMMXASASAMSSMSASXSSSMSM",
      "XAMASXXXAAMXAMXMMSMMMXMAAAAMASAMXMMSMSASASMSSMMMMAMXAAAXXMMXSAMAXAAMAMXMMAMAMMXSSSSMXSAMXAXSASMAASXMASMAXXAXMAMAMXXAMAAMSAMAMAXMAMSAMMAAXMAX",
      "SAMASXMMMMMMAMAXXAAAMMSXMMXMXMAMMSXMASAAASAASAMXMASMMSMMMASAMASXMMSSMXAXAAMASXAMAAASAMXMSMMXAMXSXXAMAXMXMSMSMASMSMXAXSMMAAMSSSXMAMXXSAMSMSXS",
      "MAMASXXAAMXSAMXMMMSXSAMAMXMAXSAMAMXMAMXMXMMMSXMASXSMMAXAMAMASMMXAXAAAMXMSASXSMMSMSMMMSAMAXXMXMAMASMMMSXMAMXAMMSAAXSMMMAMMXMAAMMSXMSXSAMXAXAA",
      "SXMASMXSXSASXMSMSMMASMSXMAASXSMSSSXMMSSXSSXXSXSXXXMASMXSMSSXMMMSSMSSMSAAMAMAMXAXMAXAMMMXXSAMAMXMMAMAAXMSMSMXXXXMAXXMASAMXMMMSMAXAMMXSXMMSMAM",
      "AAMXMAXXAMMSAMXMAAMAMMSMXXXMMMMAASAMXAXMASXXMASAMSMMXXXMAXAMXAAMAXMXAMMMMAMXMMMSSMSSSXSXAMMMAMAXXASMMMMSAAMSAMXMXXAMASMMMASXMMSXXMAMMMMAXMXM",
      "AAMAMSXSAMXSAMASXSMXSASXSMMSAAMMSMXAMASAAMMMMSMSMAAXSMSMMMMSSMSSXMMMSMSASXSAXAMXAAAASAMMMMASMSASXMSASXAMMMAXAAXMXSMMAMMAMXSASAXMXSASASMASMAS",
      "XSSMXAXMASAMASMSAXASMMSASMASMMSXAXXSMSXMMAAAXAAXSSMMSAAXMASAMXXMASAAAMAMSASAMSXMMMMSMXMAMMMSXAAXMASAMMSMASXSMMXSAMMAASMMXMXAMASMASXSMSAAMMAS",
      "XMAXSMMSAMASAMXMMMSMSXMAMMASAXXXMSMAMXAASMSSSMSMAMMAXSMSSXSASMMMASMSSSMXMXMSMMASAMXAMXSSSSXXMASMMMMSMAMSXXAMXAAMASASXMXSAMSMMMMMASXMASMMXMMS",
      "AMAMXAAAMSMMMAXASXMASXMAMMASXMSXXAXMSMMMXXMMAMAMAMMMMMXXMMSAMAAMMSXMAMXAXMAXXSAMXMSXXAMMAXAXAAAAXMXAMSMMMMSMMMSXMMAXASASAMAAAXSMMSMMAMXAXSMM",
      "MMASMMMSMAMAXXSASAMAMMXMSMMSMAXXSMSXAASMMMMSAMMMMMXSAMSXMAXASXMSXMASMMMSAMAXMMMSAMAAMSMMAMXMMXSSMSMMXMAXSAMAXAXAAMMMMMASXMMSMXAAAMMSASMSXSAM",
      "XSMXAAAMXXXXAAMXMXMMMASMXMAMAMMMAMMXSMMAAAAMXSXSAAAMAMSMMSSMMMMSXSAMXAAXXMXSXAASASMSMXAMXSXASAMAMXAXAMXMMASXMASAMXAAXMAMXXXAMXSMMSASXSAMASMM",
      "MMSMSSSSXSAMSMMXSXXAXXAMAMAMXAXXAXXAXASXMMSSXMAMMMSXSMXASMMAAMASMMMSSMSSMMSMMMMSMMXAAXAMASMMMASMMSMMASMXSAMASXSMMSSMSMMMAMSMSAMAXMASMMMMAMAM",
      "SAMXAMXMAMXMMASASASMSMMSSSMSSMXSSSSMSAMXMAMAMMMMSAXAMXSMSASXMMASAXMAMXAMAXXAAMXMAXMMSMXMASAASAMMAMSAMXXMMMSAMMSAAXXXSAMSMAAAMASMMMAMXAXMAMSM",
      "MAMMXSAMXMAXSAMASXMAMXXAXAXXAMXSAAAAMMSXMSMMXSXASAMXMXXAXAAMAMXSMMMASMMMSMMSMSMMMASMMXSMXSXMMSSMAMXMMSASAMMXSASMMSSXMAMAXSMSMAMAXMASMSSXSAMX",
      "SSMXMXASMSAXMASAMAAAMMMSSMMSASMMMSMMSAMMMMAMAXMAMAMXMSMMMMMSAMXSXMMASASAXAAMXXAXMAMAAAXSASMMXAMXAXMSASMMMXMMMASAMAMMSAMXXXMAXXSAMSAMAXMAMMMS",
      "XAAMSXSMMMMMSAMASXMASMAXXAAXMXAAAMXXMAXSASAMXSMSMMMASAMXAXXXASAXAMSXSAMXMSSSMXSXMASMMMSMXSASMSSSSSXMASXAAAMXSASAMMSASASMMSSMMMMMAMAMSMMAMSSS",
      "SMMMMMMAXMSMMASAMXSASMMMSMSSMSSMMSMSSMMMAMXXAMAMAASXSXSSSSMSMMMSMMAAMXMMXAAAAAXAXAMXAXXXASAMAXMAAAXMAMXMXSAAMASAMXMXSAMAAMMMMXAMAMAMMAXXXMAM",
      "AAAXAASMMMAMXXMASXMXMASXXAAXMAXXXMAMXXMMSMSMAMAMSAMAXXXAAXAAMAXXASXSMAMMMMSSMSSSMSSSXMAXMMSMMMMMMMXMAXXMAAMXMAMMXAMMMMSMMSMASMMSASXSMSSMSMSM",
      "SSMSSXMAASASMMMXMXMAMMMMMMMMMMMSASMSSSXAAAXAAMXXMAMXMMMMMMSMSSSSXMAAXXXAXXAXMXAASAAMMAXSXAMXXAAXSMSXSMSMSMXAAAAXSMSAMAAXSMMASAAMASAAAXSAAAAM",
      "MAAXMMSSMSASMASMSMMMSASAMXSMAXSAMXAAAAMSSSXSXSXXMSMASXSASXAAXAXMMMMMMSXSMMMSAMSMMMSMXMSAMXSMXSSSXASAXAXAAXMSMXSASMMAMSMSAMMMSMMMSMMMMMMSMSMS",
      "SMMMXMAMAMAMMASXAAXXAAMXXASMAMMAXMMMMMXXXAMMMMMMAAMXMAMASXMSMMMAAAMAMMAXXMAXMXMAXXXXXMMMXMSAXMMMMAMAMSMSMSAMAAMASMSXMAMXMMXAXAMXAAXMASAMXAAX",
      "SAMXXMASAMSSMASXSMMMMXMXMSSMMSSSMMSAMXMMMMAAAAAMXSMSMAMSMMXXAXSMSXSASMAMAMSSXSMSMMMMMSAMXAMMMMAMSMSMMAMAAAAMMMMMMAAXSAMMXMMMMAMXMMMAAMAXSAXX",
      "SAMMXSAMXSXAMASAMXSAXSAMXXXMXAAXMASMXMAAAXSSSSXSXXAXMAMXAXXXMMSAMAXAXMASAMAAASAAAAXAAMASMMMXMXAMAAAMMMASAMXMAAAXMMMMMAMXAXAMMXMXMAMMSSMMSASM",
      "SAMXAMAMSMXSMXMAMAMSXMSAXSAMMMMMSMMMSXSSMMAAXMASAMXMASMSMMSMMAMXMAMSMSASASMMAMXMXMXMMSAXAXMXMASMMSMMAXMXXSXSSSSSXMXMSAMSMMMSAMSMSASAXAMAMMAM",
      "SAMMXSAMAAXXSSMMMAMAXMMSMMASASXAXAAXSAMAMXMAMMXMMMMAXXMAAASMMASAMXMXAMASMMXXASMMSASMXMMSMMMMXMMAXMMSSSXMASAMAAAMXMAMXSXAAAAMAMAAXMAMMAMXXSAM",
      "SAMXAXASMSMMAXASXSXSASAXXSXMAAAXSMMMMAMXMMSMMMMMXXXSXSMSMMXAMAXXMSMMXMAMAAXMAMAAXAAXAMXAMAXAAXSXMSAAMMXAAMAMMMMMMMXSAMMSSMXSAMXMMSMSSSMSMSAS",
      "SASMMSMMMAAMMSMMAAAMAAASAMXMXMSMMXSASXMASAAMAAAAAMAAAXXMASMXMMSMXAXMASXMXMMSSMMMMMMSSMSXSXSSMXSAMMMSSXSSSSSMASAAMSAMXSAMMAXAMXSMAXXAAXAAASAM",
      "SXMAAXAASMXMXAMMMMMMXMXMXMAMAMXASASXSASAMSSSSSSXMASMSMASAMMMSMAMSASAMXAMSXXAXMAXXAAXAAXAXMMAMASAMAAMMAXAAAXSASMMXMASAAAMMSMXXASMMMMMSMXMAMAM",
      "SMSMMXMMMAASMMSSMMMXAMMXMSXSSSSXMMSMXXMAMAMAMXMMMXMAXMMMXSAAAMAMSMMMXSAMMAMMSAMXSMSSMMMSMASMMAMASMSSMMMMMMMMXXAMSMMMMSSXAMASMASASAMSMMXXSSXM",
      "XASASMSSMSMMASAMAAAASMMAAMAMAAAXMXSMMSXSMSSXMASASAMXMMXMASMSSSMMSAMAASAAXXMXAAMAMAXAXAMAXMAMMSMMMMAMMXAAAXMXSMSMSAAMAAXMXMAMMXMASMSAAMMAXMAS",
      "MMSAAAASMAMXMMAMMMMXMASAXSASMMMAMAMAXMAXAAMXSXSXXASXMSMMMMXAXAXAXAMMMMSMSAMXMMMASAMMMMSSSMSXAMXSMMASMSXMXXMASXXASMMMMSSMSMAMMSMXSMSMSMAAXMAM",
      "AXMXMMMXXAMASXMMSMSMSMMMMSASAXXXMMMAMMAMMMSAMXSASMMMXAXASXMMSSMMXSMAAXAMXMMSMAXASXAXSXAAXAXMMSMAASASASASXMMASMMMMMSXMAMAAXAMMSMMSAMAXASMXSAM",
      "SXAXXSAMXSSXSAMMAAAMAMAMXMXMMXMASXXSSMXSMXMASMMAMXAMXMSMSAAAAAASAXSXSAMXMAAMMSMASXSAMMMXMAMAXMASAMAMAMAMMAMAXAXAAAMAMAXXSMSAAXAAMAMAXMAAXXAM",
      "XMMSAMASAAMASXMSMXMSASASAMASXXMAMMAXAMAXAASMMAMSMSSMAXAMXMMMSMMMAMAMXAXSSMMSAAMXMMMAMXMXSASXMMMMXMXMXMSMSAMMMMMSSMSAMMSMMAMMXMMSSMMMSSMMMSSM",
      "XMXAMSAMXAMAMXMXXMXSASMSASASASMMSSMSAMASMMSAMXMAAAXSXSAMSASAXAMMMMAMSMMMAMAMMSSMXXXAMAMXSAMXASAMXMAMXMXASASXASAMAXSAMXAAMAMXSXMAXAAXXAAAAAAX",
      "XSMSXSXASMSXSMSMSAAMXMASXMASASXAAAXSXMASMASXSSSMMMXMMSAMXAMASMXAXSAMXXASAMAMSXMXAMXSSMSAMXSSMMXSASXXAAMAMMMXMMAMXMXXMSMSMAMXMXMAXSMSXSMMMSSM",
      "MAAXAMXXMASXMAAAMSSMAMMMXSAMMMMMSSMXSAMXMMSAXMAMMMAAXMMMMAMMMXMMMMAASMXSMXSMMAMMXMAXAMMAMAMASASMXSMSSSMXMASAMXAMXMMSASXAXXSAMAXMAMXSAMASAAMM",
      "AMMMSMMMMSMXMSMSMMMXSSMXAMASXXAXMAXAMXSAMXMMMSMMASMSMXSAMASASASXMSMMASASXAXASAMSMMMSAMXSMMMAMXSMAXMAMXXASXSASXSMAAAMAMMMMXSXSMSSXMAMXSAMMSSS",
      "SMSMXAXAMMAMAAXXXMAAMMXMMSMMMXMXSAMXXMAMASAXMAASXSAXMASASXSASXSAXSXSMAXXMAMAMMMSAXASAMAMASMSSMMMXMMMXMAMMAMAMXMSSSXMSMSMSMMAMAAAXSSSXMXMSAXA",
      "XAAMSSMMMSAAXSSMMMMSMSAXMAMASMXAXAMXMMMAMXAXSMMMAMXMMMSMMXMMMMMMMXASXMMMMSMMMSAMMSMMMMMSAMAAXAMXAXAXMASASXMMMAMMMMAMXAAAAXMAMMMSMMXAMMXMMMSM",
      "MSMXAMXAASASXMAMSAMXMSAMMASAAMMSSSMMAAXAAMSMSMXSAMASXXMAMSMMSAAXXMMMAXAXAMASAMAMAAAASMXMXMMMSSMSXSXSMMMAMXAASXXAAXAMMMMXMSMMXAAMAAXMSXMMAAAX",
      "AAXMXSMMMXXMXSAMXASXXMAMSAMMSXAMMAAXSXSXSXMAXAASASASXMASXSAAXMAMMMASMMSMMSAMAXXMSSSMSXAMMSXXAMAXAAMSASMAMSSMSXMSMSASXSXMAMXMXMMSXMXXSAXSXSXS",
      "SMXSASAMMSMSXMAXSAMXAMXMAXXXXMXSXSMMMAMMMAMXMMMMXMASXMAMASMMMSSXXSASAMMAMMMXSMSMXAMXSMXSAAXMMMXMXMASAMXAMAXAXMXAAMXXAAASXSASASMMASMAMXMAAMAS",
      "XXAMASMMMAASXMAMXXAMXXSMSMSMMASAXMAMXXMASMMSSMMSSMAMMMMMXMAAXXAAXMMSAMSXMAMAXAASMAMXXMXMMSAMXMAXAAXMMMSSMAMMMSXMSSMXMMMMASASASAMAXMAMAMASMAM",
      "SSXMAMAAMMSMMXSMMMSMMXMAAAXXAMAMXXAMSMMXMXAXAAMAAMASXAASASXMSMMMMAASMMAMSAXSMSMSXSMMMSAAXMMSSSSSMXXXMMAMMSXSAMXXAAMAMASMMMXMAMMMSXSASXSAMXXS",
      "MMMMASMMMMAMMAXAMAXAXSASMSMMXSASMSMMAAMMSMMMSMMXMMXSMSMSASMMAMAXXMMSXXXXAMXMAAMSXMAAAMMMMSASAMMAMASMMMASAMXMASXMXXSMSAXAXASMSXXXAAXAMAMAXXMA",
      "MAAXAMXSASAXMMSSMASAMAAMAXMAXMAMMAMMSSMAAASAXASMXMAMMMAMAMAMAMMSMMMMMMXMAXAMXMASASXMSSXSAMMMAMSXMASAAMMMXXMXMMMMSXMAMMSSMMSAAAXMMSMSMMMSSMAM",
      "SSSSXMAMAXAMAAAAMMMXMMSMXMASXMSMMSSMMXMSSSMASAMAAMMSAMXMAXSSMSAMXAAAAMSSSMSSSXMMXMAXXAAAXSAXMMSXMMXXMMSMMXSASXSAAAMAMXAASAMXMXMXXAXXASAAAASX",
      "MAAXXSSMSMMSMMSSMXSSSXXXSXMMMMAASAAAMXMAXMMAMAMSXSXMASXMAMMAMMXMXSXSSMAXMAMAMXXSASXMMSMMXMASAAMASMMSSMAAMXSAMAMXSXSAXMMMMMSMSAAXMSXSAMMSSMMS",
      "MMMMMAMAMXXAXXAMXXAAXAMASAXMASMSAMSMMMMSMMMSSXMXXMASMMAMMAMAMXAMXAXMAMASMSMSMSMSASASAMAXXXAAMXSAMAAAAXSAMMMSMAMAMXSASXSMXAAAMMXSAAAMSMMAMMAX",
      "SASXAMMAMXSMSSMSSMMSMXMASMMMXXXMMAXAMXAAAXAAMXAMASASASXMAXMAMSASMSMSAMSXAAAAMXAMAMAMSSXMSMSXSXMASMMMSMMAXSAXSAMASAMAMAAMMSMSMAAMMMSMMMAAXMAS",
      "MXMMSXMXSXSAAAAAMAAMAXMASXMSAMXMXMSAMMXXMMMSSMXSAMXSAMMSASXSMMMSAMASMMAMSMMMSMSMMMXMXMXAXAAAXXSMMASXAASAMMAMSMSAMAMAMSMMAAAXXMMXAAAAXSSMMXMA",
      "XAMMMMSAMXMAXMMMSMMSSMMMSAMXMXAXAXSXMASXSSXAMMAMASAMXMAMMXAMXAMMAMXSAMMMMXAXMXMAAXSXASMMMSMSMAAMMAMSMMMAMSXMXAMAMXMMXXAMSSXMSMXSSMSMMAMAMASX",
      "SMSAAAMASASMMXMAAXMXMMSXMAMASMMXMMMSMXXAAXMXSMMSAMASAMSSXMAMSMMSSMXXXXXXAXSXSASMMAASAAAXMXAXXMMSMSXXXSMMMXSMMXMMXMXXAXAMMAASXXAXXAMASASAMMMS",
      "MASXSXSAXMAXAAMSSSMMMAMAMSMASAAXSMMSMSMMMMMSMXXMXSMMXSAAXSAMAMMXMMXMMMXMMXXASXSXMXMSMSSMSMMMMSAMXXAMXSAAMAAXXMASAAAMXSSMMSMMAMMSMAMAMASXSAAX",
      "MAMXMMMMSAMSSMXAMXMAMAMAMAMXMMMMMAMXAAMAAAXXXAAAMSAMXMMSMSAMAMSASXAXAXASMAMMMAMXMXXMAXMAMASAAMASXMASASXMMSSXMMASMSSMXAMXAAMMXAAAMAMAMXMASMMM",
      "MASXMASAMXXAAXXXMASMSAXSSMSMXAMAMAMXMMSSSSSMMMSXASXSMMMMXSXMSMSAMSMSXMAMMXSASASXMXSMMMMSMASMMMXMXMAMAMMXXMXMAMASXAAMSAMMSXSXSMSXSSSMXXMAMMSM",
      "MAMXMAMMSAMSSMMAMMSMMXSAAXAASMMXSASMSMXAMAXAAXMAMXAXXAMMAMMAXAMMMAMAMMSMSMMASAXAMAMAMMAMMAMXMSAMSMSSSSMXMAMXSMASMSSMSAMXMASAMXXAXMAMXMMAXMAM",
      "MSSSSXXAXMAMXASAXAXAMMAXMMMXMAAASAMAAMMASASMMMXXXMSMXSSMMMAMMXMASASASXAAAAMXMMMAMMSAMMASMMXAMSAMMAXAAAXAMXMAMMMSMAAAMAMMMAMAMAMASXMMMXAMXSAM",
      "XXAAMMMSMSXMSMMMMASMSXMMXMXAMMMMMAMSMSXAMXSXMMSSMAXAAXXXSASXMXMASASXSXMSMSXAASMMAXSAXSASASMSMSMMMMMMMMSXSAMXSAMXMSXSSSMXMASMMXSASAMASMASASAS",
      "SMMMMXXAAXAAXMASMMMXSMXXAMXXSXAXSMMXASMMMMMAAASAXMMMMSAASAMXSXSAMMMXSAXXMMMMSMAAAAMAMMXSMMAXAMXSXMAXMASAAXSASASAMXAXAMXMMMXMAMMXSAMAMAXMASAA",
      "AAXXMMSMSSMMMMMMAASASASMMMAMXMXXXXAXMSSXMAXSMXSXMMASAMAMMAMAXAMASMMASMMAAAXXAMMMMSSMXMXMAMAMAMASAMSMSAMXMXMMSAMXAMXMAMAMAXMASAMXSAMSSSXMAMMM",
      "MSMMAXAXAXMAMAXSSMMASXXAAAXXSXSAMMXSXMXSSSMMXMXMMSAMASXXMSMMMMMMSAMMMXSSMMMMSSSXMXAXMMASAMXSAMAXAMAXMASXXXMXMAMXXAASXMXMASMMXXMXMMMMAXXMXMSX",
      "XXASMAXMASXXXMXMMSMAMXXMMMSAMAMXXAAXAXAMXMAXASAMXSXSAMXXXMAXMXSXSAMXXMAMASXAAAAXXXAMMSASXMXXMSSSMMSMSAMXSXSASAMMAXXMMXSMAMMSMMSASMSMAMXMXMXM",
      "XMAMXMXSMMXMAXAAASXMMMSXAAMMMAMXMSASMMSMSMXMAMAMASMMASXSMSMXMAMASXMASMASAMMMMSMSSMSMAMXSAXSXMAMAASAMXASXMASASAAXAMXMAAMMSMMAAASMMAXMAMASAMAS",
      "XAMSMMMSAAASMMSMMMAAXAAMMXMSXXMAXAMXMAXASAMAMSSMASXSAMMXMAAAMASXMAMAXSMMXXAAXAAAXAAMXSASXMMXMASXMMSMSMMXMAMAXMSMXMAMMXSAXSSMSMXAMSMMMSMSASAS",
      "MMSAAAAMMMMSAAMMMSSMMSSXAMSMASXMMXSAMXMXMAXXXAXMAMMMMXMASXSXMAXAXXMMXSXXMASMSMMMSSXXAMXMXMXAMAMXMSMXMXAMSSMSMAMAMSASXAMMXAXAAXMXMAAAAAASAMXS",
      "AXAMSMXXXAXMMMMAAXAAXAXMXSAAAMAXSAMASXMMSAMSMMSMASMMSXSASMXAMSSMASMSASMSMAMASXXMAMXSXMAXAMAMSSSMXAXAXMMXAAAXMAMAMSAXXMSSSMXSMMAMSSMMMSMSXSXS",
      "SMMXXXSSSSMXXXXMAMSXMSSSXSXMAMXMMASAMAMASAXXAXMXASAAXAMXXAMSMAAXMAAMAXMASAMMMMSMMSXAASASMSMMAMAXMXSSMAXMSMMMMAMSMMSMMXAAAXMMSXXXAAMSXXASAMAS",
      "XAXMSMXAAMAASMMXSAMAXAAMAXMASMSXMMMMXXMAMXMXXMXMXSMMSSMMXMAAMXMMXMXMMMSAMMMSAAAXXMXSXMASAAMMSSMMAXAAMXMMXMAAXMXMSAAXAXMMXMAAASAMMSMSAMMMAMAM",
      "XSAAAAMMMMSMAAMAXAMAMMMMSMMMMAASASASXXMASMSAXAMXXMAMAAXXAASXSXSXMXXAMXMAMSAXMSSSSMXXMMXSMXMAAAASMMMXSXMXASMSXSAAMXSXSXSMSMSMAMMXXXXMMMMXAMAS",
      "AMMSMSXMXAAXSSMSSXMSXSAMMSSSMXMAMSAMAAMXAAAAASMMMSSSMMSMMMMAXASXMASAMXMMMMAMMAMXAMASAAAXMXMMXSAMAAXAAAXSAMXMASMSMXMAMAAMAMAMAAXXMMMMXSMMSSMS",
      "XXAMAMASMMSXXAMXMXXMASAXAAXMASXMAXAMSXMMMSXMMMAAAXAAAMSAXAMXMMMSXMAAAAMMMMAMSAMMAMMSMMMAMXAMAXMSSMMSSSMMXSAMMMXAXAMXMSMSAMXXMMSAAAAAXAAAAAAM",
      "XMAXASAMMMMXSAMAMMMMMMMMMSMMMXAXMMMMXAXMXMAMXSSMMMSMMMXAMMXMAXAAMXSMSXMAXMAXSAMSSXMSXSMAMAMMMXAXMAXMAAXXASASXMSSSXSXAXMSAXSXSASXSSSSMMMMMSMM",
      "XMMMMMMMMASXMASXXAMAAAMXAXASMSMMXXSASMMSASAMXXASXMAMXSMAMXMSSMMSSMAAAMSSSMMMSAMAMAXXAXXXMASAXMSMMMMMSMMXXSXMAXXAAAXMSMMMSMAAMXMAXXAMAMXSAMAS",
      "XAAMAAMMSASASAMMSSSSMSSMASAMAAMXMXMASAASXSASXSAMMMASAXXAMAAAAAAAAXMMMMAAAASAMXMASXMMMMMSSXSMXSXASXSAXAMSAXXMXMMMMMMAASXAMMMMMMXSAMXMSAAMASAM",
      "MMMSMXMAMMSAMAMMAMXXMAMMMMMMXMSAMAMAMMAMXSASXMMMAMAMAMSXSMMSSMMXXMXXXMMMMMMASMSMAMAAAAAXAMXMSMSMMMMMXAMXMASMASMMSMXSAXMASAMAMAAMXXMAXMMMMMXM",
      "XSAAMXMASXMXSSMMASXMMAMAMASXXAXAXAXAMAMXXMAMAXXSSSMMSMMAAAAXAXXXSSMXAASXSXSAMMASMSXMXSSXAMAXAAXSXMAMMSMASAXMAMAAXMAXMASASXSASMMSSMAXMASMSASX",
      "AMSSMMSASMMAMMASAMAMSASMSMASAXSSSXSSXSAMXMAMXMASASXAAAMSSMSSSMSMAAAXSAMAMAXMXXAMXMXSXMMMSMXXASMXMSMSAMSMXMSMSSMMMMXSMMMMMASASXXAXMASXMMAMAMX",
      "MMMMAXSXSAMAAXAMASAMAXSMAMMXSXXMAXAMAMAAASASMMXMAMMXMSMXAAAAXAAAMMMMAAMMMSMMMMASXAASASAAAMXSAMXXAAAMAMAMMXMAMAXAXXAXXMASMMMAMAMXSMAXAAMSMSMM",
      "XAASMMSASXSMSMXSXXMXSXMXXMMAXMAXXXSMAMXSASXSAXAMMMSSXAXSMMMSMSMMXAAAMXXXAXAMXAMAMMMSXMMMXSAAXXMAXSSMMMAMAMMSMSMMSMXMXMAMAAMXMASMAMASMMMXXMAX",
      "SXMMAAMMMAMAAMMMMMSXMAMMXAMXMSSMMMMSXSAMXSMSAMSMSAMMSMMAAAMAAMAMSXMMSSXMASMMAXAXAAAMXMXMAMMSMMMSAMXAAXXMXSAMMXSAMXMMMMXSMMSMXAXASMAMMASXMXMM",
      "AMSXMMMXMAMXMSXSAAXASXMSSMASAAXSAAXAMMXXMSMMSMAAMXSMMSAMSSSMSMAMMAMAAXXAAXXSAMXSSMSAMXAMXMXAAXAAAXSSSSMAAMASXAAXMMSAASASASAXMMSSXMAMSAMXMASM",
      "SXSAXMAMMMSMAXASMXSAXSAXXMAMMAMSSSMSMXSMMMAMAXMXMMXMAMMMAAAMXXXMMAMMSSMMMSMMMSAMASMXASXSAMMSSMSSSMMXAAMMMSAMMSSMSAMMSMASXMASMXMMASXMMXSMSAXA",
      "MASAMMSSMAAXXMASAXMMMMXMXMAMAMAMXXAAXMMAASAMMXSASXMMSSMMMSMMAMMSSMSXAMAAXAAAAMASMMMSXSMMMSAMXAXXXAXMMMMMAMASAMAAMXSXMMXMAMMMMAMSMMSMAMAMMMSS",
      "MAMAXMAAMSMXXMMMMMMMAMAAMSSSMSSSMMSMSASMMSASXAMAMAMAMXMAXAXMASAXAMMMMSSXSSSMMSXMAAXMAMXAAMMSMMMSSSMASMSMAAAMMSMMMASAMXAMXXAASAMAAAXXAAAMXMAM",
      "MSSSMMSMMAXXMXMAAAAXAMMSAAAAAAAAAXAAXXXMASMMMSSMXSMAMAXSMSXSAXMMAMXAMXMAMAMAASMXMAMMAMMSMXMAXXAXAMXMAXXMAXXSXMASMASAMMMMSMSXSXSXMXSMXXSSXSAS",
      "XMAXSXMMSSSMAAMSSSSSMSMMMMMMMMMXSSMSSMXMAMAAXMAMAMAMXSMXAXAMAMSSSMSMSAMAMMMMXSASMMXSASXXMASASXXSMSSSMXMAXSASASXMMMSMMMAXMAMXSMMXSAMASMMMXMAS",
      "SMSMSMSAAAAMAMMMAAMMMAXXASXSXSMAXXXAMXAMSSSSSMXMASASMMAMSMMMSMMAXASASXSAMXAMAMAMMAASMSASMXMASXAMAMAAXSXXXAAXAMXXXMMMSSMSMSMAXAXAMAMAXAAAXMAM",
      "SMMASAMMMSMMAMAXXXSMSSSSMXAMAAMMMMMMSMXSAXXMAMXSXSASAMXMMAMAMASAMXMAMXSAMSAMXMAMMMXSAMAMASAMXAMMAMXMAXAMSMMMSSXMAXAAXAAAAAASMMMMSSMMSSMSMMSS",
      "SAMAMMSXMXAMXSXMSAAAAMAAMMSMSMMAAXAAAXMMMSMSXMASAMMMMMAMXAMASMMAMMMAMAMAMMASXMSMXXAMXMXMASMXMXMSXSMSMMMMMAAMXAASXMMSSMMMSMXMAAAAAXAXAMAMXAAM",
      "SAMXSASAMSXMXAAAMXMMMMXAMAXAAXSXSSMSSXXAXMXMASASMXAAASMSSXMASXXSMAXAMMSMSSSMXSAAXSAMXAAMXMXXMAMMXSAAAMAAMMMSMSMMAAAAXAAAAMASMMMMSMSMMMAMMMSS",
      "XAMXMASAMAASMMMMXXSAMXMSMMSSMMSAXAMXMMSMSMXSAMASXSMSXMAAMAMAMMMXSXMXMASAAMAASMMMMAAMSSMMAXMASAXMAMSMSSSSXSAXAXASXMMSSXMSXXMMAXSXXAXAASAMAAXA",
      "SMMXSAXAMSSMAAMXMASAXXMAAMAMMAMXMAMAXMAAAAXXAMXMXMAXASMSSXMAMAAMAXSMXMMMMSMMMMAAMSMMMAXSASMMSMMMAXAMAMAAAMAMXMAMXMAXMMMMXXXMSMXAMXMXMMAXSSSM",
      "SSMAMMSMMMMMSMSXSAMXMSSSSMASMSSMSAMASXMSMSMSXMSMMXAMAMAAXXXXSMSXMMAAMXXAMXXXAXSSMMMXSAMXXMAXXAXSXMSAAMMMMMMMXMAMAMXMMAMMXSAAMAMSMMSMSMSMAAMX",
      "SAMXSXAXAMXAMXXAMXXAAAAAXMMMXAAMSXSMXAAXXMAMMAAAMXMMSMMMMSMXMAMAASMSSMSSSSMMMMMXAAAXMMMMXSMMMMMMAAXSMXSAMAAXXXASMSSMSXSAAMMMMAMAAXAASAXAMXMS",
      "MXMSXXMSMSMASAMXMASMMMMMMXMXMMSMSASMSMXMMMAMMMSASXMAMAMXAAMXMAXXAMXAAAAAAMSASAMXSMMMMXAAMMSAAMAXXMXASASASXSSXSASAAAAAXMMMSSMSSMMAMMXMAMAXAXS",
      "MSMMAMXAAMXMMXMAXXMXAXXASAMXXAMXMAMASXAMAXXXAXXAXXMMXMMAMASXMXMMSMMSSSMMMMSXSASMXMXAXSMSXASXSXSSSMSMMMMMMMAAMMXMMSMMMMMAXAAMAXXXSASASXMXSMSS",
      "MAAMXMSMSMAMXSSMXSMSSMSASMSAMXMAMXMSMSASMMMSASMSMMMSAMXSSMMXMAXSASAMMMMAMXXASAMMXSSXXAXAMMMMMMXAMAAMMMMAAXMSMMAXAAAAXMSSSSSMASMXMASMSASXSMAM",
      "SSSMAAXAAXAMXAAMMSAAAAMXMAXASASMMMXXAXMAMSAMXAAMAAASASXAAAXMMSXMAMXSASMXSAMXMSMMXMAMXMASXXAAXXMAMSMSASXSMXXMAXSMSSXMSAAAXAAMASXAMXMXSAMAXMXM",
      "XAAXXSMSMSXXMSAMAMMMMMMMSASMMMAXAXASAMSXMMAMAMXMSXMSXMXSSMMMAMAMXMMSAMAAMXMXAAMMASAMXMSMXSSSMMMSMXAMXSAAMXSXMSAMAXAAXMAMMXMMASXMSAMXMAMSMMSS",
      "MMMMXXAAMMMXXMAMMMAXAMXMMXXXAXMSSMAMXXXAXSXMAXXMAMXMAMAMXXXMASXMXSAMXMMASAMSSMXSAMMXXSMSAXMAXSAMAMMMSMMMMASAMAMMAMMXMASMSMSMXXXSAMXMSXMXAAAS",
      "SAAXMMSMSAMXXSAMSSSSXMAAXSMSMMXAAMAMMASXMAXSXMAMAXSMSMASAMXMASMMMMXSAMXASAMXAAXMMMSMMMAMMSMSMMASXXSXAMXMMASAMMXMSSMAXAXAXMASAXSMMSAMXAASXMMS",
      "AXASAAAASMMSMMAMAAAAASMSMMASASMSMSAXSAXMMSMMAMAMXXSAXMXMAMAMXXAMAMASASMMSXMMMMMXAAAAXMAMXXAAASAMXASMMSMXMAXAMXXMMAXSSMMMXAMXMXXAAXASXMAMAAXX",
      "MSMMXSMMMMXSMSMMMMMXMXAAAMAMXMAMASAXMXMXXXAMAMXSMAMAMMSMXMASXXSSSMASXMXAMASXMAMXMSSMMSASAMXMAMSSMMMAAAXXMMSSMMXMSAMXAXXMASXMASMXMSMMMMASMMSS",
      "XAAMMMMAAXAMAMXAMXMMMMSSMMMSAMMMMMMMXSAMXXMSMSAXMAMAMXSAAMSMMAAAAMAMAMMASAMMSASAMMMAASMXAMXXXMXMAXXSAMXMSAAAMXMXMASXMMAMAMASXXAXMAXAMSMSAAAA",
      "SSMMAAMSMMXSASMSMXXAXAXMXAAMXMAAXAMSSSSSMSAAXMASXSSXSAMMMMAAMXMXMMASAMSXMASXSASASAMMMSMSSSMSXSAMMSAMXXSAMMSMMAAASXMMXMAMMSXMXXXMSASMMSAMMMMS",
      "AAMSXMMAMAMMASAXAASMMXSSSMSSMSSSSXSMAMAMAAMSMMAMXAXAMXSXMSSSSSMSXMXSXMXMSAMXMXMAMAMSAXXAAAAAXMASMAXMXMAXMAAAMXXXSAXXASMSXMASAMSAMXXMAMAMXSXX",
      "MMMSASXSXSXMAMMMMMXMAMAAAAMXXAAXXMAMXMAMSMXMXMSSMSMXMMXAXMAMXAAAXSASMSAMMASMMMMAMSMMXSMMSMMMSSMMMXMMMMMMMSSSMSSXXXMSASAXASAMAAAXSXSMSSSMAMXX",
      "XAMSAMAASAMMXXXXSXMASMMMMMMMMMSXSAMXXSXMMAMXMAXAAXXXXASMMMSMSMMMSMAMASXMSXMAAXSXSXXXMAAMAAAXAXAMSMAXAXXXMMAAXMAMMMAXMMMXXMAXXMSMMXAAAAAMXSAM",
      "MMMMAMXMMXAXSSSXMASAMXSSMSMSAMAAAMMXMASMXSMMMASMMMSAMMSAAXMAMSAXXMAMXMMXMASMSXMSAMXAMMSSMSSXMSXMAAXSXSMSSMSMMMAMAAMSXAMMSSMSSMXAMSMMMSMMASAS",
      "MSXSSMMXMXSXAAAMSAMMSMMAAAASMSMSSSMSAMXMAMAXAXAXAMAMXAMXMMMAMAMSSMASAAAAMXSAMXSAMXMMXAAAMAXAAAMSMSAMXSAAXAMXASASXSSXMMSAAAAMMASAMAXAXMAMXMAM",
      "MAAMAMXAMMMSSXMMMXSASXSMMMMMMXAAAAAMMMSMASXMSXMSMXAXMMSASASASMMAMSMSMSSXSAMAMXXXXASAMMSSMASMMMMMXAAAAMMMSXMMMSASAXMAXMMMSMMMMXMASXSSSSXMMMMM",
      "MMSMMSSMSAAMXAMXXSMMSAXAAXAMXMMMSMMMAAMMASXMMAXAMMSMSASASASMSXMAMAMXXMXXMXSAMXASXMSASXAXMASAAMAMXSMMMMAAMASXXMAMMMMSMSXAAXASXASXXXSAMXMMAASA",
      "XAAAXAXASMSMSMMAMXMXMMMSXSASAAXXXASMMXMMMXMAMMMSMMAAMXMMMMMXMASXSMSSXSMXMXSASXAXMASAMMSSSXSMMSSSXMAASXMAMAXXXMASXSSXAXSSXSXXSASASXXMAXAXXSAS",
      "MMSSSMSXXAAAAASXMAXSAAAAASAMXXASAXMAMAASASXSAMXMASMXMAMXXSMAMAXAAXAAAXAAMMXAMMSMMXMMMXMAMAMMXAMXAXSXSAMAMSSMXSMSAMXMMMMMMMMMMSMAMXAMASMSMMMA",
      "SAAAAXMMMMMSMXMAMAASMMMSMMAMXASXMXSAMMSMASAXAMASMMAASAMMMMMAMMSSMMXMASXMSASXXAMXMMSXSAMAMAMMMSSSMMXASAMSMMAAXMAMMMXSSMSAAAAASAMXMSSMAXXAASXM",
      "SMMXMAAXAAXXMASXMMXXMAXMASMMMMMAMMMAXXAMAMASAMMSAMSMSAMXAAXMSAAAXAXXMXMASMAAMXMAASXAAXSXSXSAMAMAXAMAMXMAAXXMXMAMASAMAAAMMMSSSMSAMXAMMSXMMMXX",
      "XASXMSAMXSMAXMXAXSAMMSSSMMSSSXSAMMSAMMMMASASAMAMMXXMSXMSSSSSMMMSMMSSMAMMMXMXXXMMMAMAMMMXMMMMMMSSXMMXMASXSMASMSXSASAMMMMMSMMMMXSASXMSASMASXXS",
      "XAMAXMXSAAMXMXXMMMASAXAAXAMXMASASXMAMXASXMASXMASMSMAMAMAAAXMASAXAAAASASAMAXMMMXSMMAAXAMAMAAXXMAAAAMAMMMAXMAXAAXMAMXMMAXXXAAMXMSXMAMMAXXAMAMM",
      "SSMMMMASMXXMASMSASAMMSSMMXSAAXXASXSASMMMAXAMMSXXAAAASAMMSMASAMXSMMSMMMSASXXXAAAXAMSXSSSSSXSMMMMSMMMAMXMSSMSSSMMMSMASXMSMSSMMAXXMASXMAMMMSSMM",
      "AAAXXMASXMXXSXXXAMMAMXXXAAMMMSMMMAMASMXSMMXSAMMMSMSMSASXAXMMXSASAXXXMMSXMASMSMSMAMMAXMAXMXXAMMXAMXSMSAMMAAAAAXXAMMXAMXMAAAASMXAMSMAMSMSMAAMX",
      "MXAMXMXSAMSMMMSMAMSSSSSMMSMSASAAMSMMMMXXXASMXMAXXAXASAMMXXXSXMASXMMXSAMASAMXXAMMSMMSMMXMSASAMXXXXAXXSASMMMMSMMMASMXMSAMMSSMMSSXMAXAMAAAMMMMS",
      "SASMSXASAMXAAAXMAMXMAAAMAAAMASXMXAXAAMXMMSSMSSMSMAMAMAMXSMASMMAMMMAAMASAMASAMXMAMMAMAMAAMASXMMMSMMSASAMXASXXASMAAXAXMASMMMMAMMMSMXMSMSMSMMAM",
      "XAMAAMMSAMMSMSXSASXMMMMMMSMMAMXSSMSSXSAMXXMMAASXMMMAMSXAAMAMAMMSAMMXSAMASXMMSSMASMASXMMSMAMMXMASASAXMAMSAMASAXMSSSMSSSMMAAMASAAASAAAAAAAXMAS",
      "XXMAMMXSMMXAXXMSMSAMXXXXXXXMXSXXAXMMXSASXMXMSSMXAASXSAMXSMSSXMASMMSAMXMXXXSXSMSSXMMSMXSXMASMXMSSSMASMMMSASXMMSAMXMXXMMASMSSXSXSASMSMSMSMSSMS",
    ];

    let word = "XMAS";
    let count = count_occurrences(&grid, word);
    println!("The word '{}' appears {} times in the grid.", word, count);
}

fn count_occurrences(grid: &[&str], word: &str) -> usize {
    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    // Directions: (row_offset, col_offset)
    let directions = [
        (0, 1),   // Horizontal right
        (0, -1),  // Horizontal left
        (1, 0),   // Vertical down
        (-1, 0),  // Vertical up
        (1, 1),   // Diagonal top-left to bottom-right
        (-1, -1), // Diagonal bottom-right to top-left
        (1, -1),  // Diagonal bottom-left to top-right
        (-1, 1),  // Diagonal top-right to bottom-left
    ];

    for row in 0..rows {
        for col in 0..cols {
            for &(dr, dc) in &directions {
                if check_word(grid, word, row, col, dr, dc) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn check_word(grid: &[&str], word: &str, start_row: usize, start_col: usize, dr: isize, dc: isize) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let word_len = word.len() as isize;

    let mut row = start_row as isize;
    let mut col = start_col as isize;

    for i in 0..word_len {
        if row < 0 || row >= rows || col < 0 || col >= cols {
            return false;
        }
        if grid[row as usize].chars().nth(col as usize) != Some(word.chars().nth(i as usize).unwrap()) {
            return false;
        }
        row += dr;
        col += dc;
    }

    true
}

