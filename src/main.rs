use tomasulo_sim::{executer, Instruction};

fn main() {
    let insts = [
        r"
        LD F6 34+ R2
        LD F2 45+ R3
        MULTD F0 F2 F4
        SUBD F8 F6 F2
        DIVD F10 F0 F6
        ADDD F6 F8 F2",
        r"
        LD F2 0 R2
        LD F4 0 R3
        DIVD F0 F4 F2
        MULTD F6 F0 F2
        ADDD F0 F4 F2
        SD F6 0 R3
        MULTD F6 F0 F2
        SD F6 0 R1",
    ];

    let parsed_insts = insts
        .iter()
        .map(|s| {
            s.lines()
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<Instruction>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for insts in parsed_insts.iter() {
        let mut executer = executer::Executer::new();
        executer.add_insts(insts);
        executer.run();
    }
}
