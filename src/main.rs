use sha2::{
    digest::{generic_array::GenericArray, typenum::U32},
    Digest,
};

fn main() {
    let salt = "Hello world!".as_bytes();

    let hash: GenericArray<u8, U32> = sha2::Sha256::new().chain_update(salt).finalize();

    println!("HASH: {hash:?}")
}
