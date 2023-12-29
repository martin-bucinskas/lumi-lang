use super::REPL;
use crate::util::visualize_program;
use log::{error, info};

impl REPL {
    pub(crate) fn command_quit(&self, args: Vec<String>) {
        info!("Exiting...");
        std::process::exit(0);
    }

    pub(crate) fn command_history(&self, args: Vec<String>) {
        for command in &self.command_buffer {
            info!("{}", command);
        }
    }

    pub(crate) fn command_clear_program(&mut self, args: Vec<String>) {
        info!("Clearing the program vector...");
        self.vm.program.clear();
    }

    pub(crate) fn command_clear_registers(&mut self, args: Vec<String>) {
        info!("Clearing the VMs registers...");
        self.vm.registers = [0; 32];
        self.vm.float_registers = [0.0; 32];
    }

    pub(crate) fn command_registers(&self, args: Vec<String>) {
        info!("Listing registers and their contents: ");
        info!("{:#?}", self.vm.registers);
    }

    pub(crate) fn command_symbols(&mut self, args: Vec<String>) {
        info!("Listing symbols: ");
        info!("{:#?}", self.asm.symbols);
    }

    pub(crate) fn command_hex_dump(&mut self, args: Vec<String>) {
        info!("Currently loaded program: ");
        visualize_program(self.vm.get_program(), None);
    }

    pub(crate) fn command_spawn(&mut self, args: Vec<String>) {
        let file_to_run = args.get(0).unwrap();

        let binary = self.get_binary_data_from_file(file_to_run.to_string());
        if binary.is_none() {
            error!("Failed to read file");
            return;
        }

        self.vm.get_program().append(&mut binary.unwrap());
        let _events = self.scheduler.get_thread(self.vm.clone());
    }

    pub(crate) fn command_run(&mut self, args: Vec<String>) {
        let file_to_run = args.get(0).unwrap();

        let binary = self.get_binary_data_from_file(file_to_run.to_string());
        if binary.is_none() {
            error!("Failed to read file");
            return;
        }

        self.vm.get_program().append(&mut binary.unwrap());
        let events = self.vm.run();
        for event in &events {
            info!("{:#?}", event);
        }
    }

    pub(crate) fn command_load_program(&mut self, args: Vec<String>) {
        let file_to_load = args.get(0).unwrap();

        let binary = self.get_binary_data_from_file(file_to_load.to_string());
        if binary.is_none() {
            error!("Failed to read file");
            return;
        }

        self.vm.get_program().append(&mut binary.unwrap());
    }

    pub(crate) fn command_assemble(&mut self, args: Vec<String>) {
        let file_to_assemble = args.get(0).unwrap();

        match self.asm.assemble_file(&file_to_assemble) {
            true => info!("File successfully assembled"),
            false => error!("Failed to assemble file"),
        };
    }
}
