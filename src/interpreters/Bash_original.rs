//Interpreter:| Bash_original       | bash, shell |
//############|_____________________|_____________|________________<- delimiters to help formatting,
//############| Interpretername     | language    | comment
// Keep (but modify the first line after the :) if you wish to have this interpreter listed via SnipList
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Bash_original {
    support_level: SupportLevel,
    data: DataHolder,
    code: String,
    bash_work_dir: String,
    main_file_path: String,
}

impl Interpreter for Bash_original {
    fn new_with_level(data: DataHolder, level: SupportLevel) -> Box<Bash_original> {
        let bwd = data.work_dir.clone() + "/bash-original";
        let mut builder = DirBuilder::new();
        builder.recursive(true);
        builder
            .create(&bwd)
            .expect("Could not create directory for bash-original");
        let mfp = bwd.clone() + "/main.sh";
        Box::new(Bash_original {
            data,
            support_level: level,
            code: String::from(""),
            bash_work_dir: bwd,
            main_file_path: mfp,
        })
    }

    fn get_name() -> String {
        String::from("Bash_original")
    }

    fn behave_repl_like_default() -> bool {
        true
    }

    fn get_supported_languages() -> Vec<String> {
        vec![
            String::from("bash"),
            String::from("shell"),
            String::from("sh"),
        ]
    }

    fn get_current_level(&self) -> SupportLevel {
        self.support_level
    }
    fn set_current_level(&mut self, level: SupportLevel) {
        self.support_level = level;
    }

    fn get_data(&self) -> DataHolder {
        self.data.clone()
    }

    fn get_max_support_level() -> SupportLevel {
        SupportLevel::Bloc
    }

    fn fetch_code(&mut self) -> Result<(), SniprunError> {
        if !self
            .data
            .current_bloc
            .replace(&[' ', '\t', '\n', '\r'][..], "")
            .is_empty()
            && self.get_current_level() >= SupportLevel::Bloc
        {
            self.code = self.data.current_bloc.clone();
        } else if !self.data.current_line.replace(" ", "").is_empty()
            && self.get_current_level() >= SupportLevel::Line
        {
            self.code = self.data.current_line.clone();
        } else {
            self.code = String::from("");
        }
        Ok(())
    }

    fn add_boilerplate(&mut self) -> Result<(), SniprunError> {
        //add shebang just in case
        self.code = String::from("#!/usr/bin/env bash \n") + &self.code;
        Ok(())
    }

    fn build(&mut self) -> Result<(), SniprunError> {
        let mut _file =
            File::create(&self.main_file_path).expect("Failed to create file for bash-original");

        write(&self.main_file_path, &self.code).expect("Unable to write to file for bash-original");
        Ok(())
    }

    fn execute(&mut self) -> Result<String, SniprunError> {
        let output = Command::new("bash")
            .arg(&self.main_file_path)
            .output()
            .expect("Unable to start process");
        info!("yay from bash interpreter");
        if output.status.success() {
            return Ok(String::from_utf8(output.stdout).unwrap());
        } else {
            return Err(SniprunError::RuntimeError(
                String::from_utf8(output.stderr).unwrap(),
            ));
        }
    }
}

impl ReplLikeInterpreter for Bash_original {
    fn fetch_code_repl(&mut self) -> Result<(), SniprunError> {
        self.fetch_code()?;
        let previous_code = self.read_previous_code();
        self.code = previous_code + "\n" + &self.code;
        Ok(())
    }

    fn add_boilerplate_repl(&mut self) -> Result<(), SniprunError> {
        self.add_boilerplate()
    }

    fn build_repl(&mut self) -> Result<(), SniprunError> {
        self.build()
    }

    fn execute_repl(&mut self) -> Result<String, SniprunError> {
        fn strip_prints(code: &str) -> String {
            let mut striped_code = String::new();
            let print_statements = vec!["echo ", "print "];
            let mut count = 0;
            for line in code.lines() {
                //basic splitting only
                let opening_bracket = line.matches("{").count();
                let closing_bracket = line.matches("}").count();
                count += opening_bracket - closing_bracket;
                if count <= 0 {
                    if print_statements.iter().all(|ps| !line.contains(ps)) {
                        // does not contains any print statement
                        striped_code.push_str(line);
                        striped_code.push_str("\n");
                    }
                } else {
                    striped_code.push_str(line);
                    striped_code.push_str("\n");
                }
            }
            return striped_code;
        }

        let res = self.execute();
        if res.is_ok() {
            let _ = self.fetch_code();
            self.save_code(strip_prints(&self.code));
        }

        info!("executed as repl");
        return res;
    }
}
