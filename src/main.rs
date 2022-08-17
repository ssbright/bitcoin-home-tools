extern crate bitcoincore_rpc;
extern crate dotenv;
extern crate dotenv_codegen;


use dotenv_codegen::dotenv;
use bitcoincore_rpc::{Auth, Client, RpcApi};




fn main() {

    let rpc = Client::new(&dotenv!("IP").to_string(),
                          Auth::UserPass(dotenv!("RPC_USER").to_string(),
                                         dotenv!("RPC_PASSWORD").to_string())).unwrap();
    let best_block_hash = rpc.get_best_block_hash().unwrap();
    let get_mining_info = rpc.get_mining_info().unwrap();
    let get_wallet_info = rpc.get_wallet_info().unwrap();
    //let import_address = rpc.import_address(&dotenv!("W1x").to_string(), "testing", false).unwrap();
    println!("best block hash: {}", best_block_hash);
    println!("get mining info: {:?}", get_mining_info);
    println!("get wallet info: {:?}", get_wallet_info);
   // println!("import address: {:?}", import_address);
}

