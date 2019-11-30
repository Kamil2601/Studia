(declare-const p Bool)
(declare-const q Bool)

(push)
(assert (and (or p q) (or (not p) (not q)) (or p (not q)) ))
(check-sat)
(get-model)
(pop)

(push)
(assert (and (or p q) (=> p q) (=> q p) (or (not p) (not q))))
(check-sat)
(get-model)
(pop)

(push)
(assert (forall ((p Bool) (q Bool)) (and (or p q) (or (not p) (not q)) (or p (not q)) )))
(check-sat)
(get-model)
(pop)

(push)
(assert (forall ((p Bool) (q Bool)) (and (or p q) (=> p q) (=> q p) (or (not p) (not q)))))
(check-sat)
(get-model)
(pop)