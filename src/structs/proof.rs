use PublicId;

pub struct Proof<P: PublicId> {
    pub public_id: P,
    pub signature: P::Signature,
}

impl<P: PublicId> Proof<P> {
    pub fn public_id(&self) -> &P {
        &self.public_id
    }

    pub fn signature(&self) -> &P::Signature {
        &self.signature
    }

    pub fn is_valid(&self, data: &[u8]) -> bool {
        self.public_id.verify_signature(&self.signature, data)
    }
}
