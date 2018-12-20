-- Basically a clone of the C version
-- <https://github.com/chrisdone/camel-case-bench/blob/ede2683ea10137cf32eb7c3d375ea1e2a1d7baf2/impls/c-inplace.c#L1>
-- It uses Haskell equivalents of the same kinds of operations.

{-# LANGUAGE BangPatterns #-}
{-# LANGUAGE MultiWayIf #-}
import qualified Data.Vector.Storable.Mutable as SVM
import           System.IO
import           Data.Word
import           System.IO (stdout)
import           GHC.Ptr
main :: IO ()
main = do
  buf <- SVM.new bufferSize
  let loop was_space0 = do
        size <- SVM.unsafeWith buf (\ptr -> hGetBuf stdin ptr bufferSize)
        if size == 0
          then pure ()
          else let reader (!dest, !src, !was_space)
                     | src < size = do
                       b <- SVM.read buf src
                       let ord = fromIntegral . fromEnum :: Char -> Word8
                       if | b == ord '\n' || (ord 'A' <= b && b <= ord 'Z') ->
                            do SVM.write buf dest b
                               reader (dest + 1, src + 1, False)
                          | ord 'a' <= b && b <= ord 'z' ->
                            do SVM.write
                                 buf
                                 dest
                                 (if was_space
                                    then b - 32
                                    else b)
                               reader (dest + 1, src + 1, False)
                          | otherwise -> reader (dest, src + 1, True)
                     | otherwise = pure (dest, was_space)
                   writer :: Ptr Word8 -> Int -> IO ()
                   writer (!start) (!dest)
                     | dest > 0 = do
                       written <- hPutBufNonBlocking stdout start dest
                       writer (plusPtr start written) (dest - written)
                     | otherwise = pure ()
                in do (dest0, was_space) <- reader (0, 0, was_space0)
                      SVM.unsafeWith buf (\ptr -> writer ptr dest0)
                      loop was_space
  loop False
  where
    bufferSize = 65536
