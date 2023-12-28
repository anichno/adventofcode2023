use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target<'a> {
    Accept,
    Reject,
    NextWorkflow(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RatingCategory {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Condition {
    category: RatingCategory,
    operation: Operation,
    value: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rule<'a> {
    condition: Option<Condition>,
    target: Target<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

fn parse<'a>(input: &'a [&str]) -> (HashMap<&'a str, Vec<Rule<'a>>>, Vec<Part>) {
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();

    let mut parsing_workflows = true;
    for line in input {
        if parsing_workflows {
            if line.is_empty() {
                parsing_workflows = false;
                continue;
            }
            let (name, rules_str) = line.split_once('{').unwrap();
            let mut rules = Vec::new();
            for rule_txt in rules_str.trim_end_matches('}').split(',') {
                let (condition, target) = if let Some((cond_txt, tgt)) = rule_txt.split_once(':') {
                    let category = cond_txt.as_bytes()[0];
                    let category = match category {
                        b'x' => RatingCategory::ExtremelyCoolLooking,
                        b'm' => RatingCategory::Musical,
                        b'a' => RatingCategory::Aerodynamic,
                        b's' => RatingCategory::Shiny,
                        _ => panic!("invalid category: {category}"),
                    };
                    let operation = match cond_txt.as_bytes()[1] {
                        b'<' => Operation::LessThan,
                        b'>' => Operation::GreaterThan,
                        _ => panic!("invalid operation: {}", cond_txt.chars().nth(1).unwrap()),
                    };

                    let value = cond_txt[2..].parse().unwrap();
                    (
                        Some(Condition {
                            category,
                            operation,
                            value,
                        }),
                        tgt,
                    )
                } else {
                    // only target, no condition
                    (None, rule_txt)
                };

                let target = match target {
                    "A" => Target::Accept,
                    "R" => Target::Reject,
                    _ => Target::NextWorkflow(target),
                };

                rules.push(Rule { condition, target });
            }
            workflows.insert(name, rules);
        } else {
            let (x, line) = line.trim_start_matches("{x=").split_once(",m=").unwrap();
            let x = x.parse().unwrap();
            let (m, line) = line.split_once(",a=").unwrap();
            let m = m.parse().unwrap();
            let (a, line) = line.split_once(",s=").unwrap();
            let a = a.parse().unwrap();
            let s = line.trim_end_matches('}').parse().unwrap();

            parts.push(Part { x, m, a, s });
        }
    }
    (workflows, parts)
}

fn solve1(input: &[&str]) -> usize {
    let (workflows, parts) = parse(input);
    let mut total = 0;

    for part in parts {
        let mut cur_workflow = workflows.get("in").unwrap().iter();
        loop {
            let rule = cur_workflow.next().expect("ran out of rules");
            let send_to_target = if let Some(condition) = rule.condition {
                // evaluate rule
                let test_val = match condition.category {
                    RatingCategory::ExtremelyCoolLooking => part.x,
                    RatingCategory::Musical => part.m,
                    RatingCategory::Aerodynamic => part.a,
                    RatingCategory::Shiny => part.s,
                };
                match condition.operation {
                    Operation::LessThan => test_val < condition.value,
                    Operation::GreaterThan => test_val > condition.value,
                }
            } else {
                true
            };

            if send_to_target {
                match rule.target {
                    Target::Accept => {
                        total += part.x + part.m + part.a + part.s;
                        break;
                    }
                    Target::Reject => break,
                    Target::NextWorkflow(tgt_workflow) => {
                        cur_workflow = workflows.get(tgt_workflow).unwrap().iter();
                    }
                }
            }
        }
    }

    total
}

#[derive(Debug, Clone, Copy)]
struct XmasState {
    x_bounds: (usize, usize),
    m_bounds: (usize, usize),
    a_bounds: (usize, usize),
    s_bounds: (usize, usize),
}

fn resolve_accept_inputs(
    workflows: &HashMap<&str, Vec<Rule>>,
    cur_workflow: Target,
    rule_offset: usize,
    total_accept_parts: &mut usize,
    xmas_state: XmasState,
) {
    match cur_workflow {
        Target::Accept => {
            *total_accept_parts += (xmas_state.x_bounds.1 + 1 - xmas_state.x_bounds.0)
                * (xmas_state.m_bounds.1 + 1 - xmas_state.m_bounds.0)
                * (xmas_state.a_bounds.1 + 1 - xmas_state.a_bounds.0)
                * (xmas_state.s_bounds.1 + 1 - xmas_state.s_bounds.0);
        }
        Target::Reject => (),
        Target::NextWorkflow(workflow_id) => {
            let workflow = workflows.get(workflow_id).unwrap();
            let rule = workflow[rule_offset];

            if let Some(condition) = rule.condition {
                // variant: we observe this rule
                if !matches!(rule.target, Target::Reject) {
                    let mut split_xmas_state = xmas_state;
                    let range_to_mod = match condition.category {
                        RatingCategory::ExtremelyCoolLooking => &mut split_xmas_state.x_bounds,
                        RatingCategory::Musical => &mut split_xmas_state.m_bounds,
                        RatingCategory::Aerodynamic => &mut split_xmas_state.a_bounds,
                        RatingCategory::Shiny => &mut split_xmas_state.s_bounds,
                    };
                    match condition.operation {
                        Operation::LessThan => {
                            range_to_mod.1 = range_to_mod.1.min(condition.value - 1)
                        }
                        Operation::GreaterThan => {
                            range_to_mod.0 = range_to_mod.0.max(condition.value + 1)
                        }
                    }

                    // make sure this range is still possible
                    if range_to_mod.0 <= range_to_mod.1 {
                        resolve_accept_inputs(
                            workflows,
                            rule.target,
                            0,
                            total_accept_parts,
                            split_xmas_state,
                        );
                    }
                }

                // variant: we DO NOT observe this rule
                let mut split_xmas_state = xmas_state;
                let range_to_mod = match condition.category {
                    RatingCategory::ExtremelyCoolLooking => &mut split_xmas_state.x_bounds,
                    RatingCategory::Musical => &mut split_xmas_state.m_bounds,
                    RatingCategory::Aerodynamic => &mut split_xmas_state.a_bounds,
                    RatingCategory::Shiny => &mut split_xmas_state.s_bounds,
                };

                // do negated operation
                match condition.operation {
                    Operation::LessThan => range_to_mod.0 = range_to_mod.0.max(condition.value),
                    Operation::GreaterThan => range_to_mod.1 = range_to_mod.1.min(condition.value),
                }

                // make sure this range is still possible
                if range_to_mod.0 <= range_to_mod.1 {
                    resolve_accept_inputs(
                        workflows,
                        cur_workflow,
                        rule_offset + 1,
                        total_accept_parts,
                        split_xmas_state,
                    );
                }
            } else {
                resolve_accept_inputs(workflows, rule.target, 0, total_accept_parts, xmas_state);
            }
        }
    }
}

fn solve2(input: &[&str]) -> usize {
    let (workflows, _) = parse(input);
    let mut total_accept_parts = 0;

    resolve_accept_inputs(
        &workflows,
        Target::NextWorkflow("in"),
        0,
        &mut total_accept_parts,
        XmasState {
            x_bounds: (1, 4000),
            m_bounds: (1, 4000),
            a_bounds: (1, 4000),
            s_bounds: (1, 4000),
        },
    );

    total_accept_parts
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "px{a<2006:qkq,m>2090:A,rfg}",
        "pv{a>1716:R,A}",
        "lnx{m>1548:A,A}",
        "rfg{s<537:gd,x>2440:R,A}",
        "qs{s>3448:A,lnx}",
        "qkq{x<1416:A,crn}",
        "crn{x>2662:A,R}",
        "in{s<1351:px,qqz}",
        "qqz{s>2770:qs,m<1801:hdj,R}",
        "gd{a>3333:R,R}",
        "hdj{m>838:A,pv}",
        "",
        "{x=787,m=2655,a=1222,s=2876}",
        "{x=1679,m=44,a=2067,s=496}",
        "{x=2036,m=264,a=79,s=2244}",
        "{x=2461,m=1339,a=466,s=291}",
        "{x=2127,m=1623,a=2188,s=1013}",
    ];

    #[test]
    fn test1() {
        assert_eq!(solve1(INPUT), 19114)
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(INPUT), 167409079868000)
    }
}
