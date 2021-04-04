use sysinfo::{SystemExt, System, ProcessExt};

fn get_cur_procs(system: &mut System) -> Vec<String>{
    system.refresh_all();

    let mut  result: Vec<String> = vec![];

    system.get_processes().iter().for_each(|(_, name)|{
        result.push(String::from(name.name()));
    });

    result
}

fn main() {
    let mut system = System::new_all();
    #[warn(unused_variables)]
    let current_procs = get_cur_procs(&mut system);
}
