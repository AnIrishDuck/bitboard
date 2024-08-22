use crate::rank::Rank;
use serde::{Deserialize, Serialize};
use std::ops::Not;

/// Represent a player.
#[derive(PartialOrd, PartialEq, Eq, Copy, Clone, Debug, Hash, Serialize, Deserialize)]
pub enum Player {
    White,
    Black,
}

/// How many players are there?
pub const NUM_PLAYERS: usize = 2;
/// List all players
pub const ALL_PLAYERS: [Player; NUM_PLAYERS] = [Player::White, Player::Black];

impl Player {
    /// Convert the `Player` to a `usize` for table lookups.
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }

    /// Convert a `Player` to my backrank, which represents the starting rank
    /// for my pieces.
    #[inline]
    pub fn to_my_backrank(&self) -> Rank {
        match *self {
            Player::White => Rank::First,
            Player::Black => Rank::Eighth,
        }
    }

    /// Convert a `Player` to my opponents backrank, which represents the starting rank for the
    /// opponents pieces.
    #[inline]
    pub fn to_their_backrank(&self) -> Rank {
        match *self {
            Player::White => Rank::Eighth,
            Player::Black => Rank::First,
        }
    }

    /// Convert a `Player` to my second rank, which represents the starting rank for my pawns.
    #[inline]
    pub fn to_second_rank(&self) -> Rank {
        match *self {
            Player::White => Rank::Second,
            Player::Black => Rank::Seventh,
        }
    }

    /// Convert a `Player` to my fourth rank, which represents the rank of my pawns when
    /// moving two squares forward.
    #[inline]
    pub fn to_fourth_rank(&self) -> Rank {
        match *self {
            Player::White => Rank::Fourth,
            Player::Black => Rank::Fifth,
        }
    }

    /// Convert a `Player` to my seventh rank, which represents the rank before pawn promotion.
    #[inline]
    pub fn to_seventh_rank(&self) -> Rank {
        match *self {
            Player::White => Rank::Seventh,
            Player::Black => Rank::Second,
        }
    }

    pub fn dot(&self) -> &'static str {
        match self {
            Player::White => "w",
            Player::Black => "b",
        }
    }
}

impl Not for Player {
    type Output = Player;

    /// Get the other player.
    #[inline]
    fn not(self) -> Player {
        if self == Player::White {
            Player::Black
        } else {
            Player::White
        }
    }
}
