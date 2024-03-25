use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::{
        short_weierstrass::curves::bls12_381::curve::BLS12381Curve,
        traits::IsEllipticCurve,
    },
};

fn main() {
    let secret_key: u64 = 0x6C616D6264617370;
    let public_key = BLS12381Curve::generator().operate_with_self(secret_key);
    let public_key = public_key.to_affine();
    println!("🔐 Public key:\n{}", public_key.x());
}
