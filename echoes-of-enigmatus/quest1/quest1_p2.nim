# Wyatt Geckle
#
# Everybody Codes Echoes of Enigmatus Quest 1 Part 2

from os import commandLineParams, paramCount
from std/bitops import testBit
from std/sequtils import mapIt
from std/streams import close, newFileStream, readLine
from std/strformat import fmt
from std/strscans import scanf
from std/strutils import join, parseUInt

const numRemainders = 5

proc modularPow(base: uint, exponent: uint, modulus: uint): uint =
  result = 1
  var mutableBase = base mod modulus
  var mutableExponent = exponent
  while mutableExponent > 0:
    if mutableExponent.testBit(0):
      result = (result * mutableBase) mod modulus
    mutableExponent = mutableExponent shr 1
    mutableBase = (mutableBase * mutableBase) mod modulus

proc eni(base: uint, exponent: uint, modulus: uint): uint =
  var remaindersList: seq[uint]
  if exponent <= numRemainders:
    var score = 1u
    remaindersList = newSeq[uint](exponent)
    for i in 0..exponent-1:
      score = (score * base) mod modulus
      remaindersList[exponent - 1 - i] = score
  else:
    var score = modularPow(base, exponent - numRemainders, modulus)
    remaindersList = newSeq[uint](numRemainders)
    for i in 0..numRemainders-1:
      score = (score * base) mod modulus
      remaindersList[numRemainders - 1 - i] = score
  parseUInt(remaindersList.mapIt($it).join)

proc main() = 
  let argc = paramCount()
  let argv = commandLineParams()

  if argc < 1:
    writeLine(stderr, "Please provide the notes file.");
    quit(1)

  var strm = newFileStream(argv[0], fmRead)
  if isNil(strm):
    writeLine(stderr, fmt"An error occurred while opening {argv[0]}.")
    quit(1)

  var line: string
  var eniMaxResult = 0u

  while strm.readLine(line):
    const lineFormat = "A=$i B=$i C=$i X=$i Y=$i Z=$i M=$i"
    var a, b, c, x, y, z, m: int
    let validLine = scanf(line, lineFormat, a, b, c, x, y, z, m)
    if not validLine or a < 0 or b < 0 or c < 0 or
        x < 0 or y < 0 or z < 0 or m <= 0:
      writeLine(stderr, "The provided notes file is invalid.");
      quit(1)
    let eni1 = eni(uint(a), uint(x), uint(m))
    let eni2 = eni(uint(b), uint(y), uint(m))
    let eni3 = eni(uint(c), uint(z), uint(m))
    eniMaxResult = max(eniMaxResult, eni1 + eni2 + eni3)

  strm.close()

  echo eniMaxResult

main()
