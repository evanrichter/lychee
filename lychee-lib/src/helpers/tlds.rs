use phf::phf_set;

/// A list of top level domains by popularity rank
/// Taken from <https://dnsinstitute.com/research/popular-tld-rank/>
/// We add the dot at the beginning for faster lookups without `String`
/// allocation
pub(super) static TOP_TLDS: phf::Set<&'static str> = phf_set! {
    ".com",
    ".net",
    ".ru",
    ".org",
    ".info",
    ".in",
    ".ir",
    ".uk",
    ".au",
    ".de",
    ".ua",
    ".ca",
    ".tr",
    ".co",
    ".jp",
    ".vn",
    ".cn",
    ".gr",
    ".fr",
    ".tk",
    ".tw",
    ".id",
    ".br",
    ".io",
    ".xyz",
    ".it",
    ".nl",
    ".pl",
    ".za",
    ".us",
    ".eu",
    ".mx",
    ".ch",
    ".biz",
    ".me",
    ".il",
    ".es",
    ".online",
    ".by",
    ".x",
    ".nz",
    ".kr",
    ".cz",
    ".ro",
    ".cf",
    ".ar",
    ".club",
    ".my",
    ".tv",
    ".kz",
    ".cl",
    ".pk",
    ".pro",
    ".site",
    ".th",
    ".se",
    ".sg",
    ".cc",
    ".be",
    ".rs",
    ".top",
    ".ga",
    ".ma",
    ".hu",
    ".ae",
    ".su",
    ".dk",
    ".hk",
    ".at",
    ".ml",
    ".shop",
    ".store",
    ".ng",
    ".np",
    ".no",
    ".app",
    ".live",
    ".pe",
    ".ph",
    ".ie",
    ".lk",
    ".gq",
    ".edu",
    ".fi",
    ".ai",
    ".sa",
    ".pw",
    ".tech",
    ".bd",
    ".sk",
    ".ke",
    ".pt",
    ".az",
    ".space",
    ".mk",
    ".ge",
    ".tn",
    ".lt",
    ".dev",
    ".to",
    ".gov",
};

/// A list of top level domains by popularity rank
/// This is identical to the above list with the exception of an additional slash at the end to
/// simplify checks and avoid extraneous `String` allocations at runtime.
pub(super) static TOP_TLDS_CONTAINS: phf::Set<&'static str> = phf_set! {
    ".com/",
    ".net/",
    ".ru/",
    ".org/",
    ".info/",
    ".in/",
    ".ir/",
    ".uk/",
    ".au/",
    ".de/",
    ".ua/",
    ".ca/",
    ".tr/",
    ".co/",
    ".jp/",
    ".vn/",
    ".cn/",
    ".gr/",
    ".fr/",
    ".tk/",
    ".tw/",
    ".id/",
    ".br/",
    ".io/",
    ".xyz/",
    ".it/",
    ".nl/",
    ".pl/",
    ".za/",
    ".us/",
    ".eu/",
    ".mx/",
    ".ch/",
    ".biz/",
    ".me/",
    ".il/",
    ".es/",
    ".online/",
    ".by/",
    ".x/",
    ".nz/",
    ".kr/",
    ".cz/",
    ".ro/",
    ".cf/",
    ".ar/",
    ".club/",
    ".my/",
    ".tv/",
    ".kz/",
    ".cl/",
    ".pk/",
    ".pro/",
    ".site/",
    ".th/",
    ".se/",
    ".sg/",
    ".cc/",
    ".be/",
    ".rs/",
    ".top/",
    ".ga/",
    ".ma/",
    ".hu/",
    ".ae/",
    ".su/",
    ".dk/",
    ".hk/",
    ".at/",
    ".ml/",
    ".shop/",
    ".store/",
    ".ng/",
    ".np/",
    ".no/",
    ".app/",
    ".live/",
    ".pe/",
    ".ph/",
    ".ie/",
    ".lk/",
    ".gq/",
    ".edu/",
    ".fi/",
    ".ai/",
    ".sa/",
    ".pw/",
    ".tech/",
    ".bd/",
    ".sk/",
    ".ke/",
    ".pt/",
    ".az/",
    ".space/",
    ".mk/",
    ".ge/",
    ".tn/",
    ".lt/",
    ".dev/",
    ".to/",
    ".gov/",
};
