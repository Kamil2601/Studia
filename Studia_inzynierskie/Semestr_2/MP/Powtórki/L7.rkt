#lang racket
(define (f)
   (let ([ y 10])
   (lambda (x) (+ x y))))

(define y 0)
((f) 5)

