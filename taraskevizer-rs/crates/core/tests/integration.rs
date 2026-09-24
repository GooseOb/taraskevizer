use taraskevizer_core::{
    alphabetic, apply_highlight_diff, config::*, html_config_options, phonetic, tarask,
    HTML_WRAPPERS,
};

fn default_cfg() -> TaraskConfig {
    TaraskConfig::default()
}

fn test_cases(cfg: &TaraskConfig, cases: &[(&str, &str)]) {
    for (input, expected) in cases {
        let result = tarask(input, cfg);
        assert_eq!(&result, expected, "input: {input:?}");
    }
}

fn test_no_change_cases(cfg: &TaraskConfig, cases: &[&str]) {
    for input in cases {
        let result = tarask(input, cfg);
        assert_eq!(&result, input, "input: {input:?}");
    }
}

fn test_phonetic_cases(cfg: &TaraskConfig, cases: &[(&str, &str)]) {
    for (input, expected) in cases {
        let result = phonetic(input, cfg);
        assert_eq!(&result, expected, "input: {input:?}");
    }
}

// --- Wrapper helpers for highlighting / HTML tests ---

fn def_var_no(s: &str) -> String {
    s.strip_prefix('(')
        .and_then(|s| s.split('|').next())
        .map(|s| s.to_string())
        .unwrap_or_else(|| s.to_string())
}

fn def_var_first(s: &str) -> String {
    s.strip_prefix('(')
        .and_then(|s| s.strip_suffix(')'))
        .and_then(|s| s.split('|').nth(1))
        .map(|s| s.to_string())
        .unwrap_or_else(|| s.to_string())
}

// JS highlighting test applies highlightDiff as a SEPARATE step after tarask()
// (which runs with no wrappers). Avoid letter_h wrapper to prevent g-wrapping ґ.
fn highlight_wrappers() -> Wrappers {
    Wrappers {
        fix: Some(|s| format!("[{s}]")),
        letter_h: None,
        variable: VariationWrappers {
            all: |s| s.to_string(),
            first: def_var_first,
            no: def_var_no,
        },
    }
}

// ===== taraskevization change cases =====
// JS: test/cases/taraskevization.ts → change

#[test]
fn test_taraskevization_change() {
    let cfg = default_cfg();
    let cases: &[(&str, &str)] = &[
        ("з медзі", "зь (медзі|сьпіжу)"),
        ("жыццясцвярджальны план", "жыцьцясьцьвярджальны плян"),
        ("Отарыналарынгалагічнае варэнне оталарынголага", "Отарыналярынгалягічнае варэньне оталярынголяга"),
        ("секцыя трыумфу селекцыйных селектараў", "сэкцыя трыюмфу сэлекцыйных сэлектараў"),
        ("эскалатар эскалацыі вайберу", "эскалятар эскаляцыі вайбэру"),
        ("скандый, натрый, натрыю, натрыевы, тэхнецыевы, тэхнецый, паладый, галіевы", "сканд, натар, натру, натравы, тэхнэцавы, тэхнэц, паляд, галевы"),
        ("Веерштрас, Лейбніц, Эбінгаўз/Эбінгхаўз", "Ваерштрас, Ляйбніц, Эбінґгаўз/Эбінґгаўз"),
        ("марыянетка фон Неймана", "марыянэтка фон Ноймана"),
        ("лакацыя алакацыі рэлакантаў", "лякацыя алякацыі рэлякантаў"),
        ("кіслародны каланіяльны баланс", "(кіслародн|тленав)ы каляніяльны балянс"),
        ("фларэнційска-бернардзінскі іанічны іон Тэсея", "флярэнтыйска-бэрнардынскі іянічны іён Тэсэя"),
        (" двухсотгоддзе вэксальных векслераў вёскі Цвярачус ", " двухсотгодзьдзе вэкс(э|а)льных вэксьлераў вёскі (Цьвярачус|Цьверач) "),
        ("залісся і энергія залісся", "за(лісься|лесься|лесьсе) і энэрґія за(лісься|лесься|лесьсе)"),
        ("бугацці сірыус", "буґацьці сырыюс"),
        ("скіпетр вахабіта-джыхадзіста", "скіпэтар вагабіта-джыгадзіста"),
        ("гуслендам", "гуслэндам"),
        ("лен Вермланд", "лэн Вэрмлянд"),
        ("Квін оф Камберленд у Белфасце", "К(ві|ўі)н оф Камбэрлэнд у Бэлфасьце"),
        ("дысемінацыя сейсмічнае актыўнасці нектарынаў", "дысэмінацыя сэйсьмічнае актыўнасьці нэктарынаў"),
        ("перыгелій", "пэрыгелій"),
        ("квест на квесце", "квэст на квэсьце"),
        ("касцях касьцях", "касц(я|ё)х касьцях"),
        ("эксплуатацыя эксплуатуе", "эксп(лё|лю)атацыя эксп(лё|лю)атуе"),
        ("хімічная хімічка i хімік займаюцца хіміяй", "(хэ|хі)мічная (хэ|хі)мічка i (хэ|хі)мік займаюцца (хэ|хі)міяй"),
        ("Блакіраваць блок блогаў бландына Зеўса і феміннай Феміды", "Блякаваць блёк блёгаў бляндына Зэўса і фэміннай Тэміды"),
        ("Абасідскі Халіфат абасідаў", "Абасыдзкі Халіфат абасыдаў"),
        ("Апсіда, адвербіялізацыя, адрэнаміметыкі, азена, акрапетальны, аксерафтол, фенамід, аберацыя, авеста", "Апсыда, адвэрбіялізацыя, адрэнамімэтыкі, азэна, акрапэтальны, аксэрафтол, фэнамід, абэрацыя, авэста"),
        ("аксель, акселерограф, акселятар", "аксэль, аксэлерограф, аксэлятар"),
        ("каталог калегіумаў біялагічных біёлагаў на лоджыі", "каталёг калеґіюмаў біялягічных біёлягаў на лёджыі"),
        ("аналагічны вектар ідэалогіі семіцкіх ідэолагаў асіміляцыі семітаў Аланду", "аналягічны вэктар ідэалёгіі сэміцкіх ідэолягаў асыміляцыі сэмітаў Алянду"),
        ("арнамент на фундаменце", "арнамэнт на фундамэнце"),
        ("медыцынскі медыятар метаінфармуе метанавую камету на камеце", "мэдыцынскі мэдыятар мэтаінфармуе мэтанавую камэту на камэце"),
        ("экспедыцыя на прэзідыум вікіпедыі", "экспэдыцыя на прэзыдыюм вікіпэдыі"),
        ("сарбент абсарбент у секцыі секстантаў", "сарбэнт абсарбэнт у сэкцыі сэкстантаў"),
        ("Гегель", "Гэґель"),
        ("ва Украіне, ва УКРАІНЕ, ВА УКРАІНЕ, у БЕЛДІУУ, у УАСНІГЛ", "ва Ўкраіне, ва УКРАІНЕ, ВА УКРАІНЕ, у БЕЛДІУУ, у УАСНІГЛ"),
        ("СМІ, СНІД", "СМІ, СНІД"),
        ("Відзскі з Відзаў", "Відзкі зь Відзаў"),
        ("не кат, не\u{00A0}кат", "ня кат, ня\u{00A0}кат"),
        ("Не\u{0020}маючы, не\u{00A0}маючы, не\u{2009}маючы", "Ня\u{0020}маючы, ня\u{00A0}маючы, ня\u{2009}маючы"),
        ("вялікі &#40не)", "вялікі &#40не)"),
        ("не нейкі", "ня нейкі"),
        ("не зробіш", "ня зробіш"),
        ("наддзіманне", "наддзіманьне"),
        ("без імглаў", "бязь імглаў"),
        ("акварыум", "акварыюм"),
        ("анчоус", "анчоўс"),
        ("архіварыус", "архіварыюс"),
        ("вакуум", "вакуўм"),
        ("індывідуум", "індывідуўм"),
        ("калоквіум", "калёквіюм"),
        ("кампендыум", "кампэндыюм"),
        ("прэзідыум", "прэзыдыюм"),
        ("каунас", "(каўнас|коўна)"),
        ("кансіліум", "кансыліюм"),
        ("кансорцыум банкаў", "кансорцыюм банкаў"),
        ("кантынуум", "кантынуўм"),
        ("натарыус", "натарыюс"),
        ("подыум", "подыюм"),
        ("радыус", "радыюс"),
        ("сімпозіум", "сымпозіюм"),
        ("страус", "стра(ў|ву)с"),
        ("соус", "соўс"),
        ("опіум", "опіюм"),
        ("харыус", "харыюс"),
        ("шлагбаум", "шлягбаўм"),
        ("Штраус", "Штраўс"),
        ("эленіум", "эленіюм"),
        ("эксперымент", "экспэрымэнт"),
        ("бяспецы", "бясьпецы"),
        ("клубны", "клюбны"),
        ("дзвярах", "дзьвяр(а|о)х"),
        ("мензурка", "мэнзурка"),
        ("медуза", "мэдуза"),
        ("спелеалогія", "спэлеалёгія"),
        ("спінер", "сьпінэр"),
        ("хімера", "хімэра"),
        ("парапет", "парапэт"),
        ("пелікан", "пэлікан"),
        ("апетыт", "апэтыт"),
        ("наўтылус", "наўтылюс"),
        ("вербаваць", "вэрбаваць"),
        ("зала", "заля"),
        ("паспець", "пасьпець"),
        ("трафей", "трафэй"),
        ("Васіліскам", "Базыліскам"),
        ("не дам", "ня дам"),
        ("бензін", "бэнзын"),
        ("часцей гасцей", "часьцей гасьцей"),
        ("Меларэн", "Мэлярэн"),
        ("Розенберг", "Розэнбэрґ"),
        ("ферментацыі", "фэрмэнтацыі"),
        ("фетышызм", "фэтышызм"),
        ("кібернетыка", "кібэрнэтыка"),
        ("спартсменак", "спартсмэнак"),
        ("бэтсмен", "бэтсмэн"),
        ("розава", "р(оза|ужо)ва"),
        ("вестманланд", "вэстманлянд"),
        ("сіксцінскай", "сыкстынскай"),
        ("Гейзенберг", "Гайзэнбэрґ"),
        ("дыспетчар партвейну", "дыспэтчар партвайну"),
        ("Фенаскандыі", "Фэнаскандыі"),
        ("Каралеўства свеваў", "Каралеўства свэваў"),
        ("у натрыі і самарыі", "у натры і самары"),
        ("бегемот", "бэгэмот"),
        ("макаа", "макао"),
        ("даўгаўпілс", "(даўгаўпілс|дзьвінец)"),
        ("Беніта Мусаліні", "Бэніта Мусаліні"),
        ("секунд", "(сэкунд|сэкундаў)"),
        ("месенджар", "мэсэндж(э|а)р"),
        ("менеджар", "мэнэдж(а|э)р"),
        ("не знаю, не знае", "ня знаю, ня знае"),
        ("Людвіг ван Бетховен", "Людвіґ ван Бэтговэн"),
        ("Гутэнберг", "Гутэнбэрґ"),
        ("Пабла", "Паблё"),
        ("заблакіраваць, заблакаваць", "заблякаваць, заблякаваць"),
        ("паланізм", "палянізм"),
        ("без імглаў", "бязь імглаў"),
    ];
    test_cases(&cfg, cases);
}

// ===== no-change cases =====
// JS: test/cases/taraskevization.ts → noChange

#[test]
fn test_no_change() {
    let cfg = default_cfg();
    let cases: &[&str] = &[
        "чатырохстворкавая шызафрэнія рыма-каталіцкага капірайта згарэлага лаўрышава-жармінскі",
        "случы салоніцкі васкрасенскі з календулай паланілі луіша",
        "звышскопішча апалонікаў",
        "праґрама УЭСТ",
        "падскокнуць і надскокнуць",
        "масфільм",
        "тлець, тля, для, днець, адмірал, адʼютант, мацнець, падвесьці, пэндзлік, на тле, пры мардве, у таварыстве, Латвія, Людвік, Людміла, Мардвілка, Цнянка, Ядвіга (побач зь Ядзьвіга), на Цне",
        "гай",
        "Не пра мяне",
        "  \n слова\n\n",
        "сінус",
        "платоў",
        "клубень",
        "Павелічаны Павел",
        "згаладалага",
        "Лаўрышаўскі, Некрашэвіча",
        "Дзісенскім дыяцэзіяй",
        "гоншчык",
        "марозавіцкага",
        "бегства Галкіна",
        "Сяргей Сяргеевіч",
    ];
    test_no_change_cases(&cfg, cases);
}

// ===== g-words (ґ) =====
// JS: test/cases/taraskevization.ts → gwords

#[test]
fn test_gwords() {
    let cfg = default_cfg();
    let cases: &[(&str, &str)] = &[
        ("Аргентына", "Арґентына"),
        ("Ангельшчына", "Анґельшчына"),
        ("Гедзімін", "Ґедзімін"),
        ("Гейл", "Ґейл"),
        ("Гергетаць", "Ґерґетаць"),
        ("Гервяты", "Ґервяты"),
        ("інтэлігентны", "інтэліґентны"),
        ("фогель", "фоґель"),
        ("швагер", "шваґер"),
        ("Руген", "Руґен"),
        ("Рэгенсбург", "Рэґенсбурґ"),
        ("Джгір", "Джґір"),
        ("гільдыя", "ґільдыя"),
        ("гімназія", "ґімназія"),
        ("Ганконг", "Ганконґ"),
        ("Васка да Гама", "Васка да Ґама"),
        ("Тэлеграм, Інстаграм", "Тэлеґрам, Інстаґрам"),
        (
            "Агата, Агаце, Агаты, Агатам, Агатаю, Агатах",
            "Аґата, Аґаце, Аґаты, Аґатам, Аґатаю, Аґатах",
        ),
        ("Гегель", "Гэґель"),
    ];
    test_cases(&cfg, cases);
}

// ===== special syntax =====
// JS: test/cases/special-syntax.ts

#[test]
fn test_special_syntax() {
    let cfg = default_cfg();
    let cases: &[(&str, &str)] = &[
        ("Планета", "Плянэта"),
        ("<Планета>", "<Планета>"),
        ("<Планета> SMS <План>", "<Планета> SMS <План>"),
        ("<.Планета>", "Планета"),
        ("<,Планета>", "<Плянэта>"),
        ("<*Планета>", "<Планета>"),
        ("<*.Планета>", "Планета"),
        ("<*,Планета>", "<Плянэта>"),
        ("<--[<*.ТУТ>]", "<--[<*.ТУТ>]"),
        ("<Медык> МЕДЫК", "<Медык> МЕДЫК"),
        ("<Медык> <МЕДЫК", "<Медык> <МЕДЫК"),
        ("<Медык> > медык >", "<Медык> > мэдык >"),
        ("<Медык> > МЕДЫК >", "<Медык> > МЕДЫК >"),
        ("<.>", ""),
        ("<*.>", ""),
        ("<,>", ""),
        ("<*,>", ""),
        ("<<Планета> планета", "<<Планета> плянэта"),
        ("<\\>Планета>", "<>Планета>"),
        ("<>", "<>"),
        ("<.<.>>", "<.>"),
        ("<<Планета>>", "<<Планета>>"),
        ("<Планета <<= Планета>", "<Планета <<= Планета>"),
    ];
    test_cases(&cfg, cases);
}

#[test]
fn test_special_syntax_latin() {
    let cfg = TaraskConfig {
        abc: Alphabet::Latin,
        ..Default::default()
    };
    let cases: &[(&str, &str)] = &[
        ("Планета", "Planeta"),
        ("<Планета>", "<Планета>"),
        ("<*Планета>", "<Płanieta>"),
        ("<.Планета>", "Планета"),
        ("<*.Планета>", "Płanieta"),
        ("<*.Планета гарбузоў АААА>", "Płanieta harbuzoŭ AAAA"),
        ("<,Планета>", "<Planeta>"),
        ("<*,Планета>", "<Planeta>"),
    ];
    test_cases(&cfg, cases);
}

// ===== case restoring =====
// JS: test/cases/case-restoring.ts

#[test]
fn test_case_restoring_escape_caps() {
    let cfg = default_cfg();
    let cases: &[(&str, &str)] = &[
        ("ПЛАНЕТА", "ПЛАНЕТА"),
        ("планета", "плянэта"),
        ("Планета", "Плянэта"),
        ("ПлАНеТа", "ПлАНеТа"),
        ("ПланетА", "ПЛЯНЭТА"),
        ("Cлова", "Cлова"),
        ("CловA", "CловA"),
        ("CЛОВА", "CЛОВА"),
        ("СлОвА", "СлОвА"),
    ];
    test_cases(&cfg, cases);
}

#[test]
fn test_case_restoring_no_escape_caps() {
    let cfg = TaraskConfig {
        do_escape_capitalized: false,
        ..Default::default()
    };
    let cases: &[(&str, &str)] = &[
        ("ПЛАНЕТА", "ПЛЯНЭТА"),
        ("планета", "плянэта"),
        ("Планета", "Плянэта"),
        ("ПлАНеТа", "Плянэта"),
        ("ПланетА", "ПЛЯНЭТА"),
        ("Cлова", "Cлова"),
        ("CловA", "CловA"),
        ("CЛОВА", "CЛОВА"),
        ("СлОвА", "СлОвА"),
    ];
    test_cases(&cfg, cases);
}

// ===== latin / latin-ji conversion =====
// JS: test/cases/alphabet-conversion.ts + test/cases/latin.ts

#[test]
fn test_latin_conversion() {
    let cfg = TaraskConfig {
        abc: Alphabet::Latin,
        ..Default::default()
    };
    let cases: &[(&str, &str)] = &[
        ("У яе ёсьць плянэта", "U jaje jość planeta"),
        ("еёёююяя", "jejojojujujaja"),
        ("Вельмі Харошы", "Vielmi Charošy"),
        ("Харошы", "Charošy"),
        ("верабі", "vierabi"),
        ("Масква", "Maskva"),
        ("Здароўе здароўе ЗДАРОЎЕ", "Zdaroŭje zdaroŭje ZDAROŬJE"),
        ("падвор'і ПАДВОР'І", "padvorji PADVORJI"),
        (
            "У Іўі худы жвавы чорт у зялёнай камізэльцы пабег пад'есці фаршу з юшкай",
            "U Iŭi chudy žvavy čort u zialonaj kamizelcy pabieh padjeści faršu ź juškaj",
        ),
        (
            "Я жорстка заб'ю проста ў сэрца гэты расквечаны профіль, што ходзіць ля маёй хаты",
            "Ja žorstka zabju prosta ŭ serca hety raskviečany profil, što chodzić la majoj chaty",
        ),
        ("грошы", "hrošy"),
        ("Англійская", "An(glij|giel)skaja"),
        ("Я 77-ы", "Ja 77-y"),
        ("Я Фанат", "Ja Fanat"),
        ("Мы з LGB Alliance", "My z LGB Alliance"),
        ("łiłia; UVAŁIŁASIA ŁIŁIA", "łiłia; UVAŁIŁASIA ŁIŁIA"),
    ];
    test_cases(&cfg, cases);
}

#[test]
fn test_latin_ji_conversion() {
    let cfg = TaraskConfig {
        abc: Alphabet::LatinJi,
        ..Default::default()
    };
    let cases: &[(&str, &str)] = &[
        ("пры інстытуце", "pry jinstytucie"),
        ("вераб'і", "vierabji"),
        ("Харошы", "Charošy"),
        ("поіць", "pojić"),
        ("у ім", "u jim"),
        ("іста", "jista"),
        ("Іншы ў ігрушы", "Jinšy ŭ ihrušy"),
        ("церазь ямы", "cieraź jamy"),
        ("ня йснуе Іры", "nia jsnuje Jiry"),
        ("маленькія іклы", "maleńkija jikły"),
        ("бязь зьвесткаў", "biaź źviestkaŭ"),
        ("з імглой", "z imhłoj"),
        ("імгла", "imhła"),
        ("з імною", "z imnoju"),
        ("яна йдзе зь ім паіць коні", "jana jdzie ź jim pajić koni"),
        ("мая інфармацыя", "maja jinfarmacyja"),
        ("знайшлі ...", "znajšli ..."),
        ("ні слуху ...", "ni słuchu ..."),
        ("пралезьці ...", "praleźci ..."),
        (">Я ня маю ...", ">Ja nia maju ..."),
        ("ext>Я Фанат, ...", "ext>Ja Fanat, ..."),
        ("ісьці забаронена ...", "iści zabaroniena ..."),
        ("сталкеру. І мы", "stałkieru. I my"),
        ("У іх", "U jich"),
        ("цалкам іншы,", "całkam jinšy,"),
        ("вачох іхных", "vačoch jichnych"),
        ("... і іншае", "... i jinšaje"),
        (">Я 032, уцяміў", ">Ja 032, uciamiŭ"),
        ("КАІР", "KAJIR"),
        (". Імаверна", ". Imavierna"),
        ("Я Інфарматар", "Ja Jinfarmatar"),
        ("<lia>а</lia>", "<lia>a</lia>"),
        ("Я 77-ы", "Ja 77-y"),
        ("я і ён", "ja j jon"),
        ("я і ўваліўся", "ja j uvaliŭsia"),
        ("Я і Ўваліўся", "Ja j Uvaliŭsia"),
        ("Я І ЎВАЛІЎСЯ", "JA J UVALIŬSIA"),
        ("Белая ікра", "Biełaja jikra"),
        ("БЕЛАЯ ІКРА", "BIEŁAJA JIKRA"),
        ("ЁЮ", "JOJU"),
        ("Забраць інфармацыю", "Zabrać infarmacyju"),
        ("Зь іншым", "Ź jinšym"),
        ("яна (ён)", "jana (jon)"),
        ("яна &#40ён)", "jana &#40jon)"),
        ("нейкіх", "niejkich"),
        ("прыцягнуў і ...", "pryciahnuŭ i ..."),
        ("ІЛ-86", "IŁ-86"),
        ("іклы", "jikły"),
        ("łiłia; UVAŁIŁASIA ŁIŁIA", "łiłia; UVAŁIŁASIA ŁIŁIA"),
    ];
    test_cases(&cfg, cases);
}

// ===== i→j replacement =====
// JS: test/cases/itoj.ts

#[test]
fn test_itoj() {
    let cfg = default_cfg();
    assert_eq!(tarask("Яна і ён", &cfg), "Яна і ён");

    let cfg_never = TaraskConfig {
        j: JMode::Never,
        ..Default::default()
    };
    assert_eq!(tarask("Яна і ён", &cfg_never), "Яна і ён");

    let cfg_always = TaraskConfig {
        j: JMode::Always,
        ..Default::default()
    };
    assert_eq!(tarask("Яна і ён", &cfg_always), "Яна й ён");

    let cfg_latin_never = TaraskConfig {
        j: JMode::Never,
        abc: Alphabet::Latin,
        ..Default::default()
    };
    assert_eq!(tarask("Яна і ён", &cfg_latin_never), "Jana i jon");

    let cfg_latin_always = TaraskConfig {
        j: JMode::Always,
        abc: Alphabet::Latin,
        ..Default::default()
    };
    assert_eq!(tarask("Яна і ён", &cfg_latin_always), "Jana j jon");
}

// ===== phonetization =====
// JS: test/cases/phonetization.ts

#[test]
fn test_phonetization() {
    let cfg = default_cfg();
    let cases: &[(&str, &str)] = &[
        ("я і смяяўся", "я й сьмяяўся"),
        ("іхняя планета", "йіхняя планета"),
        ("не маю часу", "ня маю часу"),
        ("без вокнаў", "бяз вокнаў"),
        ("абаіх", "абайіх"),
        ("на печцы", "на пеццы"),
        ("на дошцы", "на досцы"),
        ("на дарожцы", "на даросцы"),
        ("грузчык", "грушчык"),
        ("сшытак", "шшытак"),
        ("возмешся", "возьмесься"),
        ("езджу", "ежджу"),
        ("пясчаны", "пяшчаны"),
        ("не дурань", "ня дурань"),
        ("тэатр", "тэатар"),
        ("прабачце", "прабачце"),
        ("з іншага", "зь йіншага"),
        ("не справімся", "ня справімся"),
        ("не выгарыць", "ня выгарыць"),
        ("не збегчы", "ня зьбегчы"),
        ("не стану", "не стану"),
        ("не з'явішся", "не зьявісься"),
        ("з імі", "зь імі"),
        ("параіў", "парайіў"),
        ("Яны ідуць", "Яны йідуць"),
        ("але і ён", "але й ён"),
        ("не хочаш", "ня хочаш"),
        ("без клубных", "без клубных"),
        ("Пераможца", "Перамосца"),
        ("не ста\u{301}ну", "ня ста\u{301}ну"),
        ("з і\u{301}мі", "зь йі\u{301}мі"),
        ("пара\u{301}іў", "пара\u{301}йіў"),
        ("Яны\u{301} іду\u{301}ць", "Яны\u{301} йіду\u{301}ць"),
        ("але\u{301} і ён", "але\u{301} й ён"),
        ("не з'я\u{301}вішся", "ня зья\u{301}вісься"),
        ("не хо\u{301}чаш", "ня хо\u{301}чаш"),
        ("без клу\u{301}бных", "бяз клу\u{301}бных"),
        ("Перамо\u{301}жца", "Перамо\u{301}сца"),
        ("бязь імглаў", "бязь імглаў"),
        ("праз імглаў", "празь імглаў"),
    ];
    test_phonetic_cases(&cfg, cases);
}

// ===== alphabet conversion: arabic =====
// JS: test/cases/arabic.ts + test/cases/alphabet-conversion.ts → arabic

#[test]
fn test_arabic_conversion() {
    let cfg = TaraskConfig {
        abc: Alphabet::Arabic,
        ..Default::default()
    };
    let cases: &[(&str, &str)] = &[
        ("планета", "پْـلانَطْاَ"),
        ("надзіманне", "نْاَࢮِمْاَنَّ"),
        ("Планета", "پْـلانَطْاَ"),
    ];
    for (input, expected) in cases {
        let result = alphabetic(input, &cfg);
        assert_eq!(&result, expected, "input: {input:?}");
    }

    let cfg_tarask = TaraskConfig {
        abc: Alphabet::Arabic,
        ..Default::default()
    };
    let tarask_cases: &[(&str, &str)] = &[
        ("Планета Čalaviekaŭ!", "پْـلانَطْاَ Čalaviekaŭ!"),
        ("Прывет, Čalaviek!", "پْرِوَطْ، Čalaviek!"),
    ];
    for (input, expected) in tarask_cases {
        let result = tarask(input, &cfg_tarask);
        assert_eq!(&result, expected, "input: {input:?}");
    }
}

// ===== multiline =====
// JS: test/cases/multiline.ts

#[test]
fn test_multiline() {
    let cfg = default_cfg();
    let cases: &[(&str, &str)] = &[
        ("жыццясцвярджальны\nплан", "жыцьцясьцьвярджальны\nплян"),
        ("жыццясцвярджальны\r\nплан", "жыцьцясьцьвярджальны\r\nплян"),
        (
            "жыццясцвярджальны\r\n\t\t\tплан",
            "жыцьцясьцьвярджальны\r\n\t\t\tплян",
        ),
    ];
    test_cases(&cfg, cases);
}

#[test]
fn test_multiline_html() {
    let cfg = TaraskConfig {
        wrappers: Some(HTML_WRAPPERS),
        variations: VariationMode::No,
        new_line: "<br>".into(),
        ..Default::default()
    };
    let cases: &[(&str, &str)] = &[(
        "жыццясцвярджальны\n\t\t\tплан",
        "жыц<tarF>ьцясьць</tarF>вярджальны<br>\t\t\tпл<tarF>я</tarF>н",
    )];
    for (input, expected) in cases {
        let result = tarask(input, &cfg);
        assert_eq!(&result, expected, "input: {input:?}");
    }
}

// ===== nbsp preservation =====
// JS: test/cases/taraskevization.ts → embedded

#[test]
fn test_nbsp() {
    let cfg = default_cfg();
    let nbsp = '\u{00A0}';
    let input = format!("І.{}С.{}Тургенева.", nbsp, nbsp);
    let expected = format!("І.{}С.{}Турґенева.", nbsp, nbsp);
    let result = tarask(&input, &cfg);
    assert_eq!(
        result, expected,
        "NBSP should be preserved and case should not be lost"
    );
}

// ===== escape upper =====

#[test]
fn test_escape_upper() {
    let cfg = default_cfg();
    let cases: &[(&str, &str)] = &[
        ("ва Украіне", "ва Ўкраіне"),
        ("СМІ", "СМІ"),
        ("СНІД", "СНІД"),
    ];
    test_cases(&cfg, cases);
}

// ===== variations =====
// JS: test/cases/non-html-options.ts

#[test]
fn test_variations() {
    let cfg_all = TaraskConfig {
        variations: VariationMode::All,
        ..Default::default()
    };
    assert_eq!(tarask("Гродна", &cfg_all), "(Гродна|Горадня)");

    let cfg_no = TaraskConfig {
        variations: VariationMode::No,
        ..Default::default()
    };
    assert_eq!(tarask("Гродна", &cfg_no), "Гродна");

    let cfg_first = TaraskConfig {
        variations: VariationMode::First,
        ..Default::default()
    };
    assert_eq!(tarask("Гродна", &cfg_first), "Горадня");
}

// ===== highlighting (diff-based) =====
// JS: test/cases/highlighting.ts

#[test]
fn test_highlighting() {
    let cfg = TaraskConfig {
        wrappers: Some(highlight_wrappers()),
        ..Default::default()
    };
    // Single-arg cases: run tarask, then highlight diff with original text
    let cases: &[(&str, &str)] = &[
        ("планета", "пл[я]н[э]та"),
        ("смех", "с[ь]мех"),
        ("балкон", "бал[ь]кон"),
        ("брэст", "[(брэст|берасьце)]"),
        ("балонья", "бал[ёньн]я"),
        ("бернардзінцы", "б[эрнарды]нцы"),
        ("мекка", "м[э]ка"),
        ("санкцый", "санкц[(ый|ыяў)]"),
        ("баторый", "батор[ы]"),
        ("сцэнарый", "сцэна[р]"),
        ("Віктор Гюго", "Віктор [Ю]ґо"),
        ("казахскі", "каз[ас]кі"),
    ];
    for (input, expected) in cases {
        let result = tarask(input, &cfg);
        assert_eq!(&result, expected, "input: {input:?}");
    }
}

// 2-arg highlighting: tests apply_highlight_diff directly (matching JS highlightDiff with two strings)
// JS test runner: highlightDiff([input[1]], [input[0]], true, (t) => `[${t}]`)
#[test]
fn test_highlight_diff_two_arg() {
    let fix = |s: &str| format!("[{}]", s);
    let cases: &[((&str, &str), &str)] = &[
        (("planeta", "płanieta"), "p[lan]eta"),
        (("Persanalny", "Piersanalny"), "[Pe]rsanalny"),
        (("абба", "абвба"), "а[бб]а"),
        (("ббаа", "бвбаа"), "[бб]аа"),
        (("аабб", "аабвб"), "аа[бб]"),
    ];
    for ((word, orig), expected) in cases {
        let result = apply_highlight_diff(word, orig, true, &fix);
        assert_eq!(&result, *expected, "input: {word:?} vs {orig:?}");
    }
}

// Regression: overlapping prefix/suffix matches (wlen != olen, repeated chars)
// used to panic with an inverted slice index in highlight_diff_variable.
#[test]
fn test_highlight_diff_overlapping_region() {
    let fix = |s: &str| format!("[{s}]");
    for (word, orig) in [("aaa", "aa"), ("aaaa", "aa"), ("aaa", "a"), ("aa", "aaa")] {
        let _ = apply_highlight_diff(word, orig, true, &fix);
        let _ = apply_highlight_diff(word, orig, false, &fix);
    }
}

// ===== HTML options =====
// JS: test/cases/html-options.ts (uses htmlConfigOptions: g=false, newLine=<br>)

// ===== HTML options =====
// JS: test/cases/html-options.ts (uses htmlConfigOptions: g=false, newLine=<br>, leftAngleBracket=&lt)

#[test]
fn test_html_options() {
    // Base config = htmlConfigOptions: wrappers=html, g=false, newLine=<br>,
    // leftAngleBracket=&lt.
    let mut cfg = html_config_options();
    cfg.variations = VariationMode::No;
    let cases: &[(&str, &str)] = &[
        (
            "жыццясцвярджальны план",
            "жыц<tarF>ьцясьць</tarF>вярджальны пл<tarF>я</tarF>н",
        ),
        ("??????", "??????"),
        ("газета", "<tarH>г</tarH>аз<tarF>э</tarF>та"),
    ];
    for (input, expected) in cases {
        let result = tarask(input, &cfg);
        assert_eq!(&result, expected, "input: {input:?}");
    }

    // With g=true (ґ conversion)
    let mut cfg_g = html_config_options();
    cfg_g.variations = VariationMode::No;
    cfg_g.g = true;
    let cases_g: &[(&str, &str)] = &[("газета", "<tarH>ґ</tarH>аз<tarF>э</tarF>та")];
    for (input, expected) in cases_g {
        let result = tarask(input, &cfg_g);
        assert_eq!(&result, expected, "input: {input:?}");
    }

    // With special syntax
    let mut cfg_special = html_config_options();
    cfg_special.variations = VariationMode::No;
    let cases_special: &[(&str, &str)] = &[
        ("<,Планета>", "&ltПл<tarF>я</tarF>н<tarF>э</tarF>та>"),
        (
            "&lt <,Планета>",
            "&lt &ltПл<tarF>я</tarF>н<tarF>э</tarF>та>",
        ),
    ];
    for (input, expected) in cases_special {
        let result = tarask(input, &cfg_special);
        assert_eq!(&result, expected, "input: {input:?}");
    }
}
