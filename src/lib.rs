#![cfg_attr(not(any(test, feature = "std")), no_std)]

use ink_core::{
    env::println,
    env::AccountId,
    memory::format,
    storage,
};
use ink_lang::contract;

// To win - s1 * s2 * s3 must equal 1 for X
// s1 * s2 * s3 must equal 8 for O
// Any 0's in a row will cause final result to be 0
// If any 2's, will invalidate X's
// If any 1's, will invalidate O's

const BLANK: i8 = 0;
const X: i8 = 1;
const O: i8 = 2;

const DRAW: i8 = 0;
const X_WINS: i8 = 1;
const O_WINS: i8 = 8;
const ONGOING: i8 = 3;

contract! {
    struct InkTacToe {
        /// The current player turn (true = x, false = o)
        turn: storage::Value<bool>,
        // x player account ID
        x_player: storage::Value<AccountId>,
        // o player account ID
        o_player: storage::Value<AccountId>,

        spot_0_0: storage::Value<i8>,
        spot_0_1: storage::Value<i8>,
        spot_0_2: storage::Value<i8>,
        spot_1_0: storage::Value<i8>,
        spot_1_1: storage::Value<i8>,
        spot_1_2: storage::Value<i8>,
        spot_2_0: storage::Value<i8>,
        spot_2_1: storage::Value<i8>,
        spot_2_2: storage::Value<i8>,

    }

    impl Deploy for InkTacToe {
        
        fn deploy(&mut self) {
            // Initializes our state to true (x player)

            self.turn.set(true);

            // No account IDs set yet
            // self.x_player.set(AccountId::try_from([0x0; 32]).unwrap());
            // self.o_player.set(AccountId::try_from([0x0; 32]).unwrap());

            // Clear the board
            self.spot_0_0.set(0);
            self.spot_0_1.set(0);
            self.spot_0_2.set(0);
            self.spot_1_0.set(0);
            self.spot_1_1.set(0);
            self.spot_1_2.set(0);
            self.spot_2_0.set(0);
            self.spot_2_1.set(0);
            self.spot_2_2.set(0);

            
        }
    }

    // Public interface
    
    impl InkTacToe {

        pub(external) fn claim_x_player(&mut self) {
            *self.x_player = env.caller();
            println(&format!("X player is now: {:?}", *self.x_player));
            
        }

        pub(external) fn claim_o_player(&mut self) {
            *self.o_player = env.caller();
            println(&format!("O player is now: {:?}", *self.o_player));

        }
        
        pub(external) fn take_turn(&mut self, x: i8, y: i8) {
            println(&format!("Before Storage Turn: {:?}", *self.turn));
            self.flip();
            println(&format!("After Storage Turn: {:?}", *self.turn));
            let status: i8 = self.win();
            println(&format!("Status: {:?}", status));
        }
        
    }

    // Private methods
    
    impl InkTacToe {
        fn flip(&mut self) {
            if *self.turn == true {
                *self.turn = false;
            } else {
                *self.turn = true;
            }
        }

        fn win(&self) -> i8 {

            let mut result: i8 = ONGOING;
            let mut more_turns: bool = false;
            
            let a: [i8; 8] = [
                *self.spot_0_0 * *self.spot_0_1 * *self.spot_0_2, // row 1
                *self.spot_1_0 * *self.spot_1_1 * *self.spot_1_2, // row 2
                *self.spot_2_0 * *self.spot_2_1 * *self.spot_2_2, // row 3
                *self.spot_0_0 * *self.spot_1_0 * *self.spot_2_0, // column 1
                *self.spot_0_1 * *self.spot_1_1 * *self.spot_2_1, // column 2
                *self.spot_0_2 * *self.spot_1_2 * *self.spot_2_2, // column 3
                *self.spot_0_0 * *self.spot_1_1 * *self.spot_2_2, // ul -> lr diagonal
                *self.spot_0_2 * *self.spot_1_1 * *self.spot_2_0, // ur -> ll diagonal
            ];

            for &elem in a.iter() {
                match elem {
                    0 => more_turns = true, // there are more spaces to select
                    X_WINS => return X_WINS, // X has won!
                    O_WINS => return O_WINS, // O has won!
                    _ => (), // full but mixed-row
                }
            }
            
            if more_turns {
                return ONGOING
            } else {
                return DRAW
            }
            
        }
    }
}

// #[cfg(all(test, feature = "test-env"))]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let mut contract = InkTacToe::deploy_mock();
//         assert_eq!(contract.get(), false);
//         contract.flip();
//         assert_eq!(contract.get(), true);
//     }
// }
