Implementation of a [XML 1.0](https://www.w3.org/TR/REC-xml/#NT-XMLDecl)-parser using [pest](https://github.com/pest-parser/pest).

# ⛳ Roadmap

1. Syntax Parser
- [ ] write the grammar files

- [ ] add data structures
( - [ ] add methods for manipulating data structures & writing the data back )

# 🎓 Personal Learnings

- Markup- and Programming-Languages are just a sequence of *strings/bytes* in the end
- Specification != Implementation is possible
- writing Grammar-Rules in [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar)
- Unicode-Encoding

# XML Overview

->see [XML 1.0 Spec: Character Data and Markup](https://www.w3.org/TR/REC-xml/#syntax)

**Markup** = Define document's <u>structure</u> (or contain *metadata*)
- `STag`(start-tag),`ETag`(end-tag),`EmptyElemTag`
- `EntityRef`,`CharRef`
- `Comment`
- `CDStart`,`CDEnd`
- `doctypedecl` (DTD)
- `PI`s(Processing Instructions)
- `XMLDecl` (`EncodingDecl`,`VersionInfo`,`SDDecl`:standalone decl)
- [`TextDecl`](https://www.w3.org/TR/REC-xml/#NT-TextDecl) (for external parsed entities)

**Character Data** = document's <u>content</u>
