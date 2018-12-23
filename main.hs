#!/usr/bin/env stack
-- stack --resolver lts-12.12 script --package criterion --package process

import Criterion
import Criterion.Main
import System.Process

main = do
  mapM_
    (\((compiler, makeArgs), name) -> rawSystem compiler (makeArgs name))
    impls
  defaultMain
    [ bench
      name
      (nfIO
         (rawSystem
            "sh"
            ["-c", "cat war-and-peace.txt | exes/" ++ name ++ " > /dev/null"]))
    | (_compiler, name) <- impls
    ]
  where
    impls =
      [ (gpp, "c-inplace")
      , (ghc, "haskell-chrisdone")
      , (rust, "rust-inplace")
      , (rust, "rust-iterator")
      , (ghc, "haskell-bytestring-simple")
      , (ghc, "haskell-string")
      ]
    gpp =
      ("g++", \name -> ["impls/" ++ name ++ ".c", "-O2", "-o", "exes/" ++ name])
    rust =
      ( "rustc"
      , \name -> ["impls/" ++ name ++ ".rs", "-O", "-o", "exes/" ++ name])
    ghc =
      ("ghc", \name -> ["impls/" ++ name ++ ".hs", "-O2", "-o", "exes/" ++ name])
