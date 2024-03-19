use lambdaworks_math::elliptic_curve::{
    short_weierstrass::curves::bls12_381, 
    traits::IsEllipticCurve
};

use lambdaworks_math;

use bls12_381::curve::BLS12381FieldElement;
use bls12_381::curve::BLS12381Curve;

mod public_key;

fn main() {
    let bls_private_key = BLS12381FieldElement::from_hex_unchecked("0x6C616D6264617370");
    
    let (public_key_x, public_key_y) =  public_key::get_public_key(bls_private_key);

    println!("The public key coordonnates are {:?} and {:?}", public_key_x.to_hex(), public_key_y.to_hex());
}
