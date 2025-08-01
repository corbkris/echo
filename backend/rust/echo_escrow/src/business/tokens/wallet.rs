use std::str::FromStr;

use bip32::{DerivationPath, XPrv};
use bip39::{Language, Mnemonic};
use bitcoin::{
    PublicKey as BitcoinPubKey, address::Address, network::Network,
    secp256k1::PublicKey as SecpPublicKey,
};
use bs58;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

const XRP_ADDRESS_PREFIX: u8 = 0x00;

pub fn generate_xmr_wallet() {}

pub fn generate_xrp_wallet() {
    let mnemonic = Mnemonic::generate_in(Language::English, 24).unwrap();
    let seed = mnemonic.to_seed("get from user");

    let mut xprv = XPrv::new(seed).unwrap();
    let path = DerivationPath::from_str("m/44'/144'/0'/0/0").unwrap();

    for child_number in path.into_iter() {
        xprv = xprv.derive_child(child_number).unwrap();
    }

    let pubkey_bytes = xprv.public_key().to_bytes();

    let sha256 = Sha256::digest(pubkey_bytes);
    let ripmd = Ripemd160::digest(sha256);

    let mut address_bytes = Vec::with_capacity(21);
    address_bytes.push(XRP_ADDRESS_PREFIX);
    address_bytes.extend_from_slice(&ripmd);

    let xrp_address = bs58::encode(address_bytes).with_check().into_string();

    println!("XRP Address: {}", xrp_address);
}

pub fn generate_btc_wallet() {
    let mnemonic = Mnemonic::generate_in(Language::English, 24).unwrap();
    let seed = mnemonic.to_seed("get from user");

    let mut xprv = XPrv::new(seed).unwrap();
    let path = DerivationPath::from_str("m/44'/0'/0'/0/0").unwrap();

    for child_number in path.into_iter() {
        xprv = xprv.derive_child(child_number).unwrap();
    }

    let pubkey_bytes = xprv.public_key().to_bytes();
    let secp_pubkey = SecpPublicKey::from_slice(&pubkey_bytes).unwrap();

    let bitcoin_pubkey = BitcoinPubKey {
        compressed: true,
        inner: secp_pubkey,
    };

    let btc_address = Address::p2pkh(bitcoin_pubkey, Network::Bitcoin);
    println!("{}", btc_address)
}
