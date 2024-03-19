use lambdaworks_math::elliptic_curve::{
    short_weierstrass::curves::bls12_381, 
    traits::IsEllipticCurve
};

use lambdaworks_math;

use bls12_381::curve::BLS12381FieldElement;
use bls12_381::curve::BLS12381Curve;

// This function allows to get the public key using BLS12381 implemented in lambdaworks
pub fn get_public_key(private_key_field_elt : BLS12381FieldElement) -> (BLS12381FieldElement, BLS12381FieldElement) {
    // I am getting the generator by using the BLS12381 curve
    let G = BLS12381Curve::generator();

    // Here are the coordinates x and y of the generator
    let x = G.x();
    let y = G.y();

    // I am doing K = k * G (K being the public key and k the private key)
    (x * private_key_field_elt.clone(), y * private_key_field_elt)
}

