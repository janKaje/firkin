// big ol number things hahahha

mod sign;
mod smallrational;
mod int;
// mod number;

// use number::Number;
// use rug::{Integer, Rational, Float};

// pub(crate) enum Expression {
//     Num(Number),
//     Func(Function)
// }

// pub(crate) enum Function {
//     Add(&Value,&Value),
//     Sub(&Value,&Value),
//     Mul(&Value,&Value),
//     Div(&Value,&Value),
//     Pow(&Value,&Value), // 2nd is exponent, remember to search internally for e
//     Log(&Value,&Value), // 2nd is base
//     Abs(&Value),
//     Floor(&Value),
//     Ceil(&Value),
//     Round(&Value),
//     Fac(&Value), // factorial
// }

// pub(crate) struct Value {
//     value: Expression,
//     cached_eval: Option<Number>,
// }

// // implement each Function
// impl Value {
//     pub(crate) fn add(&self, other: &Value) -> Value {
//         Value { value: Expression::Func(Function::Add(&self, &other)), cached_eval: None }
//     }
//     pub(crate) fn sub(&self, other: &Value) -> Value {
//         Value { value: Expression::Func(Function::Sub(&self, &other)), cached_eval: None }
//     }
//     pub(crate) fn mul(&self, other: &Value) -> Value {
//         Value { value: Expression::Func(Function::Mul(&self, &other)), cached_eval: None }
//     }
//     pub(crate) fn div(&self, other: &Value) -> Value {
//         Value { value: Expression::Func(Function::Div(&self, &other)), cached_eval: None }
//     }
//     pub(crate) fn pow(&self, other: &Value) -> Value {
//         Value { value: Expression::Func(Function::Pow(&self, &other)), cached_eval: None }
//     }
//     pub(crate) fn log(&self, other: &Value) -> Value {
//         Value { value: Expression::Func(Function::Log(&self, &other)), cached_eval: None }
//     }

//     pub(crate) fn sin(&self) -> Value {
//         Value { value: Expression::Func(Function::Sin(&self)), cached_eval: None }
//     }
//     pub(crate) fn cos(&self) -> Value {
//         Value { value: Expression::Func(Function::Cos(&self)), cached_eval: None }
//     }
//     pub(crate) fn asin(&self) -> Value {
//         Value { value: Expression::Func(Function::Asin(&self)), cached_eval: None }
//     }
//     pub(crate) fn acos(&self) -> Value {
//         Value { value: Expression::Func(Function::Acos(&self)), cached_eval: None }
//     }
//     pub(crate) fn abs(&self) -> Value {
//         Value { value: Expression::Func(Function::Abs(&self)), cached_eval: None }
//     }
//     pub(crate) fn floor(&self) -> Value {
//         Value { value: Expression::Func(Function::Floor(&self)), cached_eval: None }
//     }
//     pub(crate) fn ceil(&self) -> Value {
//         Value { value: Expression::Func(Function::Ceil(&self)), cached_eval: None }
//     }
//     pub(crate) fn round(&self) -> Value {
//         Value { value: Expression::Func(Function::Round(&self)), cached_eval: None }
//     }
//     pub(crate) fn fac(&self) -> Value {
//         Value { value: Expression::Func(Function::Fac(&self)), cached_eval: None }
//     }

//     pub(crate) fn eval_to_at_least_precision(&self, prec:u32) -> Number {
//         if let Some(n) = self.cached_eval {
//             match n {
//                 Number::Float(f) => {
//                     if f.prec() >= prec {
//                         return n
//                     }
//                 },
//                 _ => return n
//             }
//         }

//         match self.value {
//             Expression::Num(n) => n,
//             Expression::Func(f) => f.eval_to_at_least_precision(prec)
//         }
//     }
// }

// impl Function {
//     pub(crate) fn eval_to_at_least_precision(&self, prec:u32) -> Number {
//         let prec_plus_1 = match prec {
//             u32::MAX => u32::MAX,
//             _ => prec + 1,
//         };
//         match self {
//             Function::Add(left, right) => {
//                 left.eval_to_at_least_precision(prec_plus_1) + right.eval_to_at_least_precision(prec_plus_1)
//             },
//             Function::Sub(left, right) => todo!(),
//             Function::Mul(left, right) => todo!(),
//             Function::Div(left, right) => todo!(),
//             Function::Pow(base, exp) => todo!(),
//             Function::Log(value, base) => todo!(),
//             Function::Floor(value) => todo!(),
//             Function::Ceil(value) => todo!(),
//             Function::Round(value) => todo!(),
//             Function::Fac(value) => todo!(),
//             Function::Abs(value) => todo!(),
//         }
//     }
// }
