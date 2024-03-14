
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::field_extension::BLS12381PrimeField;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381;
use lambdaworks_math::{cyclic_group::IsGroup, elliptic_curve::traits::IsEllipticCurve, field::element::FieldElement};

fn main() {
    let generator_ = bls12_381::curve::BLS12381Curve::generator();
    let sk = u64::from_str_radix("6C616D6264617370", 16).expect("Failed to parse key");
    let pk = generator_.operate_with_self(sk);

    println!("Public key is {:?}", pk);
}
