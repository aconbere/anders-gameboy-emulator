enum Args8 {
    R(Registers8),
    N,
    Mem
}

type Instruction = fn(registers: &mut Registers, mmu: &mut mmu::MMU, args: &Vec<u8>) -> u8

struct Op1 {
    arg: Args8,
    label: String,
    func: Instruction,
}


fn ADD(arg: Args8, label: String) -> ADD {
    Op1 {
        arg: arg,
        label: label,
        func: 
    }
}

/* add_n_a::new(Args8::R(Registers8::B)).call
