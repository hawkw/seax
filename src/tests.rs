use super::{scheme,svm};

use test::Bencher;

macro_rules! impl_bench {
    ($name:ident, $it:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                svm::eval_program(
                    scheme::compile($it)
                    .unwrap(), true)
                    .unwrap()
            })
        }

    }
}

impl_bench!( bench_list_creation,
    "(cons 10 (cons 20 nil))"
);

impl_bench!( bench_list_car,
    "(car (cons 20 (cons 10 nil)))"
);

impl_bench!( bench_list_cdr,
    "(cdr (cons 20 (cons 10 nil)))"
);

impl_bench!( bench_simple_add,
    "(+ 10 10)"
);

impl_bench!( bench_nested_arith,
    "(- 20 (+ 5 5))"
);

impl_bench!( bench_basic_branching_1,
    "(if (= 0 (- 1 1)) #t #f)"
);

impl_bench!( bench_basic_branching_2,
    "(+ 10 (if (nil? nil) 10 20))"
);

impl_bench!( bench_lambda_ap,
    "((lambda (x y) (+ x y)) 2 3)"
);

impl_bench!( bench_nested_lambda,
    "((lambda (z) ((lambda (x y) (+ (- x y) z)) 3 5)) 6)"
);
