use std::convert::TryInto;
use sha1::{Sha1, Digest};

fn left_rotate(value: u32, bits: u32) -> u32 {
    (value << bits) | (value >> (32 - bits))
}
fn sha11(input: &[u8]) -> String{
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xEFCDAB89;
    let mut h2: u32 = 0x98BADCFE;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xC3D2E1F0;

    let mut padded_data = Vec::from(input);
    padded_data.push(128);
    let bit_len = (input.len() as u64) * 8;

    while (padded_data.len() * 8) % 512 != 448 {
        padded_data.push(0);
    }

    //println!("{:?}", padded_data);
    //println!("{}", bit_len);


    padded_data.extend_from_slice(&bit_len.to_be_bytes());
    //println!("{:?}", padded_data);

    for chunk in padded_data.chunks(64) {
        let mut w = [0u32; 80];
        for (i, word) in chunk.chunks(4).enumerate() {
            w[i] = u32::from_be_bytes(word.try_into().unwrap());
        }

        for i in 16..80 {
            w[i] = left_rotate(w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16], 1);
        }
        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;

        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                60..=79 => (b ^ c ^ d, 0xCA62C1D6),
                _ => unreachable!(),
            };

            let temp = left_rotate(a, 5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);

            e = d;
            d = c;
            c = left_rotate(b, 30);
            b = a;
            a = temp;
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }
    //let result = vec![h0.to_be_bytes(), h1.to_be_bytes(),h2.to_be_bytes(),h3.to_be_bytes(), h4.to_be_bytes()];
    let result = format!("{:08x}{:08x}{:08x}{:08x}{:08x}", h0, h1, h2, h3, h4);
    result
}



fn main() {
    let data = b"Maks";
    let hash1 = sha11(data);
    //println!("{}", hash1);
    let hash2 = Sha1::digest(data).iter()
        .map(|&byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join("");
    println!("{}\n{}", hash1, hash2);


}
