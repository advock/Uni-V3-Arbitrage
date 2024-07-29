use ethers::prelude::*;

pub fn convert_to_address(address: &str) -> Address {
    address.parse::<Address>().unwrap()
}

pub fn bind(name: &str, abi: &str) {
    let name: String = name.to_string();
    let bindings = Abigen::new(&name, abi).unwrap().generate().unwrap();
    let path: String = format!("src/contract_modules/bindings/{}.rs", name);
    match std::fs::File::create(path.clone()) {
        Ok(_) => {}
        Err(_) => {}
    }
    bindings.write_to_file(&path).unwrap();
}
