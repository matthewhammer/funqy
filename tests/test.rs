#[macro_use]
extern crate funqy;
use funqy::ast::*;
use funqy::engine::*;
use funqy::eval::*;
use funqy::parser::*;

use std::rc::Rc;

extern crate num;
use num::complex::Complex;

fn round(f: Cf32, d: i32) -> Cf32 {
	let m = real!(10_f32.powi(d));
	let f = f * m;
	Complex::new(f.re.round(), f.im.round()) / m
}

#[test]
fn test_parser() {
	
	let exp = parse_file("tests/scripts/Test.fqy").expect("Could not parse file");
	let ctx = Context::new();
	
	println!("{:?}", exp);
	println!(">> {}", eval_exp(&exp, &ctx));
}

// #[test]
fn test_eval() {

	// let a = Exp::Tuple(vec![
	// 	Exp::Var("x"),
	// 	Exp::Var("y"),
	// ]);
	
	// let b = Exp::Tuple(vec![
	// 	Exp::Var("F"),
	// 	Exp::Var("T"),
	// ]);
	
	// let ret = Exp::Extract(Rc::new(Exp::Var("state")), vec![
	// 	Exp::Var("F"),
	// 	Exp::Tuple(vec![Exp::Var("F"), Exp::Var("T")]),
	// 	Exp::Tuple(vec![Exp::Var("T"), Exp::Var("F")]),
	// 	Exp::Var("T"),
	// ]);
	
	// let exp = Exp::Scope(vec![
	// 	Decl::Data("Bool", vec!["F", "T"]),
	// 	Decl::Let(Pat::Var("x"), Exp::Var("T")),
	// 	Decl::Let(Pat::Var("y"), Exp::Var("F")),
	// 	Decl::Let(Pat::Var("state"), Exp::Sup(Rc::new(a), Rc::new(b))),
	// ], Rc::new(Exp::Tuple(vec![
	// 	ret.clone(),
	// 	Exp::Measure(Rc::new(ret)),
	// ])));
	
	let exp = Exp::Tuple(vec![]);
	
	let ctx = Context::new();
	let result = eval_exp(&exp, &ctx);
	
	println!("\n >> {}\n", result);
}

// #[test]
fn test_engine() {
	// fn zero() -> State {vec![real!(1)]}
	// fn one() -> State {vec![real!(0), real!(1)]}
	
	// fn not(s: S2) -> S2 {
	// 	s.extract(S2::one(), S2::zero())
	// }
	
	// fn had(s: S2) -> S2 {
	// 	s.extract(
	// 		S2::zero().sup(S2::one()),
	// 		S2::zero().sup(S2::one().phase_flip()))
	// }
	
	// fn cnot(a: S2, b: S2) -> S2 {
	// 	a.extract(
	// 		b.clone(),
	// 		not(b),
	// 	)
	// }
	
	// fn test4(s: State<S2>) -> State<S2> {
	// 	s.extract(
	// 		(S2::zero(), S2::zero().sup(S2::one())),
	// 		(S2::zero(), S2::zero()),
	// 	)
	// }
	
	// let s = had(had(S2::zero()));
	
	// let s = Zero.sup(One).extract(
	// 	One,
	// 	Zero,
	// );
	
	// let (x, y) = s;
	// println!("{} {}", round(x, 4), round(y, 4));
	
	// let a = get_state(3);
	// let b = get_state(2).phase_flip();
	let a = get_state(0);
	let b = get_state(1).phase_flip();
	let c = get_state(2);
	
	// let s = a.sup(b);
	let s = a.sup(get_state(1).phase_flip()).sup(get_state(2)).extract(vec![b, c]);
	
	let mut i = 0;
	let mag = s.iter().fold(real!(0), move |a, b| a + (b * b));
	println!("State: [{}] ({})", s.len(), mag);
	for x in s {
		let pow = if num::traits::Zero::is_zero(&x) {real!(0)} else {x * x};
		println!("{}  {}%\t{} ", i, (pow * real!(100) / mag).re.round() as usize, round(x, 4));
		i += 1;
	}
}