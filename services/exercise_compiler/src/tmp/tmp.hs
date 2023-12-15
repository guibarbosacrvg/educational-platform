
-- Type your code here
bubblesort :: Ord a => [a] -> [a]
bubblesort [] = []
bubblesort xs = bubblesort (init xs) ++ [last xs]

main = print $ bubblesort [10, 5, 2, 3]