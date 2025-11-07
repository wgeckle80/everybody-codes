# Wyatt Geckle
#
# Everybody Codes Echoes of Enigmatus Quest 1 Part 3

from os import commandLineParams, paramCount
from std/enumerate import enumerate
from std/sequtils import foldl
from std/streams import close, newFileStream, readLine
from std/strformat import fmt
from std/strscans import scanf

proc eni(base: uint, exponent: uint, modulus: uint): uint =
  let baseMod = base mod modulus
  var modularBase = (baseMod * baseMod) mod modulus
  var cycleValues = @[baseMod, modularBase]
  var cycleStartIndex = -1

  # Keep multiplying the base by itself mod modulus until a repeat is
  # encountered, or until the base becomes 0 mod modulus.
  while modularBase != 0:
    modularBase = (modularBase * baseMod) mod modulus
    for i, value in enumerate(cycleValues):
      if modularBase == value:
        cycleStartIndex = i
        break
    if cycleStartIndex != -1:
      break
    cycleValues.add(modularBase)

  let noCycle = modularBase == 0
  if noCycle:
    result = cycleValues.foldl(a + b)
  else:
    if cycleStartIndex > 0:
      result = cyclevalues[0..cycleStartIndex-1].foldl(a + b)

    let numCycleValues = uint(cycleValues.len - cycleStartIndex)
    let fullCycles = (exponent - uint(cycleStartIndex)) div numCycleValues
    result += fullCycles * cycleValues[cycleStartIndex..^1].foldl(a + b)

    let remaining = (int(exponent) - cycleStartIndex) mod int(numCycleValues)
    if remaining > 0:
      let endIndex = cycleStartIndex + remaining - 1
      result += cycleValues[cycleStartIndex..endIndex].foldl(a + b)

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
