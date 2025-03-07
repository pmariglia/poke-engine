# Changelog

## [v0.0.33](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.33) - 2025-03-07

### Features

- Proper Side Condition Durations Tracking - ([cf792b5](https://github.com/pmariglia/poke-engine/commit/cf792b562c9ac189c860f152754d31865e28edbc))

- Corrosion - ([5cc1ba2](https://github.com/pmariglia/poke-engine/commit/5cc1ba2c86f7528a2a7293d029df2277411b5723))

- Photongeyser, sunsteelstrike, and moongeistbeam ignore some abilities - ([9f974e3](https://github.com/pmariglia/poke-engine/commit/9f974e3863e069bc852299a1714f0edba92f25a5))


### Bug Fixes

- Firstimpression in damage calc always assumes it is your first turn on the field - ([2a92280](https://github.com/pmariglia/poke-engine/commit/2a92280a9177a0ce35ca95358d7be60828af6952))

- Transformation items cannot be removed - ([7438a68](https://github.com/pmariglia/poke-engine/commit/7438a68a259b6ad3217fcfe5f56322d77cde5054))

- Rustedsword and rustedshield are permanent items - ([ccc18d3](https://github.com/pmariglia/poke-engine/commit/ccc18d35bbedad2d8fffa5ba36224539217d40fd))

- Adaptability and STAB with tera - ([4b324a9](https://github.com/pmariglia/poke-engine/commit/4b324a9746cb304590962f5f020bd33700973311))

- Check for moldbreaker before doing unaware negation in damage calc - ([cdaa6eb](https://github.com/pmariglia/poke-engine/commit/cdaa6ebb1e923e1677d29c0058ba4660039f40a0))

- Goodasgold only blocks moves that target the other side - ([5fba9d6](https://github.com/pmariglia/poke-engine/commit/5fba9d6d6326f47552e1eb15bddc9286d3a73b54))

## [v0.0.32](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.32) - 2025-02-27

### Features

- Strengthsap interaction with liquidooze - ([0d69c04](https://github.com/pmariglia/poke-engine/commit/0d69c04ed28cbf92c6234eb8bf6974866640f322))

- Consecutive protect - ([fb42c50](https://github.com/pmariglia/poke-engine/commit/fb42c50cf789e64bbcb3664366601c344bffd1ae))


### Bug Fixes

- LeechSeed never affects grass types - ([23de644](https://github.com/pmariglia/poke-engine/commit/23de64430f70b0814f538f069dfc164173e66cdd))

- AssaultVest prevents status moves - ([9cbcb06](https://github.com/pmariglia/poke-engine/commit/9cbcb06afd06749e18bf6c632a14e51c396b385d))

- Toxicspikes and stickyweb are not blocked by magicguard - ([92f5b96](https://github.com/pmariglia/poke-engine/commit/92f5b96c080c79d0480fae257dede56ae5ee1a15))


### Miscellaneous Tasks

- Add test to make sure lifeorb into protect does not recoil - ([73221a2](https://github.com/pmariglia/poke-engine/commit/73221a20dacd1eba1caaef90f3712d68dd12e808))

- Test for whirlwind into goodasgold - ([29f7efb](https://github.com/pmariglia/poke-engine/commit/29f7efbaba948cb7443af25bfe0fedf76549978f))


### Misc

- Add tests to ensure suckerpunch/thunderclap functionality - ([28995dc](https://github.com/pmariglia/poke-engine/commit/28995dc048ce6a8cf58b2e0cc9f93d0e170f2495))

## [v0.0.31](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.31) - 2025-02-01

### Bug Fixes

- Magicbounce reflects reflectable volatiles - ([6d4bb79](https://github.com/pmariglia/poke-engine/commit/6d4bb799dd8786a63b370d83c425d7b590f1578c))


### Miscellaneous Tasks

- Remove moves 5 and 6 - ([4c2c614](https://github.com/pmariglia/poke-engine/commit/4c2c6147d0ef121c28780a5d47d8d2341605f2ef))

## [v0.0.29](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.29) - 2025-02-01

### Features

- [**breaking**] Typechange revert on switchout if types != base_types - ([f21af63](https://github.com/pmariglia/poke-engine/commit/f21af63a1820c0453725e40b8e69510c3d1b08ed))


### Bug Fixes

- PartiallyTrapped is removed if the opposing side switches out - ([53bfe17](https://github.com/pmariglia/poke-engine/commit/53bfe178561877d1d36c022d6467acf8ed57fffb))

## [v0.0.28](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.28) - 2025-01-26

### Features

- HealBlock volatile prevents healing moves - ([9bec851](https://github.com/pmariglia/poke-engine/commit/9bec85114359df8a221bfd2631aaf7255fe078a4))

- Gen1 paralysis cannot be applied from secondary if one of the defender's types is the same as the move's type - ([74f5ce0](https://github.com/pmariglia/poke-engine/commit/74f5ce0143dd1cda134dca0651f4cb132cbb1289))

- Triplearrows multi secondaries - ([5d324d7](https://github.com/pmariglia/poke-engine/commit/5d324d7bd93c62502c386191af3d971d23c88636))

- Firstimpression - ([2730a0b](https://github.com/pmariglia/poke-engine/commit/2730a0b9518a4ff6ca5691a1807011183610fb3f))

- Cramorant's GulpMissile - ([a276f97](https://github.com/pmariglia/poke-engine/commit/a276f97e9f6c0abd84ecb7f46ffdcb8017b712aa))

- Doubleshock and burnup - ([4058439](https://github.com/pmariglia/poke-engine/commit/4058439c640c73a246d4584f52d512f637b288bc))

- Endure - ([bdf7cf7](https://github.com/pmariglia/poke-engine/commit/bdf7cf715061499a539526c406c9c8ebbbc4d97b))

- Grassyglide - ([4f26234](https://github.com/pmariglia/poke-engine/commit/4f262347f59b4a0bf5e6a45da86bdf24e1baf910))

- Speed Ties - ([d8c7970](https://github.com/pmariglia/poke-engine/commit/d8c79704397af1016064ffde730253ae167faffc))


### Bug Fixes

- Iceface lets secondary effects happen - ([b2d38b2](https://github.com/pmariglia/poke-engine/commit/b2d38b29262ffbb764e1f64128de38d733c75f19))

- Pressure only decrements PP when the move targets the pkmn with pressure - ([ee27e7e](https://github.com/pmariglia/poke-engine/commit/ee27e7e2e05b863d154f06b1367a39786011fa2f))

- Defog fails vs good as gold - ([6f62087](https://github.com/pmariglia/poke-engine/commit/6f62087c750f1e961105ffe622ae8bac045f05fa))

- Tripleaxel does 40 damage per hit - ([e9aa044](https://github.com/pmariglia/poke-engine/commit/e9aa044ad2b671d9fe8d102f5dca851f953dcf69))

## [v0.0.27](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.27) - 2025-01-12

### Features

- Triple-axel is treated as triple-hit move - ([c26f0ab](https://github.com/pmariglia/poke-engine/commit/c26f0ab9368312e1dbf11e9426dde7532d3a3c7d))

- Implement miracleseed and spelltag - ([1672f71](https://github.com/pmariglia/poke-engine/commit/1672f71e62204065014168f9d0f2fc6f2e8ee3cc))

- Loadeddice giving 4 hits - ([db90a52](https://github.com/pmariglia/poke-engine/commit/db90a5215387df2a8704b935cb07c6b7ebb08bdd))


### Bug Fixes

- Dont switch to 0hp pokemon in team preview - ([7e9bfab](https://github.com/pmariglia/poke-engine/commit/7e9bfab59c29b60e03f98ec465f66953a6884e0c))

- Re-implement DestinyBond Correctly - ([5ed81fc](https://github.com/pmariglia/poke-engine/commit/5ed81fcd0d5b948b6ed65bcd22b9a6f151b9fd4e))

- Gen9 activates tera - ([c80439d](https://github.com/pmariglia/poke-engine/commit/c80439dd3e18e604a77c85a1167757590db80a15))

- Gen7+ hit self in confusion is 1/3 instead of 1/2 - ([11fa285](https://github.com/pmariglia/poke-engine/commit/11fa2858741c05caf7b61b1579cc88e45338622d))

- Adaptability and terastallization boost is 2.25 - ([3720436](https://github.com/pmariglia/poke-engine/commit/3720436d5e991770e0dfe9a9d021e8d32d89ace1))

- Assaultvest does not lower damage if special move targets defense - ([7a1825d](https://github.com/pmariglia/poke-engine/commit/7a1825d825d806b02d514b093c32b57e2eb9654d))

- Mycelium might ability ignoring only applies when using status moves - ([15b23f1](https://github.com/pmariglia/poke-engine/commit/15b23f15797c8ddc6c4b4b6429da31c7c6276a61))

- Clearing all effects of a move gets rid of all flags - ([6c46a55](https://github.com/pmariglia/poke-engine/commit/6c46a55a863c307f56c829258d02f4da8755dd1f))

- Revelationdance uses tera type if terastallized - ([e7cdb76](https://github.com/pmariglia/poke-engine/commit/e7cdb766ada4658244567bc05ee8c0f58ad8aea9))

- Terastallization base power floor happens before anything else - ([a1e2e47](https://github.com/pmariglia/poke-engine/commit/a1e2e4709aa550bb27d37bf7a63bbf5d5434db3d))

- Tera penalty 50 -> 75 - ([5eab0ec](https://github.com/pmariglia/poke-engine/commit/5eab0ecaaeb93b13c4457e268f937dc1d8517e20))

- StrengthSap fails at -6 attack on defender - ([5f7d149](https://github.com/pmariglia/poke-engine/commit/5f7d14910598c5983af51c5cb8cafcd23f799bf3))


### Revert

- Gen1 status eval increases - ([f203757](https://github.com/pmariglia/poke-engine/commit/f203757f37bcef0eb319b8064c6296cf16f2513d))

## [v0.0.26](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.26) - 2025-01-05

### Features

- Gen1 stat modification glitches - ([bf08c28](https://github.com/pmariglia/poke-engine/commit/bf08c287b2359e571125d26371d0cba1eef98c98))


### Bug Fixes

- Gen4 lifeorb does not do recoil when hitting a substitute - ([fc240f6](https://github.com/pmariglia/poke-engine/commit/fc240f63f9c27b634b062d0c051711cc563ff28b))

- Thunderclap is increased priority - ([32c040a](https://github.com/pmariglia/poke-engine/commit/32c040a6667d86cf45c6585849e4d07ea0c4b3e4))

## [v0.0.25](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.25) - 2024-12-31

### Features

- Morpeko / MorpekoHangry forme changes - ([263260e](https://github.com/pmariglia/poke-engine/commit/263260e64d3b121300878e627a9be5581740eced))

- Wishiwashi forme change - ([4295ef8](https://github.com/pmariglia/poke-engine/commit/4295ef824c5182f81168f6446a9a17858d0a4355))

- Palafin-hero formechange on switchout - ([518b96e](https://github.com/pmariglia/poke-engine/commit/518b96e19d89c164cd3d106227396cd09335aaf6))

- Eiscue iceface formechange when getting hit and when switching into snow/hail - ([03fdb03](https://github.com/pmariglia/poke-engine/commit/03fdb039f3f5f9d6aee512c363f2db8b5617afbe))


### Bug Fixes

- Refactor forme change instructions to use PokemonName rather than a formechange enum - ([f1f2f0a](https://github.com/pmariglia/poke-engine/commit/f1f2f0acbcb9c79147e64c5d1c1567eac1522b5e))

- Mimikyu busting is conditional on pkmn id being equal to mimikyu - ([3c13510](https://github.com/pmariglia/poke-engine/commit/3c13510ce487b6142bafa89bfab477cb462caf7c))

## [v0.0.24](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.24) - 2024-12-29

### Features

- [**breaking**] ChangeStat Instructions, Natures & EVs, Minior/MiniorMeteor FormeChange - ([6330453](https://github.com/pmariglia/poke-engine/commit/6330453d04c71f0540f23b92d1cd9907d22b9fbc))


### Bug Fixes

- Less-than-or-equal for activating berries based on HP - ([8f36e81](https://github.com/pmariglia/poke-engine/commit/8f36e81428b99ee54dfa433c138b4b07595d3165))

- Formatting - ([5c13f9c](https://github.com/pmariglia/poke-engine/commit/5c13f9c7d031c4a17aa59391bf54757a535dbf8a))

## [v0.0.23](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.23) - 2024-12-28

### Features

- Consider critical strikes - ([4cc71a1](https://github.com/pmariglia/poke-engine/commit/4cc71a173cf53c6d8a82ddfdd227590359a0e1e2))

- Guaranteed Crit moves and BattleArmor / ShellArmor abilities - ([942d812](https://github.com/pmariglia/poke-engine/commit/942d812d5741f316df9d34be73c987994c68c9a2))


### Bug Fixes

- Check para/sleep/confusion before "before_move()" - ([c1de340](https://github.com/pmariglia/poke-engine/commit/c1de3409ff6e0daa8920678b6d34e2b0931376d4))

- SleepTalk works when a pokemon is asleep without rest - ([c041f23](https://github.com/pmariglia/poke-engine/commit/c041f234656bea8783d72b5b1ef101bd934edd0f))


### Revert

- "fix: Check para/sleep/confusion before "before_move()"" - ([7eecfa9](https://github.com/pmariglia/poke-engine/commit/7eecfa99dd9fea85edee3fb191147e05aff241d7))

## [v0.0.22](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.22) - 2024-12-17

### Bug Fixes

- Ghost are immune to counter, not fighting - ([168934c](https://github.com/pmariglia/poke-engine/commit/168934c9ba806fb3f17b6d9061efd791de3884ef))

- Gen2 counter/mirrorcoat interactions with hiddenpower - ([b068ebe](https://github.com/pmariglia/poke-engine/commit/b068ebebe33188cc013be968d76c0d352419ad8a))

- Truant should allow you to use whatever move you want, but not execute that move - ([5385c04](https://github.com/pmariglia/poke-engine/commit/5385c04e4754a2ed25fcce557d4c303496fd0ba0))


### Miscellaneous Tasks

- Some groundwork for instruction generation on the python side - ([446faa2](https://github.com/pmariglia/poke-engine/commit/446faa2d2bb227f1916ff2bee8e59f774903f001))

## [v0.0.21](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.21) - 2024-12-14

### Performance

- Gen1 evaluation tuning - ([d0e6b34](https://github.com/pmariglia/poke-engine/commit/d0e6b34384947b9080a0c40edd3ec89667ad3c85))


### Miscellaneous Tasks

- Remove gen2 unused eval values - ([ce96591](https://github.com/pmariglia/poke-engine/commit/ce965918a312a9504b0fcf240506e8dca6d1c3d5))

## [v0.0.20](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.20) - 2024-12-12

### Features

- Truant and its interaction with recharge moves - ([d285b48](https://github.com/pmariglia/poke-engine/commit/d285b48b1e8abe6f1152ae53d24c91861ae9bd5b))

- Implement damage roll branching for mcts depths 0 and 1 - ([7dcdad8](https://github.com/pmariglia/poke-engine/commit/7dcdad8cf3efc6ce50a3fa093f1f18c90c742dc2))

- Asoneglastrier / asonespectrier have the effects of chillingneigh / grimneigh - ([44145d2](https://github.com/pmariglia/poke-engine/commit/44145d221b9bbd102e1134db4a066e9385c413a1))

- Battlebond gen9 - ([05f838d](https://github.com/pmariglia/poke-engine/commit/05f838d33c994d23aad48f8c18804d8b20c32e2a))


### Bug Fixes

- Orbs/globes affect all formes - ([f263a05](https://github.com/pmariglia/poke-engine/commit/f263a0541c79128fcbc466b0a8d062123963ec04))


### Miscellaneous Tasks

- Remove crate::* usages that my editor liked to insert - ([d2aafb3](https://github.com/pmariglia/poke-engine/commit/d2aafb3d98e55e0be0307b41aec0e1caf3a831a0))

## [v0.0.19](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.19) - 2024-12-08

### Features

- Branch if damage is a range to kill - ([11e8f32](https://github.com/pmariglia/poke-engine/commit/11e8f3222382d721bed3df8eae48bff60e7fe665))

- Force repr for enums defined by define_enum_with_from_str. Implement into() and from() - ([84933a6](https://github.com/pmariglia/poke-engine/commit/84933a6ea443e2fb1f8566415e68aeaaf9d2f24d))

- Gen2 berries: Mint & Miracle - ([c3ecd64](https://github.com/pmariglia/poke-engine/commit/c3ecd649912d6057d2ffd10f88c0c530657a6ef1))

- Freeze clause gen1/2 - ([7406b37](https://github.com/pmariglia/poke-engine/commit/7406b37835eae8d165a5667873aea50fda6e98e6))


### Bug Fixes

- Don't combine duplicate instructions in run_move - ([e9a6858](https://github.com/pmariglia/poke-engine/commit/e9a68583c36796d27df74b17760c98f525a0606d))

- Confusion stopped by substitute - ([7df6916](https://github.com/pmariglia/poke-engine/commit/7df69161a42a4083afb51b38d65f2c5babf0a98d))

- Refactor belly drum to actually work - ([a84f3b2](https://github.com/pmariglia/poke-engine/commit/a84f3b2410dcf04f7e00999367f2209f82bb88c1))

- Growth doesnt boost attack in earlier gens - ([3df5146](https://github.com/pmariglia/poke-engine/commit/3df51460cc61f2b2a132afc1899b7f23402423e3))

- Bellydrum damage instruction should do half HP - ([9219ae0](https://github.com/pmariglia/poke-engine/commit/9219ae0a4b08b7b0adb176862b3a7924f2b79daa))

- Refactor wish instruction to use 2 bytes less - ([5b1573d](https://github.com/pmariglia/poke-engine/commit/5b1573d71a2ac1b581728e0f630f5d1febc3be93))

- Refactor substitute instruction to use 2 bytes less - ([a7036d8](https://github.com/pmariglia/poke-engine/commit/a7036d8a810adc7b1a678cc8c3c35c3815539ee8))

- Stellar is typeless for dmg calc - ([4986805](https://github.com/pmariglia/poke-engine/commit/49868058bedd65dbc506fd8c00a275da8eef5654))


### Miscellaneous Tasks

- Readme serialize -> state - ([5d3b71d](https://github.com/pmariglia/poke-engine/commit/5d3b71d0cb96191ca5747917819c6ee17d1018fa))

- Add test to make sure pp not decremented when asleep - ([4a08839](https://github.com/pmariglia/poke-engine/commit/4a088399e1c844f82b5f916b6c663a0058b92b75))

## [v0.0.18](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.18) - 2024-12-01

### Features

- Gen1 compatibility - ([0ee705c](https://github.com/pmariglia/poke-engine/commit/0ee705ccfdc33766c4bcb8f1dadce253bf3f69e2))

## [v0.0.17](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.17) - 2024-11-28

### Features

- Ogerpon masks cant be removed - ([ac16f27](https://github.com/pmariglia/poke-engine/commit/ac16f2731822d556f9841826e5236c093487d870))

- GriseousCore as GriseousOrb clone - ([036e011](https://github.com/pmariglia/poke-engine/commit/036e0118626fecaa2342f6b0b7edff05e8bff9b1))

- Allow 'switch' to signify switching in damage calc - ([db7c22a](https://github.com/pmariglia/poke-engine/commit/db7c22a988ac9aa5701dec547f0c871829fc3c82))

- Add LUSTROUSGLOBE - ([7095f90](https://github.com/pmariglia/poke-engine/commit/7095f90192aa2c9c03766415c2b06d80268dbb57))

- Charge 2x electric move - ([9743ea0](https://github.com/pmariglia/poke-engine/commit/9743ea03bd1fecd1f7f4dfdf0195e746c7bb47c1))

- Implement ADAMANTCRYSTAL - ([848de22](https://github.com/pmariglia/poke-engine/commit/848de22cdd70dd58042bf37a0ae555db9f8ff57f))

- Implement aurawheel type switch - ([dde8bb2](https://github.com/pmariglia/poke-engine/commit/dde8bb29fcf185c6cafb6da700b115ce7546a73e))

- Implement forme_change instruction - ([9ed3888](https://github.com/pmariglia/poke-engine/commit/9ed3888b453b84760d51acd709c1290c9f14a4a0))

- Implement gen3 stuff - ([6c51891](https://github.com/pmariglia/poke-engine/commit/6c518915732d2822582527d307a8525239a9e4f2))

- Gen2 support - ([27c48cd](https://github.com/pmariglia/poke-engine/commit/27c48cdfec6d7848ec99b3bf6572ab6ce264ed2b))

- Implement thief - ([61de20e](https://github.com/pmariglia/poke-engine/commit/61de20e5933d3584feefab2796b0fb47013cf8ed))

- Safeguard protects from status if they are coming from the opponent's move - ([b4908ac](https://github.com/pmariglia/poke-engine/commit/b4908ac05a3ae6076a11a7566b345485d1bf20db))

- Gen2 splitting of code - ([83af057](https://github.com/pmariglia/poke-engine/commit/83af05741da686553970c10c44529d51ca8e0443))


### Bug Fixes

- Has_type checks tera-type if terastallized - ([80dde9b](https://github.com/pmariglia/poke-engine/commit/80dde9b9609eebc71f08d940203da272534e1a34))

- Hp based ability boosts are less than or equal, not less-than - ([b633045](https://github.com/pmariglia/poke-engine/commit/b6330450a56234842eb4e7d23aad9f6378feeff0))

- Type_effectiveness_modifier checks tera-type - ([def932d](https://github.com/pmariglia/poke-engine/commit/def932db9b27af710b99601ad58e79e31f37276f))

- Knockoff damage boost is based on item being permanent - ([3b5b578](https://github.com/pmariglia/poke-engine/commit/3b5b578cecb3616235b955ab0d416b9dfe9ed86e))

- Supreme overlord should count fainted pkmn - ([e64ff95](https://github.com/pmariglia/poke-engine/commit/e64ff95e894685b72cbf53276ec6cb557c14077d))

- Some mechanics bugfixes: - ([baf31f6](https://github.com/pmariglia/poke-engine/commit/baf31f6539ed4d730ee17daa6d0c5611cde987b9))

- Sheerforce boosted by certain volatiles - ([71d3d4d](https://github.com/pmariglia/poke-engine/commit/71d3d4d1e4a24c48a515edd2348c5fa9c60a8018))

- Imports behind feature - ([f940363](https://github.com/pmariglia/poke-engine/commit/f94036311e8eae4f34b016228fc7bfda38ce1a7c))

- Allow switch move when calculating damage through python bindings - ([3143d1a](https://github.com/pmariglia/poke-engine/commit/3143d1a2ce9642264210e05005cec12d734c737c))

- AlluringVoice is a special move (howd that happen lol) - ([2586dd5](https://github.com/pmariglia/poke-engine/commit/2586dd560eecf09e5108dd16ae8f2001ffde24d7))

- Lusterpurge bp boost in gen9 - ([f66d135](https://github.com/pmariglia/poke-engine/commit/f66d135f22f8eedaca4a50d424b925c958aa7edc))

- Mistball bp boost gen9 - ([0042e7e](https://github.com/pmariglia/poke-engine/commit/0042e7e63fe8b5234943e84f3b183b538e9f15fa))

- Psychic Terrain makes priority moves have no effect - ([c9ef19f](https://github.com/pmariglia/poke-engine/commit/c9ef19fdfea13dd7bb64598ade6a83261f625e54))

- Sleep clause shouldn't count fainted pkmn - ([528745c](https://github.com/pmariglia/poke-engine/commit/528745cc9d4e80827279395d1d76fad25afc8d6b))

- Change logic to align with the meaning of item_is_permanent - ([5d06959](https://github.com/pmariglia/poke-engine/commit/5d06959d7a630ab7ac1668649dac1787e5824a2b))

- Hardrock -> hardstone - ([900edce](https://github.com/pmariglia/poke-engine/commit/900edcea331fef282d4198322a9111dd57e8cf5b))


### Miscellaneous Tasks

- Remove remove_low_chance_instructions - ([64828d9](https://github.com/pmariglia/poke-engine/commit/64828d9c07b12ac42de88485806a5d2dd66a73e5))

- Delete 3000 lines of serialization code - ([07bd935](https://github.com/pmariglia/poke-engine/commit/07bd935de7c1ef040ab55cd13e3a07a750dfe151))


### Refactor

- Create enum for PokemonName - ([5e05e6a](https://github.com/pmariglia/poke-engine/commit/5e05e6aa35194a252a1595c5474f08f3f96b0870))

- Combine before_move and modify_move - ([589af92](https://github.com/pmariglia/poke-engine/commit/589af92ecaaaef10e58a40031438f829aaad1e0d))

## [v0.0.16](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.16) - 2024-11-10

### Features

- [**breaking**] Implement FutureSight - ([37beed5](https://github.com/pmariglia/poke-engine/commit/37beed547ec1f1148aa6632fb0313de34d384720))

- Implement triattack - ([b8bf4e7](https://github.com/pmariglia/poke-engine/commit/b8bf4e709e50879c85a1f876e5c9b35b45b5eff1))

- Implement population bomb (approximation) and widelens - ([ec996e6](https://github.com/pmariglia/poke-engine/commit/ec996e63f6baaa86edf2bbdf994d1cbf1aca53a1))


### Bug Fixes

- Using a move that is your tera-type while terastallized makes minimum BP=60 - ([20779c2](https://github.com/pmariglia/poke-engine/commit/20779c276a4eff9421d87216e9c1f37f60a4dda3))

## [v0.0.15](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.15) - 2024-11-10

### Features

- Terablast physical/special check and typechange - ([fb0de03](https://github.com/pmariglia/poke-engine/commit/fb0de032a2d511905aeef342165520c430a408d2))

- Implement magnet - ([0cc5594](https://github.com/pmariglia/poke-engine/commit/0cc5594aebcd15334a679e790718d6d91bdaee4e))

- Implement waterbubble - ([a5d4e02](https://github.com/pmariglia/poke-engine/commit/a5d4e02d39897039930aad7ee4db4f853d124511))

- Implement * of ruin abilities - ([3ff639e](https://github.com/pmariglia/poke-engine/commit/3ff639e68d3ea2a753fe4f5111d553b8caf6cfe0))

- Implement Protosynthesis / Quarkdrive - ([9609f2f](https://github.com/pmariglia/poke-engine/commit/9609f2f10a45be9d8a9a901a84d7d20c2bad6590))

- Implement Thermal Exchange - ([8ef6572](https://github.com/pmariglia/poke-engine/commit/8ef65722c3e3139cc38c4c9f6884b51158351dfa))

- Implement Toxic Chain - ([8235837](https://github.com/pmariglia/poke-engine/commit/8235837f3646628a24c141d217b046da0300d047))

- Implement WellBakedBody - ([86b7999](https://github.com/pmariglia/poke-engine/commit/86b7999880b9d4029ea61b6789107abb306ec3f0))

- Add remaining gen9 ignorable abilities - ([2eac648](https://github.com/pmariglia/poke-engine/commit/2eac648e94fef1fed73d04bce189431e8bd1dd03))

- Implement Good as Gold - ([2f54eec](https://github.com/pmariglia/poke-engine/commit/2f54eec7c17a35b648b28671b30c8dd45247203a))

- Implement Dark immunity to prankster gen7 onward - ([a35d909](https://github.com/pmariglia/poke-engine/commit/a35d909b005c001ac232eaa066da25598e451608))


### Bug Fixes

- Lightball boosts all pikachu formes - ([f93a8bb](https://github.com/pmariglia/poke-engine/commit/f93a8bb226e1064e09174b083fec8d48311f1040))

## [v0.0.14](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.14) - 2024-11-09

### Features

- Add remaining gen9 moves - ([bf46836](https://github.com/pmariglia/poke-engine/commit/bf46836d579976bea72ce86fb9b9636a59d74314))

- Implement burning bulwark - ([05df708](https://github.com/pmariglia/poke-engine/commit/05df70897d9a4da592d5130e9b20f31a2072fb6e))

- Implement GigatonHammer and Bloodmoon - ([955ccc9](https://github.com/pmariglia/poke-engine/commit/955ccc9f645597a5970c950d762ab3e0a5d39725))

- Implement electrodrift - ([0497bdf](https://github.com/pmariglia/poke-engine/commit/0497bdf7c87a527564eff378d1944107730cfe7c))

- Comeuppance as metalburst clone - ([6329438](https://github.com/pmariglia/poke-engine/commit/63294380da38650735d5935d0e4b604a270ac5f8))

- Implement meteorbeam and electroshot - ([526b839](https://github.com/pmariglia/poke-engine/commit/526b83987aca7a412aa796d8d629e6ef37f653fb))

- Implement hardpress - ([e5bd684](https://github.com/pmariglia/poke-engine/commit/e5bd68454f9804dd0083bcd9ed317adb37eccf61))

- Implement icespinner - ([344c1f4](https://github.com/pmariglia/poke-engine/commit/344c1f4ae50455bbc33d3a485a001360bf68cb9c))

- Implement last respects - ([476d9b5](https://github.com/pmariglia/poke-engine/commit/476d9b53c31268d6137e7608f1a6e8400104bdc2))

- Implement mortalspin - ([754212a](https://github.com/pmariglia/poke-engine/commit/754212a8dc96cd729ef0a39b30ab52a581fe4b8a))

- Implement ragingbull - ([e9e80f9](https://github.com/pmariglia/poke-engine/commit/e9e80f9afcfac45b53beaea8560c4e89e884819b))

- Thunderclap as suckerpunch clone - ([4ac9137](https://github.com/pmariglia/poke-engine/commit/4ac9137e5380f323df16cc3cfdd23a00703f2cfb))

- Implement tidyup hazard/sub removal - ([91f5ba1](https://github.com/pmariglia/poke-engine/commit/91f5ba1c12bd615bc24dbc4ecb9939f113567d84))

- Implement upperhand - ([c107790](https://github.com/pmariglia/poke-engine/commit/c10779069d96f22109cbd59efbff42e13323e8eb))

- Implement ogerpon masks & ivycudgel - ([483bffd](https://github.com/pmariglia/poke-engine/commit/483bffd9fc15063e89a1e4cd7f946d8c3dd953b6))

- Embody aspect abilities on switchin - ([b30099b](https://github.com/pmariglia/poke-engine/commit/b30099b78dec1e20e51a9b42947d1d545994867c))

- [**breaking**] Terastallization (wip) - ([cb3fc51](https://github.com/pmariglia/poke-engine/commit/cb3fc514ce4853d9746b78c5190e1ff78d0ed59d))


### Bug Fixes

- Dont zero-out naturesmadness/ruination if target is immune to normal - thats only for superfang - ([8803a7d](https://github.com/pmariglia/poke-engine/commit/8803a7daa211b970d689be231a5319581644f4e8))


### Miscellaneous Tasks

- Remove unused flags - ([272cd0d](https://github.com/pmariglia/poke-engine/commit/272cd0d456e20acdd333b63240856cc3979ae6af))

- Add slicing/wind flags - ([8df9a0b](https://github.com/pmariglia/poke-engine/commit/8df9a0b479e2ccade335563ca8468b7f72789c02))

- Implement wind rider & sharpness - ([e23a473](https://github.com/pmariglia/poke-engine/commit/e23a473e16d60b8577896aceea1b8a828da8af9e))

- Check for physical before negative guts burn bp reduction - ([86418ad](https://github.com/pmariglia/poke-engine/commit/86418ade42fefbf527b3935282d758e6c6ef8a7c))

## [v0.0.13](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.13) - 2024-10-25

### Features

- Decrementing PP - ([b68e884](https://github.com/pmariglia/poke-engine/commit/b68e884bbdccef281e48ba450539c8403916d482))

- Implement Gen9 Snow - ([88610b9](https://github.com/pmariglia/poke-engine/commit/88610b9720518cd97f83edde2ef41b13dd96f4aa))


### Bug Fixes

- Fix explosion related moves - ([b6a39f6](https://github.com/pmariglia/poke-engine/commit/b6a39f62a0f40211c713cd080fef15d3be64f312))


### Performance

- Refactor SetDamageDealt instruction - ([4974dd6](https://github.com/pmariglia/poke-engine/commit/4974dd694767de938fa468be8844d58474c9004f))


### Miscellaneous Tasks

- Add test for ceaselessedge - ([e72e45c](https://github.com/pmariglia/poke-engine/commit/e72e45ca7153b0351926971c76514cac5b93d813))

- Some damage_calc refactoring - ([3131bdc](https://github.com/pmariglia/poke-engine/commit/3131bdc23fdbc3c9e7186a6b6400e7a9cf64549c))

- Update pyo3 - ([ae4d0d0](https://github.com/pmariglia/poke-engine/commit/ae4d0d07d717ee8d6b2235259aee9d78fa8aaef7))


### Refactor

- Add an amount to DecrementPP instruction - ([ad972b8](https://github.com/pmariglia/poke-engine/commit/ad972b8d42a8d668d8c150f0a1bc462b5365f016))

- Remove last_used_move & damage_dealt features - ([54b48fa](https://github.com/pmariglia/poke-engine/commit/54b48fab7251411451bda3ca86d261d62e60dc25))

## [v0.0.12](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.12) - 2024-10-13

### Features

- Implement ArmorTail - ([dadad97](https://github.com/pmariglia/poke-engine/commit/dadad977a982b949002a9bbd543c419444609818))

- Implement ChillingNeigh & GrimNeigh - ([64f6848](https://github.com/pmariglia/poke-engine/commit/64f68487376507ccb273f6db22acb8ae4191bfb4))

- Implement AuraBreak - ([19235b2](https://github.com/pmariglia/poke-engine/commit/19235b2b9bf8eb893b9b0063354241b25d1495ab))

- Implement Gorilla Tactics - ([f4c8c76](https://github.com/pmariglia/poke-engine/commit/f4c8c76f0e99db95d5dd82e16b102ec77899d87f))

- Implement GuardDog - ([c154be4](https://github.com/pmariglia/poke-engine/commit/c154be4da7ba79297a416d62979a3c320ec2d9cc))

- Implement Hadron Engine - ([f1441cc](https://github.com/pmariglia/poke-engine/commit/f1441cc1543de54679daf6d95563e2ad6eb62e53))

- Implement Innards Out - ([ea2e7ba](https://github.com/pmariglia/poke-engine/commit/ea2e7ba0329115beb73bdbc429a21ab6914e16e9))

- Implement Minds Eye - ([8931219](https://github.com/pmariglia/poke-engine/commit/8931219928f63be5eb6086f7be861f4d35c4c20b))

- Implement Mycelium Might - ([b907b89](https://github.com/pmariglia/poke-engine/commit/b907b89c2388c5d6c9ba6c3b4b0a5ff00e23dedd))

- Implement Neutralizing Gas - ([d6cdfc3](https://github.com/pmariglia/poke-engine/commit/d6cdfc3a26b55eb48fde1a769469fc40f2a7e8aa))

- Implement PerishBody - ([efedf6a](https://github.com/pmariglia/poke-engine/commit/efedf6a0bf4fa0dd4467cac1d85c8b14e145a03a))

- Implement Orichalcum Pulse - ([3a29fff](https://github.com/pmariglia/poke-engine/commit/3a29fff338cbd7a49d8242224d2225c824c7d9ba))

- Implement Sand Spit - ([ab7fdbe](https://github.com/pmariglia/poke-engine/commit/ab7fdbe4c64805674f447b1bf4fed25326527b7f))

- Implment SteelySpirit - ([0e2d72b](https://github.com/pmariglia/poke-engine/commit/0e2d72b4f5c14eae1b3e4a0f0b553215302666b4))

- Implement toxicdebris - ([595420f](https://github.com/pmariglia/poke-engine/commit/595420f288247b678800afa6c626cba4b02a799a))

- Implement Adrenaline Orb - ([1987f88](https://github.com/pmariglia/poke-engine/commit/1987f8821dc67f0a1a8c57be5279df6a8e89df5e))

- Implement Silvally Memories - ([3436ce6](https://github.com/pmariglia/poke-engine/commit/3436ce67db66bfbd359f1980e81e873a64800bc2))


### Performance

- MCTS Performance Improvements - ([efd9b00](https://github.com/pmariglia/poke-engine/commit/efd9b00c01cfbeafd396fce9a205e439c2c49e8d))


### Miscellaneous Tasks

- Test for eartheater - ([bb16559](https://github.com/pmariglia/poke-engine/commit/bb16559b5edda316fc40ac36d9383b1bd500c53d))

- Implement Seed Sower - ([00ec032](https://github.com/pmariglia/poke-engine/commit/00ec032bc0b1c8ddb3609895e02f4951faaa3df8))

- Refactor some logic around LastUsedMove - ([f4076b1](https://github.com/pmariglia/poke-engine/commit/f4076b190fad46e37d372ac68ea605ed0d83039a))

## [v0.0.11](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.11) - 2024-09-30

### Features

- [**breaking**] Proper Sleep Turn Tracking - ([2281f7d](https://github.com/pmariglia/poke-engine/commit/2281f7d51fc00a65b84949868eb6d7c937b056fa))


### Bug Fixes

- Fix some interactions around pivot moves - ([6fedefa](https://github.com/pmariglia/poke-engine/commit/6fedefadc21bdee9ce0f89a09c52240a169dec95))


### Miscellaneous Tasks

- Fix some compilation warnings around unused variables - ([40f3480](https://github.com/pmariglia/poke-engine/commit/40f34802070e61da14bb977f10848544e610a27d))

## [v0.0.10](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.10) - 2024-09-29

### Features

- [**breaking**] Track turns remaining for weather/terrain/trickroom. - ([13e9373](https://github.com/pmariglia/poke-engine/commit/13e93735b5d0963668c654a956383cec2a04e5d9))

- Implement ironball - ([d834548](https://github.com/pmariglia/poke-engine/commit/d8345488c716127255500a39529d5fb9d6f8a8fa))


### Performance

- Modify mcts rollout parameter to be the change in evaluation rather than the absolute evaluation - ([3fd0072](https://github.com/pmariglia/poke-engine/commit/3fd0072f6135826066e65ad31171395255531b1b))


### Miscellaneous Tasks

- Add exit-early to release script if git working tree is not clean - ([ceb85a1](https://github.com/pmariglia/poke-engine/commit/ceb85a13485f50b93867f2cf2cfb3116ddcc80a1))

## [v0.0.9](https://github.com/pmariglia/poke-engine/releases/tag/v0.0.9) - 2024-09-18

### Miscellaneous Tasks

- Remove  from auto publish trigger - ([8b1e34b](https://github.com/pmariglia/poke-engine/commit/8b1e34b5c0b0e4f6aaf4239c04d912e45a0f1c70))

- Refactor release script process - ([1250555](https://github.com/pmariglia/poke-engine/commit/1250555f1949c63b53d9746728447f53b0a2fa66))


### Repo

- README updates - ([1b95784](https://github.com/pmariglia/poke-engine/commit/1b9578442a802b303eb10ef04496dd9de9133ad5))

## [v0.0.8](https://github.com/pmariglia/poke-engine/releases/tag/0.0.8) - 2024-09-15

### Miscellaneous Tasks

- Some Makefile/Changelog fixes - ([3b3f31b](https://github.com/pmariglia/poke-engine/commit/3b3f31b106af9b6d8f7d2068ea53eb3a643b9c83))

- Add release script - ([ec2a5b5](https://github.com/pmariglia/poke-engine/commit/ec2a5b56d06e837bcd5ab8c745e0f7aea02da301))

## [v0.0.7](https://github.com/pmariglia/poke-engine/releases/tag/0.0.7) - 2024-09-15

### Features

- Implement Pickpocket, Magician, and StickyHold Abilities - ([d729469](https://github.com/pmariglia/poke-engine/commit/d729469ebdfbad8e99bf18202c4ad49ec8df2f75))

### Miscellaneous Tasks

- Remove unnecessary return statements - ([9c3315b](https://github.com/pmariglia/poke-engine/commit/9c3315bb11daafc1457451c8c8d061dfffeefff4))

## [0.0.6](https://github.com/pmariglia/poke-engine/releases/tag/0.0.6) - 2024-09-14

### Features

- Implement Yawn - ([698a2e6](https://github.com/pmariglia/poke-engine/commit/698a2e6806a79ed4008a94fe47d6f5481a92f15a))
- Implement Haze - ([585f25e](https://github.com/pmariglia/poke-engine/commit/585f25e5996138eb99688c622bb6c2eb804ff104))
- Implement ClearSmog statboost-clearing effect - ([f95129a](https://github.com/pmariglia/poke-engine/commit/f95129a32e7ef3c19637e9841675ab7fa4d5010a))
