use ethers::{
    prelude::{k256::ecdsa::SigningKey, Http, TransactionRequest},
    providers::{Middleware, Provider},
    signers::{Signer, Wallet},
    types::H256,
};
use std::str::FromStr;

struct NxWallet {
    wallet: Wallet<SigningKey>,
    provider: Provider<Http>,
}

impl NxWallet {
    async fn send_transaction(
        &self,
        tx: TransactionRequest,
    ) -> Result<H256, Box<dyn std::error::Error>> {
        // Sign the transaction
        // let signer: Signer = &self.wallet.try_into().unwrap();
        let signed_tx = self.provider.sign(tx, &self.provider).await?;

        // Send the signed transaction
        let tx_hash = self.provider.send_transaction(signed_tx).await?;

        Ok(tx_hash)
    }
}
