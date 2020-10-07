//! Minimal implementation of the SHA-1 algorithm.
//!
//! Apart from the fact that you shoud **not** use SHA-1, this is **not** an 
//! implementation that is suitable for, or otherwise intended for any kind of
//! practical cryptographic use outside of this particular application. 
//!
//! Do not even think about using this code somewhere else.
//!
//! For right now, we just assert that message size is a multiple of 64. I'm 
//! not sure what the hardware behavior is (either the SHA engine disregards 
//! messages which aren't a multiple of 64-bytes long, or it always performs 
//! DMA reads in 64-byte chunks).

const K: [u32; 4] = [ 0x5a82_7999, 0x6ed9_eba1, 0x8f1b_bcdc, 0xca62_c1d6, ];

pub struct Sha1State {
    pub digest: [u32; 5],
    pub buf: [u8; 64],
}
impl Sha1State {
    pub fn new() -> Self {
        Sha1State { digest: [0; 5], buf: [0; 64] }
    }
    pub fn reset(&mut self) {
        self.digest = [0; 5];
        self.buf = [0; 64];
    }
}

impl Sha1State {
    pub fn update(&mut self, input: &Vec<u8>) {
        assert!(input.len() % 64 == 0);
        for chunk in input.chunks(64) {
            self.buf.copy_from_slice(chunk);
            self.process_message();
        }
    }

    // I wonder if this would perform better if you unrolled everything?
    fn process_message(&mut self) {
        let k = K;
        let mut a = self.digest[0];
        let mut b = self.digest[1];
        let mut c = self.digest[2];
        let mut d = self.digest[3];
        let mut e = self.digest[4];

        let mut w = [0u32; 80];
        for (idx, wb) in self.buf.chunks(4).enumerate() {
            let mut word = [0u8; 4];
            word.copy_from_slice(wb);
            let word = u32::from_be_bytes(word);
            w[idx] = word;
        }

        for t in 16..80 {
            let word = w[t-3] ^ w[t-8] ^ w[t-14] ^ w[t-16];
            w[t] = word.rotate_left(1);
        }

        for t in 0..20 {
            //let temp = a.rotate_left(5) + ((b & c) | ((!b) & d)) + e + w[t] + K[0];
            let temp = a.rotate_left(5)
                .wrapping_add((b & c) | ((!b) & d))
                .wrapping_add(e)
                .wrapping_add(w[t])
                .wrapping_add(K[0]);

            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        for t in 20..40 {
            //let temp = a.rotate_left(5) + (b ^ c ^ d) + e + w[t] + K[1];
            let temp = a.rotate_left(5)
                .wrapping_add(b ^ c ^ d)
                .wrapping_add(e)
                .wrapping_add(w[t])
                .wrapping_add(K[1]);

            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        for t in 40..60 {
            //let temp = a.rotate_left(5) + ((b & c) | (b & d) | (c & d)) + e + w[t] + K[2];
            let temp = a.rotate_left(5)
                .wrapping_add((b & c) | (b & d) | (c & d))
                .wrapping_add(e)
                .wrapping_add(w[t])
                .wrapping_add(K[2]);

            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        for t in 60..80 {
            let temp = a.rotate_left(5)
                .wrapping_add(b ^ c ^ d)
                .wrapping_add(e)
                .wrapping_add(w[t])
                .wrapping_add(K[3]);

            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        //self.digest[0] += a;
        //self.digest[1] += b;
        //self.digest[2] += c;
        //self.digest[3] += d;
        //self.digest[4] += e;
        self.digest[0] = self.digest[0].wrapping_add(a);
        self.digest[1] = self.digest[1].wrapping_add(b);
        self.digest[2] = self.digest[2].wrapping_add(c);
        self.digest[3] = self.digest[3].wrapping_add(d);
        self.digest[4] = self.digest[4].wrapping_add(e);

    }
}
