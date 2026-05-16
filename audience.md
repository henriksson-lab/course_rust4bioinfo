# Audience

Target: 4th-year biology students who have completed one introductory programming course in R, Python, or (worst case) MATLAB.

## What we can expect them to know

### Biology / domain
- Solid grounding in molecular biology, genetics, and cell biology
- Familiarity with common data types: DNA/RNA/protein sequences, gene expression tables, phylogenies, alignments
- Awareness of common file formats by name (FASTA, FASTQ, VCF, BAM, GFF) even if they haven't parsed them
- Basic statistics: mean, variance, t-test, p-values, simple linear models
- Experience reading scientific papers and following methods sections

### Programming (basic)
- Variables, assignment, basic arithmetic
- Built-in numeric, string, and boolean types
- `if`/`else` conditionals
- `for` and `while` loops
- Writing and calling simple functions
- Using arrays/lists/vectors and indexing into them (1-based in R/MATLAB, 0-based in Python)
- Reading a CSV/TSV into a data frame and doing simple filtering/plotting
- Running a script from RStudio, a Jupyter notebook, or the MATLAB IDE
- Installing packages via `install.packages()`, `pip`, or the MATLAB add-on browser

## What we should NOT assume they know

### Programming concepts
- Static typing, type annotations, generics
- Compilation vs. interpretation; what a binary is
- Stack vs. heap, references, pointers, ownership, borrowing, lifetimes
- Manual or scope-based memory management (they've only known garbage collection)
- Pass-by-value vs. pass-by-reference semantics
- Mutability as a distinct concept from assignment
- Algebraic data types (enums with data, pattern matching, `Option`/`Result`)
- Traits, interfaces, polymorphism beyond duck typing
- Iterators as first-class objects (vs. just `for x in list`)
- Closures, higher-order functions (beyond `lapply`/`map` if even that)
- Error handling beyond `try`/`except`; no exposure to `Result`-style returns
- Concurrency, threads, async, data races
- Unit testing, integration testing, CI

### Tooling
- The command line beyond `cd`, `ls`, maybe `python script.py`
- Environment variables, `PATH`, shell configuration
- Build systems, dependency resolution, lockfiles
- `cargo`, `make`, or any non-language-bundled tool
- Compilers and linkers; reading compiler error messages
- Git beyond `git clone` and possibly `git pull`
- Branching, merging, pull requests
- Editors/IDEs beyond RStudio or Jupyter; no exposure to LSP, debuggers, profilers
- SSH, remote servers, HPC schedulers (SLURM etc.) — some may have brushed this in a lab

### Software engineering
- Code organization across multiple files / modules / crates
- Semantic versioning
- API design, separating library code from scripts
- Performance reasoning: algorithmic complexity, cache behavior, allocations
- Reading and writing documentation (docstrings, doc comments)
- Reproducibility practices (locked environments, containers)

## Pedagogical implications
- Lead every new concept with a concrete bioinformatics example, not an abstract one
- Compare Rust constructs to R/Python/MATLAB equivalents whenever possible
- Treat the borrow checker as the single biggest cognitive hurdle and budget time accordingly
- Spend real time on the toolchain (`rustup`, `cargo`, the terminal) before writing much code
- Defer advanced topics (traits, generics, async, unsafe) until the basics are firm
- Assume zero familiarity with compiler errors — explicitly teach how to read them
