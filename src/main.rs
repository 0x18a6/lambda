use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::{
        traits::IsEllipticCurve,
        short_weierstrass::{
            curves::bls12_381::curve::BLS12381Curve,
            point::{Endianness, PointFormat},
        },
    },
};
use hex;

fn main() {
    let secret_key: u64 = 0x6C616D6264617370;

    // public key generation
    let public_key = BLS12381Curve::generator().operate_with_self(secret_key);

    // public key serialization
    let public_key = public_key.serialize(PointFormat::Projective, Endianness::BigEndian);
    let public_key = hex::encode(public_key);

    println!("🔐 Public key is: {}", public_key);
}