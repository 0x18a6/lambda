use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::{
        short_weierstrass::{curves::bls12_381::curve::BLS12381Curve, point::{Endianness, PointFormat}},
        traits::IsEllipticCurve,
    },
};

fn main() {
    // public key computation
    let secret_key: u64 = 0x6C616D6264617370;
    let public_key = BLS12381Curve::generator().operate_with_self(secret_key);

    // public key serialization
    // see: https://github.com/lambdaclass/lambdaworks/blob/019cfcfdc82ee51f50be81ee5bfbb479a0482181/math/src/elliptic_curve/short_weierstrass/point.rs#L237
    let public_key = public_key.serialize(PointFormat::Uncompressed, Endianness::BigEndian);
    let public_key = hex::encode(public_key);

    println!("🔐 Public key in uncompressed format:\n{}", public_key);
}