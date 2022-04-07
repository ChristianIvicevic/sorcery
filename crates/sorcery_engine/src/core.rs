use std::collections::BTreeSet;

#[cfg(test)]
use derive_builder::Builder;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

/// Opaque type to reference a player within a game.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PlayerId(pub(crate) u32);

/// 201.2. A card’s name is always considered to be the English version of its name, regardless of
///        printed language.
#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Name(pub(crate) String);

/// 102.1. A player is one of the people in the game. The active player is the player whose turn it
///        is. The other players are nonactive players.
pub(crate) struct Player {
    pub(crate) id: PlayerId,
    pub(crate) name: String,
    pub(crate) life: i64,
}

/// 105.1. There are five colors in the Magic game: white, blue, black, red, and green.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub(crate) enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

/// 105.2. An object can be one or more of the five colors, or it can be no color at all. An object
///        is the color or colors of the mana symbols in its mana cost, regardless of the color of
///        its frame. An object’s color or colors may also be defined by a color indicator or a
///        characteristic-defining ability. See rule 202.2.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ColorKind {
    /// 105.2a A monocolored object is exactly one of the five colors.
    Monocolored(Color),
    /// 105.2b A multicolored object is two or more of the five colors.
    Multicolored(BTreeSet<Color>),
    /// 105.2c A colorless object has no color.
    Colorless,
}

/// 105.5. If an effect refers to a color pair, it means exactly two of the five colors. There are
///        ten color pairs: white and blue, white and black, blue and black, blue and red, black and
///        red, black and green, red and green, red and white, green and white, and green and blue.
pub(crate) enum ColorPair {
    WhiteBlue,
    WhiteBlack,
    BlueBlack,
    BlueRed,
    BlackRed,
    BlackGreen,
    RedGreen,
    RedWhite,
    GreenWhite,
    GreenBlue,
}

/// 106.1. Mana is the primary resource in the game. Players spend mana to pay costs, usually when
///        casting spells and activating abilities.
pub(crate) enum Mana {
    /// 106.1a There are five colors of mana: white, blue, black, red, and green.
    Monocolored(Color),
    /// 106.1b There are six types of mana: white, blue, black, red, green, and colorless.
    Colorless,
}

/// 106.4. When an effect instructs a player to add mana, that mana goes into a player’s mana pool.
///        From there, it can be used to pay costs immediately, or it can stay in the player’s mana
///        pool as unspent mana. Each player’s mana pool empties at the end of each step and phase,
///        and the player is said to lose this mana. Cards with abilities that produce mana or refer
///        to unspent mana have received errata in the Oracle(TM) card reference to no longer
///        explicitly refer to the mana pool.
pub(crate) struct ManaPool {
    mana: Vec<Mana>,
}

/// 107.4. The mana symbols are {W}, {U}, {B}, {R}, {G}, and {C}; the numerical symbols {0}, {1},
///        {2}, {3}, {4}, and so on; the variable symbol {X}; the hybrid symbols {W/U}, {W/B},
///        {U/B}, {U/R}, {B/R}, {B/G}, {R/G}, {R/W}, {G/W}, and {G/U}; the monocolored hybrid
///        symbols {2/W}, {2/U}, {2/B}, {2/R}, and {2/G}; the Phyrexian mana symbols {W/P}, {U/P},
///        {B/P}, {R/P}, and {G/P}; the hybrid Phyrexian symbols {W/U/P}, {W/B/P}, {U/B/P}, {U/R/P},
///        {B/R/P}, {B/G/P}, {R/G/P}, {R/W/P}, {G/W/P}, and {G/U/P}; and the snow mana symbol {S}.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub(crate) enum ManaSymbol {
    /// 107.4a There are five primary colored mana symbols: {W} is white, {U} blue, {B} black, {R}
    ///        red, and {G} green. These symbols are used to represent colored mana, and also to
    ///        represent colored mana in costs. Colored mana in costs can be paid only with the
    ///        appropriate color of mana. See rule 202, “Mana Cost and Color.”
    Colored(Color),
    /// 107.4b Numerical symbols (such as {1}) and variable symbols (such as {X}) represent generic
    ///        mana in costs. Generic mana in costs can be paid with any type of mana. For more
    ///        information about {X}, see rule 107.3.
    Generic(u64),
    Variable,
    /// 107.4c The colorless mana symbol {C} is used to represent one colorless mana, and also to
    ///        represent a cost that can be paid only with one colorless mana.
    Colorless,
    // 107.4d The symbol {0} represents zero mana and is used as a placeholder for a cost that can
    //        be paid with no resources. (See rule 118.5.)
    //
    // 107.4e Hybrid mana symbols are also colored mana symbols. Each one represents a cost that can
    //        be paid in one of two ways, as represented by the two halves of the symbol. A hybrid
    //        symbol such as {W/U} can be paid with either white or blue mana, and a monocolored
    //        hybrid symbol such as {2/B} can be paid with either one black mana or two mana of any
    //        type. A hybrid mana symbol is all of its component colors.
    //
    // Example: {G/W}{G/W} can be paid by spending {G}{G}, {G}{W}, or {W}{W}.
    //
    // 107.4f Phyrexian mana symbols are colored mana symbols: {W/P} is white, {U/P} is blue, {B/P}
    //        is black, {R/P} is red, and {G/P} is green. A Phyrexian mana symbol represents a cost
    //        that can be paid either with one mana of its color or by paying 2 life. There are also
    //        ten hybrid Phyrexian mana symbols. A hybrid Phyrexian mana symbol represents a cost
    //        that can be paid with one mana of either of its component colors or by paying 2 life.
    //        A hybrid Phyrexian mana symbol is both of its component colors.
    //
    // Example: {W/P}{W/P} can be paid by spending {W}{W}, by spending {W} and paying 2 life, or by
    //          paying 4 life.
    //
    // 107.4g In rules text, the Phyrexian symbol {P} with no colored background means any of the
    //        fifteen Phyrexian mana symbols.
    //
    // 107.4h When used in a cost, the snow mana symbol {S} represents a cost that can be paid with
    //        one mana of any type produced by a snow source (see rule 106.3). Effects that reduce
    //        the amount of generic mana you pay don’t affect {S} costs. The {S} symbol can also be
    //        used to refer to mana of any type produced by a snow source spent to pay a cost. Snow
    //        is neither a color nor a type of mana.
}

/// 202.1. A card’s mana cost is indicated by mana symbols near the top of the card. (See rule
///        107.4.) On most cards, these symbols are printed in the upper right corner. Some cards
///        from the Future Sight set have alternate frames in which the mana symbols appear to the
///        left of the illustration.
#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct ManaCost(pub(crate) IndexSet<ManaSymbol>);

/// 207.1. The text box is printed on the lower half of the card. It usually contains rules text
///        defining the card’s abilities.
#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct RulesText(pub(crate) String);

/// 209.1. Each planeswalker card has a loyalty number printed in its lower right corner. This
///        indicates its loyalty while it’s not on the battlefield, and it also indicates that the
///        planeswalker enters the battlefield with that many loyalty counters on it.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub(crate) struct Loyalty(pub(crate) u64);

/// 212.1. Each card features text printed below the text box that has no effect on game play. Not
///        all card sets were printed with all of the information listed below on each card.
///
/// 212.1a Most card sets feature collector numbers. This information is printed in the form
///        [card number]/[total cards in the set] or simply [card number]. Some cards, such as
///        unique cards in Planeswalker Decks, have card numbers that exceed the listed total number
///        of cards.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub(crate) struct CollectorNumber(pub(crate) u64);

/// 200.1. The parts of a card are name, mana cost, illustration, color indicator, type line,
///        expansion symbol, text box, power and toughness, loyalty, hand modifier, life modifier,
///        illustration credit, legal text, and collector number. Some cards may have more than one
///        of any or all of these parts.
///
/// 200.2. Some parts of a card are also characteristics of the object that has them. See rule
///        109.3.
#[derive(Serialize, Deserialize)]
#[cfg_attr(
    test,
    derive(Builder),
    builder(pattern = "owned", setter(strip_option), default)
)]
pub(crate) struct Card {
    /// 201.1. The name of a card is printed on its upper left corner.
    pub(crate) name: Name,
    /// 202.1. A card’s mana cost is indicated by mana symbols near the top of the card. (See rule
    ///        107.4.) On most cards, these symbols are printed in the upper right corner. Some
    ///        cards from the Future Sight set have alternate frames in which the mana symbols
    ///        appear to the left of the illustration.
    pub(crate) mana_cost: Option<ManaCost>,
    /// 204.1. The color indicator is printed to the left of the type line directly below the
    ///        illustration. It consists of a circular symbol filled in with one or more colors. A
    ///        color indicator is usually found on nonland cards without a mana cost.
    pub(crate) color_indicator: Option<ColorKind>,
    /// 205.1. The type line is printed directly below the illustration. It contains the card’s card
    ///        type(s). It also contains the card’s subtype(s) and supertype(s), if applicable.
    pub(crate) type_line: TypeLine,
    /// 206.1. The expansion symbol indicates which Magic set a card is from. It’s a small icon
    ///        normally printed below the right edge of the illustration. It has no effect on game
    ///        play.
    pub(crate) expansion_symbol: ExpansionSymbol,
    /// 207.1. The text box is printed on the lower half of the card. It usually contains rules text
    ///        defining the card’s abilities.
    pub(crate) rules_text: RulesText,
    /// 208.1. A creature card has two numbers separated by a slash printed in its lower right
    ///        corner. The first number is its power (the amount of damage it deals in combat); the
    ///        second is its toughness (the amount of damage needed to destroy it). For example, 2/3
    ///        means the object has power 2 and toughness 3. Power and toughness can be modified or
    ///        set to particular values by effects.
    pub(crate) pt: Option<PtCharacteristic>,
    /// 209.1. Each planeswalker card has a loyalty number printed in its lower right corner. This
    ///        indicates its loyalty while it’s not on the battlefield, and it also indicates that
    ///        the planeswalker enters the battlefield with that many loyalty counters on it.
    pub(crate) loyalty: Option<Loyalty>,
    /// 212.1. Each card features text printed below the text box that has no effect on game play.
    ///        Not all card sets were printed with all of the information listed below on each card.
    ///
    /// 212.1a Most card sets feature collector numbers. This information is printed in the form
    ///        [card number]/[total cards in the set] or simply [card number]. Some cards, such as
    ///        unique cards in Planeswalker Decks, have card numbers that exceed the listed total
    ///        number of cards.
    pub(crate) collector_number: CollectorNumber,
}

#[cfg(test)]
impl Default for Card {
    /// Default implementation that yields an empty card used for testing in combination with the
    /// [`CardBuilder`].
    fn default() -> Self {
        Self {
            name: Name("Test Card".to_string()),
            mana_cost: None,
            color_indicator: None,
            type_line: TypeLine {
                card_type: [].into(),
                subtype: [].into(),
                supertype: [].into(),
            },
            expansion_symbol: ExpansionSymbol {
                set: "TEST".into(),
                rarity: Rarity::Common,
            },
            rules_text: RulesText("".into()),
            pt: None,
            loyalty: None,
            collector_number: CollectorNumber(0),
        }
    }
}

impl Card {
    /// Returns a [`CardBuilder`].
    #[cfg(test)]
    pub(crate) fn builder() -> CardBuilder {
        CardBuilder::default()
    }

    /// 202.2. An object is the color or colors of the mana symbols in its mana cost, regardless of
    ///        the color of its frame.
    pub(crate) fn color(&self) -> ColorKind {
        if let Some(ref color_indicator) = self.color_indicator {
            return color_indicator.clone();
        }

        let colors = self
            .mana_cost
            .as_ref()
            .map(|it| {
                it.0.iter().fold(BTreeSet::new(), |mut colors, symbol| {
                    // TODO: Extend this to a match clause once hybrid mana symbols and other have
                    //  been implemented.
                    if let ManaSymbol::Colored(color) = symbol {
                        colors.insert(*color);
                    }
                    colors
                })
            })
            .filter(|it| !it.is_empty());

        match colors {
            None => ColorKind::Colorless,
            Some(colors) => match colors.len() {
                1 => ColorKind::Monocolored(*colors.iter().next().unwrap()),
                _ => ColorKind::Multicolored(colors),
            },
        }
    }
}

/// 205.1. The type line is printed directly below the illustration. It contains the card’s card
///        type(s). It also contains the card’s subtype(s) and supertype(s), if applicable.
#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct TypeLine {
    /// 205.2a The card types are artifact, conspiracy, creature, dungeon, enchantment, instant,
    ///        land, phenomenon, plane, planeswalker, scheme, sorcery, tribal, and vanguard. See
    ///        section 3, “Card Types.”
    pub(crate) card_type: IndexSet<CardType>,
    /// 205.3a A card can have one or more subtypes printed on its type line.
    pub(crate) subtype: IndexSet<Subtype>,
    /// 205.4a An object can have one or more supertypes. A card’s supertypes are printed directly
    ///        before its card types. The supertypes are basic, legendary, ongoing, snow, and world.
    pub(crate) supertype: IndexSet<Supertype>,
}

/// 206.1. The expansion symbol indicates which Magic set a card is from. It’s a small icon normally
///        printed below the right edge of the illustration. It has no effect on game play.
#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct ExpansionSymbol {
    // TODO: Figure out whether to use a String or enum for this.
    pub(crate) set: String,
    pub(crate) rarity: Rarity,
}

/// 206.2. The color of the expansion symbol indicates the rarity of the card within its set. A
///        red-orange symbol indicates the card is mythic rare. A gold symbol indicates the card is
///        rare. A silver symbol indicates the card is uncommon. A black or white symbol indicates
///        the card is common or is a basic land. A purple symbol signifies a special rarity; to
///        date, only the Time Spiral® “timeshifted” cards, which were rarer than that set’s rare
///        cards, have had purple expansion symbols. (Prior to the Exodus TM set, all expansion
///        symbols were black, regardless of rarity. Also, prior to the Sixth Edition core set, with
///        the exception of the Simplified Chinese Fifth Edition core set, Magic core sets didn’t
///        have expansion symbols at all.)
#[derive(Copy, Clone, Serialize, Deserialize)]
pub(crate) enum Rarity {
    MythicRare,
    Rare,
    Uncommon,
    Common,
    BasicLand,
    Timeshifted,
}

/// 300.1. The card types are artifact, conspiracy, creature, dungeon, enchantment, instant, land,
///        phenomenon, plane, planeswalker, scheme, sorcery, tribal, and vanguard. See section 3,
///        “Card Types.”
/// 300.2. Some objects have more than one card type (for example, an artifact creature). Such
///        objects combine the aspects of each of those card types, and are subject to spells and
///        abilities that affect either or all of those card types.
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub(crate) enum CardType {
    Artifact,
    Conspiracy,
    Creature,
    Dungeon,
    Enchantment,
    Instant,
    Land,
    Phenomenon,
    Plane,
    Planeswalker,
    Scheme,
    Sorcery,
    Tribal,
    Vanguard,
}

/// 205.3a A card can have one or more subtypes printed on its type line.
///
/// 205.3b Subtypes of each card type except plane are always single words and are listed after
///        a long dash. Each word after the dash is a separate subtype; such objects may have
///        multiple types. Subtypes of planes are also listed after a long dash, but may be
///        multiple words; all words after the dash are, collectively, a single subtype.
///
/// Example: “Basic Land — Mountain” means the card is a land with the subtype Mountain.
///          “Creature — Goblin Wizard” means the card is a creature with the subtypes Goblin
///          and Wizard. “Artifact — Equipment” means the card is an artifact with the subtype
///          Equipment.
///
/// 205.3c If a card with multiple card types has one or more subtypes, each subtype is
///        correlated to its appropriate card type.
///
/// Example: Dryad Arbor’s type line says “Land Creature — Forest Dryad.” Forest is a land type,
///          and Dryad is a creature type.
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub(crate) enum Subtype {
    Artifact(ArtifactType),
    Creature(CreatureType),
    Enchantment(EnchantmentType),
    Land(LandType),
    Plane(PlanarType),
    Planeswalker(PlaneswalkerType),
    Spell(SpellType),
}

/// 301.3. Artifact subtypes are always a single word and are listed after a long dash: “Artifact —
///        Equipment.” Artifact subtypes are also called artifact types. Artifacts may have multiple
///        subtypes. See rule 205.3g for the complete list of artifact types.
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub(crate) enum ArtifactType {
    Blood,
    Clue,
    Contraption,
    Equipment,
    Food,
    Fortification,
    Gold,
    Treasure,
    Vehicle,
}

/// 302.3. Creature subtypes are always a single word and are listed after a long dash: “Creature —
///        Human Soldier,” “Artifact Creature — Golem,” and so on. Creature subtypes are also called
///        creature types. Creatures may have multiple subtypes. See rule 205.3m for the complete
///        list of creature types.
///
/// Example: “Creature — Goblin Wizard” means the card is a creature with the subtypes Goblin and
///          Wizard.
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub(crate) enum CreatureType {
    Advisor,
    Aetherborn,
    Ally,
    Angel,
    Antelope,
    Ape,
    Archer,
    Archon,
    Army,
    Artificer,
    Assassin,
    AssemblyWorker,
    Atog,
    Aurochs,
    Avatar,
    Azra,
    Badger,
    Barbarian,
    Bard,
    Basilisk,
    Bat,
    Bear,
    Beast,
    Beeble,
    Beholder,
    Berserker,
    Bird,
    Blinkmoth,
    Boar,
    Bringer,
    Brushwagg,
    Camarid,
    Camel,
    Caribou,
    Carrier,
    Cat,
    Centaur,
    Cephalid,
    Chimera,
    Citizen,
    Cleric,
    Cockatrice,
    Construct,
    Coward,
    Crab,
    Crocodile,
    Cyclops,
    Dauthi,
    Demigod,
    Demon,
    Deserter,
    Devil,
    Dinosaur,
    Djinn,
    Dog,
    Dragon,
    Drake,
    Dreadnought,
    Drone,
    Druid,
    Dryad,
    Dwarf,
    Efreet,
    Egg,
    Elder,
    Eldrazi,
    Elemental,
    Elephant,
    Elf,
    Elk,
    Eye,
    Faerie,
    Ferret,
    Fish,
    Flagbearer,
    Fox,
    Fractal,
    Frog,
    Fungus,
    Gargoyle,
    Germ,
    Giant,
    Gnoll,
    Gnome,
    Goat,
    Goblin,
    God,
    Golem,
    Gorgon,
    Graveborn,
    Gremlin,
    Griffin,
    Hag,
    Halfling,
    Hamster,
    Harpy,
    Hellion,
    Hippo,
    Hippogriff,
    Homarid,
    Homunculus,
    Horror,
    Horse,
    Human,
    Hydra,
    Hyena,
    Illusion,
    Imp,
    Incarnation,
    Inkling,
    Insect,
    Jackal,
    Jellyfish,
    Juggernaut,
    Kavu,
    Kirin,
    Kithkin,
    Knight,
    Kobold,
    Kor,
    Kraken,
    Lamia,
    Lammasu,
    Leech,
    Leviathan,
    Lhurgoyf,
    Licid,
    Lizard,
    Manticore,
    Masticore,
    Mercenary,
    Merfolk,
    Metathran,
    Minion,
    Minotaur,
    Mole,
    Monger,
    Mongoose,
    Monk,
    Monkey,
    Moonfolk,
    Mouse,
    Mutant,
    Myr,
    Mystic,
    Naga,
    Nautilus,
    Nephilim,
    Nightmare,
    Nightstalker,
    Ninja,
    Noble,
    Noggle,
    Nomad,
    Nymph,
    Octopus,
    Ogre,
    Ooze,
    Orb,
    Orc,
    Orgg,
    Otter,
    Ouphe,
    Ox,
    Oyster,
    Pangolin,
    Peasant,
    Pegasus,
    Pentavite,
    Pest,
    Phelddagrif,
    Phoenix,
    Phyrexian,
    Pilot,
    Pincher,
    Pirate,
    Plant,
    Praetor,
    Prism,
    Processor,
    Rabbit,
    Ranger,
    Rat,
    Rebel,
    Reflection,
    Rhino,
    Rigger,
    Rogue,
    Sable,
    Salamander,
    Samurai,
    Sand,
    Saproling,
    Satyr,
    Scarecrow,
    Scion,
    Scorpion,
    Scout,
    Sculpture,
    Serf,
    Serpent,
    Servo,
    Shade,
    Shaman,
    Shapeshifter,
    Shark,
    Sheep,
    Siren,
    Skeleton,
    Slith,
    Sliver,
    Slug,
    Snake,
    Soldier,
    Soltari,
    Spawn,
    Specter,
    Spellshaper,
    Sphinx,
    Spider,
    Spike,
    Spirit,
    Splinter,
    Sponge,
    Squid,
    Squirrel,
    Starfish,
    Surrakar,
    Survivor,
    Tentacle,
    Tetravite,
    Thalakos,
    Thopter,
    Thrull,
    Tiefling,
    Treefolk,
    Trilobite,
    Triskelavite,
    Troll,
    Turtle,
    Unicorn,
    Vampire,
    Vedalken,
    Viashino,
    Volver,
    Wall,
    Warlock,
    Warrior,
    Weird,
    Werewolf,
    Whale,
    Wizard,
    Wolf,
    Wolverine,
    Wombat,
    Worm,
    Wraith,
    Wurm,
    Yeti,
    Zombie,
    Zubera,
}

/// 303.3. Enchantment subtypes are always a single word and are listed after a long dash:
///        “Enchantment — Shrine.” Each word after the dash is a separate subtype. Enchantment
///        subtypes are also called enchantment types. Enchantments may have multiple subtypes.
///        See rule 205.3h for the complete list of enchantment types.
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub(crate) enum EnchantmentType {
    Aura,
    Cartouche,
    Class,
    Curse,
    Rune,
    Saga,
    Shard,
    Shrine,
}

/// 305.5. Land subtypes are always a single word and are listed after a long dash. Land subtypes
///        are also called land types. Lands may have multiple subtypes. See rule 205.3i for the
///        complete list of land types.
///
/// Example: “Basic Land — Mountain” means the card is a land with the subtype Mountain.
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub(crate) enum LandType {
    Basic(BasicLandType),
    Desert,
    Gate,
    Lair,
    Locus,
    Mine,
    PowerPlant,
    Tower,
    Urzas,
}

/// 305.6. The basic land types are Plains, Island, Swamp, Mountain, and Forest. If an object uses
///        the words “basic land type,” it’s referring to one of these subtypes. An object with the
///        land card type and a basic land type has the intrinsic ability “{T}: Add [mana symbol],”
///        even if the text box doesn’t actually contain that text or the object has no text box.
///        For Plains, [mana symbol] is {W}; for Islands, {U}; for Swamps, {B}; for Mountains, {R};
///        and for Forests, {G}. See rule 107.4a. See also rule 605, “Mana Abilities.”
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub(crate) enum BasicLandType {
    Forest,
    Island,
    Mountain,
    Plains,
    Swamp,
}

/// 306.3. Planeswalker subtypes are always a single word and are listed after a long dash:
///        “Planeswalker — Jace.” Each word after the dash is a separate subtype. Planeswalker
///        subtypes are also called planeswalker types. Planeswalkers may have multiple subtypes.
///        See rule 205.3j for the complete list of planeswalker types.
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub(crate) enum PlaneswalkerType {
    Ajani,
    Aminatou,
    Angrath,
    Arlinn,
    Ashiok,
    Bahamut,
    Basri,
    Bolas,
    Calix,
    Chandra,
    Dack,
    Dakkon,
    Daretti,
    Davriel,
    Dihada,
    Domri,
    Dovin,
    Ellywick,
    Elspeth,
    Estrid,
    Freyalise,
    Garruk,
    Gideon,
    Grist,
    Huatli,
    Jace,
    Jaya,
    Jeska,
    Kaito,
    Karn,
    Kasmina,
    Kaya,
    Kiora,
    Koth,
    Liliana,
    Lolth,
    Lukka,
    Mordenkainen,
    Nahiri,
    Narset,
    Niko,
    Nissa,
    Nixilis,
    Oko,
    Ral,
    Rowan,
    Saheeli,
    Samut,
    Sarkhan,
    Serra,
    Sorin,
    Szat,
    Tamiyo,
    Teferi,
    Teyo,
    Tezzeret,
    Tibalt,
    Tyvar,
    Ugin,
    Venser,
    Vivien,
    Vraska,
    Will,
    Windgrace,
    Wrenn,
    Xenagos,
    Yanggu,
    Yanling,
    Zariel,
}

/// 304.3. Instant subtypes are always a single word and are listed after a long dash: “Instant —
///        Arcane.” Each word after the dash is a separate subtype. The set of instant subtypes is
///        the same as the set of sorcery subtypes; these subtypes are called spell types. Instants
///        may have multiple subtypes. See rule 205.3k for the complete list of spell types.
///
/// 307.3. Sorcery subtypes are always a single word and are listed after a long dash: “Sorcery —
///        Arcane.” Each word after the dash is a separate subtype. The set of sorcery subtypes is
///        the same as the set of instant subtypes; these subtypes are called spell types. Sorceries
///        may have multiple subtypes. See rule 205.3k for the complete list of spell types.
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub(crate) enum SpellType {
    Adventure,
    Arcane,
    Lesson,
    Trap,
}

/// 310.3. Plane subtypes are listed after a long dash, and may be multiple words: “Plane — Serra’s
///        Realm.” All words after the dash are, collectively, a single subtype. Planar subtypes are
///        called planar types. A plane can have only one subtype. See rule 205.3n for the complete
///        list of planar types.
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub(crate) enum PlanarType {
    Alara,
    Arkhos,
    Azgol,
    Belenon,
    BolassMeditationRealm,
    Dominaria,
    Equilor,
    Ergamon,
    Fabacin,
    Innistrad,
    Iquatana,
    Ir,
    Kaldheim,
    Kamigawa,
    Karsus,
    Kephalai,
    Kinshala,
    Kolbahan,
    Kyneth,
    Lorwyn,
    Luvion,
    Mercadia,
    Mirrodin,
    Moag,
    Mongseng,
    Muraganda,
    NewPhyrexia,
    Phyrexia,
    Pyrulea,
    Rabiah,
    Rath,
    Ravnica,
    Regatha,
    Segovia,
    SerrasRealm,
    Shadowmoor,
    Shandalar,
    Ulgrotha,
    Valla,
    Vryn,
    Wildfire,
    Xerex,
    Zendikar,
}

/// 205.4a An object can have one or more supertypes. A card’s supertypes are printed directly
///        before its card types. The supertypes are basic, legendary, ongoing, snow, and world.
///
/// 205.4b An object’s supertype is independent of its card type and subtype, even though some
///        supertypes are closely identified with specific card types. Changing an object’s card
///        types or subtypes won’t change its supertypes. Changing an object’s supertypes won’t
///        change its card types or subtypes. When an object gains or loses a supertype, it retains
///        any other supertypes it had.
///
/// Example: An ability reads, “All lands are 1/1 creatures that are still lands.” If any of the
///          affected lands were legendary, they are still legendary.
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub(crate) enum Supertype {
    Basic,
    Legendary,
    Ongoing,
    Snow,
    World,
}

/// 208.1. A creature card has two numbers separated by a slash printed in its lower right corner.
///        The first number is its power (the amount of damage it deals in combat); the second is
///        its toughness (the amount of damage needed to destroy it). For example, 2/3 means the
///        object has power 2 and toughness 3. Power and toughness can be modified or set to
///        particular values by effects.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub(crate) struct PtCharacteristic {
    pub(crate) power: PtValue,
    pub(crate) toughness: PtValue,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub(crate) enum PtValue {
    Fixed(i64),
    /// 208.2. Rather than a fixed number, some creature cards have power and/or toughness that
    ///        includes a star (*).
    /// 208.2a The card may have a characteristic-defining ability that sets its power and/or
    ///        toughness according to some stated condition. (See rule 604.3.) Such an ability is
    ///        worded “[This creature’s] [power or toughness] is equal to . . .” or
    ///        “[This creature’s] power and toughness are each equal to . . .” This ability
    ///        functions everywhere, even outside the game. If the ability needs to use a number
    ///        that can’t be determined, including inside a calculation, use 0 instead of that
    ///        number.
    ///
    /// Example: Lost Order of Jarkeld has power and toughness each equal to 1+*. It has the
    ///          abilities “As Lost Order of Jarkeld enters the battlefield, choose an opponent” and
    ///          “Lost Order of Jarkeld’s power and toughness are each equal to 1 plus the number of
    ///          creatures the chosen player controls.” While Lost Order of Jarkeld isn’t on the
    ///          battlefield, there won’t be a chosen player. Its power and toughness will each be
    ///          equal to 1 plus 0, so it’s 1/1.
    Variable,
}

/// 400.1. A zone is a place where objects can be during a game. There are normally seven zones:
///        library, hand, battlefield, graveyard, stack, exile, and command. Some older cards also
///        use the ante zone. Each player has their own library, hand, and graveyard. The other
///        zones are shared by all players.
#[derive(PartialEq, Eq)]
pub(crate) enum Zone {
    Library(PlayerId),
    Hand(PlayerId),
    Battlefield,
    Graveyard(PlayerId),
    Stack,
    Exile,
    Command,
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn color_order_is_stable() {
        let colors = BTreeSet::from([
            Color::Black,
            Color::Blue,
            Color::Green,
            Color::Red,
            Color::White,
        ]);
        let mut iter = colors.iter();
        assert_eq!(iter.next(), Some(&Color::White));
        assert_eq!(iter.next(), Some(&Color::Blue));
        assert_eq!(iter.next(), Some(&Color::Black));
        assert_eq!(iter.next(), Some(&Color::Red));
        assert_eq!(iter.next(), Some(&Color::Green));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn lands_are_colorless() {
        // Our default empty card has no mana cost or color indicator, thus it behaves like a land.
        let card = Card::builder().build().unwrap();
        assert_eq!(card.color(), ColorKind::Colorless);
    }

    #[test]
    fn generic_mana_is_colorless() {
        // Build a card with mana cost {1}.
        let card = Card::builder()
            .mana_cost(ManaCost([ManaSymbol::Generic(1)].into()))
            .build()
            .unwrap();
        assert_eq!(card.color(), ColorKind::Colorless);
    }

    #[test]
    fn mixed_mana_costs_have_the_correct_color() {
        // Build a card with mana cost {2}{B}{B}{G}.
        let card = Card::builder()
            .mana_cost(ManaCost(
                [
                    ManaSymbol::Generic(2),
                    ManaSymbol::Colored(Color::Black),
                    ManaSymbol::Colored(Color::Black),
                    ManaSymbol::Colored(Color::Green),
                ]
                .into(),
            ))
            .build()
            .unwrap();
        assert_eq!(
            card.color(),
            ColorKind::Multicolored([Color::Black, Color::Green].into())
        );
    }
}
