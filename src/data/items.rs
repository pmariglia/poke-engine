use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::state::Pokemon;
use crate::state::State;

type ModifySpeedFn = fn(&State, &Pokemon) -> f32;

lazy_static! {
    static ref ITEMS: HashMap<String, Item> = {
        let mut items: HashMap<String, Item> = HashMap::new();

        items.insert("bitterberry".to_string(), Item { modify_speed: None });

        items.insert("tartapple".to_string(), Item { modify_speed: None });

        items.insert("pinapberry".to_string(), Item { modify_speed: None });

        items.insert("tr95".to_string(), Item { modify_speed: None });

        items.insert("fairymemory".to_string(), Item { modify_speed: None });

        items.insert("hondewberry".to_string(), Item { modify_speed: None });

        items.insert("medichamite".to_string(), Item { modify_speed: None });

        items.insert("scizorite".to_string(), Item { modify_speed: None });

        items.insert("aerodactylite".to_string(), Item { modify_speed: None });

        items.insert("psychicgem".to_string(), Item { modify_speed: None });

        items.insert("tr19".to_string(), Item { modify_speed: None });

        items.insert("scopelens".to_string(), Item { modify_speed: None });

        items.insert("choicespecs".to_string(), Item { modify_speed: None });

        items.insert("safariball".to_string(), Item { modify_speed: None });

        items.insert("bigroot".to_string(), Item { modify_speed: None });

        items.insert("tr99".to_string(), Item { modify_speed: None });

        items.insert("tr34".to_string(), Item { modify_speed: None });

        items.insert("sweetapple".to_string(), Item { modify_speed: None });

        items.insert("bugmemory".to_string(), Item { modify_speed: None });

        items.insert("destinyknot".to_string(), Item { modify_speed: None });

        items.insert("netball".to_string(), Item { modify_speed: None });

        items.insert("mawilite".to_string(), Item { modify_speed: None });

        items.insert("expertbelt".to_string(), Item { modify_speed: None });

        items.insert("darkiniumz".to_string(), Item { modify_speed: None });

        items.insert("rindoberry".to_string(), Item { modify_speed: None });

        items.insert("gengarite".to_string(), Item { modify_speed: None });

        items.insert("fossilizeddino".to_string(), Item { modify_speed: None });

        items.insert("widelens".to_string(), Item { modify_speed: None });

        items.insert("tr90".to_string(), Item { modify_speed: None });

        items.insert("cameruptite".to_string(), Item { modify_speed: None });

        items.insert("tr15".to_string(), Item { modify_speed: None });

        items.insert("apicotberry".to_string(), Item { modify_speed: None });

        items.insert("blueorb".to_string(), Item { modify_speed: None });

        items.insert("floatstone".to_string(), Item { modify_speed: None });

        items.insert("airballoon".to_string(), Item { modify_speed: None });

        items.insert("poisonmemory".to_string(), Item { modify_speed: None });

        items.insert("dubiousdisc".to_string(), Item { modify_speed: None });

        items.insert("deepseascale".to_string(), Item { modify_speed: None });

        items.insert("crucibellite".to_string(), Item { modify_speed: None });

        items.insert("tr43".to_string(), Item { modify_speed: None });

        items.insert("loveball".to_string(), Item { modify_speed: None });

        items.insert("luminousmoss".to_string(), Item { modify_speed: None });

        items.insert("electricgem".to_string(), Item { modify_speed: None });

        items.insert("pikaniumz".to_string(), Item { modify_speed: None });

        items.insert("tr36".to_string(), Item { modify_speed: None });

        items.insert("tr01".to_string(), Item { modify_speed: None });

        items.insert("strawberrysweet".to_string(), Item { modify_speed: None });

        items.insert("lustrousorb".to_string(), Item { modify_speed: None });

        items.insert("venusaurite".to_string(), Item { modify_speed: None });

        items.insert("fairygem".to_string(), Item { modify_speed: None });

        items.insert("tr53".to_string(), Item { modify_speed: None });

        items.insert("shellbell".to_string(), Item { modify_speed: None });

        items.insert("luxuryball".to_string(), Item { modify_speed: None });

        items.insert("spelonberry".to_string(), Item { modify_speed: None });

        items.insert("powerbracer".to_string(), Item { modify_speed: None });

        items.insert("charizarditex".to_string(), Item { modify_speed: None });

        items.insert("rowapberry".to_string(), Item { modify_speed: None });

        items.insert("tr37".to_string(), Item { modify_speed: None });

        items.insert("sachet".to_string(), Item { modify_speed: None });

        items.insert("grassyseed".to_string(), Item { modify_speed: None });

        items.insert("manectite".to_string(), Item { modify_speed: None });

        items.insert("powerband".to_string(), Item { modify_speed: None });

        items.insert("waterstone".to_string(), Item { modify_speed: None });

        items.insert("tr14".to_string(), Item { modify_speed: None });

        items.insert("armorfossil".to_string(), Item { modify_speed: None });

        items.insert("sunstone".to_string(), Item { modify_speed: None });

        items.insert("steelixite".to_string(), Item { modify_speed: None });

        items.insert("psychiumz".to_string(), Item { modify_speed: None });

        items.insert("tr77".to_string(), Item { modify_speed: None });

        items.insert("thunderstone".to_string(), Item { modify_speed: None });

        items.insert("dawnstone".to_string(), Item { modify_speed: None });

        items.insert("chestoberry".to_string(), Item { modify_speed: None });

        items.insert("tr42".to_string(), Item { modify_speed: None });

        items.insert("rustedsword".to_string(), Item { modify_speed: None });

        items.insert("tr92".to_string(), Item { modify_speed: None });

        items.insert("tr93".to_string(), Item { modify_speed: None });

        items.insert("shucaberry".to_string(), Item { modify_speed: None });

        items.insert("beedrillite".to_string(), Item { modify_speed: None });

        items.insert("tr22".to_string(), Item { modify_speed: None });

        items.insert("nomelberry".to_string(), Item { modify_speed: None });

        items.insert("premierball".to_string(), Item { modify_speed: None });

        items.insert("brightpowder".to_string(), Item { modify_speed: None });

        items.insert("muscleband".to_string(), Item { modify_speed: None });

        items.insert("grassiumz".to_string(), Item { modify_speed: None });

        items.insert("electricseed".to_string(), Item { modify_speed: None });

        items.insert("helixfossil".to_string(), Item { modify_speed: None });

        items.insert("meadowplate".to_string(), Item { modify_speed: None });

        items.insert("altarianite".to_string(), Item { modify_speed: None });

        items.insert("duskstone".to_string(), Item { modify_speed: None });

        items.insert("habanberry".to_string(), Item { modify_speed: None });

        items.insert("fightinggem".to_string(), Item { modify_speed: None });

        items.insert("groundgem".to_string(), Item { modify_speed: None });

        items.insert("cobaberry".to_string(), Item { modify_speed: None });

        items.insert("electirizer".to_string(), Item { modify_speed: None });

        items.insert("redorb".to_string(), Item { modify_speed: None });

        items.insert("tr21".to_string(), Item { modify_speed: None });

        items.insert("lightball".to_string(), Item { modify_speed: None });

        items.insert("dragonmemory".to_string(), Item { modify_speed: None });

        items.insert("tr25".to_string(), Item { modify_speed: None });

        items.insert("tr58".to_string(), Item { modify_speed: None });

        items.insert("salacberry".to_string(), Item { modify_speed: None });

        items.insert("fairiumz".to_string(), Item { modify_speed: None });

        items.insert("tr88".to_string(), Item { modify_speed: None });

        items.insert("mail".to_string(), Item { modify_speed: None });

        items.insert("fistplate".to_string(), Item { modify_speed: None });

        items.insert("powerherb".to_string(), Item { modify_speed: None });

        items.insert("coverfossil".to_string(), Item { modify_speed: None });

        items.insert("tr46".to_string(), Item { modify_speed: None });

        items.insert("ultranecroziumz".to_string(), Item { modify_speed: None });

        items.insert("poweranklet".to_string(), Item { modify_speed: None });

        items.insert("fightiniumz".to_string(), Item { modify_speed: None });

        items.insert("aguavberry".to_string(), Item { modify_speed: None });

        items.insert("poisonbarb".to_string(), Item { modify_speed: None });

        items.insert("qualotberry".to_string(), Item { modify_speed: None });

        items.insert("starfberry".to_string(), Item { modify_speed: None });

        items.insert("tr10".to_string(), Item { modify_speed: None });

        items.insert("mewtwonitey".to_string(), Item { modify_speed: None });

        items.insert("metalcoat".to_string(), Item { modify_speed: None });

        items.insert("tr45".to_string(), Item { modify_speed: None });

        items.insert("darkgem".to_string(), Item { modify_speed: None });

        items.insert("razorclaw".to_string(), Item { modify_speed: None });

        items.insert("shinystone".to_string(), Item { modify_speed: None });

        items.insert("kasibberry".to_string(), Item { modify_speed: None });

        items.insert("wepearberry".to_string(), Item { modify_speed: None });

        items.insert("lightclay".to_string(), Item { modify_speed: None });

        items.insert("payapaberry".to_string(), Item { modify_speed: None });

        items.insert("watergem".to_string(), Item { modify_speed: None });

        items.insert("babiriberry".to_string(), Item { modify_speed: None });

        items.insert("durinberry".to_string(), Item { modify_speed: None });

        items.insert("silverpowder".to_string(), Item { modify_speed: None });

        items.insert("insectplate".to_string(), Item { modify_speed: None });

        items.insert("tr72".to_string(), Item { modify_speed: None });

        items.insert("berrysweet".to_string(), Item { modify_speed: None });

        items.insert("roseliberry".to_string(), Item { modify_speed: None });

        items.insert("tr27".to_string(), Item { modify_speed: None });

        items.insert("tr54".to_string(), Item { modify_speed: None });

        items.insert("slowbronite".to_string(), Item { modify_speed: None });

        items.insert("sailfossil".to_string(), Item { modify_speed: None });

        items.insert("tr63".to_string(), Item { modify_speed: None });

        items.insert("skullfossil".to_string(), Item { modify_speed: None });

        items.insert("miracleberry".to_string(), Item { modify_speed: None });

        items.insert("lunaliumz".to_string(), Item { modify_speed: None });

        items.insert("absolite".to_string(), Item { modify_speed: None });

        items.insert("oranberry".to_string(), Item { modify_speed: None });

        items.insert("fossilizeddrake".to_string(), Item { modify_speed: None });

        items.insert("mintberry".to_string(), Item { modify_speed: None });

        items.insert("buggem".to_string(), Item { modify_speed: None });

        items.insert("dragoniumz".to_string(), Item { modify_speed: None });

        items.insert("flyingmemory".to_string(), Item { modify_speed: None });

        items.insert("ghostgem".to_string(), Item { modify_speed: None });

        items.insert("quickclaw".to_string(), Item { modify_speed: None });

        items.insert("quickpowder".to_string(), Item { modify_speed: None });

        items.insert("tr07".to_string(), Item { modify_speed: None });

        items.insert("decidiumz".to_string(), Item { modify_speed: None });

        items.insert("blazikenite".to_string(), Item { modify_speed: None });

        items.insert("rootfossil".to_string(), Item { modify_speed: None });

        items.insert("tr96".to_string(), Item { modify_speed: None });

        items.insert("tr79".to_string(), Item { modify_speed: None });

        items.insert("reapercloth".to_string(), Item { modify_speed: None });

        items.insert("berryjuice".to_string(), Item { modify_speed: None });

        items.insert("tr78".to_string(), Item { modify_speed: None });

        items.insert("blacksludge".to_string(), Item { modify_speed: None });

        items.insert("sharpedonite".to_string(), Item { modify_speed: None });

        items.insert("tr33".to_string(), Item { modify_speed: None });

        items.insert("mistyseed".to_string(), Item { modify_speed: None });

        items.insert("charizarditey".to_string(), Item { modify_speed: None });

        items.insert("upgrade".to_string(), Item { modify_speed: None });

        items.insert("tr61".to_string(), Item { modify_speed: None });

        items.insert("salamencite".to_string(), Item { modify_speed: None });

        items.insert("firiumz".to_string(), Item { modify_speed: None });

        items.insert("levelball".to_string(), Item { modify_speed: None });

        items.insert("whippeddream".to_string(), Item { modify_speed: None });

        items.insert("zoomlens".to_string(), Item { modify_speed: None });

        items.insert("twistedspoon".to_string(), Item { modify_speed: None });

        items.insert("buginiumz".to_string(), Item { modify_speed: None });

        items.insert("marshadiumz".to_string(), Item { modify_speed: None });

        items.insert("tr39".to_string(), Item { modify_speed: None });

        items.insert("energypowder".to_string(), Item { modify_speed: None });

        items.insert("tr62".to_string(), Item { modify_speed: None });

        items.insert("icestone".to_string(), Item { modify_speed: None });

        items.insert("lucarionite".to_string(), Item { modify_speed: None });

        items.insert("electricmemory".to_string(), Item { modify_speed: None });

        items.insert("tr16".to_string(), Item { modify_speed: None });

        items.insert("tr00".to_string(), Item { modify_speed: None });

        items.insert("blukberry".to_string(), Item { modify_speed: None });

        items.insert("luckypunch".to_string(), Item { modify_speed: None });

        items.insert("psncureberry".to_string(), Item { modify_speed: None });

        items.insert("kebiaberry".to_string(), Item { modify_speed: None });

        items.insert("darkmemory".to_string(), Item { modify_speed: None });

        items.insert("audinite".to_string(), Item { modify_speed: None });

        items.insert("groundiumz".to_string(), Item { modify_speed: None });

        items.insert("poisoniumz".to_string(), Item { modify_speed: None });

        items.insert("enigmaberry".to_string(), Item { modify_speed: None });

        items.insert("tr09".to_string(), Item { modify_speed: None });

        items.insert("tr24".to_string(), Item { modify_speed: None });

        items.insert("souldew".to_string(), Item { modify_speed: None });

        items.insert("cherishball".to_string(), Item { modify_speed: None });

        items.insert("rockmemory".to_string(), Item { modify_speed: None });

        items.insert("firegem".to_string(), Item { modify_speed: None });

        items.insert("leppaberry".to_string(), Item { modify_speed: None });

        items.insert("seaincense".to_string(), Item { modify_speed: None });

        items.insert("dousedrive".to_string(), Item { modify_speed: None });

        items.insert("tr17".to_string(), Item { modify_speed: None });

        items.insert("bottlecap".to_string(), Item { modify_speed: None });

        items.insert("shedshell".to_string(), Item { modify_speed: None });

        items.insert("tr26".to_string(), Item { modify_speed: None });

        items.insert("pinkbow".to_string(), Item { modify_speed: None });

        items.insert("tr30".to_string(), Item { modify_speed: None });

        items.insert("ganlonberry".to_string(), Item { modify_speed: None });

        items.insert("dreamball".to_string(), Item { modify_speed: None });

        items.insert("chilanberry".to_string(), Item { modify_speed: None });

        items.insert("blunderpolicy".to_string(), Item { modify_speed: None });

        items.insert("iciumz".to_string(), Item { modify_speed: None });

        items.insert("roseincense".to_string(), Item { modify_speed: None });

        items.insert("sportball".to_string(), Item { modify_speed: None });

        items.insert("cheriberry".to_string(), Item { modify_speed: None });

        items.insert("pinsirite".to_string(), Item { modify_speed: None });

        items.insert("razorfang".to_string(), Item { modify_speed: None });

        items.insert("blastoisinite".to_string(), Item { modify_speed: None });

        items.insert("fightingmemory".to_string(), Item { modify_speed: None });

        items.insert("tr49".to_string(), Item { modify_speed: None });

        items.insert("absorbbulb".to_string(), Item { modify_speed: None });

        items.insert("pechaberry".to_string(), Item { modify_speed: None });

        items.insert("heavydutyboots".to_string(), Item { modify_speed: None });

        items.insert("ribbonsweet".to_string(), Item { modify_speed: None });

        items.insert("rockgem".to_string(), Item { modify_speed: None });

        items.insert("waveincense".to_string(), Item { modify_speed: None });

        items.insert("miracleseed".to_string(), Item { modify_speed: None });

        items.insert("tyranitarite".to_string(), Item { modify_speed: None });

        items.insert("ironplate".to_string(), Item { modify_speed: None });

        items.insert("timerball".to_string(), Item { modify_speed: None });

        items.insert("pikashuniumz".to_string(), Item { modify_speed: None });

        items.insert("tr57".to_string(), Item { modify_speed: None });

        items.insert("tr47".to_string(), Item { modify_speed: None });

        items.insert("cloversweet".to_string(), Item { modify_speed: None });

        items.insert("parkball".to_string(), Item { modify_speed: None });

        items.insert("tr73".to_string(), Item { modify_speed: None });

        items.insert("pidgeotite".to_string(), Item { modify_speed: None });

        items.insert("nestball".to_string(), Item { modify_speed: None });

        items.insert("throatspray".to_string(), Item { modify_speed: None });

        items.insert("chopleberry".to_string(), Item { modify_speed: None });

        items.insert(
            "choicescarf".to_string(),
            Item {
                modify_speed: Some(|_state, _pkmn| {
                    return 1.5;
                }),
            },
        );

        items.insert("friendball".to_string(), Item { modify_speed: None });

        items.insert("jabocaberry".to_string(), Item { modify_speed: None });

        items.insert("tr89".to_string(), Item { modify_speed: None });

        items.insert("spookyplate".to_string(), Item { modify_speed: None });

        items.insert("tr81".to_string(), Item { modify_speed: None });

        items.insert("tr48".to_string(), Item { modify_speed: None });

        items.insert("passhoberry".to_string(), Item { modify_speed: None });

        items.insert("firememory".to_string(), Item { modify_speed: None });

        items.insert("adrenalineorb".to_string(), Item { modify_speed: None });

        items.insert("tr76".to_string(), Item { modify_speed: None });

        items.insert("figyberry".to_string(), Item { modify_speed: None });

        items.insert("przcureberry".to_string(), Item { modify_speed: None });

        items.insert("magmarizer".to_string(), Item { modify_speed: None });

        items.insert("whiteherb".to_string(), Item { modify_speed: None });

        items.insert("sitrusberry".to_string(), Item { modify_speed: None });

        items.insert("stick".to_string(), Item { modify_speed: None });

        items.insert("tr23".to_string(), Item { modify_speed: None });

        items.insert("focusband".to_string(), Item { modify_speed: None });

        items.insert("machobrace".to_string(), Item { modify_speed: None });

        items.insert("snowball".to_string(), Item { modify_speed: None });

        items.insert("iapapaberry".to_string(), Item { modify_speed: None });

        items.insert("metalpowder".to_string(), Item { modify_speed: None });

        items.insert("polkadotbow".to_string(), Item { modify_speed: None });

        items.insert("micleberry".to_string(), Item { modify_speed: None });

        items.insert("tr66".to_string(), Item { modify_speed: None });

        items.insert("tr55".to_string(), Item { modify_speed: None });

        items.insert("metagrossite".to_string(), Item { modify_speed: None });

        items.insert("laggingtail".to_string(), Item { modify_speed: None });

        items.insert("rarebone".to_string(), Item { modify_speed: None });

        items.insert("watermemory".to_string(), Item { modify_speed: None });

        items.insert("terrainextender".to_string(), Item { modify_speed: None });

        items.insert("wacanberry".to_string(), Item { modify_speed: None });

        items.insert("icememory".to_string(), Item { modify_speed: None });

        items.insert("solganiumz".to_string(), Item { modify_speed: None });

        items.insert("berserkgene".to_string(), Item { modify_speed: None });

        items.insert("kelpsyberry".to_string(), Item { modify_speed: None });

        items.insert("marangaberry".to_string(), Item { modify_speed: None });

        items.insert("tr85".to_string(), Item { modify_speed: None });

        items.insert("flameorb".to_string(), Item { modify_speed: None });

        items.insert("griseousorb".to_string(), Item { modify_speed: None });

        items.insert("keeberry".to_string(), Item { modify_speed: None });

        items.insert("mindplate".to_string(), Item { modify_speed: None });

        items.insert("steelmemory".to_string(), Item { modify_speed: None });

        items.insert("tr68".to_string(), Item { modify_speed: None });

        items.insert("tr69".to_string(), Item { modify_speed: None });

        items.insert("aloraichiumz".to_string(), Item { modify_speed: None });

        items.insert("healball".to_string(), Item { modify_speed: None });

        items.insert("tr67".to_string(), Item { modify_speed: None });

        items.insert("magostberry".to_string(), Item { modify_speed: None });

        items.insert("tr06".to_string(), Item { modify_speed: None });

        items.insert("tr40".to_string(), Item { modify_speed: None });

        items.insert("chilldrive".to_string(), Item { modify_speed: None });

        items.insert("latiasite".to_string(), Item { modify_speed: None });

        items.insert("wiseglasses".to_string(), Item { modify_speed: None });

        items.insert("masterball".to_string(), Item { modify_speed: None });

        items.insert("persimberry".to_string(), Item { modify_speed: None });

        items.insert("tamatoberry".to_string(), Item { modify_speed: None });

        items.insert("tr65".to_string(), Item { modify_speed: None });

        items.insert("hardstone".to_string(), Item { modify_speed: None });

        items.insert("chippedpot".to_string(), Item { modify_speed: None });

        items.insert("kingsrock".to_string(), Item { modify_speed: None });

        items.insert("tr35".to_string(), Item { modify_speed: None });

        items.insert("inciniumz".to_string(), Item { modify_speed: None });

        items.insert("beastball".to_string(), Item { modify_speed: None });

        items.insert("adamantorb".to_string(), Item { modify_speed: None });

        items.insert("firestone".to_string(), Item { modify_speed: None });

        items.insert("tr91".to_string(), Item { modify_speed: None });

        items.insert("nanabberry".to_string(), Item { modify_speed: None });

        items.insert("stoneplate".to_string(), Item { modify_speed: None });

        items.insert("lureball".to_string(), Item { modify_speed: None });

        items.insert("tr87".to_string(), Item { modify_speed: None });

        items.insert("smoothrock".to_string(), Item { modify_speed: None });

        items.insert("liechiberry".to_string(), Item { modify_speed: None });

        items.insert("roomservice".to_string(), Item { modify_speed: None });

        items.insert("mimikiumz".to_string(), Item { modify_speed: None });

        items.insert("mewniumz".to_string(), Item { modify_speed: None });

        items.insert("lifeorb".to_string(), Item { modify_speed: None });

        items.insert("garchompite".to_string(), Item { modify_speed: None });

        items.insert("powerlens".to_string(), Item { modify_speed: None });

        items.insert("tr20".to_string(), Item { modify_speed: None });

        items.insert("assaultvest".to_string(), Item { modify_speed: None });

        items.insert("tr70".to_string(), Item { modify_speed: None });

        items.insert("moonball".to_string(), Item { modify_speed: None });

        items.insert("tr03".to_string(), Item { modify_speed: None });

        items.insert("tr11".to_string(), Item { modify_speed: None });

        items.insert("electriumz".to_string(), Item { modify_speed: None });

        items.insert("flowersweet".to_string(), Item { modify_speed: None });

        items.insert("magoberry".to_string(), Item { modify_speed: None });

        items.insert("rustedshield".to_string(), Item { modify_speed: None });

        items.insert("splashplate".to_string(), Item { modify_speed: None });

        items.insert("primariumz".to_string(), Item { modify_speed: None });

        items.insert("gripclaw".to_string(), Item { modify_speed: None });

        items.insert("rockyhelmet".to_string(), Item { modify_speed: None });

        items.insert("lansatberry".to_string(), Item { modify_speed: None });

        items.insert("ultraball".to_string(), Item { modify_speed: None });

        items.insert("normalgem".to_string(), Item { modify_speed: None });

        items.insert("leek".to_string(), Item { modify_speed: None });

        items.insert("toxicplate".to_string(), Item { modify_speed: None });

        items.insert("belueberry".to_string(), Item { modify_speed: None });

        items.insert("safetygoggles".to_string(), Item { modify_speed: None });

        items.insert("pixieplate".to_string(), Item { modify_speed: None });

        items.insert("ampharosite".to_string(), Item { modify_speed: None });

        items.insert("galaricacuff".to_string(), Item { modify_speed: None });

        items.insert("tr44".to_string(), Item { modify_speed: None });

        items.insert("spelltag".to_string(), Item { modify_speed: None });

        items.insert("galladite".to_string(), Item { modify_speed: None });

        items.insert("dracoplate".to_string(), Item { modify_speed: None });

        items.insert("protector".to_string(), Item { modify_speed: None });

        items.insert("watmelberry".to_string(), Item { modify_speed: None });

        items.insert("poisongem".to_string(), Item { modify_speed: None });

        items.insert("dragonfang".to_string(), Item { modify_speed: None });

        items.insert("tr02".to_string(), Item { modify_speed: None });

        items.insert("heatrock".to_string(), Item { modify_speed: None });

        items.insert("tr52".to_string(), Item { modify_speed: None });

        items.insert("custapberry".to_string(), Item { modify_speed: None });

        items.insert("protectivepads".to_string(), Item { modify_speed: None });

        items.insert("duskball".to_string(), Item { modify_speed: None });

        items.insert("crackedpot".to_string(), Item { modify_speed: None });

        items.insert("aspearberry".to_string(), Item { modify_speed: None });

        items.insert("bindingband".to_string(), Item { modify_speed: None });

        items.insert("charcoal".to_string(), Item { modify_speed: None });

        items.insert("earthplate".to_string(), Item { modify_speed: None });

        items.insert("wateriumz".to_string(), Item { modify_speed: None });

        items.insert("magnet".to_string(), Item { modify_speed: None });

        items.insert("cellbattery".to_string(), Item { modify_speed: None });

        items.insert("galaricawreath".to_string(), Item { modify_speed: None });

        items.insert("grassmemory".to_string(), Item { modify_speed: None });

        items.insert("laxincense".to_string(), Item { modify_speed: None });

        items.insert("rockiumz".to_string(), Item { modify_speed: None });

        items.insert("dragonscale".to_string(), Item { modify_speed: None });

        items.insert("dreadplate".to_string(), Item { modify_speed: None });

        items.insert("banettite".to_string(), Item { modify_speed: None });

        items.insert("damprock".to_string(), Item { modify_speed: None });

        items.insert("tr86".to_string(), Item { modify_speed: None });

        items.insert("tr59".to_string(), Item { modify_speed: None });

        items.insert("tr50".to_string(), Item { modify_speed: None });

        items.insert("zapplate".to_string(), Item { modify_speed: None });

        items.insert("softsand".to_string(), Item { modify_speed: None });

        items.insert("icegem".to_string(), Item { modify_speed: None });

        items.insert("oldamber".to_string(), Item { modify_speed: None });

        items.insert("fullincense".to_string(), Item { modify_speed: None });

        items.insert("pokeball".to_string(), Item { modify_speed: None });

        items.insert("fossilizedfish".to_string(), Item { modify_speed: None });

        items.insert("weaknesspolicy".to_string(), Item { modify_speed: None });

        items.insert("mewtwonitex".to_string(), Item { modify_speed: None });

        items.insert("tr84".to_string(), Item { modify_speed: None });

        items.insert("burntberry".to_string(), Item { modify_speed: None });

        items.insert("domefossil".to_string(), Item { modify_speed: None });

        items.insert("kommoniumz".to_string(), Item { modify_speed: None });

        items.insert("eviolite".to_string(), Item { modify_speed: None });

        items.insert("mysticwater".to_string(), Item { modify_speed: None });

        items.insert("burndrive".to_string(), Item { modify_speed: None });

        items.insert("stickybarb".to_string(), Item { modify_speed: None });

        items.insert("blackbelt".to_string(), Item { modify_speed: None });

        items.insert("plumefossil".to_string(), Item { modify_speed: None });

        items.insert("gardevoirite".to_string(), Item { modify_speed: None });

        items.insert("alakazite".to_string(), Item { modify_speed: None });

        items.insert("flyiniumz".to_string(), Item { modify_speed: None });

        items.insert("ghostmemory".to_string(), Item { modify_speed: None });

        items.insert("tr29".to_string(), Item { modify_speed: None });

        items.insert("psychicseed".to_string(), Item { modify_speed: None });

        items.insert("lopunnite".to_string(), Item { modify_speed: None });

        items.insert("pamtreberry".to_string(), Item { modify_speed: None });

        items.insert("silkscarf".to_string(), Item { modify_speed: None });

        items.insert("jawfossil".to_string(), Item { modify_speed: None });

        items.insert("rockincense".to_string(), Item { modify_speed: None });

        items.insert("tr74".to_string(), Item { modify_speed: None });

        items.insert("clawfossil".to_string(), Item { modify_speed: None });

        items.insert("tr94".to_string(), Item { modify_speed: None });

        items.insert("focussash".to_string(), Item { modify_speed: None });

        items.insert("metronome".to_string(), Item { modify_speed: None });

        items.insert("powerweight".to_string(), Item { modify_speed: None });

        items.insert("cornnberry".to_string(), Item { modify_speed: None });

        items.insert("grassgem".to_string(), Item { modify_speed: None });

        items.insert("eeviumz".to_string(), Item { modify_speed: None });

        items.insert("yacheberry".to_string(), Item { modify_speed: None });

        items.insert("steelgem".to_string(), Item { modify_speed: None });

        items.insert("chartiberry".to_string(), Item { modify_speed: None });

        items.insert("choiceband".to_string(), Item { modify_speed: None });

        items.insert("tr56".to_string(), Item { modify_speed: None });

        items.insert("heavyball".to_string(), Item { modify_speed: None });

        items.insert("ghostiumz".to_string(), Item { modify_speed: None });

        items.insert("quickball".to_string(), Item { modify_speed: None });

        items.insert("goldbottlecap".to_string(), Item { modify_speed: None });

        items.insert("petayaberry".to_string(), Item { modify_speed: None });

        items.insert("berry".to_string(), Item { modify_speed: None });

        items.insert("aggronite".to_string(), Item { modify_speed: None });

        items.insert("tr05".to_string(), Item { modify_speed: None });

        items.insert("tangaberry".to_string(), Item { modify_speed: None });

        items.insert("sablenite".to_string(), Item { modify_speed: None });

        items.insert("skyplate".to_string(), Item { modify_speed: None });

        items.insert("greatball".to_string(), Item { modify_speed: None });

        items.insert("mysteryberry".to_string(), Item { modify_speed: None });

        items.insert("glalitite".to_string(), Item { modify_speed: None });

        items.insert("shockdrive".to_string(), Item { modify_speed: None });

        items.insert("ovalstone".to_string(), Item { modify_speed: None });

        items.insert("groundmemory".to_string(), Item { modify_speed: None });

        items.insert("tr71".to_string(), Item { modify_speed: None });

        items.insert("moonstone".to_string(), Item { modify_speed: None });

        items.insert("tr13".to_string(), Item { modify_speed: None });

        items.insert("rabutaberry".to_string(), Item { modify_speed: None });

        items.insert("sharpbeak".to_string(), Item { modify_speed: None });

        items.insert("lycaniumz".to_string(), Item { modify_speed: None });

        items.insert("nevermeltice".to_string(), Item { modify_speed: None });

        items.insert("ejectbutton".to_string(), Item { modify_speed: None });

        items.insert("tr75".to_string(), Item { modify_speed: None });

        items.insert("tr80".to_string(), Item { modify_speed: None });

        items.insert("fastball".to_string(), Item { modify_speed: None });

        items.insert("lovesweet".to_string(), Item { modify_speed: None });

        items.insert("diveball".to_string(), Item { modify_speed: None });

        items.insert("rawstberry".to_string(), Item { modify_speed: None });

        items.insert("ejectpack".to_string(), Item { modify_speed: None });

        items.insert("dragongem".to_string(), Item { modify_speed: None });

        items.insert("tr28".to_string(), Item { modify_speed: None });

        items.insert("swampertite".to_string(), Item { modify_speed: None });

        items.insert("prismscale".to_string(), Item { modify_speed: None });

        items.insert("tr51".to_string(), Item { modify_speed: None });

        items.insert("icicleplate".to_string(), Item { modify_speed: None });

        items.insert("oddincense".to_string(), Item { modify_speed: None });

        items.insert("tapuniumz".to_string(), Item { modify_speed: None });

        items.insert("wikiberry".to_string(), Item { modify_speed: None });

        items.insert("tr12".to_string(), Item { modify_speed: None });

        items.insert("tr60".to_string(), Item { modify_speed: None });

        items.insert("razzberry".to_string(), Item { modify_speed: None });

        items.insert("diancite".to_string(), Item { modify_speed: None });

        items.insert("colburberry".to_string(), Item { modify_speed: None });

        items.insert("houndoominite".to_string(), Item { modify_speed: None });

        items.insert("blackglasses".to_string(), Item { modify_speed: None });

        items.insert("tr31".to_string(), Item { modify_speed: None });

        items.insert("psychicmemory".to_string(), Item { modify_speed: None });

        items.insert("pomegberry".to_string(), Item { modify_speed: None });

        items.insert("goldberry".to_string(), Item { modify_speed: None });

        items.insert("tr83".to_string(), Item { modify_speed: None });

        items.insert("normaliumz".to_string(), Item { modify_speed: None });

        items.insert("starsweet".to_string(), Item { modify_speed: None });

        items.insert("tr04".to_string(), Item { modify_speed: None });

        items.insert("mentalherb".to_string(), Item { modify_speed: None });

        items.insert("powerbelt".to_string(), Item { modify_speed: None });

        items.insert("deepseatooth".to_string(), Item { modify_speed: None });

        items.insert("gyaradosite".to_string(), Item { modify_speed: None });

        items.insert("snorliumz".to_string(), Item { modify_speed: None });

        items.insert("tr82".to_string(), Item { modify_speed: None });

        items.insert("tr98".to_string(), Item { modify_speed: None });

        items.insert("ringtarget".to_string(), Item { modify_speed: None });

        items.insert("leftovers".to_string(), Item { modify_speed: None });

        items.insert("tr41".to_string(), Item { modify_speed: None });

        items.insert("icyrock".to_string(), Item { modify_speed: None });

        items.insert("toxicorb".to_string(), Item { modify_speed: None });

        items.insert("heracronite".to_string(), Item { modify_speed: None });

        items.insert("latiosite".to_string(), Item { modify_speed: None });

        items.insert("tr32".to_string(), Item { modify_speed: None });

        items.insert("flyinggem".to_string(), Item { modify_speed: None });

        items.insert("abomasite".to_string(), Item { modify_speed: None });

        items.insert("lumberry".to_string(), Item { modify_speed: None });

        items.insert("sceptilite".to_string(), Item { modify_speed: None });

        items.insert("occaberry".to_string(), Item { modify_speed: None });

        items.insert("leafstone".to_string(), Item { modify_speed: None });

        items.insert("fossilizedbird".to_string(), Item { modify_speed: None });

        items.insert("flameplate".to_string(), Item { modify_speed: None });

        items.insert("steeliumz".to_string(), Item { modify_speed: None });

        items.insert("tr64".to_string(), Item { modify_speed: None });

        items.insert("utilityumbrella".to_string(), Item { modify_speed: None });

        items.insert("tr08".to_string(), Item { modify_speed: None });

        items.insert("iceberry".to_string(), Item { modify_speed: None });

        items.insert("thickclub".to_string(), Item { modify_speed: None });

        items.insert("tr18".to_string(), Item { modify_speed: None });

        items.insert("redcard".to_string(), Item { modify_speed: None });

        items.insert("tr97".to_string(), Item { modify_speed: None });

        items.insert("grepaberry".to_string(), Item { modify_speed: None });

        items.insert("kangaskhanite".to_string(), Item { modify_speed: None });

        items.insert("repeatball".to_string(), Item { modify_speed: None });

        items.insert("tr38".to_string(), Item { modify_speed: None });

        items.insert("ironball".to_string(), Item { modify_speed: None });

        items.insert("none".to_string(), Item { modify_speed: None });

        items
    };
}

pub fn get_item(ability_name: &str) -> &'static Item {
    return ITEMS
        .get(ability_name)
        .unwrap_or_else(|| panic!("Could not get item {}", ability_name));
}

pub struct Item {
    pub modify_speed: Option<ModifySpeedFn>,
}
