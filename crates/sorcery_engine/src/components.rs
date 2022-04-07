use crate::core::PlayerId;

/// 109.1. An object is an ability on the stack, a card, a copy of a card, a token, a spell, a
///        permanent, or an emblem.
///
/// 109.3. An object’s characteristics are name, mana cost, color, color indicator, card type,
///        subtype, supertype, rules text, abilities, power, toughness, loyalty, hand modifier, and
///        life modifier. Objects can have some or all of these characteristics. Any other
///        information about an object isn’t a characteristic. For example, characteristics don’t
///        include whether a permanent is tapped, a spell’s target, an object’s owner or controller,
///        what an Aura enchants, and so on.
pub(crate) struct Object;

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
