(define-fun is_max ((m Int) (x Int) (y Int)) Bool
    (and
        (>= m x)
        (>= m y)
        (or
            (= m x)
            (= m y)
        )
    )
)
(define-fun max ((x Int) (y Int)) Int
    (ite (> x y) x y)
)

(declare-const x Int)
(declare-const y Int)
(declare-const m Int)
(assert
    (and
        (= m (max x y))
        (not (is_max m x y))
    )
)
(check-sat)
(get-model)
