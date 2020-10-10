(declare-datatypes () ((Lst (cons (head Int) (tail Lst)) (nil))))
(declare-datatypes () ((Tree (node (data Int) (left Tree) (right Tree)) (leaf))))
(declare-datatypes () ((Pair (mkpair (first Int) (second Int)))
                       (ZLst (zcons (zhead Pair) (ztail ZLst)) (znil))))
(declare-fun P (Int) Bool)
(declare-fun f (Int) Int)
(declare-fun less (Int Int) Bool)
(declare-fun plus (Int Int) Int)
(declare-fun minus (Int Int) Int)
(declare-fun mult (Int Int) Int)
(declare-fun nmax (Int Int) Int)
(declare-fun nmin (Int Int) Int)
(declare-fun append (Lst Lst) Lst)
(declare-fun len (Lst) Int)
(declare-fun drop (Int Lst) Lst)
(declare-fun take (Int Lst) Lst)
(declare-fun count (Int Lst) Int)
(declare-fun last (Lst) Int)
(declare-fun butlast (Lst) Lst)
(declare-fun mem (Int Lst) Bool)
(declare-fun delete (Int Lst) Lst)
(declare-fun rev (Lst) Lst)
(declare-fun lmap (Lst) Lst)
(declare-fun filter (Lst) Lst)
(declare-fun dropWhile (Lst) Lst)
(declare-fun takeWhile (Lst) Lst)
(declare-fun ins1 (Int Lst) Lst)
(declare-fun insort (Int Lst) Lst)
(declare-fun sorted (Lst) Bool)
(declare-fun sort (Lst) Lst)
(declare-fun zip (Lst Lst) ZLst)
(declare-fun zappend (ZLst ZLst) ZLst)
(declare-fun zdrop (Int ZLst) ZLst)
(declare-fun ztake (Int ZLst) ZLst)
(declare-fun zrev (ZLst) ZLst)
(declare-fun mirror (Tree) Tree)
(declare-fun height (Tree) Int)
(define-fun leq ((x Int) (y Int)) Bool (or (= x y) (less x y)))
(assert (forall ((n Int)) (=> (>= n 0) (= (minus 0 n) 0))))
(assert (forall ((n Int)) (=> (>= n 0) (= (minus n 0) n))))
(assert (forall ((n Int) (m Int)) (=> (and (>= n 0) (>= m 0)) (= (minus (+ 1 n) (+ 1 m)) (minus n m)))))
(assert (forall ((n Int) (m Int)) (=> (and (>= n 0) (>= m 0)) (= (minus n m) (ite (< n m) 0 (- n m))))))
(assert (forall ((x Lst)) (= (append nil x) x)))
(assert (forall ((x Int) (y Lst) (z Lst)) (= (append (cons x y) z) (cons x (append y z)))))
(assert (= (len nil) 0))
(assert (forall ((x Int) (y Lst)) (= (len (cons x y)) (+ 1 (len y)))))
(assert (forall ((x Lst)) (>= (len x) 0)))
(assert (forall ((x Int)) (= (take x nil) nil)))
(assert (forall ((x Lst)) (= (take 0 x) nil)))
(assert (forall ((x Int) (y Int) (z Lst)) (=> (>= x 0) (= (take (+ x 1) (cons y z)) (cons y (take x z))))))
(assert (not 
(forall ((n Int) (xs Lst) (ys Lst)) (=> (>= n 0) (= (take n (append xs ys)) (append (take n xs) (take (minus n (len xs)) ys))))) ; G82 
))
(check-sat)
