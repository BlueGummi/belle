macro_rules! and(lhs: reg, rhs: reg) {
    nand %lhs, %rhs
    nand %lhs, %lhs
} ; lhs holds the AND result

macro_rules! or(lhs: reg, rhs: reg) {
    nand %lhs, %lhs
    nand %rhs, %rhs
    nand %lhs, %rhs
}

macro_rules! not(argument: reg) {
    nand %argument, %argument
}

macro_rules! nor(lhs: reg, rhs: reg) {
    nand %lhs, %lhs
    nand %rhs, %rhs
    nand %lhs, %rhs
    nand %lhs, %lhs
}
