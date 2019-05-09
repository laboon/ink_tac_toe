#![cfg_attr(not(any(test, feature = "std")), no_std)]

use ink_core::{
    env::println,
    env::AccountId,
    memory::format,
    storage,
};
use ink_lang::contract;

contract! {
    struct InkTacToe {
        /// The current player turn (true = x, false = o)
        turn: storage::Value<bool>,
        // x player account ID
        x_player: storage::Value<AccountId>,
        // o player account ID
        o_player: storage::Value<AccountId>,

        spot_0_0: storage::Value<u32>,
        spot_0_1: storage::Value<u32>,
        spot_0_2: storage::Value<u32>,
        spot_1_0: storage::Value<u32>,
        spot_1_1: storage::Value<u32>,
        spot_1_2: storage::Value<u32>,
        spot_2_0: storage::Value<u32>,
        spot_2_1: storage::Value<u32>,
        spot_2_2: storage::Value<u32>,

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
        }

        pub(external) fn claim_o_player(&mut self) {
            *self.o_player = env.caller();
        }
        
        pub(external) fn take_turn(&mut self) {
            println(&format!("Before Storage Turn: {:?}", *self.turn));
            self.flip();
            println(&format!("After Storage Turn: {:?}", *self.turn));
            
        }
        
        /// Returns the current state.
        pub(external) fn get(&self) -> bool {
            println(&format!("Storage Turn: {:?}", *self.turn));
            *self.turn
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
