use crate::{Player, PlayerId};

/// 100.1. These Magic rules apply to any Magic game with two or more players, including two-player
///        games and multiplayer games.
pub struct Game {
    world: hecs::World,
    players: Vec<Player>,
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
            .collect();

        Self {
            world: hecs::World::new(),
            players,
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
    pub(crate) fn world_mut(&mut self) -> &mut hecs::World {
        &mut self.world
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        components::{Name, Object, ObjectBundle, ObjectType, Owner},
        BasicLandType, CardType, Color, LandType, Subtype, Supertype, Zone,
    };

    #[test]
    fn sample_game() {
        let mut game = Game::new(2);

        let mut players = game.players().iter();
        let first_player = players.next().unwrap().id;

        let _forest_card_entity = game.world_mut().spawn(ObjectBundle {
            color: Color::Green,
            name: Name("Forest".into()),
            object: Object,
            object_type: ObjectType {
                card_type: [CardType::Land].into(),
                subtype: [Subtype::Land(LandType::Basic(BasicLandType::Forest))].into(),
                supertype: [Supertype::Basic].into(),
            },
            owner: Owner(first_player),
            zone: Zone::Library(first_player),
        });
    }
}
