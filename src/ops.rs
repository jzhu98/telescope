use ast::{Atom, Expr};
use error::*;

fn unwrap_atoms<I>(args: I) -> Result<Vec<Atom>>
    where I: Iterator<Item = Expr>
{
    args.map(Expr::atom)
        .collect::<Option<Vec<_>>>()
        .ok_or("expected atom".into())
}

fn check_args(f: &str, args: &[Expr], arity: usize) -> Result<()> {
    if args.len() == arity {
        Ok(())
    } else {
        Err(format!("{} expected {} arguments", f, arity).into())
    }
}

pub fn add(args: &[Expr]) -> Result<Expr> {
    // Unwrap to atoms
    let atoms = unwrap_atoms(args.iter().cloned())?;

    // Check all arguments are numeric
    if atoms.iter().all(Atom::is_num) {
        if atoms.iter().any(Atom::is_flt) {
            // If any are float, promote to float and perform float addition
            Ok(Expr::from(atoms.into_iter()
                .map(|a| a.map_int(|x: i64| x as f64))
                .map(|x| x.flt().unwrap())
                .sum::<f64>()))
        } else {
            // Otherwise perform integer addition
            Ok(Expr::from(atoms.into_iter()
                .map(|x| x.int().unwrap())
                .sum::<i64>()))
        }
    } else {
        Err("#[+] expected numeric".into())
    }
}

pub fn sub(args: &[Expr]) -> Result<Expr> {
    // Unwrap to atoms
    let atoms = unwrap_atoms(args.iter().cloned())?;

    // Check all arguments are numeric
    if !atoms.iter().all(Atom::is_num) {
        return Err("#[-] expected numeric".into());
    }

    // If any are float, promote all to float and perform float addition
    if atoms.iter().any(Atom::is_flt) {
        // If one argument, negate and return
        if atoms.len() == 1 {
            let mut atoms = atoms;
            return Ok(Expr::from(atoms.remove(0).map_flt(|x| -x)))
        }
        let mut nums = atoms.into_iter()
            .map(|a| a.map_int(|x: i64| x as f64))
            .map(|x| x.flt().unwrap());
            
        nums.next()
            .map(|first| {
                Expr::from(nums.fold(first, |acc, x| acc - x))
            }).ok_or("#[-] expected 1 (negate) or 2+ (subtract) arguments".into())
    }
    // Otherwise, perform integer addition
    else {
        // If one argument, negate and return
        if atoms.len() == 1 {
            let mut atoms = atoms;
            return Ok(Expr::from(atoms.remove(0).map_int(|x| -x)))
        }
        let mut nums = atoms.iter()
            .map(|x| x.int().unwrap());
        
        nums.next()
            .map(|first| {
                Expr::from(nums.fold(first, |acc, x| acc - x))
            }).ok_or("Expected 1 (negation) or 2+ (subtraction) arguments".into())
    }
}

pub fn mul(args: &[Expr]) -> Result<Expr> {
    // Unwrap to atoms
    let atoms = unwrap_atoms(args.iter().cloned())?;
    
    // Check all arguments are numeric
    if atoms.iter().all(Atom::is_num) {
        if atoms.iter().any(Atom::is_flt) {
            // If any are float, promote to float and perform float addition
            Ok(Expr::from(atoms.into_iter()
                .map(|a| a.map_int(|x: i64| x as f64))
                .map(|x| x.flt().unwrap())
                .product::<f64>()))
        } else {
            // Otherwise perform integer addition
            Ok(Expr::from(atoms.into_iter()
                .map(|x| x.int().unwrap())
                .product::<i64>()))
        }
    } else {
        Err("#[*] expected numeric".into())
    }
}

pub fn div(args: &[Expr]) -> Result<Expr> {
    // Unwrap to atoms
    let atoms = unwrap_atoms(args.iter().cloned())?;

    // Check all arguments are numeric
    if !atoms.iter().all(Atom::is_num) {
        return Err("#[-] expected numeric".into());
    }

    // If any are float, promote all to float and perform float addition
    if atoms.iter().any(Atom::is_flt) {
        let mut nums = atoms.into_iter()
            .map(|a| a.map_int(|x: i64| x as f64))
            .map(|x| x.flt().unwrap());
            
        nums.next()
            .map(|first| {
                Expr::from(nums.fold(first, |acc, x| acc / x))
            }).ok_or("#[/] expected at least 2 args".into())
    }
    // Otherwise, perform integer addition
    else {
        let mut nums = atoms.iter()
            .map(|x| x.int().unwrap());
        
        nums.next()
            .ok_or("#[/] expected at least 2 args".into())
            .and_then(|mut first| {
                for num in nums {
                    if num == 0 {
                        return Err("division by zero".into());
                    }
                    first /= num;
                }
                Ok(Expr::from(first))
            })
    }
}

pub fn equal(args: &[Expr]) -> Result<Expr> {
    check_args("#[=]", args, 2)?;
    Ok(Expr::from(args[0] == args[1]))
}

pub fn less(args: &[Expr]) -> Result<Expr> {
    check_args("#[<]", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Atom(ref a), &Expr::Atom(ref b)) => Ok(Expr::from(a < b)),
        _ => Err(format!("comparison undefined for: {}, {}", args[0], args[1]).into()),
    }
}

pub fn less_eq(args: &[Expr]) -> Result<Expr> {
    check_args("#[<=]", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Atom(ref a), &Expr::Atom(ref b)) => Ok(Expr::from(a <= b)),
        _ => Err(format!("comparison undefined for: {}, {}", args[0], args[1]).into()),
    }
}

pub fn greater(args: &[Expr]) -> Result<Expr> {
    check_args("#[>]", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Atom(ref a), &Expr::Atom(ref b)) => Ok(Expr::from(a > b)),
        _ => Err(format!("comparison undefined for: {}, {}", args[0], args[1]).into()),
    }
}

pub fn greater_eq(args: &[Expr]) -> Result<Expr> {
    check_args("#[>=]", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Atom(ref a), &Expr::Atom(ref b)) => Ok(Expr::from(a >= b)),
        _ => Err(format!("comparison undefined for: {}, {}", args[0], args[1]).into()),
    }
}

pub fn not(args: &[Expr]) -> Result<Expr> {
    check_args("#[not]", args, 1)?;
    match &args[0] {
        &Expr::Atom(Atom::Bool(b)) => Ok(Expr::from(!b)),
        _ => Err(format!("negation undefined for: {}", args[0]).into()),
    }
}

pub fn and(args: &[Expr]) -> Result<Expr> {
    // Unwrap to atoms
    let atoms = unwrap_atoms(args.iter().cloned())?;

    atoms.into_iter()
        .map(|a| a.boolean())
        .collect::<Option<Vec<_>>>()
        .ok_or("#[and] expected boolean argument".into())
        .map(|bools| {
            bools.iter().all(|b| *b)
        })
        .map(Expr::from)
}

pub fn or(args: &[Expr]) -> Result<Expr> {
    // Unwrap to atoms
    let atoms = unwrap_atoms(args.iter().cloned())?;

    atoms.into_iter()
        .map(|a| a.boolean())
        .collect::<Option<Vec<_>>>()
        .ok_or("#[or] expected boolean argument".into())
        .map(|bools| {
            bools.iter().any(|b| *b)
        })
        .map(Expr::from)
}

pub fn print(args: &[Expr]) -> Result<Expr> {
    check_args("#[print]", args, 1)?;
    println!("{}", args[0]);
    Ok(Atom::Nil.into())
}
