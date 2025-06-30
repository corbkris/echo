use bitcoin::{
    Amount, EcdsaSighashType, Script, ScriptBuf, Sequence, TxOut, Witness,
    address::Address,
    blockdata::script,
    consensus::encode,
    ecdsa::Signature,
    hashes::{Hash, sha256d},
    hex::FromHex,
    key::{PrivateKey, PublicKey},
    locktime::absolute::LockTime,
    network::Network,
    opcodes::{OP_0, all::OP_CHECKMULTISIG},
    script::PushBytesBuf,
    secp256k1::{Message, Secp256k1, SecretKey},
    sighash::SighashCache,
    transaction::{OutPoint, Transaction, TxIn, Version},
};
use reqwest::Client;
use secp256k1::rand::{RngCore, rngs::OsRng};

pub fn create_btc_address() {
    let mut rng = OsRng;
    let mut secret_key = [0u8; 32];
    rng.fill_bytes(&mut secret_key);

    let sk = SecretKey::from_slice(&secret_key).expect("failed to crete secret key");

    let private_key = PrivateKey {
        compressed: true,
        network: Network::Bitcoin.into(),
        inner: sk,
    };

    let secp = Secp256k1::new();
    let public_key = private_key.public_key(&secp);

    let address = Address::p2pkh(public_key, Network::Bitcoin);

    println!("private key: {}", private_key.to_wif());
    println!("public key: {}", public_key);
    println!("address: {}", address)
}

pub fn create_multisig_address(escrow: PublicKey, buyer: PublicKey, seller: PublicKey) {
    let redeem_script = script::Builder::new()
        .push_int(2)
        .push_key(&escrow)
        .push_key(&buyer)
        .push_key(&seller)
        .push_int(3)
        .push_opcode(OP_CHECKMULTISIG)
        .into_script();

    let script_hash = redeem_script.to_p2sh();

    let multisig_address = Address::p2sh(&script_hash, Network::Bitcoin);
    println!("address: {:?}", multisig_address);
}

pub fn create_transaction(
    middleman: Address,
    seller: Address,
    driver: Address,
    redeem_script: ScriptBuf,
) {
    // watch for funds in multi_signature_address
    // use API to get these values
    let prev_txid = sha256d::Hash::from_slice(&Vec::from_hex("").unwrap()).unwrap();
    let prev_vout = 0;

    let redeem_total = 100000000f64;
    let middleman_total = 0.005 * redeem_total;
    let seller_total = 0.99 * redeem_total;
    let driver_total = 0.005 * redeem_total;

    let prev_out = OutPoint {
        txid: prev_txid.into(),
        vout: prev_vout,
    };

    let mut tx = Transaction {
        version: Version::ONE,
        lock_time: LockTime::ZERO,
        input: vec![TxIn {
            previous_output: prev_out,
            script_sig: Script::new().into(),
            sequence: Sequence::MAX,
            witness: Witness::new(),
        }],
        output: vec![],
    };

    let seller_script = seller.script_pubkey();
    tx.output.push(TxOut {
        value: Amount::from_btc(seller_total).unwrap(),
        script_pubkey: seller_script,
    });

    let middleman_script = middleman.script_pubkey();
    tx.output.push(TxOut {
        value: Amount::from_btc(middleman_total).unwrap(),
        script_pubkey: middleman_script,
    });

    let driver_script = driver.script_pubkey();
    tx.output.push(TxOut {
        value: Amount::from_btc(driver_total).unwrap(),
        script_pubkey: driver_script,
    });

    let sighash = SighashCache::new(&tx)
        .p2wsh_signature_hash(
            0,
            &redeem_script,
            Amount::from_btc(redeem_total).unwrap(),
            EcdsaSighashType::All,
        )
        .unwrap();

    let message = Message::from_digest(sighash.to_byte_array());
    println!("message: {}", message);
}

pub fn sign_transaction(message: Message, secret_key: SecretKey) {
    let secp = Secp256k1::new();
    let sig = secp.sign_ecdsa(&message, &secret_key);
    println!("signature: {}", sig);
}

pub fn add_signatures_to_transaction(
    mut tx: Transaction,
    redeem_script: ScriptBuf,
    buyer_sig: Signature,
    seller_sig: Signature,
) {
    let transaction_signature = script::Builder::new()
        .push_opcode(OP_0)
        .push_slice(buyer_sig.serialize())
        .push_slice(seller_sig.serialize())
        .push_slice(PushBytesBuf::try_from(redeem_script.as_bytes().to_vec()).unwrap())
        .into_script();

    tx.input[0].script_sig = transaction_signature;
}

pub async fn broadcast_transaction(tx: &Transaction) -> Result<(), Box<dyn std::error::Error>> {
    let tx_hex = encode::serialize_hex(tx);

    let client = Client::new();
    let url = "https://blockstream.info/testnet/api/tx";

    let res = client.post(url).body(tx_hex.clone()).send().await?;

    if res.status().is_success() {
        println!("Transaction broadcasted successfully!");
        println!(
            "View at: https://blockstream.info/testnet/tx/{}",
            res.text().await?
        );
    } else {
        let err_msg = res.text().await?;
        println!("Failed to broadcast transaction: {}", err_msg);
    }

    Ok(())
}
