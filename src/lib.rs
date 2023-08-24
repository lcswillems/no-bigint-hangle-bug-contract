#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait Contract {
    #[init]
    fn init(&self) {
        self.f(
            BigUint::zero(),
            BigUint::zero(),
            BigUint::zero(),
            BigUint::zero(),
            BigUint::zero(),
            0,
        );
        self.f(
            BigUint::zero(),
            BigUint::zero(),
            BigUint::zero(),
            BigUint::zero(),
            BigUint::zero(),
            1,
        );
    }

    fn f(
        &self,
        arg1: BigUint,
        arg2: BigUint,
        arg3: BigUint,
        arg4: BigUint,
        arg5: BigUint,
        arg6: u64,
    ) {
        let _ = arg1 == 0;
        let _ = arg2 == 0;
        let _ = arg3 == 0;
        let _ = arg4 == 0;
        let _ = arg5 == 0;
        let _ = BigUint::from(arg6) == 0;
    }
}
