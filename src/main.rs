use rand::{distributions::Alphanumeric, Rng};
use sled::IVec;

fn main() -> sled::Result<()> {
    let path = "storage";
    let db = sled::open(path)?;

    // key and value types can be `Vec<u8>`, `[u8]`, `str`.
    let random_storage_key = create_random_string(14);
    let random_string_data = create_random_string(14);

    let contents = String::from(random_string_data).into_bytes();

    db.insert(&random_storage_key, contents)?;
    db.get(&random_storage_key)?;

    let result = db.get(&random_storage_key)?;
    let results_as_string = result_to_string(result);

    println!(
        "Value for the key:{:?} {:?}",
        &random_storage_key, &results_as_string
    );

    Ok(())
}

fn bytes_to_string(bytes: Vec<u8>) -> String {
    let string = String::from_utf8(bytes);

    match string {
        Ok(value) => value,
        Err(error) => panic!("Invalid UTF-8 Sequence provided: {}", error),
    }
}

fn result_to_string(result: Option<IVec>) -> String {
    bytes_to_string(result.unwrap().to_vec())
}

fn create_random_string(num: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(num)
        .map(char::from)
        .collect()
}
