use std::ops::RangeTo;

use ark_bls12_381::Fq;
use ark_ff::{PrimeField, Zero};

type F = Fq;

#[derive(Clone, Debug)]
pub struct PoseidonConfig<F: PrimeField> {
    /// Number of rounds in a full-round operation.
    pub full_rounds: usize,
    /// Number of rounds in a partial-round operation.
    pub partial_rounds: usize,
    /// Exponent used in S-boxes.
    pub alpha: u64,
    /// Additive Round keys. These are added before each MDS matrix application to make it an affine shift.
    /// They are indexed by `ark[round_num][state_element_index]`
    pub ark: &'static [[F; 3]],
    /// Maximally Distance Separating (MDS) Matrix.
    pub mds: &'static [[F; 3]],
    /// The rate (in terms of number of field elements).
    /// See [On the Indifferentiability of the Sponge Construction](https://iacr.org/archive/eurocrypt2008/49650180/49650180.pdf)
    /// for more details on the rate and capacity of a sponge.
    pub rate: usize,
    /// The capacity (in terms of number of field elements).
    pub capacity: usize,
}

const MDS: &'static [[F; 3]] = &[
    [
        ark_ff::MontFp!(
            "43228725308391137369947362226390319299014033584574058394339561338097152657858"
        ),
        ark_ff::MontFp!(
            "20729134655727743386784826341366384914431326428651109729494295849276339718592"
        ),
        ark_ff::MontFp!(
            "14275792724825301816674509766636153429127896752891673527373812580216824074377"
        ),
    ],
    [
        ark_ff::MontFp!(
            "3039440043015681380498693766234886011876841428799441709991632635031851609481"
        ),
        ark_ff::MontFp!(
            "6678863357926068615342013496680930722082156498064457711885464611323928471101"
        ),
        ark_ff::MontFp!(
            "37355038393562575053091209735467454314247378274125943833499651442997254948957"
        ),
    ],
    [
        ark_ff::MontFp!(
            "26481612700543967643159862864328231943993263806649000633819754663276818191580"
        ),
        ark_ff::MontFp!(
            "30103264397473155564098369644643015994024192377175707604277831692111219371047"
        ),
        ark_ff::MontFp!(
            "5712721806190262694719203887224391960978962995663881615739647362444059585747"
        ),
    ],
];
const ARK: &'static [[F; 3]] = &[
    [
        ark_ff::MontFp!(
            "44595993092652566245296379427906271087754779418564084732265552598173323099784"
        ),
        ark_ff::MontFp!(
            "23298463296221002559050231199021122673158929708101049474262017406235785365706"
        ),
        ark_ff::MontFp!(
            "34212491019164671611180318500074499609633402631511849759183986060951187784466"
        ),
    ],
    [
        ark_ff::MontFp!(
            "19098051134080182375553680073525644187968170656591203562523489333616681350367"
        ),
        ark_ff::MontFp!(
            "7027675418691353855077049716619550622043312043660992344940177187528247727783"
        ),
        ark_ff::MontFp!(
            "47642753235356257928619065424282314733361764347085604019867862722762702755609"
        ),
    ],
    [
        ark_ff::MontFp!(
            "24281836129477728386327945482863886685457469794572168729834072693507088619997"
        ),
        ark_ff::MontFp!(
            "12624893078331920791384400430193929292743809612452779381349824703573823883410"
        ),
        ark_ff::MontFp!(
            "22654862987689323504199204643771547606936339944127455903448909090318619188561"
        ),
    ],
    [
        ark_ff::MontFp!(
            "27229172992560143399715985732065737093562061782414043625359531774550940662372"
        ),
        ark_ff::MontFp!(
            "13224952063922250960936823741448973692264041750100990569445192064567307041002"
        ),
        ark_ff::MontFp!(
            "40380869235216625717296601204704413215735530626882135230693823362552484855508"
        ),
    ],
    [
        ark_ff::MontFp!(
            "4245751157938905689397184705633683893932492370323323780371834663438472308145"
        ),
        ark_ff::MontFp!(
            "8252156875535418429533049587170755750275631534314711502253775796882240991261"
        ),
        ark_ff::MontFp!(
            "32910829712934971129644416249914075073083903821282503505466324428991624789936"
        ),
    ],
    [
        ark_ff::MontFp!(
            "49412601297460128335642438246716127241669915737656789613664349252868389975962"
        ),
        ark_ff::MontFp!(
            "841661305510340459373323516098909074520942972558284146843779636353111592117"
        ),
        ark_ff::MontFp!(
            "37926489020263024391336570420006226544461516787280929232555625742588667303947"
        ),
    ],
    [
        ark_ff::MontFp!(
            "18433043696013996573551852847056868761017170818820490351056924728720017242180"
        ),
        ark_ff::MontFp!(
            "45376910275288438312773930242803223482318753992595269901397542214841496212310"
        ),
        ark_ff::MontFp!(
            "47854349410014339708332226068958253098964727682486278458389508597930796651514"
        ),
    ],
    [
        ark_ff::MontFp!(
            "32638426693771251366613055506166587312642876874690861030672730491779486904360"
        ),
        ark_ff::MontFp!(
            "19105439281696418043426755774110765432959446684037017837894045255490581318047"
        ),
        ark_ff::MontFp!(
            "13484299981373196201166722380389594773562113262309564134825386266765751213853"
        ),
    ],
    [
        ark_ff::MontFp!(
            "63360321133852659797114062808297090090814531427710842859827725871241144161"
        ),
        ark_ff::MontFp!(
            "42427543035537409467993338717379268954936885184662765745740070438835506287271"
        ),
        ark_ff::MontFp!(
            "149101987103211771991327927827692640556911620408176100290586418839323044234"
        ),
    ],
    [
        ark_ff::MontFp!(
            "8341764062226826803887898710015561861526081583071950015446833446251359696930"
        ),
        ark_ff::MontFp!(
            "45635980415044299013530304465786867101223925975971912073759959440335364441441"
        ),
        ark_ff::MontFp!(
            "49833261156201520743834327917353893365097424877680239796845398698940689734850"
        ),
    ],
    [
        ark_ff::MontFp!(
            "26764715016591436228000634284249890185894507497739511725029482580508707525029"
        ),
        ark_ff::MontFp!(
            "25054530812095491217523557726611612265064441619646263299990388543372685322499"
        ),
        ark_ff::MontFp!(
            "47654590955096246997622155031169641628093104787883934397920286718814889326452"
        ),
    ],
    [
        ark_ff::MontFp!(
            "16463825890556752307085325855351334996898686633642574805918056141310194135796"
        ),
        ark_ff::MontFp!(
            "17473961341633494489168064889016732306117097771640351649096482400214968053040"
        ),
        ark_ff::MontFp!(
            "49914603434867854893558366922996753035832008639512305549839666311012232077468"
        ),
    ],
    [
        ark_ff::MontFp!(
            "17122578514152308432111470949473865420090463026624297565504381163777697818362"
        ),
        ark_ff::MontFp!(
            "34870689836420861427379101859113225049736283485335674111421609473028315711541"
        ),
        ark_ff::MontFp!(
            "4622082908476410083286670201138165773322781640914243047922441301693321472984"
        ),
    ],
    [
        ark_ff::MontFp!(
            "6079244375752010013798561155333454682564824861645642293573415833483620500976"
        ),
        ark_ff::MontFp!(
            "2635090520059500019661864086615522409798872905401305311748231832709078452746"
        ),
        ark_ff::MontFp!(
            "19070766579582338321241892986615538320421651429118757507174186491084617237586"
        ),
    ],
    [
        ark_ff::MontFp!(
            "12622420533971517050761060317049369208980632120901481436392835424625664738526"
        ),
        ark_ff::MontFp!(
            "8965101225657199137904506150282256568170501907667138404080397024857524386266"
        ),
        ark_ff::MontFp!(
            "27085091008069524593196374148553176565775450537072498305327481366756159319838"
        ),
    ],
    [
        ark_ff::MontFp!(
            "45929056591150668409624595495643698205830429971690813312608217341940499221218"
        ),
        ark_ff::MontFp!(
            "50361689160518167880500080025023064746137161030119436080957023803101861300846"
        ),
        ark_ff::MontFp!(
            "6722586346537620732668048024627882970582133613352245923413730968378696371065"
        ),
    ],
    [
        ark_ff::MontFp!(
            "7340485916200743279276570085958556798507770452421357119145466906520506506342"
        ),
        ark_ff::MontFp!(
            "25946733168219652706630789514519162148860502996914241011500280690204368174083"
        ),
        ark_ff::MontFp!(
            "9962367658743163006517635070396368828381757404628822422306438427554934645464"
        ),
    ],
    [
        ark_ff::MontFp!(
            "7221669722700687417346373353960536661883467014204005276831020252277657076044"
        ),
        ark_ff::MontFp!(
            "21487980358388383563030903293359140836304488103090321183948009095669344637431"
        ),
        ark_ff::MontFp!(
            "44389482047246878765773958430749333249729101516826571588063797358040130313157"
        ),
    ],
    [
        ark_ff::MontFp!(
            "32887270862917330820874162842519225370447850172085449103568878409533683733185"
        ),
        ark_ff::MontFp!(
            "15453393396765207016379045014101989306173462885430532298601655955681532648226"
        ),
        ark_ff::MontFp!(
            "5478929644476681096437469958231489102974161353940993351588559414552523375472"
        ),
    ],
    [
        ark_ff::MontFp!(
            "41981370411247590312677561209178363054744730805951096631186178388981705304138"
        ),
        ark_ff::MontFp!(
            "3474136981645476955784428843999869229067282976757744542648188369810577298585"
        ),
        ark_ff::MontFp!(
            "26251477770740399889956219915654371915771248171098220204692699710414817081869"
        ),
    ],
    [
        ark_ff::MontFp!(
            "51916561889718854106125837319509539220778634838409949714061033196765117231752"
        ),
        ark_ff::MontFp!(
            "25355145802812435959748831835587713214179184608408449220418373832038339021974"
        ),
        ark_ff::MontFp!(
            "31950684570730625275416731570246297947385359051792335826965013637877068017530"
        ),
    ],
    [
        ark_ff::MontFp!(
            "40966378914980473680181850710703295982197782082391794594149984057481543436879"
        ),
        ark_ff::MontFp!(
            "1141315130963422417761731263662398620858625339733452795772225916965481730059"
        ),
        ark_ff::MontFp!(
            "9812100862165422922235757591915383485338044715409891361026651619010947646011"
        ),
    ],
    [
        ark_ff::MontFp!(
            "25276091996614379065765602410190790163396484122487585763380676888280427744737"
        ),
        ark_ff::MontFp!(
            "18512694312063606403196469408971540495273694846641903978723927656359350642619"
        ),
        ark_ff::MontFp!(
            "5791584766415439694303685437881192048262049244830616851865505314899699012588"
        ),
    ],
    [
        ark_ff::MontFp!(
            "34501536331706470927069149344450300773777486993504673779438188495686129846168"
        ),
        ark_ff::MontFp!(
            "10797737565565774079718466476236831116206064650762676383469703413649447678207"
        ),
        ark_ff::MontFp!(
            "42599392747310354323136214835734307933597896695637215127297036595538235868368"
        ),
    ],
    [
        ark_ff::MontFp!(
            "1336670998775417133322626564820911986969949054454812685145275612519924150700"
        ),
        ark_ff::MontFp!(
            "2630141283339761901081411552890260088516693208402906795133548756078952896770"
        ),
        ark_ff::MontFp!(
            "5206688943117414740600380377278238268309952400341418217132724749372435975215"
        ),
    ],
    [
        ark_ff::MontFp!(
            "10739264253827005683370721104077252560524362323422172665530191908848354339715"
        ),
        ark_ff::MontFp!(
            "48010640624945719826344492755710886355389194986527731603685956726907395779674"
        ),
        ark_ff::MontFp!(
            "47880724693177306044229143357252697148359033158394459365791331000715957339701"
        ),
    ],
    [
        ark_ff::MontFp!(
            "51658938856669444737833983076793759752280196674149218924101718974926964118996"
        ),
        ark_ff::MontFp!(
            "27558055650076329657496888512074319504342606463881203707330358472954748913263"
        ),
        ark_ff::MontFp!(
            "38886981777859313701520424626728402175860609948757992393598285291689196608037"
        ),
    ],
    [
        ark_ff::MontFp!(
            "17152756165118461969542990684402410297675979513690903033350206658079448802479"
        ),
        ark_ff::MontFp!(
            "43766946932033687220387514221943418338304186408056458476301583041390483707207"
        ),
        ark_ff::MontFp!(
            "24324495647041812436929170644873622904287038078113808264580396461953421400343"
        ),
    ],
    [
        ark_ff::MontFp!(
            "6935839211798937659784055008131602708847374430164859822530563797964932598700"
        ),
        ark_ff::MontFp!(
            "42126767398190942911395299419182514513368023621144776598842282267908712110039"
        ),
        ark_ff::MontFp!(
            "5702364486091252903915715761606014714345316580946072019346660327857498603375"
        ),
    ],
    [
        ark_ff::MontFp!(
            "28184981699552917714085740963279595942132561155181044254318202220270242523053"
        ),
        ark_ff::MontFp!(
            "27078204494010940048327822707224393686245007379331357330801926151074766130790"
        ),
        ark_ff::MontFp!(
            "5004172841233947987988267535285080365124079140142987718231874743202918551203"
        ),
    ],
    [
        ark_ff::MontFp!(
            "7974360962120296064882769128577382489451060235999590492215336103105134345602"
        ),
        ark_ff::MontFp!(
            "48062035869818179910046292951628308709251170031813126950740044942870578526376"
        ),
        ark_ff::MontFp!(
            "26361151154829600651603985995297072258262605598910254660032612019129606811983"
        ),
    ],
    [
        ark_ff::MontFp!(
            "46973867849986280770641828877435510444176572688208439836496241838832695841519"
        ),
        ark_ff::MontFp!(
            "1219439673853113792340300173186247996249367102884530407862469123523013083971"
        ),
        ark_ff::MontFp!(
            "8063356002935671186275773257019749639571745240775941450161086349727882957042"
        ),
    ],
    [
        ark_ff::MontFp!(
            "8815571992701260640209942886673939234666734294275300852283020522390608544536"
        ),
        ark_ff::MontFp!(
            "36384568984671043678320545346945893232044626942887414733675890845013312931948"
        ),
        ark_ff::MontFp!(
            "7493936589040764830842760521372106574503511314427857201860148571929278344956"
        ),
    ],
    [
        ark_ff::MontFp!(
            "26516538878265871822073279450474977673130300973488209984756372331392531193948"
        ),
        ark_ff::MontFp!(
            "3872858659373466814413243601289105962248870842202907364656526273784217311104"
        ),
        ark_ff::MontFp!(
            "8291822807524000248589997648893671538524566700364221355689839490238724479848"
        ),
    ],
    [
        ark_ff::MontFp!(
            "32842548776827046388198955038089826231531188946525483251252938248379132381248"
        ),
        ark_ff::MontFp!(
            "10749428410907700061565796335489079278748501945557710351216806276547834974736"
        ),
        ark_ff::MontFp!(
            "43342287917341177925402357903832370099402579088513884654598017447701677948416"
        ),
    ],
    [
        ark_ff::MontFp!(
            "29658571352070370791360499299098360881857072189358092237807807261478461425147"
        ),
        ark_ff::MontFp!(
            "7805182565862454238315452208989152534554369855020544477885853141626690738363"
        ),
        ark_ff::MontFp!(
            "30699555847500141715826240743138908521140760599479365867708690318477369178275"
        ),
    ],
    [
        ark_ff::MontFp!(
            "1231951350103545216624376889222508148537733140742167414518514908719103925687"
        ),
        ark_ff::MontFp!(
            "24784260089125933876714702247471508077514206350883487938806451152907502751770"
        ),
        ark_ff::MontFp!(
            "36563542611079418454711392295126742705798573252480028863133394504154697924536"
        ),
    ],
];

/// Generate default parameters (bls381-fr-only) for alpha = 17, state-size = 8
pub(crate) const fn poseidon_parameters_for_test() -> PoseidonConfig<F> {
    let alpha = 17;
    let full_rounds = 8;
    let total_rounds = 37;
    let partial_rounds = total_rounds - full_rounds;
    let capacity = 1;
    let rate = 2;
    PoseidonConfig {
        full_rounds,
        partial_rounds,
        alpha,
        ark: ARK,
        mds: MDS,
        rate,
        capacity,
    }
}

#[derive(Clone)]
pub struct PoseidonSponge<F: PrimeField> {
    /// Sponge Config
    pub parameters: PoseidonConfig<F>,

    // Sponge State
    /// Current sponge's state (current elements in the permutation block)
    pub state: Vec<F>,
}

impl<F: PrimeField> PoseidonSponge<F> {
    fn apply_s_box(&self, state: &mut [F], is_full_round: bool) {
        // Full rounds apply the S Box (x^alpha) to every element of state
        if is_full_round {
            for elem in state {
                *elem = elem.pow(&[self.parameters.alpha]);
            }
        }
        // Partial rounds apply the S Box (x^alpha) to just the first element of state
        else {
            state[0] = state[0].pow(&[self.parameters.alpha]);
        }
    }

    fn apply_ark(&self, state: &mut [F], round_number: usize) {
        for (i, state_elem) in state.iter_mut().enumerate() {
            state_elem.add_assign(&self.parameters.ark[round_number][i]);
        }
    }

    fn apply_mds(&self, state: &mut [F]) {
        let mut new_state = Vec::new();
        for i in 0..state.len() {
            let mut cur = F::zero();
            for (j, state_elem) in state.iter().enumerate() {
                let term = state_elem.mul(&self.parameters.mds[i][j]);
                cur.add_assign(&term);
            }
            new_state.push(cur);
        }
        state.clone_from_slice(&new_state[..state.len()])
    }

    fn permute(&mut self) {
        let full_rounds_over_2 = self.parameters.full_rounds / 2;
        let mut state = self.state.clone();
        for i in 0..full_rounds_over_2 {
            self.apply_ark(&mut state, i);
            self.apply_s_box(&mut state, true);
            self.apply_mds(&mut state);
        }

        for i in full_rounds_over_2..(full_rounds_over_2 + self.parameters.partial_rounds) {
            self.apply_ark(&mut state, i);
            self.apply_s_box(&mut state, false);
            self.apply_mds(&mut state);
        }

        for i in (full_rounds_over_2 + self.parameters.partial_rounds)
            ..(self.parameters.partial_rounds + self.parameters.full_rounds)
        {
            self.apply_ark(&mut state, i);
            self.apply_s_box(&mut state, true);
            self.apply_mds(&mut state);
        }
        self.state = state;
    }
}

const BLS12381POSEIDON_CONF: PoseidonConfig<F> = poseidon_parameters_for_test();

#[derive(Clone)]
pub struct Bls12381Poseidon(PoseidonSponge<F>);

impl Default for Bls12381Poseidon {
    fn default() -> Self {
        Self(PoseidonSponge {
            parameters: BLS12381POSEIDON_CONF.clone(),
            state: vec![F::zero(); BLS12381POSEIDON_CONF.rate + BLS12381POSEIDON_CONF.capacity],
        })
    }
}

impl std::ops::Index<usize> for Bls12381Poseidon {
    type Output = F;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0.state[index]
    }
}

impl std::ops::IndexMut<usize> for Bls12381Poseidon {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0.state[index]
    }
}

impl std::ops::Index<RangeTo<usize>> for Bls12381Poseidon {
    type Output = [F];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.0.state[index]
    }
}

impl std::ops::IndexMut<RangeTo<usize>> for Bls12381Poseidon {
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {
        &mut self.0.state[index]
    }
}

impl std::ops::Index<std::ops::Range<usize>> for Bls12381Poseidon {
    type Output = [F];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.0.state[index]
    }
}

impl std::ops::IndexMut<std::ops::Range<usize>> for Bls12381Poseidon {
    fn index_mut(&mut self, index: std::ops::Range<usize>) -> &mut Self::Output {
        &mut self.0.state[index]
    }
}

impl std::ops::Index<std::ops::RangeFrom<usize>> for Bls12381Poseidon {
    type Output = [F];

    fn index(&self, index: std::ops::RangeFrom<usize>) -> &Self::Output {
        &self.0.state[index]
    }
}

impl std::ops::IndexMut<std::ops::RangeFrom<usize>> for Bls12381Poseidon {
    fn index_mut(&mut self, index: std::ops::RangeFrom<usize>) -> &mut Self::Output {
        &mut self.0.state[index]
    }
}

impl zeroize::Zeroize for Bls12381Poseidon {
    fn zeroize(&mut self) {
        self.0.state.zeroize();
    }
}

impl crate::hash::sponge::Sponge for Bls12381Poseidon {
    type U = F;

    const CAPACITY: usize = 1;

    const RATE: usize = 2;

    fn new(iv: [u8; 32]) -> Self {
        assert!(Self::CAPACITY >= 1);
        let mut ark_sponge = Self::default();
        ark_sponge.0.state[Self::RATE] = F::from_be_bytes_mod_order(&iv);
        ark_sponge
    }

    fn permute(&mut self) {
        self.0.permute();
    }
}

pub type PoseidonHash = crate::hash::sponge::DuplexSponge<Bls12381Poseidon>;
