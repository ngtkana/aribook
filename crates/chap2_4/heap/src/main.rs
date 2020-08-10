use proconio::input;

fn main() {
    input!(q: usize);
    let mut heap = Vec::new();
    fn push(x: u32, heap: &mut Vec<u32>) {
        let mut i = heap.len();
        heap.push(x);
        while 0 < i {
            let p = (i - 1) / 2;
            if heap[p] < heap[i] {
                break;
            }
            heap.swap(p, i);
            i = p;
        }
    }
    fn pop(heap: &mut Vec<u32>) -> Option<u32> {
        let mut i = 0;
        loop {
            let j = i * 2 + 1;
            if heap.len() <= j {
                break;
            }
            let k = j + 1;
            let ni = if heap.len() <= k || heap[j] < heap[k] {
                j
            } else {
                k
            };
            heap.swap(i, ni);
            i = ni;
        }
        heap.pop()
    }
    for _ in 0..q {
        input!(com: String);
        match com.as_str() {
            "push" => {
                input!(x: u32);
                push(x, &mut heap);
            }
            "pop" => {
                println!("{}", pop(&mut heap).unwrap());
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod chap2_4_heap_tests {
    const BIN: &str = "chap2_4_heap";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"10
push 5
push 1
push 3
pop
push 4
pop
push 2
pop
pop
pop
"#,
            r#"1
3
2
4
5
"#,
        );
    }
}
