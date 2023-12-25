module Main where

main = do 
    print $ fact 2000 

fact :: Integer -> Integer
fact 0 = 1
fact n = n * fact(n-1)