// not required
pub fn chunk_buffer(buffer: &[u8], bytes: usize) {
    let c = buffer.chunks(bytes);
    // println!("{c:?}");

    println!("ChunksMut.len {}", c.len());
    for chunk in c {
        println!("chunk: {}", chunk.len());
        // println!("chunk: {chunk:?}");
        // for elem in chunk.iter() {
        //     println!("elem {elem}");
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const KB_1: usize = 1024;

    #[test]
    fn prepare_buffer_1_kb() {
        const BUFFER_LEN: usize = 2130; // 2,1 kB
        let b: [u8; BUFFER_LEN] = [0; BUFFER_LEN];

        chunk_buffer(b.as_slice(), KB_1);
        assert_eq!(1, 1);
    }

    #[test]
    fn buffer_concat() {
        const SIZE: usize = 5;
        let a: [u8; SIZE] = [0; SIZE];
        let b: [u8; SIZE] = [1; SIZE];
        let c: [u8; SIZE] = [2; SIZE];

        let r = [a, b, c].concat();
        println!("concat {:?}", r.as_slice());

        let ch = r.chunks(SIZE);
        // println!("ch {}", ch.len());

        let r: Vec<_> = ch.into_iter().flatten().collect();
        println!("iter flatten {:?}", r);
        println!("{}", r.len());

        assert_eq!(1, 1);
    }

    #[test]
    fn assemble_buffer_1_kb_from_2000_bytes() {
        const BUFFER_LEN: usize = 2130; // 2,1 kB
        let b: [u8; BUFFER_LEN] = [0; BUFFER_LEN];

        let c = b.chunks(KB_1);

        let r: Vec<_> = c.into_iter().flatten().collect();
        // println!("{:?}", r);
        println!("{}", r.len());

        assert_eq!(1, 1);
    }

    #[test]
    fn buffer_chunk() {
        const SIZE: usize = 5;
        let a: [u8; SIZE] = [0; SIZE];

        let c_a = a.chunks(SIZE);
        let c_b = a.chunks(2);
        let c_c = a.chunks(1);

        assert_eq!(1, c_a.len());
        assert_eq!(3, c_b.len());
        assert_eq!(5, c_c.len());
    }
}
