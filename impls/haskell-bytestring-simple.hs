{-# LANGUAGE OverloadedStrings #-}
import Data.Word
import Data.Monoid
import Data.Char (toUpper)
import qualified Data.ByteString.Lazy as L
import qualified Data.ByteString.Lazy.Char8 as L8
import Data.ByteString.Builder (Builder, lazyByteString, word8, hPutBuilder)
import System.IO (stdout)

main :: IO ()
main = L.getContents >>= hPutBuilder stdout . simple

simple :: L.ByteString -> Builder
simple = foldMap simple' . L8.lines

simple' :: L.ByteString -> Builder
simple' = foldMap upperFirst . L8.words . L.map toSpace

toSpace :: Word8 -> Word8
toSpace c
  | 65 <= c && c <= 90 = c
  | 97 <= c && c <= 122 = c
  | otherwise = 32

upperFirst :: L.ByteString -> Builder
upperFirst lbs =
  case L.uncons lbs of
    Nothing -> error "impossible!"
    Just (x, xs) ->
      word8 (if x >= 97 then x - 32 else x) <>
      lazyByteString xs <>
      "\n"
