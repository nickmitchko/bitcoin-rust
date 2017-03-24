/*********************************************************************
* CTAES -> C/C++ version                                             *
* Copyright (c) 2016 Pieter Wuille                                   *
* Distributed under the MIT software license, see the accompanying   *
* file COPYING or http://www.opensource.org/licenses/mit-license.php.*
**********************************************************************/

// Title:       CTAES Rust Port
// Filename:    ctaes.rs
// Author:      nickmitchko
// Date:        Mar. 23rd, 2017

struct AesState {
    slice: [u16; 8]
}

struct Aes128Ctx {
    rk: [AesState; 11]
}

struct Aes192Ctx {
    rk: [AesState; 13]
}

struct Aes256Ctx {
    rk: [AesState; 15]
}

/** Convert a byte to sliced form, storing it corresponding to given row and column in s */
fn convert_byte_to_slice(s: &mut AesState, mut byte: u8, r: i32, c: i32) {
    for i in 0..8 {
        *s.slice[i] |= (byte & 1) << (r * 4 + c);
        byte >>= 1;
    }
}

/** Load 16 bytes of data into 8 sliced integers */
fn convert16byte_to8slice(s: &mut AesState, &data16: u8) {
    for c in 0..4 {
        for r in 0..4 {
            *data16 += 1;
            convert_byte_to_slice(s, *(data16), r, c);
        }
    }
}

/** Convert 8 sliced integers into 16 bytes of data */
fn convert8slice_to16byte(&data16: u8, s:  &mut AesState) {
    for c in 0..4 {
        for r in 0..4 {
            let mut v: u8 = 0;
            for b in 0..8 {}
            *data16 = v;
            *data16 += 1;
        }
    }
}

fn sub_bytes(s:  &mut AesState, inv: i32) {
    /*Bit Slice loading*/
    let U0: u16 = *s.slice[7];
    let U1: u16 = *s.slice[6];
    let U2: u16 = *s.slice[5];
    let U3: u16 = *s.slice[4];
    let U4: u16 = *s.slice[3];
    let U5: u16 = *s.slice[2];
    let U6: u16 = *s.slice[1];
    let U7: u16 = *s.slice[0];

    /* Specific rounds declared here*/
    let mut T1: u16 = 0;
    let mut T2: u16 = 0;
    let mut T3: u16 = 0;
    let mut T4: u16 = 0;
    let mut T5: u16 = 0;
    let mut T6: u16 = 0;
    let mut T7: u16 = 0;
    let mut T8: u16 = 0;
    let mut T9: u16 = 0;
    let mut T10: u16 = 0;
    let mut T11: u16 = 0;
    let mut T12: u16 = 0;
    let mut T13: u16 = 0;
    let mut T14: u16 = 0;
    let mut T15: u16 = 0;
    let mut T16: u16 = 0;
    let mut T17: u16 = 0;
    let mut T18: u16 = 0;
    let mut T19: u16 = 0;
    let mut T20: u16 = 0;
    let mut T21: u16 = 0;
    let mut T22: u16 = 0;
    let mut T23: u16 = 0;
    let mut T24: u16 = 0;
    let mut T25: u16 = 0;
    let mut T26: u16 = 0;
    let mut T27: u16 = 0;
    let mut D: u16 = 0;
    let mut M1: u16 = 0;
    let mut M6: u16 = 0;
    let mut M11: u16 = 0;
    let mut M13: u16 = 0;
    let mut M15: u16 = 0;
    let mut M20: u16 = 0;
    let mut M21: u16 = 0;
    let mut M22: u16 = 0;
    let mut M23: u16 = 0;
    let mut M25: u16 = 0;
    let mut M37: u16 = 0;
    let mut M38: u16 = 0;
    let mut M39: u16 = 0;
    let mut M40: u16 = 0;
    let mut M41: u16 = 0;
    let mut M42: u16 = 0;
    let mut M43: u16 = 0;
    let mut M44: u16 = 0;
    let mut M45: u16 = 0;
    let mut M46: u16 = 0;
    let mut M47: u16 = 0;
    let mut M48: u16 = 0;
    let mut M49: u16 = 0;
    let mut M50: u16 = 0;
    let mut M51: u16 = 0;
    let mut M52: u16 = 0;
    let mut M53: u16 = 0;
    let mut M54: u16 = 0;
    let mut M55: u16 = 0;
    let mut M56: u16 = 0;
    let mut M57: u16 = 0;
    let mut M58: u16 = 0;
    let mut M59: u16 = 0;
    let mut M60: u16 = 0;
    let mut M61: u16 = 0;
    let mut M62: u16 = 0;
    let mut M63: u16 = 0;

    if int {
        /* Undo linear post-processing */
        T23 = U0 ^ U3;
        T22 = !(U1 ^ U3);
        T2 = !(U0 ^ U1);
        T1 = U3 ^ U4;
        T24 = !(U4 ^ U7);
        let R5: u16 = U6 ^ U7;
        T8 = !(U1 ^ T23);
        T19 = T22 ^ R5;
        T9 = !(U7 ^ T1);
        T10 = T2 ^ T24;
        T13 = T2 ^ R5;
        T3 = T1 ^ R5;
        T25 = !(U2 ^ T1);
        let R13: u16 = U1 ^ U6;
        T17 = !(U2 ^ T19);
        T20 = T24 ^ R13;
        T4 = U4 ^ T8;
        let R17: u16 = !(U2 ^ U5);
        let R18: u16 = !(U5 ^ U6);
        let R19: u16 = !(U2 ^ U4);
        D = U0 ^ R17;
        T6 = T22 ^ R17;
        T16 = R13 ^ R19;
        T27 = T1 ^ R18;
        T15 = T10 ^ T27;
        T14 = T10 ^ R18;
        T26 = T3 ^ T16;
    } else {
        /* Linear pre-processing. */
        T1 = U0 ^ U3;
        T2 = U0 ^ U5;
        T3 = U0 ^ U6;
        T4 = U3 ^ U5;
        T5 = U4 ^ U6;
        T6 = T1 ^ T5;
        T7 = U1 ^ U2;
        T8 = U7 ^ T6;
        T9 = U7 ^ T7;
        T10 = T6 ^ T7;
        T11 = U1 ^ U5;
        T12 = U2 ^ U5;
        T13 = T3 ^ T4;
        T14 = T6 ^ T11;
        T15 = T5 ^ T11;
        T16 = T5 ^ T12;
        T17 = T9 ^ T16;
        T18 = U3 ^ U7;
        T19 = T7 ^ T18;
        T20 = T1 ^ T19;
        T21 = U6 ^ U7;
        T22 = T7 ^ T21;
        T23 = T2 ^ T22;
        T24 = T2 ^ T10;
        T25 = T20 ^ T17;
        T26 = T3 ^ T16;
        T27 = T1 ^ T12;
        D = U7;
    }

    /* Non-linear transformation (shared between the forward and backward case) */
    M1 = T13 & T6;
    M6 = T3 & T16;
    M11 = T1 & T15;
    M13 = (T4 & T27) ^ M11;
    M15 = (T2 & T10) ^ M11;
    M20 = T14 ^ M1 ^ (T23 & T8) ^ M13;
    M21 = (T19 & D) ^ M1 ^ T24 ^ M15;
    M22 = T26 ^ M6 ^ (T22 & T9) ^ M13;
    M23 = (T20 & T17) ^ M6 ^ M15 ^ T25;
    M25 = M22 & M20;
    M37 = M21 ^ ((M20 ^ M21) & (M23 ^ M25));
    M38 = M20 ^ M25 ^ (M21 | (M20 & M23));
    M39 = M23 ^ ((M22 ^ M23) & (M21 ^ M25));
    M40 = M22 ^ M25 ^ (M23 | (M21 & M22));
    M41 = M38 ^ M40;
    M42 = M37 ^ M39;
    M43 = M37 ^ M38;
    M44 = M39 ^ M40;
    M45 = M42 ^ M41;
    M46 = M44 & T6;
    M47 = M40 & T8;
    M48 = M39 & D;
    M49 = M43 & T16;
    M50 = M38 & T9;
    M51 = M37 & T17;
    M52 = M42 & T15;
    M53 = M45 & T27;
    M54 = M41 & T10;
    M55 = M44 & T13;
    M56 = M40 & T23;
    M57 = M39 & T19;
    M58 = M43 & T3;
    M59 = M38 & T22;
    M60 = M37 & T20;
    M61 = M42 & T1;
    M62 = M45 & T4;
    M63 = M41 & T2;

    if inv {
        let P0 : u16 = M52 ^ M61;
        let P1 : u16 = M58 ^ M59;
        let P2 : u16 = M54 ^ M62;
        let P3 : u16 = M47 ^ M50;
        let P4 : u16 = M48 ^ M56;
        let P5 : u16 = M46 ^ M51;
        let P6 : u16 = M49 ^ M60;
        let P7 : u16 = P0 ^ P1;
        let P8 : u16 = M50 ^ M53;
        let P9 : u16 = M55 ^ M63;
        let P10 : u16 = M57 ^ P4;
        let P11 : u16 = P0 ^ P3;
        let P12 : u16 = M46 ^ M48;
        let P13 : u16 = M49 ^ M51;
        let P14 : u16 = M49 ^ M62;
        let P15 : u16 = M54 ^ M59;
        let P16 : u16 = M57 ^ M61;
        let P17 : u16 = M58 ^ P2;
        let P18 : u16 = M63 ^ P5;
        let P19 : u16 = P2 ^ P3;
        let P20 : u16 = P4 ^ P6;
        let P22 : u16 = P2 ^ P7;
        let P23 : u16 = P7 ^ P8;
        let P24 : u16 = P5 ^ P7;
        let P25 : u16 = P6 ^ P10;
        let P26 : u16 = P9 ^ P11;
        let P27 : u16 = P10 ^ P18;
        let P28 : u16 = P11 ^ P25;
        let P29 : u16 = P15 ^ P20;
        *s.slice[7] = P13 ^ P22;
        *s.slice[6] = P26 ^ P29;
        *s.slice[5] = P17 ^ P28;
        *s.slice[4] = P12 ^ P22;
        *s.slice[3] = P23 ^ P27;
        *s.slice[2] = P19 ^ P24;
        *s.slice[1] = P14 ^ P23;
        *s.slice[0] = P9 ^ P16;
    } else {
        /* Linear postprocessing */
        let L0 : u16 = M61 ^ M62;
        let L1 : u16 = M50 ^ M56;
        let L2 : u16 = M46 ^ M48;
        let L3 : u16 = M47 ^ M55;
        let L4 : u16 = M54 ^ M58;
        let L5 : u16 = M49 ^ M61;
        let L6 : u16 = M62 ^ L5;
        let L7 : u16 = M46 ^ L3;
        let L8 : u16 = M51 ^ M59;
        let L9 : u16 = M52 ^ M53;
        let L10 : u16 = M53 ^ L4;
        let L11 : u16 = M60 ^ L2;
        let L12 : u16 = M48 ^ M51;
        let L13 : u16 = M50 ^ L0;
        let L14 : u16 = M52 ^ M61;
        let L15 : u16 = M55 ^ L1;
        let L16 : u16 = M56 ^ L0;
        let L17 : u16 = M57 ^ L1;
        let L18 : u16 = M58 ^ L8;
        let L19 : u16 = M63 ^ L4;
        let L20 : u16 = L0 ^ L1;
        let L21 : u16 = L1 ^ L7;
        let L22 : u16 = L3 ^ L12;
        let L23 : u16 = L18 ^ L2;
        let L24 : u16 = L15 ^ L9;
        let L25 : u16 = L6 ^ L10;
        let L26 : u16 = L7 ^ L9;
        let L27 : u16 = L8 ^ L10;
        let L28 : u16 = L11 ^ L14;
        let L29 : u16 = L11 ^ L17;
        *s.slice[7] = L6 ^ L24;
        *s.slice[6] = !(L16 ^ L26);
        *s.slice[5] = !(L19 ^ L28);
        *s.slice[4] = L6 ^ L21;
        *s.slice[3] = L20 ^ L22;
        *s.slice[2] = L25 ^ L29;
        *s.slice[1] = !(L13 ^ L27);
        *s.slice[0] = !(L6 ^ L23);
    }
}

#[inline(always)]
fn BIT_RANGE(from: u16, to: u16) -> u16 {
    ((1 << ((to) - (from))) - 1) << from
}

#[inline(always)]
fn BIT_RANGE_LEFT(x: u16, from: u16, to: u16, shift: u16) -> u16 {
    ((x) & BIT_RANGE((from), (to))) << shift
}

#[inline(always)]
fn BIT_RANGE_RIGHT(x: u16, from: u16, to:u16, shift: u16) -> u16{
    ((x) & BIT_RANGE((from), (to))) >> shift
}

fn shift_rows(s: &mut AesState){
    for i in 0..8 {
        let v: u16 = *s.slice[i];
        *s.slice[i] = (v & BIT_RANGE(0, 4) | BIT_RANGE_LEFT(v, 4, 5, 3) | BIT_RANGE_RIGHT(v, 5, 8, 1) | BIT_RANGE_LEFT(v, 8, 10, 2) | BIT_RANGE_RIGHT(v, 10, 12, 2) | BIT_RANGE_LEFT(v, 12, 15, 1) | BIT_RANGE_RIGHT(v, 15, 16, 3));
    }
}

fn inv_shift_rows(s: &mut AesState){
    for i in 0..8 {
        let v: u16 = *s.slice[i];
        *s.slice[i] = (v & BIT_RANGE(0, 4) | BIT_RANGE_LEFT(v, 4, 7, 1) | BIT_RANGE_RIGHT(v, 7, 8, 3) | BIT_RANGE_LEFT(v, 8, 10, 2) | BIT_RANGE_RIGHT(v, 10, 12, 2) | BIT_RANGE_LEFT(v, 12, 13, 3) | BIT_RANGE_RIGHT(v, 13, 16, 1));
    }
}

#[inline(always)]
fn rotate(x: u16, b:u16) -> u16 {
    ((x) >> ((b) * 4)) | ((x) << ((4-(b)) * 4))
}

// TODO: implement this
fn mix_columns(){

}