use hecs::Bundle;
use indexmap::IndexSet;

use crate::core::{CardType, Color, PlayerId, Subtype, Supertype, Zone};

/// 109.1. An object is an ability on the stack, a card, a copy of a card, a token, a spell, a
///        permanent, or an emblem.
pub(crate) struct Object;

/// 201.2. A card’s name is always considered to be the English version of its name, regardless of
///        printed language.
pub(crate) struct Name(pub(crate) String);

/// 108.3. The owner of a card in the game is the player who started the game with it in their deck.
///        If a card is brought into the game from outside the game rather than starting in a
///        player’s deck, its owner is the player who brought it into the game. If a card starts the
///        game in the command zone, its owner is the player who put it into the command zone to
///        start the game. Legal ownership of a card in the game is irrelevant to the game rules
///        except for the rules for ante. (See rule 407.)
pub(crate) struct Owner(pub(crate) PlayerId);

/// 109.4. Only objects on the stack or on the battlefield have a controller. Objects that are
///        neither on the stack nor on the battlefield aren’t controlled by any player. See rule
///        108.4.
pub(crate) struct Controller(pub(crate) PlayerId);

/// 205.1. The type line is printed directly below the illustration. It contains the card’s card
///        type(s). It also contains the card’s subtype(s) and supertype(s), if applicable.
pub(crate) struct ObjectTypeLine {
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

/// 109.3. An object’s characteristics are name, mana cost, color, color indicator, card type,
///        subtype, supertype, rules text, abilities, power, toughness, loyalty, hand modifier, and
///        life modifier. Objects can have some or all of these characteristics. Any other
///        information about an object isn’t a characteristic. For example, characteristics don’t
///        include whether a permanent is tapped, a spell’s target, an object’s owner or controller,
///        what an Aura enchants, and so on.
#[derive(Bundle)]
pub(crate) struct ObjectBundle {
    pub(crate) object: Object,
    pub(crate) name: Name,
    pub(crate) color: Color,
    pub(crate) owner: Owner,
    pub(crate) object_type_line: ObjectTypeLine,
    pub(crate) zone: Zone,
}
