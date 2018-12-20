import Unsafe.Coerce
import Data.Word
import Data.Char (toUpper)

main = interact simple

simple = unlines . map simple' . lines

simple' = concat . map upperFirst . words . map toSpace

toSpace c
  | 'A' <= c && c <= 'Z' = c
  | 'a' <= c && c <= 'z' = c
  | otherwise = ' '

upperFirst (x:xs) = toUpper x : xs

    {-
(doit NotSpace)

data Saw
  = Space
  | NotSpace

doit _ [] = []
doit _ ('\n':rest) = '\n' : doit NotSpace rest
doit Space (c:rest)
  | isLower c = makeUpper c : doit NotSpace rest
  | isUpper c = c : doit NotSpace rest
  | otherwise = doit Space rest
doit NotSpace (c:rest)
  | isLower c || isUpper c = c : doit NotSpace rest
  | otherwise = doit Space rest

isLower c = 'a' <= c && c <= 'z'
isUpper c = 'A' <= c && c <= 'Z'
makeUpper c = unsafeCoerce (unsafeCoerce c + (32 :: Word32))
-}
