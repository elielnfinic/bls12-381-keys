use lambdaworks_math::elliptic_curve::{
    short_weierstrass::curves::bls12_381, 
    traits::IsEllipticCurve
};

use lambdaworks_math;

use bls12_381::curve::BLS12381FieldElement;
use bls12_381::curve::BLS12381Curve;

fn main() {
    let bls_private_key = BLS12381FieldElement::from_hex_unchecked("0x6C616D6264617370");
    // let bls_public_key_square = bls_private_key.pow(3_u32) + BLS12381FieldElement::from_hex_unchecked("0x4");    
    // let bls_public_key = bls_public_key_square.sqrt();

    // I am getting the generator by using the BLS12381 curve
    let G = BLS12381Curve::generator();
    let x = G.x();
    let y = G.y();

    // I am doing K = k * G (K being the public key and k the private key)
    let (public_key_x, public_key_y) = (x * bls_private_key.clone(), y * bls_private_key);

    println!("The private key coordonnates are {:?} and {:?}", public_key_x.to_hex(), public_key_y.to_hex());
}
