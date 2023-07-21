type R = Result<(), Box<dyn std::error::Error>>;

pub struct Kem<const N: usize> {}

impl<const N: usize> Kem<N> {
    pub fn crypto_kem_keypair(&self, pk: &mut [u8], sk: &mut [u8]) -> R {
        Ok(())
    }

    pub fn crypto_kem_enc(&self, c: &mut [u8], key: &mut [u8], pk: &[u8]) -> R {
        Ok(())
    }

    pub fn crypto_kem_dec(&self, key: &mut [u8], c: &[u8], sk: &[u8]) -> R {
        Ok(())
    }
}

mod variant1 {
    pub const N: usize = 21;
    pub const KEM: super::Kem<N> = super::Kem::<N> {};
}

mod variant2 {
    pub const N: usize = 42;
    pub const KEM: super::Kem<N> = super::Kem::<N> {};
}

fn main() -> R {
    use variant1::*;
    let mut pk = [0u8; N];
    let mut sk = [0u8; N];
    KEM.crypto_kem_keypair(&mut pk, &mut sk)
}
