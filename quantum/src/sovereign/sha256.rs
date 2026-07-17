// ╔══════════════════════════════════════════════════════════════════════════╗
// ║  SOVEREIGN/SHA256.RS — SHA-256 i Vërtetë (FIPS 180-4)                ║
// ║  GJATA LEGACY™ — Quantum Platform                                     ║
// ║                                                                          ║
// ║  Implementim i pastër, pa varësi të jashtme, no_std-friendly.         ║
// ║  Përputhet me standardin NIST FIPS 180-4.                            ║
// ║                                                                          ║
// ║  Përdoret nga Digital Lab për hash-e TRL me integritet kriptografik.   ║
// ║  Zero if/else — vetëm operacione bitwise dhe aritmetikë.             ║
// ╚══════════════════════════════════════════════════════════════════════════╝

// ─────────────────────────────────────────────────────────────────────────────
// KONSTANTET — FIPS 180-4 §4.2.2 (64 konstante: rrënjët kubike të 64 primëve)
// ─────────────────────────────────────────────────────────────────────────────

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

// Vlerat fillestare të hash-it — FIPS 180-4 §5.3.3 (rrënjët katrore të 8 primëve).
const H0: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
    0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

// ─────────────────────────────────────────────────────────────────────────────
// FUNKSIONET LOGJIKE — FIPS 180-4 §4.1.2
// ─────────────────────────────────────────────────────────────────────────────

#[inline(always)]
fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ ((!x) & z)
}

#[inline(always)]
fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

#[inline(always)]
fn big_sigma0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

#[inline(always)]
fn big_sigma1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

#[inline(always)]
fn small_sigma0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

#[inline(always)]
fn small_sigma1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

// ─────────────────────────────────────────────────────────────────────────────
// PADDING — FIPS 180-4 §5.1.1
// ─────────────────────────────────────────────────────────────────────────────

/// pad — shton bitin '1', zero-t, dhe gjatësinë 64-bit big-endian.
/// Rezultati është shumëfish i 64 byte-ve.
fn pad(message: &[u8]) -> Vec<u8> {
    let msg_len = message.len();
    let bit_len = (msg_len as u64) * 8;

    let mut padded = Vec::with_capacity(msg_len + 72);
    padded.extend_from_slice(message);

    // Shto bitin '1' (byte 0x80).
    padded.push(0x80);

    // Shto zero derisa gjatësia ≡ 56 (mod 64). Zero if — formula modulare.
    // Numri i zero-ve: (56 - (msg_len+1)) mod 64.
    let zeros_needed = (56usize.wrapping_sub(msg_len + 1)) % 64;
    padded.extend(std::iter::repeat(0u8).take(zeros_needed));

    // Shto gjatësinë në bit (64-bit big-endian).
    padded.extend_from_slice(&bit_len.to_be_bytes());

    padded
}

// ─────────────────────────────────────────────────────────────────────────────
// PROCESIMI I NJË BLLOKU — FIPS 180-4 §6.2.2
// ─────────────────────────────────────────────────────────────────────────────

/// process_block — përpunon një bllok 512-bit (64 byte), përditëson state-in.
fn process_block(state: &mut [u32; 8], block: &[u8]) {
    // 1. Përgatit message schedule W[0..64].
    let mut w = [0u32; 64];

    // W[0..16] = fjalët nga blloku (big-endian).
    let mut t = 0usize;
    while t < 16 {
        let i = t * 4;
        w[t] = ((block[i] as u32) << 24)
             | ((block[i + 1] as u32) << 16)
             | ((block[i + 2] as u32) << 8)
             | (block[i + 3] as u32);
        t += 1;
    }

    // W[16..64] = small_sigma1(W[t-2]) + W[t-7] + small_sigma0(W[t-15]) + W[t-16].
    let mut t = 16usize;
    while t < 64 {
        w[t] = small_sigma1(w[t - 2])
            .wrapping_add(w[t - 7])
            .wrapping_add(small_sigma0(w[t - 15]))
            .wrapping_add(w[t - 16]);
        t += 1;
    }

    // 2. Inicializo variablat e punës.
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let mut f = state[5];
    let mut g = state[6];
    let mut h = state[7];

    // 3. Cikli kryesor (64 raunde).
    let mut t = 0usize;
    while t < 64 {
        let t1 = h
            .wrapping_add(big_sigma1(e))
            .wrapping_add(ch(e, f, g))
            .wrapping_add(K[t])
            .wrapping_add(w[t]);
        let t2 = big_sigma0(a).wrapping_add(maj(a, b, c));

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);

        t += 1;
    }

    // 4. Përditëso state-in (kompresim).
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

// ─────────────────────────────────────────────────────────────────────────────
// API PUBLIK
// ─────────────────────────────────────────────────────────────────────────────

/// sha256 — kthen hash-in 32-byte të mesazhit.
pub fn sha256(message: &[u8]) -> [u8; 32] {
    let mut state = H0;
    let padded = pad(message);

    // Përpuno çdo bllok 64-byte.
    let mut offset = 0usize;
    while offset < padded.len() {
        process_block(&mut state, &padded[offset..offset + 64]);
        offset += 64;
    }

    // Serializo state-in në 32 byte (big-endian).
    let mut output = [0u8; 32];
    let mut i = 0usize;
    while i < 8 {
        let bytes = state[i].to_be_bytes();
        output[i * 4] = bytes[0];
        output[i * 4 + 1] = bytes[1];
        output[i * 4 + 2] = bytes[2];
        output[i * 4 + 3] = bytes[3];
        i += 1;
    }
    output
}

/// sha256_hex — kthen hash-in si string 64-karakter hex.
pub fn sha256_hex(message: &[u8]) -> String {
    let digest = sha256(message);
    let mut s = String::with_capacity(64);
    for byte in digest.iter() {
        s.push_str(&format!("{:02x}", byte));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_known_vector() {
        // SHA-256("") = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        let hash = sha256_hex(b"");
        assert_eq!(hash, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }

    #[test]
    fn abc_known_vector() {
        // SHA-256("abc") = ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
        let hash = sha256_hex(b"abc");
        assert_eq!(hash, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    }

    #[test]
    fn longer_message_known_vector() {
        // SHA-256("abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq")
        let hash = sha256_hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq");
        assert_eq!(hash, "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1");
    }

    #[test]
    fn deterministic() {
        // I njëjti input → i njëjti hash (gjithmonë).
        let h1 = sha256_hex(b"GJATA_LEGACY");
        let h2 = sha256_hex(b"GJATA_LEGACY");
        assert_eq!(h1, h2);
    }

    #[test]
    fn avalanche_effect() {
        // Ndryshim i vogël → hash krejt i ndryshëm.
        let h1 = sha256_hex(b"test");
        let h2 = sha256_hex(b"Test"); // një bit ndryshim
        assert_ne!(h1, h2);
    }

    #[test]
    fn output_is_64_hex_chars() {
        let hash = sha256_hex(b"anything");
        assert_eq!(hash.len(), 64);
        // Të gjitha karaktere hex.
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn digest_is_32_bytes() {
        let digest = sha256(b"test");
        assert_eq!(digest.len(), 32);
    }

    #[test]
    fn block_boundary_55_bytes() {
        // 55 byte — pikërisht para kufirit të padding-ut.
        let msg = vec![b'a'; 55];
        let hash = sha256_hex(&msg);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn block_boundary_56_bytes() {
        // 56 byte — detyron një bllok shtesë për gjatësinë.
        let msg = vec![b'a'; 56];
        let hash = sha256_hex(&msg);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn block_boundary_64_bytes() {
        // 64 byte — pikërisht një bllok i plotë.
        let msg = vec![b'a'; 64];
        let hash = sha256_hex(&msg);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn multi_block_message() {
        // 1000 byte — disa blloqe.
        let msg = vec![b'x'; 1000];
        let hash = sha256_hex(&msg);
        assert_eq!(hash.len(), 64);
        // Deterministik edhe për mesazhe të gjata.
        assert_eq!(hash, sha256_hex(&msg));
    }
}
