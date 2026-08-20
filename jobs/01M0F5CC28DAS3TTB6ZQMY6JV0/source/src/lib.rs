#[derive(Clone, Debug, Hash)]
pub struct ArgumentDescription {
    name: String,
    description: String,
    in_short: String,
}

impl ArgumentDescription {
    ///
    /// Create a new empty ArgumentDescription Struct 
    /// ### Usage
    /// ```
    /// use argrust::ArgumentDescription;
    /// 
    /// let argument_desc_for_install_arg: ArgumentDescription = ArgumentDescription::new();
    /// ```
    pub fn new() -> ArgumentDescription {
        ArgumentDescription{
            name: String::new(),
            description: String::new(),
            in_short: String::new(),
        }
    }

    ///
    /// add name to an ArgumentDescription type struct.
    /// ### Partial Usage
    /// ```
    /// use argrust::ArgumentDescription;
    /// 
    /// let arg_desc: ArgumentDescription = ArgumentDescription::new().name("install");
    /// ```
    /// ### Full Usage
    /// ```
    /// use argrust::{Arguments, ArgumentDescription};
    /// 
    /// let mut args = Arguments::new("None");
    /// args.add("install", ArgumentDescription::new().name("install").description("installs some code").short_argument("inst"));
    /// ```
    pub fn name(self, name: &str) -> ArgumentDescription {
        ArgumentDescription{
            name: name.to_string(),
            description: self.description.clone(),
            in_short: self.in_short.clone(),
        }
    }

    ///
    /// add description to an ArgumentDescription type struct.
    /// ### Partial Usage
    /// ```
    /// use argrust::ArgumentDescription;
    /// 
    /// let arg_desc: ArgumentDescription = ArgumentDescription::new().description("installs some code");
    /// ```
    /// ### Full Usage
    /// ```
    /// use argrust::{Arguments, ArgumentDescription};
    /// 
    /// let mut args = Arguments::new("None");
    /// args.add("install", ArgumentDescription::new().name("install").description("installs some code").short_argument("inst"));
    /// ```
    pub fn description(&self, desctription: &str) -> ArgumentDescription {
        ArgumentDescription {
            name: self.name.clone(),
            description: desctription.to_string(),
            in_short: self.in_short.clone(),
        }
    }

    ///
    /// add shorter version of the argument to an ArgumentDescription type struct.
    /// ## THIS METHOD IS MANDATORY!
    /// 
    /// ### Partial Usage
    /// ```
    /// use argrust::ArgumentDescription;
    /// 
    /// let arg_desc: ArgumentDescription = ArgumentDescription::new().short_argument("inst");
    /// ```
    /// ### Full Usage
    /// ```
    /// use argrust::{Arguments, ArgumentDescription};
    /// 
    /// let mut args = Arguments::new("None");
    /// args.add("install", ArgumentDescription::new().name("install").description("installs some code").short_argument("inst"));
    /// ```
    pub fn short_argument(&self, arg: &str) -> ArgumentDescription {
        ArgumentDescription {
            name: self.name.clone(),
            description: self.description.clone(),
            in_short: arg.to_string(),
        }
    }
}

#[derive(Clone, Debug, Hash)]
pub struct ArgData {
    description: Vec<String>,
}

impl ArgData {
    ///
    /// get a vector<String> with all info
    /// 
    /// `vector[0] = name`
    /// 
    /// `vector[1] = description`
    /// 
    /// `vector[2] = short version of the argument`
    pub fn get(&self) -> Vec<String> {
        self.description.clone()
    }

    /// Get the name from the ArgData
    pub fn get_name(&self) -> &str {
        if self.description.len() == 0 {
            return "None"
        }

        &self.description[0]
    }

    /// Get the description from the ArgData
    pub fn get_description(&self) -> &str {
        if self.description.len() <= 1 {
            return "None";
        }

        &self.description[1]
    }

    /// Get the short version of the argument from ArgData
    pub fn get_short_version(&self) -> &str {
        if self.description.len() <= 2 {
            return "None"
        }

        &self.description[2]
    }
}

#[derive(Clone, Debug, Hash)]
pub struct Arguments {
    got_args: Vec<String>,
    all_args: Vec<String>,
    arguments: Vec<String>,
    arg_data: Vec<ArgumentDescription>,
    used: Vec<String>,
    prefix: Vec<String>,
}

impl Arguments {
    ///
    /// Create a `Arguments` type struct that auto captures arguments.
    /// ### parameters:
    /// - `prefix` (String): The prefix that you are using for your arguments.
    /// 
    /// for example: for `-i` and `--install` as short and long version of argument respectively.
    /// 
    /// set prefix to [ '-', '--' ]
    /// 
    /// ### `NOTE: prefix must have at most 2 values and minimum 1 value.` 
    /// 
    /// ### Usage
    /// ```
    /// use argrust::Arguments;
    /// 
    /// let args = Arguments::new(vec!["-".to_string(), "--".to_string()]);
    /// ```
    /// 
    /// ### Extra hint
    /// If you want to create arguments without prefix, use the below implementation
    /// ```
    /// use argrust::Arguments;
    /// 
    /// let args = Arguments::new("None"); // or "Null"
    /// 
    /// // similarly, if you want to only add one prefix,
    /// let args_ = Arguments::new("-");
    /// ```
    pub fn new<T>(prefix: T) -> Arguments
    where
        T: Prefix
    {
        if prefix.if_null() {
            Arguments {
                got_args: std::env::args().skip(1).collect(),
                all_args: Vec::new(),
                arguments: Vec::new(),
                arg_data: Vec::new(),
                used: Vec::new(),
                prefix: vec!["None".to_string()],
            }
        } else if prefix.if_vector() {
            Arguments {
                got_args: std::env::args().skip(1).collect(),
                all_args: Vec::new(),
                arguments: Vec::new(),
                arg_data: Vec::new(),
                used: Vec::new(),
                prefix: prefix.return_vec(),
            }
        } else if prefix.if_str() {
            Arguments {
                got_args: std::env::args().skip(1).collect(),
                all_args: Vec::new(),
                arguments: Vec::new(),
                arg_data: Vec::new(),
                used: Vec::new(),
                prefix: prefix.return_vec(),
            }
        } else {
            Arguments {
                got_args: std::env::args().skip(1).collect(),
                all_args: Vec::new(),
                arguments: Vec::new(),
                arg_data: Vec::new(),
                used: Vec::new(),
                prefix: vec!["None".to_string()],
            }
        }
    }

    ///
    /// Define a valid argument.
    /// ### Usage
    /// ```
    /// use argrust::{Arguments, ArgumentDescription};
    /// 
    /// let mut args = Arguments::new("None");
    /// args.add(
    ///     "install",
    ///     ArgumentDescription
    /// )
    ///
    /// ```
    pub fn add(&mut self, arg: &str, description: ArgumentDescription) {
        self.all_args.push(arg.to_string());
        self.arg_data.push(description);
    }

    pub fn get_arg_description(&self, arg: &str) -> ArgData {
        let mut description: Vec<String> = Vec::new();

        for x in 0..=self.all_args.len() - 1 {
            if self.all_args[x] == arg.to_string()
                || self.arg_data[x].in_short == arg.to_string() {
                    description.push(self.arg_data[x].name.clone());
                    description.push(self.arg_data[x].description.clone());
                    description.push(self.arg_data[x].in_short.clone());
                    break;
                }
        }

        ArgData{ description }
    }

    ///
    /// Removes a defined argument from storage
    /// ### Usage
    /// ```
    /// use argrust::{Arguments, ArgumentDescription, Fetch};
    /// 
    /// let mut args = Arguments::new("None"); 
    /// args.add("install", ArgumentDescription::new().short_argument("inst"));
    /// args.add("install2", ArgumentDescription::new().short_argument("inst2"));
    /// 
    /// assert_eq!(args.fetch_defined(), vec!["install".to_string(), "install2".to_string()]);
    /// 
    /// args.remove("install");
    /// 
    /// assert_eq!(args.fetch_defined(), vec!["install2".to_string()]);
    /// ```
    pub fn remove(&mut self, arg: &str) {
        let mut new = Arguments {
            got_args: self.got_args.clone(),
            all_args: Vec::new(),
            arguments: self.arguments.clone(),
            arg_data: Vec::new(),
            used: self.used.clone(),
            prefix: self.prefix.clone(),
        };

        for x in 0..=self.all_args.len() - 1 {
            if self.all_args[x] == arg.to_string()
                || self.arg_data[x].in_short == arg.to_string()
            {
                continue;
            } else {
                new.all_args.push(self.all_args[x].clone());
                new.arg_data.push(self.arg_data[x].clone());
            }
        }

        self.all_args = new.all_args;
        self.arg_data = new.arg_data;
    }
}

pub trait Parse {
    fn parse(&mut self);
    fn raise_errors(&mut self) -> Result<(), Errors>;
    fn is_arg(&mut self, arg: &str) -> Result<bool, Errors>;
    fn is_arg_f(&self, arg: &str) -> Result<bool, Errors>;
}

impl Parse for Arguments {
    ///
    /// Parse the captured arguments against defined arguments to get all the passed arguments.
    /// 
    /// ### Usage
    /// ```
    /// use argrust::{Arguments, ArgumentDescription, Parse};
    /// 
    /// let mut args = Arguments::new("None");
    /// args.add("install", ArgumentDescription::new().short_argument("inst"));
    /// 
    /// args.parse();
    /// 
    /// // further codes based on what args are present.
    /// ```
    fn parse(&mut self) {
        if self.arguments.len() != 0 {
            self.arguments.clear();
        }

        // start
        for x in 0..=self.all_args.len() - 1 {
            if self.got_args.contains(&self.all_args[x])
                || self.got_args.contains(&self.arg_data[x].in_short) 
            {
                self.arguments.push(self.all_args[x].clone());
            }
        }
    }

    ///
    /// To raise any errors occured while defining arguments or parsing
    /// 
    /// ### Usage
    /// 
    /// ```no_run
    /// use argrust::{Arguments, ArgumentDescription, Parse};
    /// 
    /// let mut args = Arguments::new("-");
    /// args.add("install", ArgumentDescription::new().short_argument("inst"));
    /// 
    /// args.parse();
    /// match args.raise_errors() {
    ///     Ok(()) => {},
    ///     Err(e) => eprintln!("<Failed-Text>: {:?}", e)
    /// }
    /// ```
    /// 
    fn raise_errors(&mut self) -> Result<(), Errors> {
        // if self.arguments.len() == 0 {
        //     self.parse();
        // }

        // at this point, self.prefix has atleast one value.
        if self.prefix[0] == "None".to_string() {} 
        else if self.prefix.len() == 1 {
            let prefix = self.prefix[0].clone();
            for x in 0..=self.all_args.len() - 1 {
                if !self.all_args[x].starts_with(&prefix) || !self.arg_data[x].in_short.starts_with(&prefix) {
                    return Err(Errors::PrefixError(format!("argrust: \'{}\' prefix is missing while adding arguments. This error is in the code.", prefix)))
                } else {
                    continue;
                }
            }
        } else if self.prefix.len() == 2 {
            let prefix1 = self.prefix[0].clone();
            let prefix2 = self.prefix[1].clone();
            
            for x in 0..=self.all_args.len() - 1 {
                if !self.all_args[x].starts_with(&prefix1) && !self.all_args[x].starts_with(&prefix2) {
                    return Err(Errors::PrefixError(format!("argrust: \'{}\' or \'{}\' prefix is missing while adding arguments. This error is in the code.", prefix1, prefix2)))
                } else if !self.arg_data[x].in_short.starts_with(&prefix1) && !self.arg_data[x].in_short.starts_with(&prefix2) {
                    return Err(Errors::PrefixError(format!("argrust: \'{}\' or \'{}\' prefix is missing while adding arguments. This error is in the code.", prefix1, prefix2)))
                } else {
                    continue;
                }
            }
        }

        // parsing errors
        if self.arguments.len() == 0 {
            if self.got_args.len() > 0 {
                return Err(Errors::ParseErr(format!("argrust: Cannot identify arguments: {}", self.got_args[0])))
            } else {
                return Ok(())
            }
        }
        let mut short_args: Vec<String> = Vec::new();
        for x in 0..=self.arg_data.len() - 1 {
            short_args.push(self.arg_data[x].in_short.clone());
        }
        for x in 0..=self.got_args.len() - 1 {
            if self.prefix.len() == 1 && self.prefix[0] != "None".to_string() {
                if self.got_args[x].starts_with(&self.prefix[0]) && !self.arguments.contains(&self.got_args[x]) {
                    return Err(Errors::ParseErr(format!("argrust: Unknown argument: {}", self.got_args[x])))
                } else {
                    continue;
                }
            } else if self.prefix.len() == 2 {
                if self.got_args[x].starts_with(&self.prefix[0]) || self.got_args[x].starts_with(&self.prefix[1]) {
                    let mut check = false;
                    for y in 0..=self.arg_data.len() - 1 {
                        if self.got_args[x] == self.arg_data[y].in_short {
                            check = true;
                            break;
                        } else if self.got_args[x] == self.all_args[y] {
                            check = true;
                            break;
                        } else {
                            continue;
                        }
                    }
                    // println!("{}", check);
                    if check == false {
                        return Err(Errors::ParseErr(format!("argrust: Unknown argument: {}", self.got_args[x])))
                    }    
                } else {
                    continue;
                }
            }
        }

        Ok(())
    }

    ///
    /// check if an argument is present in the passed arguments pool
    /// ### `NOTE: this method can only be used once for each argument defined. calling it more than once will raise an error that the current argument is already addressed. This is for safe usage of arguments and avoiding errors in code. But if the need be, and it is absolutely necessary to check for this argument more than once, use "is_arg_f"`
    /// ### Usage
    /// ```no_run
    /// use argrust::{Arguments, ArgumentDescription, Parse};
    /// 
    /// let mut args = Arguments::new(vec!["-".to_string(), "--".to_string()]);
    /// 
    /// args.add("--install", ArgumentDescription::new().short_argument("-inst"));
    /// 
    /// if match args.is_arg("--install") {
    ///     Ok(val: bool) => val,
    ///     Err(e) => {
    ///         eprintln!("Failed: {:?}", e);
    ///         false
    ///     },
    /// } {
    ///     println!("--install argument is present.")
    /// }
    /// ```
    fn is_arg(&mut self, arg: &str) -> Result<bool, Errors> {
        if self.arguments.len() == 0 {
            return Ok(false)
        }

        // check if the arg is valid
        let mut defined_short_args: Vec<String> = Vec::new();
        for x in 0..=self.arg_data.len() - 1 {
            defined_short_args.push(self.arg_data[x].in_short.clone());
        }
        if !self.all_args.contains(&arg.to_string()) && !defined_short_args.contains(&arg.to_string()) {
            return Err(Errors::ArgumentNotFound(format!("{} is not in the defined arguments but it is queried. Check and fix it in the code.", arg)))
        }

        // check if used already
        if self.used.contains(&arg.to_string()) {
            return Err(Errors::UsedErr(format!("argrust: {} argument has already been addressed. This error is caused to repeated calling of is_arg method for the same argument more than once. This error is likely generated due to error in code. If checking of the same argument is needed more than once, use: is_arg_f.", arg)))
        }

        let mut flag: bool = false;

        // check
        for x in 0..=self.all_args.len() - 1 {
            // may be - is provided and -- is queried.
            if self.arg_data[x].in_short == arg.to_string() && self.arguments.contains(&self.all_args[x]) {
                flag = true;
                self.used.push(arg.to_string());
                self.used.push(self.all_args[x].clone());
                break;
                // may be -- is provided bur - is queried
            } else if self.all_args[x] == arg.to_string() && self.arguments.contains(&self.arg_data[x].in_short) {
                flag = true;
                self.used.push(arg.to_string());
                self.used.push(self.arg_data[x].in_short.clone());
                break;
                // may be -- is provided and -- is queried
            } else if self.all_args[x] == arg.to_string() && self.arguments.contains(&arg.to_string()) {
                flag = true;
                self.used.push(arg.to_string());
                self.used.push(self.arg_data[x].in_short.clone());
                break;
                // may be - is provided and - is queried
            } else if self.arg_data[x].in_short == arg.to_string() && self.arguments.contains(&arg.to_string()) {
                flag = true;
                self.used.push(arg.to_string());
                self.used.push(self.all_args[x].to_string());
                break;
            }
        }

        Ok(flag)
    }

    ///
    /// Simply check if an argument is in the passed arguments pool.
    /// 
    /// ### `NOTE: This is the same as "is_arg" but it can be called multiple times without any risk of errors. This method is highly discouraged.`
    /// 
    /// ### Usage
    /// 
    /// ```no_run
    /// use argrust::{Arguments, ArgumentDescription, Parse};
    /// 
    /// let mut args = Arguments::new(vec!["-".to_string(), "--".to_string()]);
    /// 
    /// args.add("--install", ArgumentDescription::new().short_argument("-inst"));
    /// 
    /// if match args.is_arg_f("--install") {
    ///     Ok(val: bool) => val,
    ///     Err(e) => {
    ///         eprintln!("Failed: {:?}", e);
    ///         false
    ///     },
    /// } {
    ///     println!("--install argument is present.")
    /// }
    /// ```
    fn is_arg_f(&self, arg: &str) -> Result<bool, Errors> {
        if self.arguments.len() == 0 {
            return Ok(false)
        }

        // check if the arg is valid
        let mut defined_short_args: Vec<String> = Vec::new();
        for x in 0..=self.arg_data.len() - 1 {
            defined_short_args.push(self.arg_data[x].in_short.clone());
        }
        if !self.all_args.contains(&arg.to_string()) && !defined_short_args.contains(&arg.to_string()) {
            return Err(Errors::ArgumentNotFound(format!("{} is not in the defined arguments but it is queried. Check and fix it in the code.", arg)))
        }

        let mut flag: bool = false;

        // check
        for x in 0..=self.all_args.len() - 1 {
            // may be - is provided and -- is queried.
            if self.arg_data[x].in_short == arg.to_string() && self.arguments.contains(&self.all_args[x]) {
                flag = true;
                break;
                // may be -- is provided bur - is queried
            } else if self.all_args[x] == arg.to_string() && self.arguments.contains(&self.arg_data[x].in_short) {
                flag = true;
                break;
                // may be -- is provided and -- is queried
            } else if self.all_args[x] == arg.to_string() && self.arguments.contains(&arg.to_string()) {
                flag = true;
                break;
                // may be - is provided and - is queried
            } else if self.arg_data[x].in_short == arg.to_string() && self.arguments.contains(&arg.to_string()) {
                flag = true;
                break;
            }
        }
        Ok(flag)
    }
}

pub trait Fetch {
    fn fetch_single(&self, arg: &str) -> Result<&str, Errors>;
    fn fetch_till_next(&self, arg: &str) -> Result<Vec<&str>, Errors>;
    fn fetch_defined(&self) -> Vec<String>;
}

impl Fetch for Arguments {
    ///
    /// Fetches a value of the queried argument if present. If not, it returns an error.
    /// 
    /// ### Usage
    /// ```no_run
    /// use argrust::{Arguments, ArgumentDescription, Parse, Fetch};
    /// 
    /// let mut args = Arguments::new(vec!["-".to_string(), "--".to_string()]);
    /// 
    /// args.add("--install", ArgumentDescription::new().short_argument("-inst"));
    /// 
    /// if match args.is_arg("--install") {
    ///     Ok(val: bool) => val,
    ///     Err(e) => {
    ///         eprintln!("Failed: {:?}", e);
    ///         false
    ///     },
    /// } {
    ///     println!("--install argument is present.")
    ///     let value = match args.fetch_single("--install") {
    ///         Ok(value: &str) => value,
    ///         Err(_e) => "",
    ///     };
    ///     
    ///     println!("value of --install argument: {]", value);
    /// }
    /// ```
    fn fetch_single(&self, arg: &str) -> Result<&str, Errors> {
        // check validity of the argument
        let mut defined_short_args: Vec<String> = Vec::new();
        for x in 0..=self.arg_data.len() - 1 {
            defined_short_args.push(self.arg_data[x].in_short.clone());
        }
        if !self.all_args.contains(&arg.to_string()) && !defined_short_args.contains(&arg.to_string()) {
            return Err(Errors::ArgumentNotFound(format!("{} is not in the defined arguments but it is queried. Check and fix it in the code.", arg)))
        }

        let mut index: usize = 0;
        // get the index of the argument
        if self.got_args.contains(&arg.to_string()) {
            // if the arg is queried as it is
            let mut count: usize = 0;
            for x in &self.got_args {
                count += 1;
                if x == arg {
                    break;
                }
            }

            index = count - 1;
        }

        // if still not found
        if index == 0 {
            for x in 0..=self.all_args.len() - 1 {
                // if query is - and present is --
                if self.arg_data[x].in_short == arg.to_string() && self.got_args.contains(&self.all_args[x]) {
                    let mut count: usize = 0;
                    for val in &self.got_args {
                        count += 1;
                        if val == &self.all_args[x] {
                            break;
                        }
                    }
                    index = count - 1;
                    // if query is -- and present is -inst
                } else if self.all_args[x] == arg.to_string() && self.got_args.contains(&self.arg_data[x].in_short) {
                    let mut count: usize = 0;
                    for val in &self.got_args {
                        count += 1;
                        if val == &self.arg_data[x].in_short {
                            break;
                        }
                    }
                    index = count - 1;
                }
            }
        }

        if index == self.got_args.len() {
            return Err(Errors::ArgumentNotFound(format!("No values provided for {}.", arg)))
        }

        Ok(&self.got_args[index + 1])
    }

    fn fetch_till_next(&self, arg: &str) -> Result<Vec<&str>, Errors> {
        // check validity of the argument
        let mut defined_short_args: Vec<String> = Vec::new();
        for x in 0..=self.arg_data.len() - 1 {
            defined_short_args.push(self.arg_data[x].in_short.clone());
        }
        if !self.all_args.contains(&arg.to_string()) && !defined_short_args.contains(&arg.to_string()) {
            return Err(Errors::ArgumentNotFound(format!("{} is not in the defined arguments but it is queried. Check and fix it in the code.", arg)))
        }

// 
// 
// 
//         
        Ok(Vec::new())
    }

    fn fetch_defined(&self) -> Vec<String> {
        self.all_args.clone()
    }
}

pub trait Display {
    fn print_defined(&self);
    fn print_hit(&self);
}

impl Display for Arguments {
    fn print_defined(&self) {
        println!("{:?}", self.all_args);
    }

    fn print_hit(&self) {
        println!("{:?}", self.arguments);
    }
}

pub trait Prefix {
    fn if_null(&self) -> bool;
    fn if_vector(&self) -> bool;
    fn if_str(&self) -> bool;
    fn return_vec(&self) -> Vec<String>;
    fn return_str(&self) -> &str;
}

impl Prefix for &str {
    fn if_null(&self) -> bool {
        if self.to_lowercase() == "none".to_string() || self.to_lowercase() == "null".to_string() {
            true
        } else {
            false
        }
    }

    fn if_vector(&self) -> bool {
        false
    }

    fn if_str(&self) -> bool {
        true
    }

    fn return_str(&self) -> &str {
        self
    }

    fn return_vec(&self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl Prefix for Vec<String> {
    fn if_null(&self) -> bool {
        if self.len() == 0 {
            true
        } else if self.len() == 1 {
            if self[0].to_lowercase() == "none".to_string() || self[0].to_lowercase() == "null".to_string() {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn if_str(&self) -> bool {
        false
    }

    fn if_vector(&self) -> bool {
        true
    }

    fn return_str(&self) -> &str {
        ""
    }

    fn return_vec(&self) -> Vec<String> {
        self.to_owned()
    }
}

#[derive(Debug)]
pub enum Errors {
    PrefixError(String),
    ParseErr(String),
    UsedErr(String),
    ArgumentNotFound(String),
}