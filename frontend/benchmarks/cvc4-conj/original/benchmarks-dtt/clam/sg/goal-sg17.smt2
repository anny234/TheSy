
(declare-datatypes () ((Lst (cons (head Int) (tail Lst)) (nil))))
(declare-datatypes () ((Tree (node (data Int) (left Tree) (right Tree)) (leaf))))
(declare-datatypes () ((Pair (mkpair (first Int) (second Int)))
                       (ZLst (zcons (zhead Pair) (ztail ZLst)) (znil))))
(declare-fun less (Int Int) Bool)
(declare-fun plus (Int Int) Int)
(declare-fun mult (Int Int) Int)
(declare-fun qmult (Int Int Int) Int)
(declare-fun exp (Int Int) Int)
(declare-fun qexp (Int Int Int) Int)
(declare-fun fac (Int) Int)
(declare-fun qfac (Int Int) Int)
(declare-fun double (Int) Int)
(declare-fun half (Int) Int)
(declare-fun even (Int) Bool)
(declare-fun append (Lst Lst) Lst)
(declare-fun len (Lst) Int)
(declare-fun drop (Int Lst) Lst)
(declare-fun take (Int Lst) Lst)
(declare-fun count (Int Lst) Int)
(declare-fun mem (Int Lst) Bool)
(declare-fun rev (Lst) Lst)
(declare-fun qreva (Lst Lst) Lst)
(declare-fun insort (Int Lst) Lst)
(declare-fun sorted (Lst) Bool)
(declare-fun sort (Lst) Lst)
(declare-fun rotate (Int Lst) Lst)
(declare-fun revflat (Tree) Lst)
(declare-fun qrevaflat (Tree Lst) Lst)
(declare-fun lst-mem (Int (Set Int)) Bool)
(declare-fun lst-subset ((Set Int) (Set Int)) Bool)
(declare-fun lst-eq ((Set Int) (Set Int)) Bool)
(declare-fun lst-intersection ((Set Int) (Set Int)) (Set Int))
(declare-fun lst-union ((Set Int) (Set Int)) (Set Int))
(define-fun leq ((x Int) (y Int)) Bool (or (= x y) (less x y)))
(assert (forall ((x Lst)) (= (append nil x) x)))
(assert (forall ((x Int) (y Lst) (z Lst)) (= (append (cons x y) z) (cons x (append y z)))))
(assert (= (rev nil) nil))
(assert (forall ((x Int) (y Lst)) (= (rev (cons x y)) (append (rev y) (cons x nil)))))

; sub-goals
(assert 
(forall ((x Lst) (y Int)) (= (rev (append x (cons y nil))) (cons y (rev x)))) ; G58 
)

; conjecture
(assert (not 
(forall ((x Lst) (y Lst)) (= (rev (rev (append x y))) (append (rev (rev x)) (rev (rev y)))))  ; G17 
))
(check-sat)
