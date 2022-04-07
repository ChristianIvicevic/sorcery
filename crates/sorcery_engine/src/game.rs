use std::collections::HashMap;

use crate::components::{Object, Owner};
use hecs::{Entity, EntityBuilder, World};
use once_cell::sync::Lazy;

use crate::core::{Card, Player, PlayerId, Zone};

/// A statically loaded database of all cards that can be used as templates to spawn new instances.
static CARD_DATABASE: Lazy<Vec<Card>> = Lazy::new(|| {
    let database = include_str!("./cards.json");
    serde_json::from_str(database).unwrap()
});

/// Returns a reference to the first card with the specified name. In case multiple cards share the
/// same name, i.e. lands or reprints in different sets, there is no guarantee the same card will be
/// selected on subsequent calls.
pub(crate) fn find_card_by_name(name: &str) -> Option<&'static Card> {
    CARD_DATABASE.iter().find(|it| it.name.0 == name)
}

/// 100.1. These Magic rules apply to any Magic game with two or more players, including two-player
///        games and multiplayer games.
pub struct Game {
    world: World,
    players: Vec<Player>,
    libraries: HashMap<PlayerId, Library>,
}

impl Game {
    /// 119.1. Each player begins the game with a starting life total of 20. Some variant games have
    ///        different starting life totals.
    pub fn new(players: u32) -> Self {
        let players = (0..players)
            .map(|it| Player {
                id: PlayerId(it),
                life: 20,
                name: format!("Player {}", it + 1),
            })
            .collect::<Vec<_>>();

        let mut libraries = HashMap::new();
        for player in &players {
            libraries.insert(player.id, Library::default());
        }

        Self {
            world: World::new(),
            players,
            libraries,
        }
    }

    /// 100.2. To play, each player needs their own deck of traditional Magic cards, small items to
    ///        represent any tokens and counters, and some way to clearly track life totals.
    ///
    /// 100.4. Each player may also have a sideboard, which is a group of additional cards the
    ///        player may use to modify their deck between games of a match.
    ///
    /// 103.1. At the start of a game, the players determine which one of them will choose who takes
    ///        the first turn. In the first game of a match (including a single-game match), the
    ///        players may use any mutually agreeable method (flipping a coin, rolling dice, etc.)
    ///        to do so. In a match of several games, the loser of the previous game chooses who
    ///        takes the first turn. If the previous game was a draw, the player who made the choice
    ///        in that game makes the choice in this game. The player chosen to take the first turn
    ///        is the starting player. The game’s default turn order begins with the starting player
    ///        and proceeds clockwise.
    ///
    /// 103.2. After the starting player has been determined, each player shuffles their deck so
    ///        that the cards are in a random order. Each player may then shuffle or cut their
    ///        opponents’ decks. The players’ decks become their libraries.
    pub fn start() {
        // TODO: Implement rule 103.1. For now we just start with player 1.
    }

    /// Returns a slice of players within the current game.
    pub(crate) fn players(&self) -> &[Player] {
        &self.players
    }

    /// Returns a mutable reference of the internal world that stores all entities. This method is
    /// only available to conveniently setup the game world from within tests and will be most
    /// likely be removed once the core gameplay loop is implemented.
    #[cfg(test)]
    pub(crate) fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    /// Spawns an instance of a [`Card`] in the specified [`Zone`].
    pub(crate) fn spawn_object(&mut self, card: &Card, zone: &Zone) {
        let mut builder = EntityBuilder::new();
        builder
            .add(Object)
            .add(card.name.clone())
            .add(card.type_line.clone())
            .add(card.expansion_symbol.clone())
            .add(card.rules_text.clone())
            .add(card.collector_number)
            .add(card.color());

        if let Some(ref mana_cost) = card.mana_cost {
            builder.add(mana_cost.clone());
        }
        if let Some(pt) = card.pt {
            builder.add(pt);
        }
        if let Some(loyalty) = card.loyalty {
            builder.add(loyalty);
        }

        match zone {
            &Zone::Library(owner) => {
                builder
                    // 108.3. The owner of a card in the game is the player who started the game
                    //        with it in their deck. [...]
                    .add(Owner(owner))
                    .add(Zone::Library(owner));

                let entity = self.world.spawn(builder.build());
                self.libraries
                    .get_mut(&owner)
                    .unwrap_or_else(|| {
                        panic!(
                            "Could not access the library of player with id {}.",
                            owner.0
                        )
                    })
                    .cards
                    .push(entity);
            }
            _ => unimplemented!(),
        }
    }
}

/// 401.1. When a game begins, each player’s deck becomes their library.
#[derive(Default)]
pub(crate) struct Library {
    pub(crate) cards: Vec<Entity>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Zone;
    use hecs::With;

    #[test]
    #[allow(clippy::needless_collect)]
    fn sample_game() {
        let mut game = Game::new(2);

        let mut players = game.players().iter();
        let first_player = players.next().unwrap().id;
        let second_player = players.next().unwrap().id;

        let mut white_deck = Vec::new();
        for _ in 0..20 {
            white_deck.push(find_card_by_name("Plains").unwrap());
            white_deck.push(find_card_by_name("Soulmender").unwrap());
        }
        for card in white_deck {
            game.spawn_object(card, &Zone::Library(first_player));
        }

        let mut green_deck = Vec::new();
        for _ in 0..20 {
            green_deck.push(find_card_by_name("Forest").unwrap());
            green_deck.push(find_card_by_name("Llanowar Elves").unwrap());
        }
        for card in green_deck {
            game.spawn_object(card, &Zone::Library(second_player));
        }

        let mut objects = game.world_mut().query::<With<Object, &Zone>>();

        let white_library = objects
            .iter()
            .filter(|(_, zone)| **zone == Zone::Library(first_player))
            .collect::<Vec<_>>();
        assert_eq!(white_library.len(), 40);

        let green_library = objects
            .iter()
            .filter(|(_, zone)| **zone == Zone::Library(second_player))
            .collect::<Vec<_>>();
        assert_eq!(green_library.len(), 40);
    }
}
