use ark_ec::AffineRepr;
use ark_pcs_bench_templates::*;
use ark_poly::{multivariate::{SparseTerm, Term}, DenseMVPolynomial, DenseUVPolynomial};
use blake2::Blake2s256;

use ark_crypto_primitives::sponge::poseidon::PoseidonSponge;
use ark_ed_on_bls12_381::{EdwardsAffine, Fr};
use ark_ff::PrimeField;
use ark_poly::univariate::DensePolynomial as DenseUnivariatePoly;
use ark_poly::multivariate::SparsePolynomial as SparseMultivariatePoly;

use ark_poly_commit::{ipa_pc::InnerProductArgPC, marlin_pc::MarlinKZG10, marlin_pst13_pc::MarlinPST13};

// used for KZG bench
use ark_std::{rand::Rng, test_rng};
use ark_bls12_381::Bls12_381;
use ark_ec::pairing::Pairing;
type UniPoly_381 = DenseUnivariatePoly<<Bls12_381 as Pairing>::ScalarField>;
type MulPoly_381 = SparseMultivariatePoly<<Bls12_381 as Pairing>::ScalarField, SparseTerm>;
type Sponge_Bls12_381 = PoseidonSponge<<Bls12_381 as Pairing>::ScalarField>;
// used for KZG bench

use rand_chacha::ChaCha20Rng;

const MIN_NUM_VARS: usize = 10;
const MAX_NUM_VARS: usize = 20;

// type UniPoly = DenseUnivariatePoly<Fr>;
// type Sponge = PoseidonSponge<<EdwardsAffine as AffineRepr>::ScalarField>;

// IPA_PC over the JubJub curve with Blake2s as the hash function
// #[allow(non_camel_case_types)]
// type IPA_JubJub = InnerProductArgPC<EdwardsAffine, Blake2s256, UniPoly, Sponge>;

// fn rand_poly_ipa_pc<F: PrimeField>(degree: usize, rng: &mut ChaCha20Rng) -> DenseUnivariatePoly<F> {
//     DenseUnivariatePoly::rand(degree, rng)
// }

// const MIN_NUM_VARS: usize = 10;
// const MAX_NUM_VARS: usize = 20;

// bench!(IPA_JubJub, rand_poly_ipa_pc);

/** ******* A test of kzg10 **/
#[allow(non_camel_case_types)]
type marlin_kzg10_test = MarlinKZG10<Bls12_381, UniPoly_381, Sponge_Bls12_381>;

fn rand_poly_marlin_kzg10<F: PrimeField>(degree: usize, rng: &mut ChaCha20Rng) -> DenseUnivariatePoly<F> {
    DenseUVPolynomial::rand(degree, rng)
}

bench!(marlin_kzg10_test, rand_poly_marlin_kzg10);

/**A test of marlin_pst13 **/
/* Failed on Sep 19th, 2024 */

#[allow(non_camel_case_types)]
type marlin_pst13_test = MarlinPST13<Bls12_381, MulPoly_381, Sponge_Bls12_381>;

// fn rand_poly_marlin_pst13<F: PrimeField>(degree: usize, rng: &mut impl Rng) -> SparseMultivariatePoly<F, SparseTerm> {
//     SparseMultivariatePoly::rand(degree, 2, rng)
// }

// bench!(marlin_pst13_test, rand_poly_marlin_pst13);
