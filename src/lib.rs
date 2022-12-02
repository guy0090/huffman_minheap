use std::num::ParseIntError;

#[derive(Debug, Clone)]
pub struct Node {
    pub c: String,
    pub f: u32,
    pub l: Option<Box<Node>>,
    pub r: Option<Box<Node>>,
}

impl Node {
    pub fn new(c: String, f: u32, l: Option<Box<Node>>, r: Option<Box<Node>>) -> Self {
        Self { c, f, l, r }
    }

    pub fn lt(&self, other: &Self) -> bool {
        self.f < other.f
    }

    pub fn le(&self, other: &Self) -> bool {
        self.f <= other.f
    }
}

pub struct MinHeap {
    pub heap: Vec<Node>,
}

impl MinHeap {
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.heap.len()
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        let tmp = self.heap[i].clone();
        self.heap[i] = self.heap[j].clone();
        self.heap[j] = tmp;
    }

    pub fn push(&mut self, node: Node) {
        self.heap.push(node);
        let mut child_index: isize = (self.size() - 1).try_into().unwrap();

        loop {
            let parent_index = match (child_index - 1) / 2 {
                -1 => self.size(),
                v => v as usize,
            };
            if self.heap[parent_index].le(&self.heap[child_index as usize]) {
                return;
            }

            self.swap(parent_index, child_index as usize);
            child_index = parent_index as isize;
            if child_index <= 0 {
                return;
            }
        }
    }

    pub fn pop(&mut self) -> Node {
        let node = self.heap[0].clone();
        let last = match self.heap.pop() {
            Some(last) => {
                if self.size() == 0 {
                    return node;
                }
                last
            }
            None => return node,
        };
        self.heap[0] = last;

        let mut parent_index = 0;
        let mut child_index = 2 * parent_index + 1;
        while child_index < self.size() {
            if child_index + 1 < self.size()
                && self.heap[child_index + 1].lt(&self.heap[child_index])
            {
                child_index += 1;
            }

            if self.heap[parent_index].le(&self.heap[child_index]) {
                return node;
            }

            self.swap(parent_index, child_index);
            parent_index = child_index;
            child_index = 2 * child_index + 1;
        }

        node
    }
}

pub fn hex_to_bytes(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
