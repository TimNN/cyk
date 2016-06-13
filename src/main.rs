use std::collections::{HashSet, BTreeMap, BTreeSet};
use std::io;
use std::io::BufRead;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Simple {
    rule: char,
    letter: char,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Complex {
    rule: char,
    first: char,
    second: char,
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut lines = stdin.lines().map(|l| l.unwrap());

    let mut start = None;
    let mut vars = HashSet::new();
    let mut simple = HashSet::new();
    let mut complex = HashSet::new();
    let mut words = Vec::new();

    for line in &mut lines {
        if line == "--" { break; }

        let (rule, rhs) = line.split_at(line.find('→').unwrap());
        let rhs = &rhs['→'.len_utf8()..];
        let rule = rule.trim();
        assert!(rule.len() == 1);
        let rule = rule.chars().next().unwrap();
        assert!(rule.is_uppercase());

        if let None = start {
            start = Some(rule)
        }

        vars.insert(rule);

        for replacement in rhs.split(',') {
            let replacement = replacement.trim();

            match replacement.len() {
                1 => {
                    assert!(replacement.chars().all(|c| c.is_lowercase()));
                    assert!(simple.insert(Simple { rule: rule, letter: replacement.chars().next().unwrap() }));
                }
                2 => {
                    assert!(replacement.chars().all(|c| c.is_uppercase()));
                    let mut chars = replacement.chars();
                    assert!(complex.insert(Complex { rule: rule, first: chars.next().unwrap(), second: chars.next().unwrap() }));
                }
                _ => println!("{}", replacement),
            }
        }
    }

    let start = start.unwrap();

    for line in &mut lines {
        words.push(line.trim().chars().collect::<Vec<_>>());
    }

    println!("{} rules ({} simple, {} complex); {} words", simple.len() + complex.len(), simple.len(), complex.len(), words.len());
    println!("start rule is `{}`", start);

    for word in words {
        macro_rules! insert {($c:expr, $k:expr, $v:expr) => {{
            println!("$V_{{{},{}}} := \\{{{}\\}}$\n", $k.0, $k.1 + 1, $v.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "));
            $c.insert($k, $v)
        }}}

        let n = word.len();
        let mut cache = BTreeMap::new();

        for k in 0 .. n {
            let v = vars.iter().filter(|v| simple.contains(&Simple { rule: **v, letter: word[k] })).cloned().collect::<BTreeSet<_>>();
            // cache.insert((1, k), v);
            insert!(cache, (1, k), v);
        }

        for l in 2 .. n + 1 {
            for k in 0 .. (n - l + 1) {
                let mut new = BTreeSet::new();

                for ll in 1 .. l {
                    for cmp in &complex {
                        if cache[&(ll,k)].contains(&cmp.first) && cache[&(l - ll, k + ll)].contains(&cmp.second) {
                            new.insert(cmp.rule);
                        }
                    }
                }

                // assert!(cache.insert((l, k), new).is_none());
                assert!(insert!(cache, (l, k), new).is_none());
            }
        }

        println!("{}: {}", word.iter().cloned().collect::<String>(), if cache[&(n, 0)].contains(&start) { "accept" } else { "reject" })
    }
}
