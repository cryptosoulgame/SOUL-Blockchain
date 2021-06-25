use crate::mock::*;
use frame_support::assert_ok;
use pallet_nft as NFT;
use primitive_types::U256;

#[test]
fn it_works_for_default_value() {
    new_test_ext(vec![1, 2]).execute_with(|| {
        assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            NFT::Types { tape: 2 }
        ));
    })
}

#[test]
fn mint_some_type() {
    new_test_ext(vec![1]).execute_with(|| {
        assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            NFT::Types { tape: 1 }
        ));
    })
}

#[test]
fn mint_and_transfer() {
    new_test_ext(vec![1, 2]).execute_with(|| {
        assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            NFT::Types { tape: 1 }
        ));
        assert_ok!(RealisGameApi::transfer_basic_nft(
            Origin::signed(1),
            2,
            U256([1, 0, 0, 0])
        ));
    })
}

#[test]
fn mint_and_burn() {
    new_test_ext(vec![1]).execute_with(|| {
        assert_ok!(RealisGameApi::mint_basic_nft(
            Origin::signed(1),
            1,
            U256([1, 0, 0, 0]),
            NFT::Types { tape: 1 }
        ));
        assert_ok!(RealisGameApi::burn_basic_nft(
            Origin::signed(1),
            U256([1, 0, 0, 0])
        ));
    })
}
