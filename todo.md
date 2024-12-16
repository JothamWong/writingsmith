# Goal

Pass in Latex/pdf/word file.
Convert into text. Maintain line info?
The idea is that one should be able to choose "modules" which analyze the text file for errors in each pass.

## TODOs

1. Pass in file []
   1. txt [x]
   2. latex
      1. might need to do stuff to extract raw text, basically ignore commands
   3. pdf
      1. to look into this. might be hard to obtain sentence meta data
   4. word
2. Text Level analysis []
   1. Weasel words [x]
   2. Passive voice
   3. To learn more about the different aspects of good technical writing and add modules as necc
3. Print output to file or terminal 
   1. Rudimentary report generation [x]
   2. Should be able to specify report format
      1. group by message type or sentence would be nice
   3. To look into how better CLI apps pipe either to file or to terminal properly
4. Generate some bad input for educational purposes.
   1. Currently rely on chatgpt to generate sample inputs.
   2. A library using LLMs to generate sample inputs for verification would be nice.
5. Reimplement this in Verus and get good at formal verification methods.

## Possible Improvements

1. The text-word level splitting potentially has 100% overlap with all modules. Could look into making that some common function instead if necessary.
2. Weasel words are currently hardcoded and may not include all possible weasel words. An exhaustive source would be nice.

## References

Inspired by https://matt.might.net/articles/shell-scripts-for-passive-voice-weasel-words-duplicates/ and <https://github.com/bnbeckwith/writegood-mode>.
