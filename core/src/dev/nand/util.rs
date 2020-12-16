
pub fn parity(input: u8) -> u8 { 
    (input.count_ones() % 2) as u8 
}

#[allow(unused_assignments)]
pub fn calc_ecc(data: &mut [u8]) -> u32 {
    let mut a = [[0u8; 2]; 12];
    let mut a0 = 0u32;
    let mut a1 = 0u32;
    let mut x = 0u8;

    for i in 0..512 {
        x = data[i];
        for j in 0..9 {
            a[3 + j][(i >> j) & 1] ^= x;
        }
    }

    x = a[3][0] ^ a[3][1];
    a[0][0] = x & 0x55;
    a[0][1] = x & 0xaa;
    a[1][0] = x & 0x33;
    a[1][1] = x & 0xcc;
    a[2][0] = x & 0x0f;
    a[2][1] = x & 0xf0;

    for j in 0..12 {
        a[j][0] = parity(a[j][0]);
        a[j][1] = parity(a[j][1]);
    }
    for j in 0..12 {
        a0 |= (a[j][0] as u32) << j;
        a1 |= (a[j][1] as u32) << j;
    }


    (a0 & 0x0000_00ff) << 24 | (a0 & 0x0000_ff00) << 8 |
    (a1 & 0x0000_00ff) << 8  | (a1 & 0x0000_ff00) >> 8
}

