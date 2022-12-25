use std::{fs, io};
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

#[derive(Clone)]
struct ProcessNode {
    pid: i32,
    comm: String,
    state: char,
    ppid: i32,
    child: Vec<Rc<RefCell<ProcessNode>>>
}

impl ProcessNode {
    fn print(&self) {
        self.print_node(0)
    }

    fn print_node(&self, indent_level: i32) {
        for _ in 0..indent_level {
            print!("  ");
        }
        println!("- {} #{}", self.comm, self.pid);
        for child in &self.child {
            child.borrow().print_node(indent_level + 1)
        }
    }
}

fn main() -> io::Result<()> {
    let proc_dir = fs::read_dir("/proc").unwrap();
    let mut nodes: Vec<Rc<RefCell<ProcessNode>>> = Vec::new();
    for path in proc_dir {
        let path = path.unwrap();
        let file_name = path.file_name();
        let path_str = file_name.to_str().unwrap();
        // 是纯数字名称目录
        if path.path().is_dir() && is_number(path_str) {
            let mut stat_file = File::open(format!("/proc/{}/stat", path_str))?;
            let mut process_info = String::new();
            stat_file.read_to_string(&mut process_info)?;
            let process_info: Vec<String> = process_info.split(" ").map(|s| s.to_string()).collect();
            nodes.push(
                Rc::new(
                    RefCell::new(
                        ProcessNode {
                            pid: process_info[0].parse().unwrap(),
                            comm: process_info[1].clone(),
                            state: process_info[2].parse().unwrap(),
                            ppid: process_info[3].parse().unwrap(),
                            child: Vec::new()
                        }
                    )
                )

            )
        }
    }
    for node in &nodes {
        if node.borrow().ppid == 0 { continue }
        if let Some(value) = nodes.iter().rfind(|n| n.borrow().pid == node.borrow().ppid) {
            value.borrow_mut().child.push(node.clone())
        }
    }
    let roots: Vec<Rc<RefCell<ProcessNode>>> = nodes.iter()
        .filter(|n| n.borrow().ppid == 0)
        .map(|n| n.clone())
        .collect();
    for root in roots {
        root.borrow().print()
    }
    Ok(())
}

fn is_number(str: &str) -> bool {
    str.parse::<i32>().is_ok()
}
