use lambdaworks_math::{
    elliptic_curve::{edwards::curves::bandersnatch::field, short_weierstrass::curves::bls12_381},
    field::{element::FieldElement, test_fields::u64_test_field::U64Field}, unsigned_integer::element::U256,
};

use lambdaworks_math;

use bls12_381::curve::BLS12381FieldElement;

fn main() {
    
    //     let generator = FieldElement::from(0x1);
    //     let p = FieldElement::from(0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab);
    //     let field = U64Field::from()
    //    ;
    //     // let curve = BLS12381Curve::from(private_key);
    //     let private_key = bls12_381::curve::BLS12381FieldElement::from(private_key);

    //     BLS12381FieldElement::

    // let p = U256::from_hex_unchecked("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
    // let x = U256::from_hex_unchecked("55066263022277343669578718895168534326250603453777594175500187360389116729240");
    // let y = U256::from_hex_unchecked("32670510020758816978083085130507043184471273380659243275938904335757337482424");

    // let p_felt = FieldElement::from_hex_unchecked("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");

    // let (p1_x, p1_y) = (U256::from_hex_unchecked("0x04"), U256::from_hex_unchecked("0x0a989badd40d6212b33cffc3f3763e9bc760f988c9926b26da9dd85e928483446346b8ed00e1de5d5ea93e354abe706c") );

    // let z = x + y;

    // println!("z = {}", z);
    // // create a u256 integer 

    let my_elt = FieldElement::from(0x6C616D6264617370);
    // let public_key = private_key * p_felt;
    // let pub_key_sq = (private_key * private_key * private_key);
    // let pub_key = pub_key_sq.sqrt();


}
