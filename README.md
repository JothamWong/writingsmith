# WordSmith

## About

WordSmith is a simple CLI tool written in Rust that aims to improve technical writing. WordSmith takes in a text file and generates a report which indicates potential areas for improvements, according to a set of modules. These modules include stuff like weasel words, passive voice and so on.

The intended use case is that you choose the wordsmith modules you want to check for, passed in via the command line argument, and each pass checks for all offending lines, before culminating into a final generated report which you can refer to for improving your technical writing.

## Using

The report is written to standard output, so you can easily pipe it to a text file for reference.

## Built-in modules

### Weasel Words

These are phrases or words that sound good but do not convey any information, commonly used to artificially inflate word count for the sake of essay submissions with mandatory minimum word counts.

They take on 3 forms: (1) salt and pepper words, (2) beholder words and (3) lazy words.

#### Salt and Pepper words

These words add no information because they are not quantitative. Examples includes `various`, `a number of`, `fairly` and `quite`.

```txt
Bad:    It is quite difficult to find untainted samples.
Better: It is difficult to find untainted samples.

Bad:    We used various methods to isolate four samples.
Better: We isolated four samples.
```

#### Beholder words

These words weaken the strength of your argument because you are making conclusions for your reader, and therefore insulting. Your readers should make the conclusion for themselves.

Furthermore, claims should always be backed by quantitative data whenever possible.

```txt
Bad:    False positives were surprisingly low. 
Better: To our surprise, false positives were low.
Good:   To our surprise, false positives were low (3%).
```

#### Lazy words

Common examples include `several`, `many`, `most`, `few` and `vast`. These examples are not ideal for technical writing because they are imprecise and not quantitative. They convey the impression that you the author have not done the work to quantitatively characterize your claim, and must be avoided.

```txt
Bad:    There is a very close match between the two semantics.
Better: There is a close match between the two semantics.
```

#### Adverbs

Adverbs are difficult to use in technical writing, and should be avoided whenever possible.

```txt
Bad:    We offer a completely different formulation of CFA.
Better: We offer a different formulation of CFA.
```

## Personal Thoughts

Technical writing is difficult.

## References

This project was inspired by this article from Matt Might, <https://matt.might.net/articles/shell-scripts-for-passive-voice-weasel-words-duplicates/> and this emacs project <https://github.com/bnbeckwith/writegood-mode>.

Notably, most of the examples were referenced from the blog post.