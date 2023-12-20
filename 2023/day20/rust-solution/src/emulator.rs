use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ops::{Add, AddAssign},
    str::FromStr,
};

const BROADCASTER: &str = "broadcaster";

type RawModule = crate::modules::Module;

#[derive(thiserror::Error, Debug)]
pub enum EmulatorErr {
    #[error("missing the broadcast module")]
    MissingBroadcast,

    #[error("unknown module format ({0})")]
    BadModule(String),

    #[error("missing module named {0}")]
    MissingModule(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Module {
    name: String,
    module: RawModule,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

impl Module {
    fn new(name: String, module: RawModule, inputs: Vec<String>, outputs: Vec<String>) -> Self {
        Self {
            name,
            module,
            inputs,
            outputs,
        }
    }

    fn transmit(&mut self, from: &str, pulse: bool) -> Option<bool> {
        match &mut self.module {
            RawModule::Broadcast(broadcast) => Some(broadcast.receive(pulse)),
            RawModule::Conjunction(con) => Some(con.receive(from, pulse)),
            RawModule::FlipFlop(ff) => ff.receive(pulse),
            RawModule::Unknown(unknown) => {
                unknown.receive(&self.name, from, pulse);
                None
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Emulator {
    modules: BTreeMap<String, Module>,
}

struct Pulse {
    from: String,
    target: String,
    value: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ExecuteResult {
    pub high: usize,
    pub low: usize,
}

impl AddAssign for ExecuteResult {
    fn add_assign(&mut self, rhs: Self) {
        self.high += rhs.high;
        self.low += rhs.low;
    }
}

impl Add for ExecuteResult {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Emulator {
    fn new(modules: BTreeMap<String, Module>) -> Self {
        Self { modules }
    }

    pub fn get_inputs(&self, module: &str) -> Result<&[String], EmulatorErr> {
        let module = self
            .modules
            .get(module)
            .ok_or_else(|| EmulatorErr::MissingModule(module.into()))?;

        Ok(&module.inputs)
    }

    pub fn click(&mut self) -> ExecuteResult {
        let mut queue = vec![Pulse {
            from: "button".into(),
            target: BROADCASTER.into(),
            value: false,
        }];

        let mut res = ExecuteResult::default();
        while let Some(pulse) = queue.pop() {
            match pulse.value {
                true => res.high += 1,
                false => res.low += 1,
            }

            let module = self.modules.get_mut(&pulse.target).unwrap();
            if let Some(value) = module.transmit(&pulse.from, pulse.value) {
                for output in module.outputs.iter() {
                    queue.push(Pulse {
                        from: pulse.target.clone(),
                        target: output.clone(),
                        value,
                    })
                }
            }
        }

        res
    }

    pub fn find_cycle_on_observer(
        mut self,
        observe: &str,
        observe_value: bool,
    ) -> Result<usize, EmulatorErr> {
        for press in 1.. {
            let mut queue = vec![Pulse {
                from: "button".into(),
                target: BROADCASTER.into(),
                value: false,
            }];

            while let Some(pulse) = queue.pop() {
                if pulse.from == observe && pulse.value == observe_value {
                    return Ok(press);
                }

                let module = self.modules.get_mut(&pulse.target).unwrap();
                if let Some(value) = module.transmit(&pulse.from, pulse.value) {
                    for output in module.outputs.iter() {
                        queue.push(Pulse {
                            from: pulse.target.clone(),
                            target: output.clone(),
                            value,
                        })
                    }
                }
            }
        }

        unreachable!()
    }
}

struct ToConnect {
    from: String,
    to: String,
}

#[derive(Default)]
pub struct EmulatorBuilder {
    modules: HashMap<String, Module>,
    to_connect: Vec<ToConnect>,
}

impl EmulatorBuilder {
    fn add_module(&mut self, name: String, module: RawModule, outputs: Vec<String>) {
        self.to_connect.extend(outputs.iter().map(|to| ToConnect {
            from: name.clone(),
            to: to.clone(),
        }));

        let module = Module::new(name, module, vec![], outputs);
        self.modules.insert(module.name.clone(), module);
    }

    fn build(mut self) -> Result<Emulator, EmulatorErr> {
        // connect inputs and generate stub loggers for unknown names
        for to_connect in self.to_connect {
            match self.modules.get_mut(&to_connect.to) {
                Some(module) => {
                    if let RawModule::Conjunction(module) = &mut module.module {
                        module.connect_input(&to_connect.from);
                    }

                    module.inputs.push(to_connect.from);
                }
                None => {
                    let module = self
                        .modules
                        .entry(to_connect.to.clone())
                        .or_insert(Module::new(
                            to_connect.to,
                            RawModule::unknown(),
                            vec![],
                            vec![],
                        ));

                    module.inputs.push(to_connect.from);
                }
            }
        }

        if self.modules.get(BROADCASTER).is_none() {
            return Err(EmulatorErr::MissingBroadcast);
        }

        Ok(Emulator::new(self.modules.into_iter().collect()))
    }
}

impl FromStr for Emulator {
    type Err = EmulatorErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut builder = EmulatorBuilder::default();

        for line in s.trim().lines() {
            let (ident, outputs) = line
                .split_once("->")
                .ok_or_else(|| EmulatorErr::BadModule(line.to_string()))?;
            let outputs = outputs
                .trim()
                .split(',')
                .map(|output| output.trim().to_string())
                .collect::<Vec<_>>();

            let ident = ident.trim();
            if ident == BROADCASTER {
                builder.add_module(ident.into(), RawModule::broadcast(), outputs);
                continue;
            }
            match ident.as_bytes() {
                [b'&', ..] => {
                    builder.add_module(ident[1..].into(), RawModule::conjunction(), outputs)
                }
                [b'%', ..] => builder.add_module(ident[1..].into(), RawModule::flipflop(), outputs),
                _ => builder.add_module(ident.into(), RawModule::unknown(), outputs),
            }
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::{Emulator, EmulatorBuilder, ExecuteResult, RawModule, BROADCASTER};

    #[test]
    fn parse_emulator() {
        let input = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;
        let mut builder = EmulatorBuilder::default();
        builder.add_module(BROADCASTER.into(), RawModule::broadcast(), vec!["a".into()]);
        builder.add_module(
            "a".into(),
            RawModule::flipflop(),
            vec!["inv".into(), "con".into()],
        );
        builder.add_module("inv".into(), RawModule::conjunction(), vec!["b".into()]);
        builder.add_module("b".into(), RawModule::flipflop(), vec!["con".into()]);
        builder.add_module(
            "con".into(),
            RawModule::conjunction(),
            vec!["output".into()],
        );
        let expected_output = builder.build().unwrap();

        let output: Emulator = input.parse().unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn check_full_cycle() {
        let mut emulator: Emulator = r#"broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output"#
            .parse()
            .unwrap();

        macro_rules! check_cycle {
            ($low: expr, $high: expr) => {
                assert_eq!(
                    emulator.click(),
                    ExecuteResult {
                        low: $low,
                        high: $high
                    }
                )
            };
        }

        check_cycle!(4, 4);
        check_cycle!(4, 2);
        check_cycle!(5, 3);
        check_cycle!(4, 2);
    }
}
