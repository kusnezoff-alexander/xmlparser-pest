Implementation of a [XML 1.0](https://www.w3.org/TR/REC-xml/#NT-XMLDecl)-parser using [pest](https://github.com/pest-parser/pest).

# ⛳ Roadmap

1. Syntax Parser
- [ ] write the grammar file(s)
    - [ ] Test: `ExternalID` (everything under ###External Entities+Expl in #XML Overview)

- [ ] add data structures
- [ ] (add methods for manipulating data structures & writing the data back )

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
- [`PI`s](https://www.w3.org/TR/REC-xml/#sec-pi) (Processing Instructions)
- `XMLDecl` (`EncodingDecl`,`VersionInfo`,`SDDecl`:standalone decl)
- [`TextDecl`](https://www.w3.org/TR/REC-xml/#NT-TextDecl) (for external parsed entities)

**Character Data** = document's <u>content</u>
- [`CDATA`](https://www.w3.org/TR/REC-xml/#sec-cdata-sect) = for escaping blocks of texts containing *markup*-strings

## Entities

=storage units
- *Document Entity* = starting point for XML processor; contained in <u>every</u> XML document
- *Parsed Entity* = contents="replacement text"
- *Unparsed Entity* = may (*or may not*) be text and if text may be other than XML (eg gif)
   - ->no restrictions on contents of *unparsed entities*!
- *Parameter Entities* = for use within DTD

### External Entities

*System Identifier* (`SystemLiteral`) = mostly a URI reference
- <u>no</u> fragment identifier (`#`) within URI!!
- relative URI = relative to XML-doc in which this *External Entity* is defined?? (unsure, see Spec [4.2.2](https://www.w3.org/TR/REC-xml/#sec-external-ent))

Example
```
<!ENTITY open-hatch
         SYSTEM "http://www.textuality.com/boilerplate/OpenHatch.xml">
<!ENTITY open-hatch
         PUBLIC "-//Textuality//TEXT Standard open-hatch boilerplate//EN"
         "http://www.textuality.com/boilerplate/OpenHatch.xml">
<!ENTITY hatch-pic
         SYSTEM "../grafix/OpenHatch.gif"
         NDATA gif >
```
- taken from [4.2.2 External Entities](https://www.w3.org/TR/REC-xml/#sec-external-ent)
