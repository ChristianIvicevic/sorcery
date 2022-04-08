use std::collections::HashMap;

use hecs::{Entity, EntityBuilder, World};
use once_cell::sync::Lazy;
use rand::prelude::SliceRandom;

use crate::{
    components::{Object, Owner},
    core::{Card, Deck, Player, PlayerId, Zone},
};

/// A statically loaded database of all cards that can be used as templates to spawn new instances.
static CARD_DATABASE: Lazy<Vec<Card>> = Lazy::new(|| {
    let database = include_str!("./cards.json");
    serde_json::from_str(database).expect("Could not initialize the card database.")
});

/// Returns a reference to the first card with the specified name. In case multiple cards share the
/// same name, i.e. lands or reprints in different sets, there is no guarantee the same card will be
/// selected on subsequent calls.
pub(crate) fn find_card_by_name(name: &str) -> Option<&'_ Card> {
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

        let libraries = players
            .iter()
            .map(|it| (it.id, Library::default()))
            .collect();

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
    pub fn start(&mut self, decks: &HashMap<PlayerId, Deck>) {
        assert_eq!(decks.len(), self.players.len());
        // TODO: Implement rule 103.1. For now we just implicitly start with player 1.

        for (&id, deck) in decks.iter() {
            for card in deck.cards() {
                self.spawn_object(card, &Zone::Library(id));
            }
        }

        for library in self.libraries.values_mut() {
            library.shuffle();
        }
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
struct Library {
    cards: Vec<Entity>,
}

impl Library {
    /// Shuffles the library using a thread-local random number generator.
    fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }
}

#[cfg(test)]
mod tests {
    use hecs::With;

    use super::*;

    #[test]
    #[allow(clippy::needless_collect)]
    fn sample_game() {
        let mut game = Game::new(2);

        let white_deck = Deck::from(&[("Plains", 30), ("Soulmender", 30)]);
        let green_deck = Deck::from(&[("Forest", 30), ("Llanowar Elves", 30)]);

        let mut players = game.players().iter();
        let first_player = players.next().expect("Could not get the first player.").id;
        let second_player = players.next().expect("Could not get the second player.").id;

        game.start(&[(first_player, white_deck), (second_player, green_deck)].into());

        let mut objects = game.world_mut().query::<With<Object, &Zone>>();

        let white_library = objects
            .iter()
            .filter(|(_, zone)| **zone == Zone::Library(first_player))
            .collect::<Vec<_>>();
        assert_eq!(white_library.len(), 60);

        let green_library = objects
            .iter()
            .filter(|(_, zone)| **zone == Zone::Library(second_player))
            .collect::<Vec<_>>();
        assert_eq!(green_library.len(), 60);
    }
}
