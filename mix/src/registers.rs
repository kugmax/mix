use crate::memory::word::Word;

struct Registers {
    rA: Word,
    rX: Word,

    rI1: ShortRegister,
    rI2: ShortRegister,
    rI3: ShortRegister,
    rI4: ShortRegister,
    rI5: ShortRegister,
    rI6: ShortRegister,

    rJ: ShortRegister,
}

struct ShortRegister {
    value: Word,
}
