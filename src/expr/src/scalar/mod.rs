// Copyright Materialize, Inc. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

use std::collections::HashSet;
use std::mem;

use serde::{Deserialize, Serialize};

use repr::adt::datetime::DateTimeUnits;
use repr::adt::regex::Regex;
use repr::strconv::ParseError;
use repr::{ColumnType, Datum, RelationType, Row, RowArena, ScalarType};

pub use self::func::ScalarFunc;

pub mod func;
pub mod like_pattern;

#[derive(Ord, PartialOrd, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum ScalarExpr {
    /// A column of the input row.
    Column(usize),
    /// A literal value.
    ///
    /// A value is either an owned datum or an error. Owned datums are
    /// represented as a [`Row`] with a single element, since we don't presently
    /// have an alternate type that owns its data.
    Literal(Result<Row, EvalError>, ColumnType),
    Call {
        func: ScalarFunc,
        exprs: Vec<ScalarExpr>,
    },
}

impl ScalarExpr {
    pub fn columns(is: &[usize]) -> Vec<ScalarExpr> {
        is.iter().map(|i| ScalarExpr::Column(*i)).collect()
    }

    pub fn column(column: usize) -> Self {
        ScalarExpr::Column(column)
    }

    pub fn literal(res: Result<Datum, EvalError>, typ: ColumnType) -> Self {
        let row = res.map(|datum| Row::pack(&[datum]));
        ScalarExpr::Literal(row, typ)
    }

    pub fn literal_ok(datum: Datum, typ: ColumnType) -> Self {
        ScalarExpr::literal(Ok(datum), typ)
    }

    pub fn literal_null(typ: ColumnType) -> Self {
        ScalarExpr::literal_ok(Datum::Null, typ)
    }

    pub fn visit1<'a, F>(&'a self, mut f: F)
    where
        F: FnMut(&'a Self),
    {
        match self {
            ScalarExpr::Column(_) => (),
            ScalarExpr::Literal(_, _) => (),
            ScalarExpr::Call { exprs, .. } => {
                for expr in exprs {
                    f(expr);
                }
            }
        }
    }

    pub fn visit<'a, F>(&'a self, f: &mut F)
    where
        F: FnMut(&'a Self),
    {
        self.visit1(|e| e.visit(f));
        f(self);
    }

    pub fn visit1_mut<'a, F>(&'a mut self, mut f: F)
    where
        F: FnMut(&'a mut Self),
    {
        match self {
            ScalarExpr::Column(_) => (),
            ScalarExpr::Literal(_, _) => (),
            ScalarExpr::Call { exprs, .. } => {
                for expr in exprs {
                    f(expr);
                }
            }
        }
    }

    pub fn visit_mut<F>(&mut self, f: &mut F)
    where
        F: FnMut(&mut Self),
    {
        self.visit1_mut(|e| e.visit_mut(f));
        f(self);
    }

    /// Rewrites column indices with their value in `permutation`.
    ///
    /// This method is applicable even when `permutation` is not a
    /// strict permutation, and it only needs to have entries for
    /// each column referenced in `self`.
    pub fn permute(&mut self, permutation: &[usize]) {
        self.visit_mut(&mut |e| {
            if let ScalarExpr::Column(old_i) = e {
                *old_i = permutation[*old_i];
            }
        });
    }

    pub fn support(&self) -> HashSet<usize> {
        let mut support = HashSet::new();
        self.visit(&mut |e| {
            if let ScalarExpr::Column(i) = e {
                support.insert(*i);
            }
        });
        support
    }

    pub fn take(&mut self) -> Self {
        mem::replace(
            self,
            ScalarExpr::literal_null(ColumnType::new(ScalarType::String)),
        )
    }

    pub fn as_literal(&self) -> Option<Result<Datum, &EvalError>> {
        if let ScalarExpr::Literal(lit, _column_type) = self {
            Some(lit.as_ref().map(|row| row.unpack_first()))
        } else {
            None
        }
    }

    pub fn as_literal_str(&self) -> Option<&str> {
        match self.as_literal() {
            Some(Ok(Datum::String(s))) => Some(s),
            _ => None,
        }
    }

    pub fn as_literal_err(&self) -> Option<&EvalError> {
        self.as_literal().and_then(|lit| lit.err())
    }

    pub fn is_literal(&self) -> bool {
        if let ScalarExpr::Literal(_, _) = self {
            true
        } else {
            false
        }
    }

    pub fn is_literal_true(&self) -> bool {
        Some(Ok(Datum::True)) == self.as_literal()
    }

    pub fn is_literal_false(&self) -> bool {
        Some(Ok(Datum::False)) == self.as_literal()
    }

    pub fn is_literal_null(&self) -> bool {
        Some(Ok(Datum::Null)) == self.as_literal()
    }

    pub fn is_literal_ok(&self) -> bool {
        match self {
            ScalarExpr::Literal(Ok(_), _typ) => true,
            _ => false,
        }
    }

    pub fn is_literal_err(&self) -> bool {
        match self {
            ScalarExpr::Literal(Err(_), _typ) => true,
            _ => false,
        }
    }

    // /// Reduces a complex expression where possible.
    // ///
    // /// ```rust
    // /// use expr::{BinaryFunc, ScalarExpr};
    // /// use repr::{ColumnType, Datum, RelationType, ScalarType};
    // ///
    // /// let expr_0 = ScalarExpr::Column(0);
    // /// let expr_t = ScalarExpr::literal_ok(Datum::True, ColumnType::new(ScalarType::Bool));
    // /// let expr_f = ScalarExpr::literal_ok(Datum::False, ColumnType::new(ScalarType::Bool));
    // ///
    // /// let mut test =
    // /// expr_t
    // ///     .clone()
    // ///     .call_binary(expr_f.clone(), BinaryFunc::And)
    // ///     .if_then_else(expr_0, expr_t.clone());
    // ///
    // /// let input_type = RelationType::new(vec![ColumnType::new(ScalarType::Int32)]);
    // /// test.reduce(&input_type);
    // /// assert_eq!(test, expr_t);
    // /// ```
    // pub fn reduce(&mut self, relation_type: &RelationType) {
    //     let temp_storage = &RowArena::new();
    //     let eval =
    //         |e: &ScalarExpr| ScalarExpr::literal(e.eval(&[], temp_storage), e.typ(&relation_type));
    //     self.visit_mut(&mut |e| match e {
    //         ScalarExpr::Column(_) | ScalarExpr::Literal(_, _) | ScalarExpr::CallNullary(_) => (),
    //         ScalarExpr::CallUnary { expr, .. } => {
    //             if expr.is_literal() {
    //                 *e = eval(e);
    //             }
    //         }
    //         ScalarExpr::CallBinary { func, expr1, expr2 } => {
    //             if expr1.is_literal() && expr2.is_literal() {
    //                 *e = eval(e);
    //             } else if (expr1.is_literal_null() || expr2.is_literal_null())
    //                 && func.props().propagates_nulls()
    //             {
    //                 *e = ScalarExpr::literal_null(e.typ(relation_type));
    //             } else if let Some(err) = expr1.as_literal_err() {
    //                 *e = ScalarExpr::literal(Err(err.clone()), e.typ(&relation_type));
    //             } else if let Some(err) = expr2.as_literal_err() {
    //                 *e = ScalarExpr::literal(Err(err.clone()), e.typ(&relation_type));
    //             } else if *func == BinaryFunc::MatchLikePattern && expr2.is_literal() {
    //                 // We can at least precompile the regex.
    //                 let pattern = expr2.as_literal_str().unwrap();
    //                 *e = match like_pattern::build_regex(&pattern) {
    //                     Ok(regex) => expr1.take().call_unary(UnaryFunc::MatchRegex(Regex(regex))),
    //                     Err(_) => ScalarExpr::literal_null(e.typ(&relation_type)),
    //                 };
    //             } else if *func == BinaryFunc::DatePartInterval && expr1.is_literal() {
    //                 let units = expr1.as_literal_str().unwrap();
    //                 *e = match units.parse::<DateTimeUnits>() {
    //                     Ok(units) => ScalarExpr::CallUnary {
    //                         func: UnaryFunc::DatePartInterval(units),
    //                         expr: Box::new(expr2.take()),
    //                     },
    //                     Err(_) => ScalarExpr::literal_null(e.typ(&relation_type)),
    //                 }
    //             } else if *func == BinaryFunc::DatePartTimestamp && expr1.is_literal() {
    //                 let units = expr1.as_literal_str().unwrap();
    //                 *e = match units.parse::<DateTimeUnits>() {
    //                     Ok(units) => ScalarExpr::CallUnary {
    //                         func: UnaryFunc::DatePartTimestamp(units),
    //                         expr: Box::new(expr2.take()),
    //                     },
    //                     Err(_) => ScalarExpr::literal_null(e.typ(&relation_type)),
    //                 }
    //             } else if *func == BinaryFunc::DatePartTimestampTz && expr1.is_literal() {
    //                 let units = expr1.as_literal_str().unwrap();
    //                 *e = match units.parse::<DateTimeUnits>() {
    //                     Ok(units) => ScalarExpr::CallUnary {
    //                         func: UnaryFunc::DatePartTimestampTz(units),
    //                         expr: Box::new(expr2.take()),
    //                     },
    //                     Err(_) => ScalarExpr::literal_null(e.typ(&relation_type)),
    //                 }
    //             } else if *func == BinaryFunc::DateTruncTimestamp && expr1.is_literal() {
    //                 let units = expr1.as_literal_str().unwrap();
    //                 *e = match units.parse::<DateTimeUnits>() {
    //                     Ok(units) => ScalarExpr::CallUnary {
    //                         func: UnaryFunc::DateTruncTimestamp(units),
    //                         expr: Box::new(expr2.take()),
    //                     },
    //                     Err(_) => ScalarExpr::literal_null(e.typ(&relation_type)),
    //                 }
    //             } else if *func == BinaryFunc::DateTruncTimestampTz && expr1.is_literal() {
    //                 let units = expr1.as_literal_str().unwrap();
    //                 *e = match units.parse::<DateTimeUnits>() {
    //                     Ok(units) => ScalarExpr::CallUnary {
    //                         func: UnaryFunc::DateTruncTimestampTz(units),
    //                         expr: Box::new(expr2.take()),
    //                     },
    //                     Err(_) => ScalarExpr::literal_null(e.typ(&relation_type)),
    //                 }
    //             } else if *func == BinaryFunc::And && (expr1.is_literal() || expr2.is_literal()) {
    //                 // If we are here, not both inputs are literals.
    //                 if expr1.is_literal_false() || expr2.is_literal_true() {
    //                     *e = expr1.take();
    //                 } else if expr2.is_literal_false() || expr1.is_literal_true() {
    //                     *e = expr2.take();
    //                 }
    //             } else if *func == BinaryFunc::Or && (expr1.is_literal() || expr2.is_literal()) {
    //                 // If we are here, not both inputs are literals.
    //                 if expr1.is_literal_true() || expr2.is_literal_false() {
    //                     *e = expr1.take();
    //                 } else if expr2.is_literal_true() || expr1.is_literal_false() {
    //                     *e = expr2.take();
    //                 }
    //             }
    //         }
    //         ScalarExpr::CallVariadic { func, exprs } => {
    //             if *func == VariadicFunc::Coalesce {
    //                 // If all inputs are null, output is null. This check must
    //                 // be done before `exprs.retain...` because `e.typ` requires
    //                 // > 0 `exprs` remain.
    //                 if exprs.iter().all(|expr| expr.is_literal_null()) {
    //                     *e = ScalarExpr::literal_null(e.typ(&relation_type));
    //                     return;
    //                 }

    //                 // Remove any null values if not all values are null.
    //                 exprs.retain(|e| !e.is_literal_null());

    //                 // Find the first argument that is a literal or non-nullable
    //                 // column. All arguments after it get ignored, so throw them
    //                 // away. This intentionally throws away errors that can
    //                 // never happen.
    //                 if let Some(i) = exprs
    //                     .iter()
    //                     .position(|e| e.is_literal() || !e.typ(&relation_type).nullable)
    //                 {
    //                     exprs.truncate(i + 1);
    //                 }

    //                 if let Some(expr) = exprs.iter_mut().find(|e| e.is_literal_err()) {
    //                     // One of the remaining arguments is an error, so
    //                     // just replace the entire coalesce with that error.
    //                     *e = expr.take();
    //                 } else if exprs.len() == 1 {
    //                     // Only one argument, so the coalesce is a no-op.
    //                     *e = exprs[0].take();
    //                 }
    //             } else if exprs.iter().all(|e| e.is_literal()) {
    //                 *e = eval(e);
    //             } else if func.props().propagates_nulls()
    //                 && exprs.iter().any(|e| e.is_literal_null())
    //             {
    //                 *e = ScalarExpr::literal_null(e.typ(&relation_type));
    //             } else if let Some(err) = exprs.iter().find_map(|e| e.as_literal_err()) {
    //                 *e = ScalarExpr::literal(Err(err.clone()), e.typ(&relation_type));
    //             }
    //         }
    //         ScalarExpr::If { cond, then, els } => {
    //             if let Some(literal) = cond.as_literal() {
    //                 match literal {
    //                     Ok(Datum::True) => *e = then.take(),
    //                     Ok(Datum::False) | Ok(Datum::Null) => *e = els.take(),
    //                     Err(_) => *e = cond.take(),
    //                     _ => unreachable!(),
    //                 }
    //             } else if then == els {
    //                 *e = then.take();
    //             } else if then.is_literal_ok() && els.is_literal_ok() {
    //                 match (then.as_literal(), els.as_literal()) {
    //                     (Some(Ok(Datum::True)), _) => {
    //                         *e = cond.take().call_binary(els.take(), BinaryFunc::Or);
    //                     }
    //                     (Some(Ok(Datum::False)), _) => {
    //                         *e = cond
    //                             .take()
    //                             .call_unary(UnaryFunc::Not)
    //                             .call_binary(els.take(), BinaryFunc::And);
    //                     }
    //                     (_, Some(Ok(Datum::True))) => {
    //                         *e = cond
    //                             .take()
    //                             .call_unary(UnaryFunc::Not)
    //                             .call_binary(then.take(), BinaryFunc::Or);
    //                     }
    //                     (_, Some(Ok(Datum::False))) => {
    //                         *e = cond.take().call_binary(then.take(), BinaryFunc::And);
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }
    //     });
    // }

    /// Adds any columns that *must* be non-Null for `self` to be non-Null.
    pub fn non_null_requirements(&self, columns: &mut HashSet<usize>) {
        match self {
            ScalarExpr::Column(col) => {
                columns.insert(*col);
            }
            ScalarExpr::Literal(..) => (),
            ScalarExpr::Call { func, exprs } => {
                if func.propagates_nulls() {
                    for expr in exprs {
                        expr.non_null_requirements(columns);
                    }
                }
            }
        }
    }

    pub fn typ(&self, relation_type: &RelationType) -> ColumnType {
        match self {
            ScalarExpr::Column(i) => relation_type.column_types[*i].clone(),
            ScalarExpr::Literal(_, typ) => typ.clone(),
            ScalarExpr::Call { func, exprs } => {
                let input_types: Vec<_> = exprs.iter().map(|e| e.typ(relation_type)).collect();
                func.output_type(&input_types)
            }
        }
    }

    pub fn eval<'a>(
        &'a self,
        datums: &[Datum<'a>],
        temp_storage: &'a RowArena,
    ) -> Result<Datum<'a>, EvalError> {
        match self {
            ScalarExpr::Column(index) => Ok(datums[*index].clone()),
            ScalarExpr::Literal(res, _column_type) => match res {
                Ok(row) => Ok(row.unpack_first()),
                Err(e) => Err(e.clone()),
            },
            ScalarExpr::Call { func, exprs } => func.eval(exprs, datums, temp_storage),
        }
    }
}

#[derive(Ord, PartialOrd, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum EvalError {
    DivisionByZero,
    NumericFieldOverflow,
    IntegerOutOfRange,
    IntervalOutOfRange,
    InvalidEncodingName(String),
    InvalidByteSequence {
        byte_sequence: String,
        encoding_name: String,
    },
    NegSqrt,
    UnknownUnits(String),
    UnsupportedDateTimeUnits(DateTimeUnits),
    UnterminatedLikeEscapeSequence,
    Parse(ParseError),
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            EvalError::DivisionByZero => f.write_str("division by zero"),
            EvalError::NumericFieldOverflow => f.write_str("numeric field overflow"),
            EvalError::IntegerOutOfRange => f.write_str("integer out of range"),
            EvalError::IntervalOutOfRange => f.write_str("interval out of range"),
            EvalError::InvalidEncodingName(name) => write!(f, "invalid encoding name '{}'", name),
            EvalError::InvalidByteSequence {
                byte_sequence,
                encoding_name,
            } => write!(
                f,
                "invalid byte sequence '{}' for encoding '{}'",
                byte_sequence, encoding_name
            ),
            EvalError::NegSqrt => f.write_str("cannot take square root of a negative number"),
            EvalError::UnknownUnits(units) => write!(f, "unknown units '{}'", units),
            EvalError::UnsupportedDateTimeUnits(units) => {
                write!(f, "unsupported timestamp units '{}'", units)
            }
            EvalError::UnterminatedLikeEscapeSequence => {
                f.write_str("unterminated escape sequence in LIKE")
            }
            EvalError::Parse(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for EvalError {}

impl From<ParseError> for EvalError {
    fn from(e: ParseError) -> EvalError {
        EvalError::Parse(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce() {
        let relation_type = RelationType::new(vec![
            ColumnType::new(ScalarType::Int64).nullable(true),
            ColumnType::new(ScalarType::Int64).nullable(true),
            ColumnType::new(ScalarType::Int64).nullable(false),
        ]);
        let col = |i| ScalarExpr::Column(i);
        let err = |e| ScalarExpr::literal(Err(e), ColumnType::new(ScalarType::Int64));
        let lit = |i| ScalarExpr::literal_ok(Datum::Int64(i), ColumnType::new(ScalarType::Int64));
        let null = || ScalarExpr::literal_null(ColumnType::new(ScalarType::Int64));

        struct TestCase {
            input: ScalarExpr,
            output: ScalarExpr,
        }

        let test_cases = vec![
            TestCase {
                input: ScalarExpr::CallVariadic {
                    func: VariadicFunc::Coalesce,
                    exprs: vec![lit(1)],
                },
                output: lit(1),
            },
            TestCase {
                input: ScalarExpr::CallVariadic {
                    func: VariadicFunc::Coalesce,
                    exprs: vec![lit(1), lit(2)],
                },
                output: lit(1),
            },
            TestCase {
                input: ScalarExpr::CallVariadic {
                    func: VariadicFunc::Coalesce,
                    exprs: vec![null(), lit(2), null()],
                },
                output: lit(2),
            },
            TestCase {
                input: ScalarExpr::CallVariadic {
                    func: VariadicFunc::Coalesce,
                    exprs: vec![null(), col(0), null(), col(1), lit(2), lit(3)],
                },
                output: ScalarExpr::CallVariadic {
                    func: VariadicFunc::Coalesce,
                    exprs: vec![col(0), col(1), lit(2)],
                },
            },
            TestCase {
                input: ScalarExpr::CallVariadic {
                    func: VariadicFunc::Coalesce,
                    exprs: vec![col(0), col(2), col(1)],
                },
                output: ScalarExpr::CallVariadic {
                    func: VariadicFunc::Coalesce,
                    exprs: vec![col(0), col(2)],
                },
            },
            TestCase {
                input: ScalarExpr::CallVariadic {
                    func: VariadicFunc::Coalesce,
                    exprs: vec![lit(1), err(EvalError::DivisionByZero)],
                },
                output: lit(1),
            },
            TestCase {
                input: ScalarExpr::CallVariadic {
                    func: VariadicFunc::Coalesce,
                    exprs: vec![col(0), err(EvalError::DivisionByZero)],
                },
                output: err(EvalError::DivisionByZero),
            },
            TestCase {
                input: ScalarExpr::CallVariadic {
                    func: VariadicFunc::Coalesce,
                    exprs: vec![
                        null(),
                        err(EvalError::DivisionByZero),
                        err(EvalError::NumericFieldOverflow),
                    ],
                },
                output: err(EvalError::DivisionByZero),
            },
            TestCase {
                input: ScalarExpr::CallVariadic {
                    func: VariadicFunc::Coalesce,
                    exprs: vec![col(0), err(EvalError::DivisionByZero)],
                },
                output: err(EvalError::DivisionByZero),
            },
        ];

        for tc in test_cases {
            let mut actual = tc.input.clone();
            actual.reduce(&relation_type);
            assert!(
                actual == tc.output,
                "input: {}\nactual: {}\nexpected: {}",
                tc.input,
                actual,
                tc.output
            );
        }
    }
}
