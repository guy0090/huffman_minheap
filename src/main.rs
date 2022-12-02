#[macro_use]
extern crate structure;

use bit_vec::BitVec;
use byteio::ReadBytes;
use lib::{hex_to_bytes, MinHeap, Node};
use std::collections::BTreeMap;

fn main() {
    let hex = "81000000000000000B000000060000002D000000090000003000000003000000310000000300000032000000020000003300000002000000340000000600000035000000030000003700000004000000380000000100000039000000020000007C000000850000001100000029000000D30C7890FB1D0E6E4B4C35DF1775BDAA90";
    let out = from_string(hex);

    println!("{:#}", out);
}

fn get_freqs(mut bytes: &[u8]) -> (usize, BTreeMap<String, u32>) {
    let mut read = 0;

    let mut freqs = BTreeMap::new();

    let s = structure!("<3I");
    let slice = bytes.read_exact(s.size());
    let (_, _, chars): (u32, u32, u32) = s.unpack(slice).unwrap();

    read += s.size();
    (0..chars).step_by(1).for_each(|_| {
        let s = structure!("<I");
        let (f,): (u32,) = s.unpack(bytes.read_exact(s.size())).unwrap();
        read += s.size();

        let s = structure!("<B3x");
        let (c,): (u8,) = s.unpack(bytes.read_exact(s.size())).unwrap();
        read += s.size();

        let c = format!("{}", c as char);
        freqs.insert(c, f);
    });

    (read, freqs)
}

fn make_tree(freq: BTreeMap<String, u32>) -> Node {
    let mut heap = MinHeap::new();
    for (c, f) in freq {
        heap.push(Node::new(c.clone(), f.clone(), None, None));
    }

    while heap.size() > 1 {
        let n1 = heap.pop().clone();
        let n2 = heap.pop().clone();
        let n = Node::new(
            format!("{}{}", n1.c, n2.c),
            n1.f + n2.f,
            Some(Box::new(n1.clone())),
            Some(Box::new(n2.clone())),
        );
        heap.push(n);
    }

    heap.pop().clone()
}

fn decode(tree: Node, packed: &[u8], packed_bits: u32) -> Result<String, ()> {
    let mut bits = BitVec::from_bytes(packed);
    bits.truncate(packed_bits as usize);

    let mut unpacked: Vec<String> = vec![];
    let mut pos = 0;

    while pos < bits.len() {
        let mut node = tree.clone();
        loop {
            if pos >= bits.len() {
                return Err(());
            }

            if bits[pos] {
                node = match node.r {
                    Some(n) => *n,
                    None => return Err(()),
                };
            } else {
                node = match node.l {
                    Some(n) => *n,
                    None => return Err(()),
                };
            }

            pos += 1;
            if node.l.is_none() && node.r.is_none() {
                break;
            }
        }
        unpacked.push(node.c);
    }

    Ok(unpacked.join(""))
}

fn from_string(hex: &str) -> String {
    let binding = hex_to_bytes(hex).unwrap();
    let mut bytes = binding.as_slice();

    let (read, freqs) = get_freqs(bytes);
    bytes.read_exact(read);
    let tree = make_tree(freqs);

    let s = structure!("<3I");
    let slice = bytes.read_exact(s.size());
    let (packed_bits, packed_bytes, _): (u32, u32, u32) = s.unpack(slice).unwrap();

    let packed = bytes.read_exact(packed_bytes as usize);

    decode(tree, packed, packed_bits).unwrap()
}
